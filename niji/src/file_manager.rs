use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	fs::{self, File},
	hash::{Hash, Hasher},
	io::{self, Read},
	path::{Path, PathBuf},
	rc::Rc
};
use thiserror::Error;

use crate::{console, files::Files};

#[derive(Debug, Error)]
pub enum Error {
	#[error("Error while accessing managed_files.csv: {0}")]
	CsvAccess(#[from] csv::Error),

	#[error("Failed to write to {0}: {1}")]
	Write(String, io::Error),

	#[error("An IO error occurred: {0}")]
	Io(io::Error),

	#[error("Writing to {0} was cancelled by the user")]
	CancelledByUser(String)
}

pub struct FileManager {
	files: Rc<Files>
}

impl FileManager {
	pub fn new(files: Rc<Files>) -> Result<Self, Error> {
		if !files.managed_files_file().exists() {
			fs::write(files.managed_files_file(), "").map_err(|e| {
				Error::Write(files.managed_files_file().to_string_lossy().into_owned(), e)
			})?;
		}

		Ok(Self { files })
	}

	pub fn manage(&self, path: &Path) -> Result<(), Error> {
		let mut managed_files = self.managed_files()?;

		if !path.exists() {
			console::debug!("Creating new managed file at {}", path.display());
			self.init_new_file(&mut managed_files, path)
		} else {
			self.manage_existing_file(&mut managed_files, path)
		}
	}

	fn init_new_file(
		&self,
		managed_files: &mut HashMap<PathBuf, u64>,
		path: &Path
	) -> Result<(), Error> {
		fs::write(path, "").map_err(|e| Error::Write(path.to_string_lossy().into_owned(), e))?;
		self.set_managed(managed_files, path.to_path_buf())?;

		console::info!("niji now manages {}", path.display());

		Ok(())
	}

	fn manage_existing_file(
		&self,
		managed_files: &mut HashMap<PathBuf, u64>,
		path: &Path
	) -> Result<(), Error> {
		if self.is_managed(managed_files, path)? {
			console::debug!("Writing to managed file at {}", path.display());
			return Ok(());
		}

		self.backup_and_replace(managed_files, path)
	}

	fn backup_and_replace(
		&self,
		managed_files: &mut HashMap<PathBuf, u64>,
		path: &Path
	) -> Result<(), Error> {
		let backup_path = Self::get_backup_path(path);

		console::warn!(
			"In order to apply your configuration, niji needs to write to {}. This would \
			 overwrite a previous version of that file that is not managed by niji. You can \
			 choose to let niji overwrite the file, or cancel the process. If you overwrite the \
			 file, the previous version will be backed up to {}.",
			path.display(),
			backup_path.display()
		);
		if !console::prompt!(default = false, "Backup and overwrite {}?", path.display()) {
			return Err(Error::CancelledByUser(path.to_string_lossy().into_owned()));
		}

		fs::copy(path, &backup_path)
			.map_err(|e| Error::Write(backup_path.to_string_lossy().into_owned(), e))?;

		self.init_new_file(managed_files, path)?;

		console::info!("Backup created at {}", backup_path.display());

		Ok(())
	}

	fn get_backup_path(path: &Path) -> PathBuf {
		let date = chrono::offset::Local::now().date_naive();
		let file_name = format!(
			"{}.backup-{date}",
			path.file_name().unwrap().to_string_lossy()
		);

		return path.parent().unwrap().join(file_name);
	}

	fn set_managed(
		&self,
		managed_files: &mut HashMap<PathBuf, u64>,
		path: PathBuf
	) -> Result<(), Error> {
		let path = path.canonicalize().map_err(Error::Io)?;

		managed_files.insert(path.clone(), Self::hash_contents(&path)?);
		self.write_managed_files(managed_files)
	}

	fn is_managed(
		&self,
		managed_files: &HashMap<PathBuf, u64>,
		path: &Path
	) -> Result<bool, Error> {
		let path = path.canonicalize().map_err(Error::Io)?;

		if let Some(known_hash) = managed_files.get(&path) {
			let current_hash = Self::hash_contents(&path)?;
			if current_hash == *known_hash {
				return Ok(true);
			}
		}

		Ok(false)
	}

	fn hash_contents(path: &Path) -> Result<u64, Error> {
		let file = File::open(path).map_err(Error::Io)?;
		let mut hasher = DefaultHasher::new();
		for byte in file.bytes() {
			byte.map_err(Error::Io)?.hash(&mut hasher);
		}
		Ok(hasher.finish())
	}

	fn managed_files(&self) -> Result<HashMap<PathBuf, u64>, Error> {
		let mut managed_files = HashMap::new();

		let mut reader = csv::ReaderBuilder::new()
			.has_headers(false)
			.from_path(self.files.managed_files_file())
			.map_err(Error::CsvAccess)?;

		for result in reader.deserialize::<(PathBuf, u64)>() {
			let (path, hash) = result?;
			managed_files.insert(path, hash);
		}
		Ok(managed_files)
	}

	fn write_managed_files(&self, managed_files: &HashMap<PathBuf, u64>) -> Result<(), Error> {
		let mut writer =
			csv::Writer::from_path(self.files.managed_files_file()).map_err(Error::CsvAccess)?;
		for (path, hash) in managed_files.iter() {
			writer.serialize((path, hash))?;
		}

		Ok(())
	}
}

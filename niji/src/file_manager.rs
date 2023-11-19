use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	fs::{self, File, OpenOptions},
	hash::{Hash, Hasher},
	io::{self, Read},
	path::{Path, PathBuf}
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
	managed_files_file: PathBuf,
	managed_files: HashMap<PathBuf, u64>
}

impl FileManager {
	pub fn new(files: &Files) -> Result<Self, Error> {
		if !files.managed_files_file().exists() {
			fs::write(files.managed_files_file(), "").map_err(|e| {
				Error::Write(files.managed_files_file().to_string_lossy().into_owned(), e)
			})?;
		}

		Ok(Self {
			managed_files_file: files.managed_files_file().to_path_buf(),
			managed_files: HashMap::new()
		})
	}

	pub fn manage(&mut self, path: &Path) -> Result<(), Error> {
		if !path.exists() {
			console::debug!("Creating new managed file at {}", path.display());
			self.init_new_file(path)
		} else {
			self.manage_existing_file(path)
		}
	}

	fn init_new_file(&mut self, path: &Path) -> Result<(), Error> {
		fs::write(path, "").map_err(|e| Error::Write(path.to_string_lossy().into_owned(), e))?;
		self.set_managed(path.to_path_buf())?;

		console::info!("niji now manages {}", path.display());

		Ok(())
	}

	fn manage_existing_file(&mut self, path: &Path) -> Result<(), Error> {
		if self.is_managed(path)? {
			console::debug!("Writing to managed file at {}", path.display());
			return Ok(());
		}

		self.backup_and_replace(path)
	}

	fn backup_and_replace(&mut self, path: &Path) -> Result<(), Error> {
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

		self.init_new_file(path)?;

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

	fn set_managed(&mut self, path: PathBuf) -> Result<(), Error> {
		let path = path.canonicalize().map_err(Error::Io)?;

		self.managed_files
			.insert(path.clone(), Self::hash_contents(&path)?);
		self.write_changes()
	}

	fn is_managed(&mut self, path: &Path) -> Result<bool, Error> {
		self.update()?;

		let path = path.canonicalize().map_err(Error::Io)?;

		if let Some(known_hash) = self.managed_files.get(&path) {
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

	fn update(&mut self) -> Result<(), Error> {
		self.managed_files.clear();

		let mut reader = csv::ReaderBuilder::new()
			.has_headers(false)
			.from_path(&self.managed_files_file)
			.map_err(Error::CsvAccess)?;

		for result in reader.deserialize::<(PathBuf, u64)>() {
			let (path, hash) = result?;
			self.managed_files.insert(path, hash);
		}
		Ok(())
	}

	fn write_changes(&self) -> Result<(), Error> {
		let mut writer =
			csv::Writer::from_path(&self.managed_files_file).map_err(Error::CsvAccess)?;
		for (path, hash) in self.managed_files.iter() {
			writer.serialize((path, hash))?;
		}

		Ok(())
	}
}

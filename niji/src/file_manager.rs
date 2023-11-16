use std::{
	collections::HashMap,
	ffi::OsString,
	fs::{self, File, OpenOptions},
	io,
	path::PathBuf
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::files::Files;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Error while reading managed_files.csv: {0}")]
	CsvRead(#[from] csv::Error),

	#[error("Failed to write to {0}: {1}")]
	Write(String, io::Error),

	#[error("An IO error occurred: {0}")]
	Io(io::Error),

	#[error("Writing to {0} was cancelled by the user")]
	CancelledByUser(String)
}

struct FileManager {
	managed_files_file: File,
	managed_files: HashMap<PathBuf, u64>
}

impl FileManager {
	pub fn new(files: &Files) -> Result<Self, Error> {
		if !files.managed_files_file().exists() {
			fs::write(files.managed_files_file(), "").map_err(|e| {
				Error::Write(files.managed_files_file().to_string_lossy().into_owned(), e)
			})?;
		}

		let mut options = OpenOptions::new();
		options.write(true);
		options.read(true);
		let managed_files_file = options.open(files.managed_files_file()).unwrap();

		Ok(Self {
			managed_files_file,
			managed_files: HashMap::new()
		})
	}

	pub fn manage(&mut self, path: PathBuf) -> Result<(), Error> {
		if !path.exists() {
			fs::write(path.clone(), "")
				.map_err(|e| Error::Write(path.to_string_lossy().into_owned(), e))?;
		}
		self.update()?;

		let Some(&hash) = self
			.managed_files
			.get(&path.canonicalize().map_err(Error::Io)?)
		else {
			todo!()
		};

		Ok(())
	}

	fn update(&mut self) -> Result<(), Error> {
		self.managed_files.clear();
		let mut reader = csv::Reader::from_reader(&self.managed_files_file);
		for result in reader.deserialize::<(PathBuf, u64)>() {
			let (path, hash) = result?;
			self.managed_files.insert(path, hash);
		}
		Ok(())
	}
}

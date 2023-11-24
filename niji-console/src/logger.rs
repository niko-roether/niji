use log::{Level, LevelFilter};

use crate::__private_api;

pub(crate) struct Logger {
	level: LevelFilter
}

impl Logger {
	pub fn new(level: LevelFilter) -> Self {
		Self { level }
	}
}

impl log::Log for Logger {
	fn enabled(&self, metadata: &log::Metadata) -> bool {
		self.level >= metadata.level()
	}

	fn log(&self, record: &log::Record) {
		match record.level() {
			Level::Error => __private_api::log_error(record.args()).unwrap(),
			Level::Warn => __private_api::log_warn(record.args()).unwrap(),
			Level::Info => __private_api::log_info(record.args()).unwrap(),
			Level::Debug => __private_api::log_debug(record.args()).unwrap(),
			Level::Trace => __private_api::log_trace(record.args()).unwrap()
		}
		self.flush();
	}

	fn flush(&self) {
		__private_api::flush().unwrap()
	}
}

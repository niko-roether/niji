use std::{fmt::Arguments, io};

use crate::console::Console;

static mut CONSOLE: Option<Console> = None;

pub(crate) fn set_console(console: Console) {
	unsafe { CONSOLE = Some(console) }
}

pub(crate) fn get_console() -> &'static Console {
	unsafe { CONSOLE.as_ref().expect("Console was not initialized") }
}

pub fn log_error(args: &Arguments) -> Result<(), io::Error> {
	get_console().log_error(args)
}

pub fn log_warn(args: &Arguments) -> Result<(), io::Error> {
	get_console().log_warn(args)
}

pub fn log_info(args: &Arguments) -> Result<(), io::Error> {
	get_console().log_info(args)
}

pub fn log_debug(args: &Arguments) -> Result<(), io::Error> {
	get_console().log_debug(args)
}

pub fn log_trace(args: &Arguments) -> Result<(), io::Error> {
	get_console().log_trace(args)
}

pub fn prompt(args: &Arguments, default: Option<bool>) -> Result<bool, io::Error> {
	get_console().prompt(args, default)
}

pub fn flush() -> Result<(), io::Error> {
	get_console().flush()
}

use std::{
	fmt,
	io::{self, Write},
	mem,
	sync::atomic::{AtomicBool, AtomicU8, Ordering}
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogLevel {
	Quiet = 0,
	Normal = 1,
	Verbose = 2
}

impl Default for LogLevel {
	#[inline]
	fn default() -> Self {
		Self::Normal
	}
}

static LOG_LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Normal as u8);
static COLOR: AtomicBool = AtomicBool::new(false);

#[inline]
pub fn set_log_level(level: LogLevel) {
	LOG_LEVEL.store(level as u8, Ordering::Release);
}

#[inline]
pub fn log_level() -> LogLevel {
	unsafe { mem::transmute(LOG_LEVEL.load(Ordering::Acquire)) }
}

#[inline]
pub fn set_color(color: bool) {
	COLOR.store(color, Ordering::Release);
}

#[inline]
pub fn has_color() -> bool {
	COLOR.load(Ordering::Acquire)
}

struct LogColorScheme {
	normal: &'static str,
	bright: &'static str
}

const RED: LogColorScheme = LogColorScheme {
	normal: "\x1b[31m",
	bright: "\x1b[91m"
};

const YELLOW: LogColorScheme = LogColorScheme {
	normal: "\x1b[33m",
	bright: "\x1b[93m"
};

const GRAY: LogColorScheme = LogColorScheme {
	normal: "\x1b[37m",
	bright: "\x1b[97m"
};

const WHITE: LogColorScheme = LogColorScheme {
	normal: "\x1b[97m",
	bright: "\x1b[97m"
};

fn log(
	args: fmt::Arguments,
	source: Option<&str>,
	level: Option<&str>,
	LogColorScheme { normal, bright }: LogColorScheme,
	newline: bool
) {
	let mut message = String::new();

	if has_color() {
		if let Some(source) = source {
			message.push_str(&format!(
				"\x1b[90m\x1b[1m[{normal}{source}\x1b[90m]\x1b[0m "
			));
		}
		if let Some(level) = level {
			message.push_str(&format!("{bright}\x1b[1m{level}\x1b[0m{normal}:\x1b[0m "));
		}
		message.push_str(&format!("{normal}{args}\x1b[0m"));
	} else {
		if let Some(source) = source {
			message.push_str(&format!("[{source}] "));
		}
		if let Some(level) = level {
			message.push_str(&format!("{level}: "));
		}
		message.push_str(&format!("{args}"));
	}
	if newline {
		message.push('\n')
	}

	print!("{message}");
}

#[inline]
pub fn __error(args: fmt::Arguments, source: Option<&str>) {
	if log_level() >= LogLevel::Normal {
		log(args, source, Some("ERROR"), RED, true);
	}
}

#[inline]
pub fn __warn(args: fmt::Arguments, source: Option<&str>) {
	if log_level() >= LogLevel::Normal {
		log(args, source, Some("WARNING"), YELLOW, true);
	}
}

#[inline]
pub fn __info(args: fmt::Arguments, source: Option<&str>) {
	if log_level() >= LogLevel::Normal {
		log(args, source, None, WHITE, true);
	}
}

#[inline]
pub fn __debug(args: fmt::Arguments, source: Option<&str>) {
	if log_level() >= LogLevel::Verbose {
		log(args, source, Some("DEBUG"), GRAY, true);
	}
}

#[inline]
pub fn __prompt(args: fmt::Arguments, default: Option<bool>, source: Option<&str>) -> bool {
	let prompt = match (has_color(), default) {
		(true, Some(true)) => format!("{args} \x1b[1m[Y/n]\x1b[0m\x1b[90m:\x1b[0m "),
		(false, Some(true)) => format!("{args} [Y/n]: "),
		(true, Some(false)) => format!("{args} \x1b[1m[y/N]\x1b[0m\x1b[90m:\x1b[0m "),
		(false, Some(false)) => format!("{args} [y/N]: "),
		(true, None) => format!("{args} \x1b[1m[y/n]\x1b[0m\x1b[90m:\x1b[0m "),
		(false, None) => format!("{args} [y/n]: ")
	};

	loop {
		log(format_args!("{prompt}"), source, None, WHITE, false);
		let mut input = String::new();
		io::stdout().flush().unwrap();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read from stdin");

		input = input.trim().to_string();

		if input.is_empty() {
			if let Some(default) = default {
				return default;
			}
		}
		match input.as_str() {
			"y" | "Y" => return true,
			"n" | "N" => return false,
			_ => ()
		}
	}
}

macro_rules! def_log_macro {
	($name:ident, $impl:ident) => {
		macro_rules! $name {
			(source = $$source:expr, $$($$arg:tt)*) => {
                crate::console::$impl(format_args!($$($$arg)*), Some($$source))
            };
			($$($$arg:tt)*) => {
                crate::console::$impl(format_args!($$($$arg)*), None)
            };
		}
	};
}

def_log_macro!(error, __error);
#[allow(clippy::single_component_path_imports)]
pub(crate) use error;

def_log_macro!(warn2, __warn);
#[allow(clippy::single_component_path_imports)]
pub(crate) use warn2 as warn;

def_log_macro!(info, __info);
#[allow(clippy::single_component_path_imports)]
pub(crate) use info;

def_log_macro!(debug, __debug);
#[allow(clippy::single_component_path_imports)]
pub(crate) use debug;

macro_rules! prompt {
    (source = $source:expr, default = $default:expr, $($arg:tt)*) => {
        crate::console::__prompt(format_args!($($arg)*), Some($default), Some($source))
    };
    (default = $default:expr, source = $source:expr, $($arg:tt)*) => {
        crate::console::__prompt(format_args!($($arg)*), Some($default), Some($source))
    };
    (source = $source:expr, $($arg:tt)*) => {
        crate::console::__prompt(format_args!($($arg)*), None, Some($source))
    };
    (default = $default:expr, $($arg:tt)*) => {
        crate::console::__prompt(format_args!($($arg)*), Some($default), None)
    };
    ($($arg:tt)*) => {
        crate::console::__prompt(format_args!($($arg)*), None, None)
    }
}

#[allow(clippy::single_component_path_imports)]
pub(crate) use prompt;

use std::{
	fmt::Arguments,
	io::{self, Write}
};

use log::LevelFilter;
use termcolor::{BufferedStandardStream, Color, ColorChoice, ColorSpec, WriteColor};

pub struct Console {
	level: LevelFilter,
	stdout: BufferedStandardStream,
	stderr: BufferedStandardStream
}

impl Console {
	pub fn new(level: LevelFilter, color_choice: ColorChoice) -> Self {
		let stdout = BufferedStandardStream::stdout(color_choice);
		let stderr = BufferedStandardStream::stderr(color_choice);

		Self {
			level,
			stdout,
			stderr
		}
	}

	pub fn log_error(&mut self, args: &Arguments) -> io::Result<()> {
		self.stdout
			.set_color(
				ColorSpec::new()
					.set_fg(Some(Color::Red))
					.set_intense(true)
					.set_bold(true)
			)
			.unwrap();

		write!(&mut self.stderr, "ERROR")?;

		self.stderr
			.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
			.unwrap();

		writeln!(&mut self.stderr, ": {args}")?;

		self.stderr.reset().unwrap();
		Ok(())
	}

	pub fn log_warn(&mut self, args: &Arguments) -> io::Result<()> {
		self.stdout
			.set_color(
				ColorSpec::new()
					.set_fg(Some(Color::Yellow))
					.set_intense(true)
					.set_bold(true)
			)
			.unwrap();

		write!(&mut self.stderr, "WARNING")?;

		self.stderr
			.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
			.unwrap();

		writeln!(&mut self.stderr, ": {args}")?;

		self.stderr.reset().unwrap();
		Ok(())
	}

	pub fn log_info(&mut self, args: &Arguments) -> io::Result<()> {
		self.stdout
			.set_color(
				ColorSpec::new()
					.set_fg(Some(Color::Blue))
					.set_intense(true)
					.set_bold(true)
			)
			.unwrap();

		write!(&mut self.stdout, "INFO")?;

		self.stdout
			.set_color(
				ColorSpec::new()
					.set_fg(Some(Color::White))
					.set_intense(true)
			)
			.unwrap();

		writeln!(&mut self.stdout, ": {args}")?;

		self.stdout.reset().unwrap();
		Ok(())
	}

	pub fn log_debug(&mut self, args: &Arguments) -> io::Result<()> {
		self.stdout
			.set_color(
				ColorSpec::new()
					.set_fg(Some(Color::White))
					.set_intense(true)
					.set_bold(true)
			)
			.unwrap();

		write!(&mut self.stdout, "DEBUG")?;

		self.stdout
			.set_color(ColorSpec::new().set_fg(Some(Color::White)))
			.unwrap();

		writeln!(&mut self.stdout, ": {args}")?;

		self.stdout.reset().unwrap();
		Ok(())
	}

	pub fn log_trace(&mut self, args: &Arguments) -> io::Result<()> {
		self.stdout
			.set_color(
				ColorSpec::new()
					.set_fg(Some(Color::White))
					.set_intense(true)
					.set_bold(true)
			)
			.unwrap();

		write!(&mut self.stdout, "TRACE")?;

		self.stdout
			.set_color(ColorSpec::new().set_fg(Some(Color::White)))
			.unwrap();

		writeln!(&mut self.stdout, ": {args}")?;

		self.stdout.reset().unwrap();
		Ok(())
	}

	pub fn prompt(&mut self, args: &Arguments, default: Option<bool>) -> io::Result<bool> {
		loop {
			self.stdout
				.set_color(
					ColorSpec::new()
						.set_fg(Some(Color::White))
						.set_intense(true)
				)
				.unwrap();

			write!(&mut self.stdout, "{args} ")?;

			self.stdout
				.set_color(
					ColorSpec::new()
						.set_fg(Some(Color::Blue))
						.set_intense(true)
						.set_bold(true)
				)
				.unwrap();

			match default {
				Some(true) => write!(&mut self.stdout, "[Y/n]")?,
				Some(false) => write!(&mut self.stdout, "[y/N]")?,
				None => write!(&mut self.stdout, "[y/n]")?
			};

			self.stdout
				.set_color(
					ColorSpec::new()
						.set_fg(Some(Color::White))
						.set_intense(true)
				)
				.unwrap();

			write!(&mut self.stdout, ": ")?;
			self.stdout.flush()?;

			let mut response = String::new();
			io::stdin().read_line(&mut response)?;

			response = response.trim().to_string().to_lowercase();

			match response.as_str() {
				"y" => return Ok(true),
				"n" => return Ok(false),
				"" => {
					if let Some(default) = default {
						return Ok(default);
					}
				}
				_ => ()
			}

			self.stdout.reset().unwrap();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fn() {
		let mut console = Console::new(LevelFilter::Info, ColorChoice::Auto);
		console.prompt(&format_args!("Test"), None).unwrap();
	}
}

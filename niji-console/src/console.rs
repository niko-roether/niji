use std::{
	fmt::Arguments,
	io::{self, Write},
	sync::Mutex
};

use termcolor::{BufferedStandardStream, Color, ColorChoice, ColorSpec, WriteColor};

pub struct Console {
	stdout: Mutex<BufferedStandardStream>,
	stderr: Mutex<BufferedStandardStream>
}

impl Console {
	pub fn new(color_choice: ColorChoice) -> Self {
		let stdout = Mutex::new(BufferedStandardStream::stdout(color_choice));
		let stderr = Mutex::new(BufferedStandardStream::stderr(color_choice));

		Self { stdout, stderr }
	}

	pub fn log_error(&self, args: &Arguments) -> io::Result<()> {
		Self::log(
			&mut self.stderr.lock().unwrap(),
			"ERROR",
			ColorSpec::new()
				.set_fg(Some(Color::Red))
				.set_intense(true)
				.set_bold(true),
			args,
			ColorSpec::new().set_fg(Some(Color::Red))
		)
	}

	pub fn log_warn(&self, args: &Arguments) -> io::Result<()> {
		Self::log(
			&mut self.stdout.lock().unwrap(),
			"WARN ",
			ColorSpec::new()
				.set_fg(Some(Color::Yellow))
				.set_intense(true)
				.set_bold(true),
			args,
			ColorSpec::new().set_fg(Some(Color::Yellow))
		)
	}

	pub fn log_info(&self, args: &Arguments) -> io::Result<()> {
		Self::log(
			&mut self.stdout.lock().unwrap(),
			"INFO ",
			ColorSpec::new()
				.set_fg(Some(Color::Blue))
				.set_intense(true)
				.set_bold(true),
			args,
			ColorSpec::new()
				.set_fg(Some(Color::White))
				.set_intense(true)
		)
	}

	pub fn log_debug(&self, args: &Arguments) -> io::Result<()> {
		Self::log(
			&mut self.stdout.lock().unwrap(),
			"DEBUG",
			ColorSpec::new().set_fg(Some(Color::White)),
			args,
			ColorSpec::new().set_fg(Some(Color::White))
		)
	}

	pub fn log_trace(&self, args: &Arguments) -> io::Result<()> {
		Self::log(
			&mut self.stdout.lock().unwrap(),
			"TRACE",
			ColorSpec::new().set_fg(Some(Color::White)),
			args,
			ColorSpec::new().set_fg(Some(Color::White))
		)
	}

	fn log(
		out: &mut BufferedStandardStream,
		tag: &str,
		tag_color: &ColorSpec,
		message: &Arguments,
		message_color: &ColorSpec
	) -> io::Result<()> {
		out.set_color(tag_color).unwrap();

		write!(out, "{tag}")?;

		out.set_color(
			ColorSpec::new()
				.set_fg(Some(Color::Black))
				.set_intense(true)
		)
		.unwrap();

		write!(out, " - ")?;

		out.set_color(message_color).unwrap();

		writeln!(out, "{message}")?;

		out.reset().unwrap();
		Ok(())
	}

	pub fn prompt(&self, args: &Arguments, default: Option<bool>) -> io::Result<bool> {
		let stdout = &mut self.stdout.lock().unwrap();

		loop {
			stdout
				.set_color(
					ColorSpec::new()
						.set_fg(Some(Color::White))
						.set_intense(true)
				)
				.unwrap();

			write!(stdout, "{args} ")?;

			stdout
				.set_color(
					ColorSpec::new()
						.set_fg(Some(Color::Blue))
						.set_intense(true)
						.set_bold(true)
				)
				.unwrap();

			match default {
				Some(true) => write!(stdout, "[Y/n]")?,
				Some(false) => write!(stdout, "[y/N]")?,
				None => write!(stdout, "[y/n]")?
			};

			stdout
				.set_color(
					ColorSpec::new()
						.set_fg(Some(Color::White))
						.set_intense(true)
				)
				.unwrap();

			write!(stdout, ": ")?;
			stdout.flush()?;

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

			stdout.reset().unwrap();
		}
	}

	pub fn flush(&self) -> Result<(), io::Error> {
		let stdout = &mut self.stdout.lock().unwrap();
		let stderr = &mut self.stderr.lock().unwrap();

		stdout.flush()?;
		stderr.flush()?;
		Ok(())
	}
}

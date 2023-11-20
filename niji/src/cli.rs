use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{
	app::NijiApp,
	console::{self, LogLevel}
};

const AUTHOR: &str = "Nicholas Roether <nicholas.roether@t-online.de>";

macro_rules! handle {
	($expr:expr, $cleanup:expr) => {
		match $expr {
			Ok(val) => val,
			Err(err) => {
				crate::console::error!("{err}");

				#[allow(clippy::redundant_closure_call)]
				$cleanup();

				return;
			}
		}
	};
	($expr:expr) => {
		handle!($expr, || ())
	};
}

pub fn run() {
	let matches = Command::new("niji")
		.author(AUTHOR)
		.about("A configurable desktop theming utility")
		.version(env!("CARGO_PKG_VERSION"))
		.subcommand_required(true)
		.arg_required_else_help(true)
		.arg(
			Arg::new("quiet")
				.long("quiet")
				.short('q')
				.action(ArgAction::SetTrue)
				.conflicts_with("verbose")
				.help("Disables all log messages")
		)
		.arg(
			Arg::new("verbose")
				.long("verbose")
				.short('v')
				.action(ArgAction::SetTrue)
				.conflicts_with("quiet")
				.help("Prints additional debug output")
		)
		.arg(
			Arg::new("color")
				.long("no-color")
				.short('b')
				.action(ArgAction::SetFalse)
				.help("Disable color output")
		)
		.subcommand(
			Command::new("apply")
				.about("Apply (or re-apply) the current theme and and configuration")
		)
		.subcommand(
			Command::new("theme")
				.about(
					"Perform actions related to themes, such as changing the theme or listing \
					 available themes"
				)
				.subcommand_required(true)
				.subcommand(Command::new("get").about("Get the name of the current theme"))
				.subcommand(
					Command::new("set")
						.about("Change the current theme")
						.arg_required_else_help(true)
						.arg(Arg::new("name").help("The name of the theme to change to"))
						.arg(
							Arg::new("apply")
								.long("no-apply")
								.short('N')
								.action(ArgAction::SetFalse)
								.help("Don't apply the theme after setting it")
						)
				)
				.subcommand(Command::new("list").about("List the names of available themes"))
				.subcommand(Command::new("reset").about(
					"Reset the current theme. Note that this will not make any changes to the \
					 emitted files!"
				))
		)
		.get_matches();

	let app = handle!(NijiApp::init());

	cmd(&app, &matches)
}

fn cmd(app: &NijiApp, args: &ArgMatches) {
	let quiet = *args.get_one::<bool>("quiet").unwrap();
	let verbose = *args.get_one::<bool>("verbose").unwrap();
	let color = *args.get_one::<bool>("color").unwrap();

	if quiet {
		console::set_log_level(LogLevel::Quiet);
	} else if verbose {
		console::set_log_level(LogLevel::Verbose)
	}

	console::set_color(color);

	match args.subcommand() {
		Some(("apply", _)) => cmd_apply(app),
		Some(("theme", args)) => cmd_theme(app, args),
		_ => unreachable!()
	}
}

fn cmd_apply(app: &NijiApp) {
	handle!(app.apply())
}

fn cmd_theme(app: &NijiApp, args: &ArgMatches) {
	match args.subcommand() {
		Some(("get", _)) => cmd_theme_get(app),
		Some(("set", args)) => cmd_theme_set(app, args),
		Some(("list", _)) => cmd_theme_list(app),
		Some(("reset", _)) => cmd_theme_reset(app),
		_ => unreachable!()
	}
}

fn cmd_theme_get(app: &NijiApp) {
	let theme = handle!(app.current_theme());

	match theme {
		Some(theme) => println!("{}", theme.name),
		None => console::error!("No theme selected")
	}
}

fn cmd_theme_set(app: &NijiApp, args: &ArgMatches) {
	let name = args.get_one::<String>("name").unwrap().as_str();
	let apply = *args.get_one::<bool>("apply").unwrap();

	let prev_theme = handle!(app.current_theme());

	handle!(app.set_theme(name));
	if apply {
		handle!(app.apply_theme(), || {
			console::debug!("Falling back to previously set theme");
			match prev_theme {
				Some(theme) => handle!(app.set_theme(&theme.name)),
				None => handle!(app.reset_theme())
			}
		});
	}
}

fn cmd_theme_list(app: &NijiApp) {
	let mut empty = true;

	for theme in app.list_themes() {
		empty = false;
		println!("{theme}")
	}

	if empty {
		console::error!("No usable themes were found");
	}
}

fn cmd_theme_reset(app: &NijiApp) {
	handle!(app.reset_theme())
}

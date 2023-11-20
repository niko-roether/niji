use clap::{Arg, ArgAction, Command, Subcommand};

const AUTHOR: &str = "Nicholas Roether <nicholas.roether@t-online.de>";

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
				.default_value("false")
				.action(ArgAction::SetTrue)
				.conflicts_with("verbose")
				.help("Disables all console output")
		)
		.arg(
			Arg::new("verbose")
				.long("verbose")
				.short('v')
				.default_value("false")
				.action(ArgAction::SetTrue)
				.conflicts_with("quiet")
				.help("Prints additional debug output")
		)
		.subcommand(
			Command::new("apply")
				.about("Apply (or re-apply) the current theme and/or and configuration")
		)
		.get_matches();
}

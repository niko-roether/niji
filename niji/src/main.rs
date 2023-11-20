#![feature(macro_metavar_expr)]

use app::NijiApp;

mod app;
mod cli;
mod config;
mod console;
mod file_manager;
mod files;
mod lua;
mod module;
mod module_manager;
mod template;
mod theme_manager;
mod types;
mod utils;

fn main() {
	console::set_color(true);

	let app = match NijiApp::init() {
		Ok(app) => app,
		Err(err) => {
			console::error!("{err}");
			return;
		}
	};

	cli::run();
}

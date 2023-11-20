#![feature(macro_metavar_expr)]

use std::{path::PathBuf, rc::Rc};

use app::NijiApp;
use config::{ColorScheme, GeneralConfig, ModuleConfig, Palette, Theme, UiTheme};
use file_manager::FileManager;
use files::Files;
use lua::runtime::{LuaRuntime, LuaRuntimeInit};
use module::Module;
use types::color::Color;
use utils::xdg::XdgDirs;

mod app;
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

	console::info!(
		"{}",
		match app.current_theme() {
			Ok(theme) => theme
				.map(|t| t.name)
				.unwrap_or("<no theme set>".to_string()),
			Err(err) => {
				console::error!("{err}");
				return;
			}
		}
	)
}

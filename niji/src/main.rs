#![feature(macro_metavar_expr)]

use std::path::PathBuf;

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

	let xdg = XdgDirs::new().unwrap();
	let files = Files::init(&xdg).unwrap();
	let file_manager = FileManager::new(&files).unwrap();

	let _config = GeneralConfig {
		icons: "Abc".to_string(),
		cursor: "Cde".to_string(),
		cursor_size: 69,
		font_family: "Comic Sans".to_string(),
		font_size: 420
	};

	let lua = LuaRuntime::new(LuaRuntimeInit { xdg, file_manager }).unwrap();

	let theme = Theme {
		ui: UiTheme {
			background: Color::default(),
			border: Color::default(),
			color_scheme: ColorScheme::Dark,
			error: Color::default(),
			info: Color::default(),
			primary: Color::default(),
			surface: Color::default(),
			text_background: Color::default(),
			text_primary: Color::default(),
			text_surface: Color::default(),
			warning: Color::default()
		},
		palette: Palette {
			black: Color::default(),
			bright_red: Color::default(),
			bright_blue: Color::default(),
			bright_green: Color::default(),
			dark_blue: Color::default(),
			cyan: Color::default(),
			dark_red: Color::default(),
			dark_gray: Color::default(),
			dark_green: Color::default(),
			light_gray: Color::default(),
			magenta: Color::default(),
			orange: Color::default(),
			purple: Color::default(),
			turquoise: Color::default(),
			white: Color::default(),
			yellow: Color::default()
		}
	};

	let module = match Module::load(
		&lua,
		&PathBuf::from("/home/niko/.config/niji/modules/test"),
		ModuleConfig::Int(69)
	) {
		Ok(m) => m,
		Err(err) => {
			console::error!("{err}");
			return;
		}
	};

	if let Err(err) = module.apply(&theme) {
		console::error!("{err}");
	};
}

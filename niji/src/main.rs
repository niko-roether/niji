use std::path::PathBuf;

use config::{ColorScheme, Config, Palette, Theme, UiTheme};
use lua::{
	module::Module,
	runtime::{LuaRuntime, LuaRuntimeInit}
};
use types::color::Color;
use utils::xdg::XdgDirs;

mod config;
mod files;
mod lua;
mod types;
mod utils;

fn main() {
	let xdg = XdgDirs::new().unwrap();

	let _config = Config {
		icons: "Abc".to_string(),
		cursor: "Cde".to_string(),
		cursor_size: 69,
		font_family: "Comic Sans".to_string(),
		font_size: 420
	};

	let lua = LuaRuntime::new(LuaRuntimeInit { xdg }).unwrap();

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

	let module =
		Module::load(&lua, &PathBuf::from("/home/niko/.config/niji/modules/test")).unwrap();

	module.apply(&theme).unwrap();
}

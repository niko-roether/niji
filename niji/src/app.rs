use std::fs;

use crate::{
	files::Files, module_manager::ModuleManager, theme_manager::ThemeManager, utils::xdg::XdgDirs
};

pub struct NijiApp {
	theme_manager: ThemeManager,
	module_manager: ModuleManager
}

macro_rules! err {
	($expr:expr) => {
		match $expr {
			Ok(value) => value,
			Err(error) => {
				crate::console::error!("{error}");
				return Err(());
			}
		}
	};
}

impl NijiApp {
	pub fn init() -> Result<Self, ()> {
		let xdg = err!(XdgDirs::new());
		let files = err!(Files::init(&xdg));

		todo!()
	}
}

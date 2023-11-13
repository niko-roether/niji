use crate::files::Files;

mod api;
mod color_math;
mod config;
mod files;
mod types;
mod utils;

fn main() {
	let files = Files::init().unwrap();

	dbg!(&files);

	for theme in files.iter_themes() {
		dbg!(theme);
	}

	for module in files.iter_modules() {
		dbg!(module);
	}
}

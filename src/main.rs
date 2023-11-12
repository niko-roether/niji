use crate::files::Files;

mod config;
mod files;
mod types;

fn main() {
	let files = Files::init().unwrap();

	dbg!(files);
}

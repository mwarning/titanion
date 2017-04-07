
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

use std::path::Path;
use std::env;

mod util {
	pub mod vector;
	pub mod actor;
	pub mod math;
	pub mod rand;
	pub mod sdl {
		pub mod displaylist;
		pub mod mainloop;
		pub mod input;
	}
}

mod ttn {
	pub mod shape;
	pub mod enemy;
	pub mod token;
	pub mod bullet;
	pub mod field;
	pub mod player;
	pub mod stage;
	pub mod pillar;
	pub mod frame;
	pub mod title;
	pub mod particle;
	pub mod letter;
	pub mod preference;
	pub mod boot;
	pub mod dummy; //skeleton dummies
}

fn main() {
	// Change working directory
	if let Some(arg0) = env::args().nth(0) {
		let path = Path::new(&arg0).parent().unwrap();
		env::set_current_dir(path);
	}

	let exit_code = boot();
	std::process::exit(exit_code);
}

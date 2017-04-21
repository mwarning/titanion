
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

mod util {
	pub mod vector;
	pub mod actor;
	pub mod math;
	pub mod rand;
	pub mod sdl {
		pub mod displaylist;
		pub mod recordableinput;
		pub mod mainloop;
		pub mod screen;
		pub mod sound;
		pub mod input;
		pub mod pad;
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
	pub mod screen;
	pub mod sound;
	pub mod dummy; //skeleton dummies
}

use std::path::Path;
use std::env;

use ttn::boot::*;


fn main() {
	// Change working directory
	if let Some(arg0) = env::args().nth(0) {
		let path = Path::new(&arg0).parent().unwrap();
		env::set_current_dir(path);
	}

	let exit_code = boot();
	std::process::exit(exit_code);
}


#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]


mod util {
	pub mod vector;
	pub mod actor;
	pub mod math;
	pub mod sdl {
		pub mod displaylist;
	}
}

mod ttn {
	pub mod shape;
	pub mod enemy;
	pub mod token;
	pub mod bullet;
	pub mod field;
	pub mod player;
	pub mod dummy; //skeleton dummies
}

fn main() {
	//call boot...
}
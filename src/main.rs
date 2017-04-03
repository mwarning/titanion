
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
	pub mod preference;
	pub mod dummy; //skeleton dummies
}

fn main() {
	//call boot...
}
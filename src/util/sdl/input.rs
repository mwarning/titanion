/*
 * Copyright 2004 Kenta Cho. Some rights reserved.
 */

use std::vec::Vec;
use ttn::dummy::*;

/**
 * Input device interface.
 */
pub trait Input {
  fn handleEvent(&mut self, event : &'static SDL_Event);
}

struct MultipleInputDevice<'a> {
	inputs : Vec<&'a Input>,
}

impl<'a> Input for MultipleInputDevice<'a> {
	fn handleEvent(&mut self, event : &'static SDL_Event) {
	    for i in &self.inputs {
	      i.handleEvent(event);
	    }
	}
}

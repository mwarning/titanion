/*
 * $Id: input.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2004 Kenta Cho. Some rights reserved.
 */
 /*
module src.util.sdl.input;


private import derelict.sdl.sdl;
*/

use std::vec::Vec;
use ttn::dummy::*;

/**
 * Input device interface.
 */
pub trait Input {
  fn handleEvent(&self, event : &SDL_Event);
}

struct MultipleInputDevice {
	inputs : Vec<&Input>,
}

impl Input for MultipleInputDevice {
	fn handleEvent(&self, event : &SDL_Event) {
	    for i in &self.inputs {
	      i.handleEvent(event);
	    }
	}
}

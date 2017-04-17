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
  fn handleEvent(&mut self, event : SDL_Event);
}

struct MultipleInputDevice<'a> {
	inputs : Vec<&'a Input>,
}

impl<'a> Input for MultipleInputDevice<'a> {
	fn handleEvent(&mut self, event : SDL_Event) {
	    for i in &self.inputs {
	      i.handleEvent(event);
	    }
	}
}

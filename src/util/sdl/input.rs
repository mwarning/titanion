/*
 * $Id: input.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2004 Kenta Cho. Some rights reserved.
 */
 /*
module src.util.sdl.input;


private import derelict.sdl.sdl;
*/

/**
 * Input device interface.
 */
trait Input {
  fn handleEvent(event : &SDL_Event);
}

trait MultipleInputDevice : Input {
	inputs : Vec!<Input>,
	fn handleEvent(&self, event : &SDL_Event) {
    for let i in self.inputs {
      i.handleEvent(event);
    }
}

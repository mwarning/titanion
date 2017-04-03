/*
 * $Id: mainloop.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.util.sdl.mainloop;


private import derelict.sdl.sdl;

private import src.util.logger;
private import src.util.rand;
private import src.util.preference;
private import src.util.sdl.frame;
private import src.util.sdl.screen;
private import src.util.sdl.input;
private import src.util.sdl.sound;
private import src.util.sdl.sdlexception;
*/

use ttn::dummy::*;
use ttn::field::*;
use ttn::frame::*;
use ttn::preference::*;
use util::sdl::input::*;


const INTERVAL_BASE : i32 = 16;
const MAX_SKIP_FRAME : i32 = 5;

/**
 * SDL main loop.
 */
pub struct MainLoop {
  noSlowdown : bool,
  event : SDL_Event,
  screen : &Screen,
  input : &Input,
  frame : &Frame,
  preference : &Preference,
  slowdownRatio : f32, 
  interval : f32,
  _slowdownStartRatio : f32,
  _slowdownMaxRatio : f32,
  done : bool,
}

impl MainLoop {
	fn new(screen : &Screen, input : &Input, frame : &Frame, preference : &Preference) -> MainLoop {
		MainLoop{
			noSlowdown : false,
			event : SDL_Event::new(),
			screen : screen,
			input : input,
			frame : frame,
			preference : preference,
			slowdownRatio : false,
			interval : INTERVAL_BASE,
			_slowdownStartRatio : 1.0,
			_slowdownMaxRatio : 1.5,
			done : false,
		}
	}

 // Initialize and load a preference.
  	fn initFirst(&mut self) {
	    self.preference.load();
	    //try {
	      self.Sound.init();
	    //} catch (SDLInitFailedException e) {
	    //  Logger.error(e);
	    //}
	    self.frame.init();
	    self.initInterval();
	}

 	// Quit and save a preference.
  	fn quitLast(&mut self) {
		self.frame.quit();
		Sound.close();
		self.preference.save();
		self.screen.closeSDL();
		SDL_Quit();
	}

	fn breakLoop(&mut self) {
		self.done = true;
	}

	fn loop1(&mut self) {
	    self.done = false;
	    let mut prvTickCount : i64 = 0;
	    self.screen.initSDL();
	    self.initFirst();
	    self.frame.start();
	    while !self.done {
	      if SDL_PollEvent(&self.event) == 0 {
	        self.event._type = SDL_USEREVENT;
	      }
	      self.input.handleEvent(&self.event);
	      if self.event._type == SDL_QUIT {
	        self.breakLoop();
	  	  }
	      let nowTick = SDL_GetTicks();
	      let itv  = self.interval as i32;
	      let mut frameNum = ((nowTick - prvTickCount) / itv) as i32;
	      if frameNum <= 0 {
	        frameNum = 1;
	        SDL_Delay((prvTickCount + itv - nowTick) as u32);
	        prvTickCount += self.interval;
	      } else if frameNum > MAX_SKIP_FRAME {
	        frameNum = MAX_SKIP_FRAME;
	        prvTickCount = nowTick;
	      } else {
	        //prvTickCount += frame * interval;
	        prvTickCount = nowTick;
	      }
	      self.slowdownRatio = 0.0;
		  for _ in 0..frameNum {
		    self.frame.move();
		  }
	      self.slowdownRatio /= frameNum as f32;
	      self.screen.clear();
	      self.frame.draw();
	      self.screen.flip();
	      if !self.noSlowdown {
	        self.calcInterval();
	      }
	    }
   	self.quitLast();
  }

  // Intentional slowdown.
  fn initInterval(&mut self) {
    self.interval = INTERVAL_BASE;
  }

  fn addSlowdownRatio(&mut self, sr : f32) {
    self.slowdownRatio += sr;
  }

  fn calcInterval(&self) {
    if self.slowdownRatio > self._slowdownStartRatio {
      let sr = self.slowdownRatio / self._slowdownStartRatio;
      if sr > self._slowdownMaxRatio {
        sr = self._slowdownMaxRatio;
      }
      self.interval += (sr * INTERVAL_BASE - self.interval) * 0.1;
    } else {
      self.interval += (INTERVAL_BASE - self.interval) * 0.08;
    }
  }

  fn slowdownStartRatio(&mut self, v : f32) -> f32 {
    self._slowdownStartRatio = v;
    v
  }

  fn slowdownMaxRatio(&mut self, v : f32) -> f32 {
    self._slowdownMaxRatio = v;
    v
  }
}

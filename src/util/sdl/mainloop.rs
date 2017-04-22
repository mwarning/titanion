/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use ttn::dummy::*;
use ttn::field::*;
use ttn::frame::*;
use ttn::preference::*;
use util::sdl::input::*;


const INTERVAL_BASE : f32 = 16.0;
const MAX_SKIP_FRAME : i32 = 5;

/**
 * SDL main loop.
 */
pub struct MainLoop<'a> {
  pub noSlowdown : bool,
  event : SDL_Event,
  //screen : &Screen, //we use frame.screen instead
  //input : &Input,
  pub frame : Frame<'a>,
  //preference : &Preference,
  slowdownRatio : f32, 
  interval : f32,
  _slowdownStartRatio : f32,
  _slowdownMaxRatio : f32,
  done : bool,
}

impl<'a> MainLoop<'a> {
	pub fn new(/*input : &Input,*/ frame : Frame /*, preference : &Preference*/) -> MainLoop<'a> {
		MainLoop{
			noSlowdown : false,
			event : SDL_Event::new(),
			//input : input, //moved into frame
			frame : frame,
			//preference : preference, //moved into frame
			slowdownRatio : 0.0,
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
	      self.frame.sound.init();
	    //} catch (SDLInitFailedException e) {
	    //  Logger.error(e);
	    //}
	    self.frame.init();
	    self.initInterval();
	}

 	// Quit and save a preference.
  	fn quitLast(&mut self) {
		self.frame.quit();
		self.frame.sound.close();
		self.preference.save();
		self.frame.screen.closeSDL();
		SDL_Quit();
	}

	fn breakLoop(&mut self) {
		self.done = true;
	}

	pub fn loop1(&mut self) {
		self.done = false;
		let mut prvTickCount : i64 = 0;
		self.frame.screen.initSDL();
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
			let itv = self.interval as i32;
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
				self.frame.move1();
			}
			self.slowdownRatio /= frameNum as f32;
			self.frame.screen.clear();
			self.frame.draw();
			self.frame.screen.flip();
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

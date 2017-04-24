/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::ptr;
use std::ops::BitOr;
use std::mem::transmute;

use util::sdl::input::*;
use util::sdl::recordableinput::*;
use ttn::dummy::*;

/**
 * Inputs from a joystick and a keyboard.
 */


const JOYSTICK_AXIS : i32 = 16384;

pub struct Pad {
  keys : &'static [u8; 16],
  pub buttonsExchanged : bool,
  stick : *const SDL_Joystick, //= null;
  state : PadState,
}

pub const DIR_NONE : i32 = 0;
pub const DIR_UP : i32 = 1;
pub const DIR_DOWN : i32 = 2;
pub const DIR_LEFT : i32 = 4;
pub const DIR_RIGHT : i32 = 8;

pub const BUTTON_NONE : i32 = 0;
pub const BUTTON_A : i32 = 16;
pub const BUTTON_B : i32 = 32;
pub const BUTTON_ANY : i32 = 48;

pub struct PadState {
  pub dir : i32,
  pub button : i32,
}

impl PadState {
  pub fn new() -> PadState {
    PadState{dir : DIR_NONE, button : BUTTON_NONE}
  }

  pub fn newInstance1(s : PadState) -> PadState {
    PadState{dir : s.dir, button : s.button}
  }

  pub fn set(&mut self, s : PadState) {
    self.dir = s.dir;
    self.button = s.button;
  }

  pub fn clear(&mut self) {
    self.dir = DIR_NONE;
    self.button = BUTTON_NONE;
  }

  pub fn read(&mut self, fd : &File) {
    let mut s : i32;
    fd.read2(&s);
    self.dir = unsafe { transmute(s & ((DIR_UP | DIR_DOWN) | (DIR_LEFT | DIR_RIGHT))) };
    self.button = unsafe { transmute(s & (BUTTON_ANY as i32)) };
  }

  pub fn write(&mut self, fd : &File) {
    let s = (self.dir as i32) | (self.button as i32);
    fd.write1(s);
  }

  /*
  //moved to PartialEq
    fn equals(&self, s : &PadState) -> bool {
      (self.dir == s.dir && self.button == s.button)
    }
  */
}

impl PartialEq for PadState {
  fn eq(&self, s: &PadState) -> bool {
    (self.dir == s.dir && self.button == s.button)
  }
}

pub struct RecordablePad {
  //inline from class RecordableInput
  inputRecord : InputRecord, //<PadState>,

  //inlined from class Pad
  pub pad : Pad,
}

impl Input for RecordablePad {
  //inlined from class Pad
  fn handleEvent(&mut self, event : SDL_Event) {
    self.pad.keys = SDL_GetKeyState(ptr::null());
  }
}

impl RecordablePad {
  //mixin RecordableInput!(PadState); //was inlined

  pub fn getState2(&self, doRecord : bool /*= true*/) -> PadState {
    let s = self.getState();
    if doRecord {
      self.record(s);
    }
    s
  }

  pub fn new() -> RecordablePad {
    RecordablePad {
      //inline from RecordableInput!(T)
      inputRecord : InputRecord::new(),
      //inlined from Pad
      pad : Pad {
        keys : SDL_GetKeyState(ptr::null()),
        buttonsExchanged : false,
        stick : SDL_JoystickOpen(0),
        state : PadState::new(),
      },
    }
  }

  //inlined from class Pad
  pub fn openJoystick(&mut self, st : *const SDL_Joystick /*= null*/) -> *const SDL_Joystick {
    if st == ptr::null() {
      if SDL_InitSubSystem(SDL_INIT_JOYSTICK) < 0 {
        return ptr::null();
      }
      self.pad.stick = SDL_JoystickOpen(0);
    } else {
      self.pad.stick = st;
    }
    self.pad.stick
  }

  //inlined from class Pad
  pub fn getState(&mut self) -> PadState {
    let x : i32 = 0;
    let y : i32 = 0;
    self.pad.state.dir = DIR_NONE;
    if self.pad.stick != ptr::null() {
      x = SDL_JoystickGetAxis(self.pad.stick, 0);
      y = SDL_JoystickGetAxis(self.pad.stick, 1);
    }
    let keys = self.pad.keys;
    if (keys[SDLK_RIGHT] == SDL_PRESSED) || (keys[SDLK_KP6] == SDL_PRESSED) || 
        (keys[SDLK_d] == SDL_PRESSED) || (keys[SDLK_l] == SDL_PRESSED) ||
        (x > JOYSTICK_AXIS) {
      self.pad.state.dir |= DIR_RIGHT;
    }
    if keys[SDLK_LEFT] == SDL_PRESSED || (keys[SDLK_KP4] == SDL_PRESSED) ||
        keys[SDLK_a] == SDL_PRESSED || (keys[SDLK_j] == SDL_PRESSED) ||
        (x < -JOYSTICK_AXIS) {
      self.pad.state.dir |= DIR_LEFT;
    }
    if (keys[SDLK_DOWN] == SDL_PRESSED) || (keys[SDLK_KP2] == SDL_PRESSED) ||
        (keys[SDLK_s] == SDL_PRESSED) || (keys[SDLK_k] == SDL_PRESSED) ||
        (y > JOYSTICK_AXIS) {
      self.pad.state.dir |= DIR_DOWN;
    }
    if (keys[SDLK_UP] == SDL_PRESSED) || (keys[SDLK_KP8] == SDL_PRESSED) ||
        (keys[SDLK_w] == SDL_PRESSED) || (keys[SDLK_i] == SDL_PRESSED) ||
        (y < -JOYSTICK_AXIS) {
      self.pad.state.dir |= DIR_UP;
    }
    self.pad.state.button = BUTTON_NONE;
    let btn1 : i32 = 0;
    let btn2 : i32 = 0;
    let leftTrigger : f32 = 0.0;
    let rightTrigger : f32  = 0.0;
    let stick = self.pad.stick;
    if stick != ptr::null() {
      btn1 = SDL_JoystickGetButton(stick, 0) + SDL_JoystickGetButton(stick, 2) +
             SDL_JoystickGetButton(stick, 4) + SDL_JoystickGetButton(stick, 6) +
             SDL_JoystickGetButton(stick, 8) + SDL_JoystickGetButton(stick, 10);
      btn2 = SDL_JoystickGetButton(stick, 1) + SDL_JoystickGetButton(stick, 3) +
             SDL_JoystickGetButton(stick, 5) + SDL_JoystickGetButton(stick, 7) +
             SDL_JoystickGetButton(stick, 9) + SDL_JoystickGetButton(stick, 11);
    }
    if (keys[SDLK_z] == SDL_PRESSED) || (keys[SDLK_PERIOD] == SDL_PRESSED) ||
        (keys[SDLK_LCTRL] == SDL_PRESSED) || (keys[SDLK_RCTRL] == SDL_PRESSED) || 
        (btn1 != 0) {
      if !self.pad.buttonsExchanged {
        self.pad.state.button |= BUTTON_A;
      } else {
        self.pad.state.button |= BUTTON_B;
      }
    }
    if (keys[SDLK_x] == SDL_PRESSED) || (keys[SDLK_SLASH] == SDL_PRESSED) ||
        (keys[SDLK_LALT] == SDL_PRESSED) || (keys[SDLK_RALT] == SDL_PRESSED) ||
        (keys[SDLK_LSHIFT] == SDL_PRESSED) || (keys[SDLK_RSHIFT] == SDL_PRESSED) ||
        (keys[SDLK_RETURN] == SDL_PRESSED) ||
        (btn2 != 0) {
      if !self.pad.buttonsExchanged {
        self.pad.state.button |= BUTTON_B;
      } else {
        self.pad.state.button |= BUTTON_A;
      }
    }
    self.pad.state
  }

  //inlined from class Pad
  pub fn getNullState(&mut self) -> PadState {
    self.pad.state.clear();
    self.pad.state
  }

  //inlined from RecordableInput!(PadState)
  pub fn startRecord(&mut self) {
    self.inputRecord = InputRecord::new();
    self.inputRecord.clear();
  }

  //inlined from RecordableInput!(PadState)
  pub fn record(&mut self, d : PadState) {
    self.inputRecord.add(d);
  }

  //inlined from RecordableInput!(PadState)
  pub fn startReplay(&mut self, pr : InputRecord) { //<PadState>) {
    self.inputRecord = pr;
    self.inputRecord.reset();
  }

  //inlined from RecordableInput!(PadState)
  pub fn replay(&mut self) -> PadState {
    if !self.inputRecord.hasNext() {
      panic!("No record data.");
    }
    self.inputRecord.next()
  }
}

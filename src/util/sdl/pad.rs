/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::ptr;
use std::ops::BitOr;

use util::sdl::input::*;
use util::sdl::recordableinput::*;
use ttn::dummy::*;

/**
 * Inputs from a joystick and a keyboard.
 */


const JOYSTICK_AXIS : i32 = 16384;

pub struct Pad {
  keys : &'static u8,
  pub buttonsExchanged : bool,
  stick : *const SDL_Joystick, //= null;
  state : PadState,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dir {
  NONE,
  UP = 1,
  DOWN = 2,
  LEFT = 4,
  RIGHT = 8,
}

impl BitOr for Dir {
    type Output = u32;

    fn bitor(self, rhs: Self) -> u32 {
      (self as u32) | (rhs as u32)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Button {
  NONE,
  A = 16,
  B = 32,
  ANY = 48,
}

impl BitOr for Button {
    type Output = u32;

    fn bitor(self, rhs: Self) -> u32 {
      (self as u32) | (rhs as u32)
    }
}

pub struct PadState {
  dir : Dir,
  button : Button,
}

impl PadState {
  pub fn new() -> PadState {
    PadState{dir : Dir::NONE, button : Button::NONE}
  }

  pub fn newInstance1(s : PadState) -> PadState {
    PadState{dir : s.dir, button : s.button}
  }

  pub fn set(&mut self, s : PadState) {
    self.dir = s.dir;
    self.button = s.button;
  }

  pub fn clear(&mut self) {
    self.dir = Dir::NONE;
    self.button = Button::NONE;
  }

  pub fn read(&mut self, fd : &File) {
    let mut s : i32;
    fd.read2(&s);
    self.dir = (s & ((Dir::UP | Dir::DOWN | Dir::LEFT | Dir::RIGHT) as i32)) as Dir;
    self.button = s & (Button::ANY as i32);
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
    self.keys = SDL_GetKeyState(ptr::null());
  }
}

impl RecordablePad {
  //mixin RecordableInput!(PadState); //was inlined

  fn getState2(&self, doRecord : bool /*= true*/) -> PadState {
    let s = self.getState();
    if doRecord {
      self.record(s);
    }
    s
  }

  pub fn new() -> RecordablePad {
    RecordablePad {
      //inline from RecordableInput!(T)
      inputRecord : InputRecord::<PadState>::new(),
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
  fn openJoystick(&mut self, st : *const SDL_Joystick /*= null*/) -> *const SDL_Joystick {
    if st == ptr::null() {
      if SDL_InitSubSystem(SDL_INIT_JOYSTICK) < 0 {
        return ptr::null();
      }
      self.pad.stick = SDL_JoystickOpen(ptr::null());
    } else {
      self.pad.stick = st;
    }
    self.pad.stick
  }

  //inlined from class Pad
  fn getState(&mut self) -> PadState {
    let x : i32 = 0;
    let y : i32 = 0;
    self.pad.state.dir = 0;
    if self.pad.stick != ptr::null() {
      x = SDL_JoystickGetAxis(self.pad.stick, 0);
      y = SDL_JoystickGetAxis(self.pad.stick, 1);
    }
    if (self.keys[SDLK_RIGHT] == SDL_PRESSED) || (self.keys[SDLK_KP6] == SDL_PRESSED) || 
        (self.keys[SDLK_d] == SDL_PRESSED) || (self.keys[SDLK_l] == SDL_PRESSED) ||
        (x > JOYSTICK_AXIS) {
      self.pad.state.dir |= Dir::RIGHT;
    }
    if self.keys[SDLK_LEFT] == SDL_PRESSED || (self.keys[SDLK_KP4] == SDL_PRESSED) ||
        self.keys[SDLK_a] == SDL_PRESSED || (self.keys[SDLK_j] == SDL_PRESSED) ||
        (x < -JOYSTICK_AXIS) {
      self.pad.state.dir |= Dir::LEFT;
    }
    if (self.keys[SDLK_DOWN] == SDL_PRESSED) || (self.keys[SDLK_KP2] == SDL_PRESSED) ||
        (self.keys[SDLK_s] == SDL_PRESSED) || (self.keys[SDLK_k] == SDL_PRESSED) ||
        (y > JOYSTICK_AXIS) {
      self.pad.state.dir |= Dir::DOWN;
    }
    if (self.keys[SDLK_UP] == SDL_PRESSED) || (self.keys[SDLK_KP8] == SDL_PRESSED) ||
        (self.keys[SDLK_w] == SDL_PRESSED) || (self.keys[SDLK_i] == SDL_PRESSED) ||
        (y < -JOYSTICK_AXIS) {
      self.pad.state.dir |= Dir::UP;
    }
    self.pad.state.button = 0;
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
    if (self.keys[SDLK_z] == SDL_PRESSED) || (self.keys[SDLK_PERIOD] == SDL_PRESSED) ||
        (self.keys[SDLK_LCTRL] == SDL_PRESSED) || (self.keys[SDLK_RCTRL] == SDL_PRESSED) || 
        btn1 {
      if !self.buttonsExchanged {
        self.pad.state.button |= Button::A;
      } else {
        self.pad.state.button |= Button::B;
      }
    }
    if (self.keys[SDLK_x] == SDL_PRESSED) || (self.keys[SDLK_SLASH] == SDL_PRESSED) ||
        (self.keys[SDLK_LALT] == SDL_PRESSED) || (self.keys[SDLK_RALT] == SDL_PRESSED) ||
        (self.keys[SDLK_LSHIFT] == SDL_PRESSED) || (self.keys[SDLK_RSHIFT] == SDL_PRESSED) ||
        (self.keys[SDLK_RETURN] == SDL_PRESSED) ||
        btn2 {
      if !self.buttonsExchanged {
        self.pad.state.button |= Button::B;
      } else {
        self.pad.state.button |= Button::A;
      }
    }
    self.pad.state
  }

  //inlined from class Pad
  fn getNullState(&mut self) -> PadState {
    self.pad.state.clear();
    self.pad.state
  }

  //inlined from RecordableInput!(PadState)
  fn startRecord(&mut self) {
    self.inputRecord = InputRecord::<PadState>::new();
    self.inputRecord.clear();
  }

  //inlined from RecordableInput!(PadState)
  fn record(&mut self, d : PadState) {
    self.inputRecord.add(d);
  }

  //inlined from RecordableInput!(PadState)
  fn startReplay(&mut self, pr : InputRecord) { //<PadState>) {
    self.inputRecord = pr;
    self.inputRecord.reset();
  }

  //inlined from RecordableInput!(PadState)
  fn replay(&mut self) -> PadState {
    if !self.inputRecord.hasNext() {
      panic!("No record data.");
    }
    self.inputRecord.next()
  }
}

/*
 * $Id: pad.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
/*
module src.util.sdl.pad;


private import std.string;
private import std.stream;

private import derelict.sdl.sdl;

private import src.util.sdl.input;
private import src.util.sdl.recordableinput;
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

struct Pad {
  keys : &'static u8,
  buttonsExchanged : bool,
  stick : Option<&'static SDL_Joystick>, //= null;
  state : PadState,
}

/*
//inlined into RecordablePad

impl Pad {
  fn new() -> Pad {
    Pad{
      keys : SDL_GetKeyState(0),
      buttonsExchanged : false,
      stick : Some(SDL_JoystickOpen(0)),
      state : PadState::new(),
    }
  }

  fn openJoystick(&mut self, st : Option(&SDL_Joystick) /*= null*/) -> Option(&SDL_Joystick) {
    if st == None {
      if SDL_InitSubSystem(SDL_INIT_JOYSTICK) < 0 {
        return None;
      }
      self.stick = Some(SDL_JoystickOpen(0));
    } else {
      self.stick = st;
    }
    self.stick
  }

  fn handleEvent(&mut self, event : &SDL_Event) {
    self.keys = SDL_GetKeyState(ptr::null());
  }

  fn getState(&mut self) -> PadState {
    let x : i32 = 0;
    let y : i32 = 0;
    self.state.dir = 0;
    if let Some(stick) = self.stick {
      x = SDL_JoystickGetAxis(stick, 0);
      y = SDL_JoystickGetAxis(stick, 1);
    }
    if (self.keys[SDLK_RIGHT] == SDL_PRESSED) || (self.keys[SDLK_KP6] == SDL_PRESSED) || 
        (self.keys[SDLK_d] == SDL_PRESSED) || (self.keys[SDLK_l] == SDL_PRESSED) ||
        (x > JOYSTICK_AXIS) {
      self.state.dir |= Dir::RIGHT;
    }
    if self.keys[SDLK_LEFT] == SDL_PRESSED || (self.keys[SDLK_KP4] == SDL_PRESSED) ||
        self.keys[SDLK_a] == SDL_PRESSED || (self.keys[SDLK_j] == SDL_PRESSED) ||
        (x < -JOYSTICK_AXIS) {
      self.state.dir |= Dir::LEFT;
    }
    if (self.keys[SDLK_DOWN] == SDL_PRESSED) || (self.keys[SDLK_KP2] == SDL_PRESSED) ||
        (self.keys[SDLK_s] == SDL_PRESSED) || (self.keys[SDLK_k] == SDL_PRESSED) ||
        (y > JOYSTICK_AXIS) {
      self.state.dir |= Dir::DOWN;
    }
    if (self.keys[SDLK_UP] == SDL_PRESSED) || (self.keys[SDLK_KP8] == SDL_PRESSED) ||
        (self.keys[SDLK_w] == SDL_PRESSED) || (self.keys[SDLK_i] == SDL_PRESSED) ||
        (y < -JOYSTICK_AXIS) {
      self.state.dir |= Dir::UP;
    }
    self.state.button = 0;
    let btn1 : i32 = 0;
    let btn2 : i32 = 0;
    let leftTrigger : f32 = 0.0;
    let rightTrigger : f32  = 0.0;
    if let Some(stick) = self.stick {
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
        self.state.button |= Button::A;
      } else {
        self.state.button |= Button::B;
      }
    }
    if (self.keys[SDLK_x] == SDL_PRESSED) || (self.keys[SDLK_SLASH] == SDL_PRESSED) ||
        (self.keys[SDLK_LALT] == SDL_PRESSED) || (self.keys[SDLK_RALT] == SDL_PRESSED) ||
        (self.keys[SDLK_LSHIFT] == SDL_PRESSED) || (self.keys[SDLK_RSHIFT] == SDL_PRESSED) ||
        (self.keys[SDLK_RETURN] == SDL_PRESSED) ||
        btn2 {
      if !self.buttonsExchanged {
        self.state.button |= Button::B;
      } else {
        self.state.button |= Button::A;
      }
    }
    self.state
  }

  fn getNullState(&mut self) -> PadState {
    self.state.clear();
    self.state
  }
}
*/

enum Dir {
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

enum Button {
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

struct PadState {
  dir : Dir,
  button : Button,
}

impl PadState {
  fn new() -> PadState {
    PadState{dir : Dir::NONE, button : Button::NONE}
  }

  fn newInstance1(s : &PadState) -> PadState {
    PadState{dir : s.dir, button : s.button}
  }

  fn set(&mut self, s : &PadState) {
    self.dir = s.dir;
    self.button = s.button;
  }

  fn clear(&mut self) {
    self.dir = Dir::NONE;
    self.button = Button::NONE;
  }

  fn read(&mut self, fd : &File) {
    let mut s : i32;
    fd.read(s);
    self.dir = s & (Dir::UP | Dir::DOWN | Dir::LEFT | Dir::RIGHT);
    self.button = s & Button::ANY;
  }

  fn write(&mut self, fd : &File) {
    let s : i32 = self.dir | self.button;
    fd.write(s);
  }

  fn equals(&self, s : &PadState) -> bool {
    (self.dir == s.dir && self.button == s.button)
  }
}

pub struct RecordablePad {
  //inline from class RecordableInput
  inputRecord : InputRecord<PadState>,

  //inlined from class Pad
  pad : Pad,
}

impl Input for RecordablePad {
}

impl RecordablePad {
  //mixin RecordableInput!(PadState); //was inlined

  fn getState2(&self, doRecord : bool /*= true*/) -> PadState {
    let s : PadState = self.getState();
    if doRecord {
      self.record(s);
    }
    s
  }

  fn new() -> RecordablePad {
    RecordablePad {
      //inline from RecordableInput!(T)
      inputRecord : InputRecord::<PadState>::new(),
      //inlined from Pad
      pad : Pad{
        keys : SDL_GetKeyState(0),
        buttonsExchanged : false,
        stick : Some(SDL_JoystickOpen(0)),
        state : PadState::new(),
      },
    }
  }

  //inlined from class Pad
  fn openJoystick(&mut self, st : Option<&'static SDL_Joystick> /*= null*/) -> Option<&'static SDL_Joystick> {
    if st == None {
      if SDL_InitSubSystem(SDL_INIT_JOYSTICK) < 0 {
        return None;
      }
      self.stick = Some(SDL_JoystickOpen(0));
    } else {
      self.stick = st;
    }
    self.stick
  }

  //inlined from class Pad
  fn handleEvent(&mut self, event : &'static SDL_Event) {
    self.keys = SDL_GetKeyState(ptr::null());
  }

  //inlined from class Pad
  fn getState(&mut self) -> PadState {
    let x : i32 = 0;
    let y : i32 = 0;
    self.state.dir = 0;
    if let Some(stick) = self.stick {
      x = SDL_JoystickGetAxis(stick, 0);
      y = SDL_JoystickGetAxis(stick, 1);
    }
    if (self.keys[SDLK_RIGHT] == SDL_PRESSED) || (self.keys[SDLK_KP6] == SDL_PRESSED) || 
        (self.keys[SDLK_d] == SDL_PRESSED) || (self.keys[SDLK_l] == SDL_PRESSED) ||
        (x > JOYSTICK_AXIS) {
      self.state.dir |= Dir::RIGHT;
    }
    if self.keys[SDLK_LEFT] == SDL_PRESSED || (self.keys[SDLK_KP4] == SDL_PRESSED) ||
        self.keys[SDLK_a] == SDL_PRESSED || (self.keys[SDLK_j] == SDL_PRESSED) ||
        (x < -JOYSTICK_AXIS) {
      self.state.dir |= Dir::LEFT;
    }
    if (self.keys[SDLK_DOWN] == SDL_PRESSED) || (self.keys[SDLK_KP2] == SDL_PRESSED) ||
        (self.keys[SDLK_s] == SDL_PRESSED) || (self.keys[SDLK_k] == SDL_PRESSED) ||
        (y > JOYSTICK_AXIS) {
      self.state.dir |= Dir::DOWN;
    }
    if (self.keys[SDLK_UP] == SDL_PRESSED) || (self.keys[SDLK_KP8] == SDL_PRESSED) ||
        (self.keys[SDLK_w] == SDL_PRESSED) || (self.keys[SDLK_i] == SDL_PRESSED) ||
        (y < -JOYSTICK_AXIS) {
      self.state.dir |= Dir::UP;
    }
    self.state.button = 0;
    let btn1 : i32 = 0;
    let btn2 : i32 = 0;
    let leftTrigger : f32 = 0.0;
    let rightTrigger : f32  = 0.0;
    if let Some(stick) = self.stick {
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
        self.state.button |= Button::A;
      } else {
        self.state.button |= Button::B;
      }
    }
    if (self.keys[SDLK_x] == SDL_PRESSED) || (self.keys[SDLK_SLASH] == SDL_PRESSED) ||
        (self.keys[SDLK_LALT] == SDL_PRESSED) || (self.keys[SDLK_RALT] == SDL_PRESSED) ||
        (self.keys[SDLK_LSHIFT] == SDL_PRESSED) || (self.keys[SDLK_RSHIFT] == SDL_PRESSED) ||
        (self.keys[SDLK_RETURN] == SDL_PRESSED) ||
        btn2 {
      if !self.buttonsExchanged {
        self.state.button |= Button::B;
      } else {
        self.state.button |= Button::A;
      }
    }
    self.state
  }

  //inlined from class Pad
  fn getNullState(&mut self) -> PadState {
    self.state.clear();
    self.state
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
  fn startReplay(&mut self, pr : InputRecord<PadState>) {
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

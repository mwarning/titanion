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

/**
 * Inputs from a joystick and a keyboard.
 */


let JOYSTICK_AXIS : i32 = 16384;

struct  Pad: Input {
  keys : &u8;
  buttonsExchanged : bool;
  stick : &SDL_Joystick, //= null;
  state : PadState;
}

impl Default for Pad {
  fn default() -> Pad{
    Pad{keys : SDL_GetKeyState(null),
      buttonsExchanged : false,
      stick : SDL_JoystickOpen(0),
      state : PadState::default(),
    }
  }
}

  fn openJoystick(&mut self, st &SDL_Joystick /*= null*/) -> &SDL_Joystick {
    if st == null {
      if SDL_InitSubSystem(SDL_INIT_JOYSTICK) < 0 {
        return null;
      }
      self.stick = SDL_JoystickOpen(0);
    } else {
      self.stick = st;
    }
    self.stick;
  }

  fn handleEvent(&mut self, event : &SDL_Event) {
    self.keys = SDL_GetKeyState(null);
  }

  fn getState(&mut self) -> &PadState {
    let x : i32 = 0;
    let y : i32 = 0;
    self.state.dir = 0;
    if self.stick {
      x = SDL_JoystickGetAxis(self.stick, 0);
      y = SDL_JoystickGetAxis(self.stick, 1);
    }
    if (self.keys[SDLK_RIGHT] == SDL_PRESSED || self.keys[SDLK_KP6] == SDL_PRESSED || 
        self.keys[SDLK_d] == SDL_PRESSED || self.keys[SDLK_l] == SDL_PRESSED ||
        x > JOYSTICK_AXIS)
      state.dir |= PadState.Dir.RIGHT;
    if (self.keys[SDLK_LEFT] == SDL_PRESSED || self.keys[SDLK_KP4] == SDL_PRESSED ||
        self.keys[SDLK_a] == SDL_PRESSED || self.keys[SDLK_j] == SDL_PRESSED ||
        x < -JOYSTICK_AXIS)
      state.dir |= PadState.Dir.LEFT;
    if (self.keys[SDLK_DOWN] == SDL_PRESSED || self.keys[SDLK_KP2] == SDL_PRESSED ||
        self.keys[SDLK_s] == SDL_PRESSED || self.keys[SDLK_k] == SDL_PRESSED ||
        y > JOYSTICK_AXIS)
      state.dir |= PadState.Dir.DOWN;
    if (self.keys[SDLK_UP] == SDL_PRESSED ||  self.keys[SDLK_KP8] == SDL_PRESSED ||
        self.keys[SDLK_w] == SDL_PRESSED || self.keys[SDLK_i] == SDL_PRESSED ||
        y < -JOYSTICK_AXIS)
      state.dir |= PadState.Dir.UP;
    state.button = 0;
    let btn1 : i32 = 0;
    let btn2 : i32 = 0;
    let leftTrigger : f32 = 0.0;
    let rightTrigger : f32  = 0.0;
    if self.stick {
      btn1 = SDL_JoystickGetButton(stick, 0) + SDL_JoystickGetButton(stick, 2) +
             SDL_JoystickGetButton(stick, 4) + SDL_JoystickGetButton(stick, 6) +
             SDL_JoystickGetButton(stick, 8) + SDL_JoystickGetButton(stick, 10);
      btn2 = SDL_JoystickGetButton(stick, 1) + SDL_JoystickGetButton(stick, 3) +
             SDL_JoystickGetButton(stick, 5) + SDL_JoystickGetButton(stick, 7) +
             SDL_JoystickGetButton(stick, 9) + SDL_JoystickGetButton(stick, 11);
    }
    if self.keys[SDLK_z] == SDL_PRESSED || self.keys[SDLK_PERIOD] == SDL_PRESSED ||
        self.keys[SDLK_LCTRL] == SDL_PRESSED || self.keys[SDLK_RCTRL] == SDL_PRESSED || 
        btn1 {
      if !buttonsExchanged {
        state.button |= PadState.Button.A;
      } else {
        state.button |= PadState.Button.B;
      }
    }
    if self.keys[SDLK_x] == SDL_PRESSED || self.keys[SDLK_SLASH] == SDL_PRESSED ||
        self.keys[SDLK_LALT] == SDL_PRESSED || self.keys[SDLK_RALT] == SDL_PRESSED ||
        self.keys[SDLK_LSHIFT] == SDL_PRESSED || self.keys[SDLK_RSHIFT] == SDL_PRESSED ||
        self.keys[SDLK_RETURN] == SDL_PRESSED ||
        btn2 {
      if !buttonsExchanged {
        state.button |= PadState.Button.B;
      } else {
        state.button |= PadState.Button.A;
      }
    }
    state;
  }

  fn getNullState(&mut self) -> &self {
    self.state.clear();
    self.state;
  }
}

enum Dir {
  NONE,
  UP, // = 1,
  DOWN, // = 2,
  LEFT, // = 4,
  RIGHT, // = 8,
};

enum Button {
  NONE,
  A, // = 16,
  B, // = 32,
  ANY, // = 48,
};


struct PadState {
  let dir : Dir,
  let button : Button,
}

impl Default for PadState {

  fn default() -> PadState {
    PadState{dir : Dir::NONE, button : Button::NONE}
  }
}

/*
  public static PadState newInstance() {
    return new PadState;
  }

  public static PadState newInstance(PadState s) {
    return new PadState(s);
  }

  public this() {
  }

  public this(PadState s) {
    this();
    set(s);
  }
*/

impl PadState {
  fn set(&mut self, s : &PadState) {
    self.dir = s.dir;
    self.button = s.button;
  }

  fn clear(&mut self) {
    self.dir = Dir::NONE;
    self.button = Button::NONE;
  }

  fn read(&mut self, fd : &File) {
    let mut s; : i32;
    fd.read(s);
    self.dir = s & (Dir.UP | Dir.DOWN | Dir.LEFT | Dir.RIGHT);
    self.button = s & Button.ANY;
  }

  fn write(&mut self, fd : &File) {
    let s : i32 = self.dir | self.button;
    fd.write(s);
  }

  fn equals(s : PadState) -> bool {
    (dir == s.dir && button == s.button)
  }
}


trait RecordablePad : Pad {
  //mixin RecordableInput!(PadState);

  fn getState(bool doRecord /*= true*/) -> PadState {
    let s : PadState = super.getState();
    if doRecord {
      record(s);
    }
    s
  }
}

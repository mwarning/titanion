/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

/**
 * Lifecycle of the game.
 */

 //moved to src/ttn/frame.rs
 /*
pub struct Frame {
  mainLoop: &MainLoop,
  abstractScreen: &SdlScreen,
  abstractInput: &Input,
  abstractPreference: &Preference,
}

impl Frame {
  fn new(
    mainloop : &MainLoop,
    abstractScreen : &SdlScreen,
    abstractInput : &Input,
    abstractPreference : &Preference
    ) {
    Frame{
      mainLoop : &mainloop,
      abstractScreen: &abstractScreen,
      abstractInput : &abstractInput,
      abstractPreference : &abstractPreference
    }
  }

  fn setMainLoop(&mut self, mainLoop : &MainLoop) {
    self.mainLoop = mainLoop;
  }

  fn setUIs(&mut self, screen : &SdlScreen, input : &Input) {
    self.abstractScreen = screen;
    self.abstractInput = input;
  }

  fn setPreference(&mut self, preference : &Preference) {
    self.abstractPreference = preference;
  }
}

//needed?
trait Frame {
  fn init();
  fn start();
  fn quit();
  fn move1();
  fn draw();
}
*/

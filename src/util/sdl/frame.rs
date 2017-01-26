/*
 * $Id: frame.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.util.sdl.frame;

private import src.util.preference;
private import src.util.sdl.mainloop;
private import src.util.sdl.screen;
private import src.util.sdl.input;
*/

/**
 * Lifecycle of the game.
 */
struct Frame {
 protected:
  MainLoop mainLoop;
  Screen abstractScreen;
  Input abstractInput;
  Preference abstractPreference;
}

impl Default for Frame {
  fn default(
    mainloop : MainLoop,
    abstractScreen : abstractScreen,
    abstractInput : abstractInput,
    abstractPreference : abstractPreference
    ) {
    Frame{
      mainLoop : mainloop,
      abstractScreen: abstractScreen,
      abstractInput : abstractInput,
      abstractPreference : abstractPreference
    }
  }
}

impl Frame {
  fn setMainLoop(&mut self, mainLoop : MainLoop) {
    self.mainLoop = mainLoop;
  }

  fn setUIs(&mut self, screen : Screen, input : Input) {
    self.abstractScreen = screen;
    self. abstractInput = input;
  }

  fn setPreference(&mut self, preference : Preference) {
    self.abstractPreference = preference;
  }
}

trait Frame {
  def init();
  def start();
  def quit();
  def move();
  def draw();
}

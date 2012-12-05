/*
 * $Id: frame.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.util.sdl.frame;

private import src.util.preference;
private import src.util.sdl.mainloop;
private import src.util.sdl.screen;
private import src.util.sdl.input;

/**
 * Lifecycle of the game.
 */
public class Frame {
 protected:
  MainLoop mainLoop;
  Screen abstractScreen;
  Input abstractInput;
  Preference abstractPreference;
 private:

  public void setMainLoop(MainLoop mainLoop) {
    this.mainLoop = mainLoop;
  }

  public void setUIs(Screen screen, Input input) {
    abstractScreen = screen;
    abstractInput = input;
  }

  public void setPreference(Preference preference) {
    abstractPreference = preference;
  }

  public abstract void init();
  public abstract void start();
  public abstract void quit();
  public abstract void move();
  public abstract void draw();
}

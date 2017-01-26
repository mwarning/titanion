/*
 * $Id: screen.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2005 Kenta Cho. Some rights reserved.
 */
//module src.util.sdl.screen;


/**
 * SDL screen handler interface.
 */
trait Screen {
  fn initSDL();
  fn closeSDL();
  fn flip();
  fn clear();
}

trait SizableScreen {
  fn windowMode() -> bool;
  fn width() -> i32;
  fn height() -> i32;
}

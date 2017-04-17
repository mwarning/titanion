/*
 * Copyright 2005 Kenta Cho. Some rights reserved.
 */

/**
 * SDL screen handler interface.
 */
pub trait SdlScreen { //was Screen
  fn initSDL(&mut self);
  fn closeSDL(&mut self);
  fn flip(&mut self);
  fn clear(&mut self);
}

pub trait SizableScreen {
  fn windowMode1(&self) -> bool;
  fn width1(&self) -> i32;
  fn height1(&self) -> i32;
}

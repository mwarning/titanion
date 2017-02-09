/*
 * $Id: displaylist.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2005 Kenta Cho. Some rights reserved.
 */
/*
module src.util.sdl.displaylist;


private import derelict.opengl.gl;

private import src.util.sdl.sdlexception;
*/

/**
 * Display list of OpenGL.
 */

// Dummy
fn glGenLists(num : i32) -> i32 {
  num
}

const GL_COMPILE : usize = 0;

fn glNewList(idx : i32, opt : usize) {
}

fn glEndList() {
}

fn glCallList(idx : i32) {
}

fn glDeleteLists(idx : i32, num : i32) {
}

//#######################################

pub struct DisplayList {
  registered : bool,
  num : i32,
  idx : i32,
  enumIdx : i32,
}

impl DisplayList {
  pub fn new(num : i32) -> DisplayList {
    DisplayList{ registered : false, num : num, idx : glGenLists(num), enumIdx : 0}
  }

  pub fn beginNewList(&mut self) {
    self.resetList();
    self.newList();
  }

  pub fn extNewList(&mut self) {
    glEndList();
    self.enumIdx += 1;
    if (self.enumIdx >= self.idx + self.num) || (self.enumIdx < self.idx) {
      panic!("Can't create new list. Index out of bound.");
    }
    glNewList(self.enumIdx, GL_COMPILE);
  }

  pub fn endNewList(&mut self) {
    glEndList();
    self.registered = true;
  }

  pub fn resetList(&mut self) {
    self.enumIdx = self.idx;
  }

  pub fn newList(&mut self) {
    glNewList(self.enumIdx, GL_COMPILE);
  }

  pub fn endList(&mut self) {
    glEndList();
    self.enumIdx += 1;
    self.registered = true;
  }

  pub fn call(&self, i : i32/*= 0*/) {
    glCallList(self.idx + i);
  }

  pub fn close(&self) {
    if self.registered {
      glDeleteLists(self.idx, self.num);
    }
  }
}

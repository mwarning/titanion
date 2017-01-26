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

struct DisplayList {
  registered : bool,
  num : i32,
  idx : i32,
  enumIdx i32,
}

impl Default for DisplayList {
  fn default -> DisplayList(num : i32) {
    DisplayList{ registered : false, num : num, idx : glGenLists(num), enumIdx : 0}
  }
}

impl DisplayList {

  fn beginNewList(&mut self) {
    self.resetList();
    self.newList();
  }

  fn extNewList(&mut self) {
    glEndList();
    self.enumIdx += 1;
    if (self.enumIdx >= self.idx + self.num) || (self.enumIdx < self.idx) {
      panic!("Can't create new list. Index out of bound.");
    }
    glNewList(self.enumIdx, GL_COMPILE);
  }

  fn endNewList(&mut self) {
    glEndList();
    self.registered = true;
  }

  fn resetList(&mut self) {
    self.enumIdx = idx;
  }

  fn newList(&mut self) {
    glNewList(self.enumIdx, GL_COMPILE);
  }

  fn endList(&mut self) {
    glEndList();
    self.enumIdx += 1;
    self.registered = true;
  }

  fn call(&self, i : i32/*= 0*/) {
    glCallList(self.idx + i);
  }

  fn close(&self) {
    if self.registered {
      glDeleteLists(self.idx, self.num);
    }
  }
}

/*
 * $Id: screen.d,v 1.2 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.screen;


private import derelict.sdl.sdl;
private import derelict.opengl.gl;
private import derelict.opengl.glu;

private import src.util.sdl.screen3d;
private import src.ttn.field;
*/

/**
 * OpenGL screen.
 */

let CAPTION = "Titanion";
let ICON_FILE_NAME = "images/ttn_icon32.bmp";

struct Screen {
	field : &Field;
}

impl Screen : Screen3D {

  fn setIcon() {
    SDL_WM_SetIcon(SDL_LoadBMP(ICON_FILE_NAME), null);
  }

 fn init() {
    setCaption(CAPTION);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
    glEnable(GL_BLEND);
    glEnable(GL_LINE_SMOOTH);
    glDisable(GL_TEXTURE_2D);
    glDisable(GL_COLOR_MATERIAL);
    glDisable(GL_LIGHTING);
    glDisable(GL_DEPTH_TEST);
    glDisable(GL_CULL_FACE);
    setClearColor(0, 0, 0, 1);
  }

  fn setField(&mut self, field : &Field) {
    self.field = field;
    self.screenResized();
  }

  fn close() {}

  fn screenResized(&mut self) {
    self.screenResized();
    let lw : f32 = ((self.width as f32) / 640.0 + (self.height as f32) / 480.0) / 2.0;
    if (lw < 1.0) {
      lw = 1.0;
    }  else if (lw > 4.0) {
      lw = 4.0;
    }
    glLineWidth(lw);
    glViewport(0, 0, self.width, self.height);
    if self.field {
      self.field.setLookAt();
    }
  }
}

/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::f32::consts::PI;
use std::ptr;

use util::sdl::screen::*;
use util::vector::*;
use ttn::field::*;
use ttn::dummy::*;

/*
 * OpenGL screen.
 */

const CAPTION : &'static str = "Titanion";
const ICON_FILE_NAME : &'static str = "images/ttn_icon32.bmp";

static mut _brightness : f32 = 1.0; // from screen3d.d

pub struct Screen {
	//field : *mut Field; //must be passed to methods via frame

// from Screen3D
  // static _brightness : f32,
  _farPlane : f32,
  _nearPlane : f32,
  _width : i32,
  _height : i32,
  pub _windowMode : bool,
}

impl SizableScreen for Screen {

  fn windowMode1(&self) -> bool {
    self._windowMode
  }

  fn width1(&self) -> i32 {
    self._width;
  }

  fn height1(&self) -> i32 {
    self._height
  }
}

impl SdlScreen for Screen {
  fn initSDL(&mut self) {
    //derelict specific
    DerelictGL::load();
    DerelictGLU::load();
    DerelictSDL::load();
      
    // Initialize SDL.
    if SDL_Init(SDL_INIT_VIDEO) < 0 {
      panic!("Unable to initialize SDL: {}", SDL_GetError());
    }
    self.setIcon();
    // Create an OpenGL screen.
    let mut videoFlags : u32 = if self._windowMode {
      SDL_OPENGL | SDL_RESIZABLE;
    } else {
      SDL_OPENGL | SDL_FULLSCREEN;
    };

    if SDL_SetVideoMode(self._width, self._height, 0, videoFlags) == None {
      panic!("Unable to create SDL screen: {}", SDL_GetError());
    }
    glViewport(0, 0, self.width, self.height);
    glClearColor(0.0, 0.0, 0.0, 0.0);
    self.resized(self._width, self._height);
    SDL_ShowCursor(SDL_DISABLE);
    self.init();
  }

  fn closeSDL(&mut self) {
    Screen::close();
    SDL_ShowCursor(SDL_ENABLE);
  }

  fn flip(&mut self) {
    Screen::handleError();
    SDL_GL_SwapBuffers();
  }

  fn clear(&mut self) {
    glClear(GL_COLOR_BUFFER_BIT);
  }
}

impl Screen {
  pub fn new(/*field : mut* Field*/) -> Screen {
    Screen{
     _farPlane : 1000.0,
     _nearPlane : 0.1,
     _width : 640,
     _height : 480,
     _windowMode : true,
   }
  }

  fn windowMode2(&mut self, v : bool) ->  bool {
    self._windowMode = v;
    v
  }

  pub fn width2(&mut self, v : i32) -> i32 {
    self._width = v;
    v
  }

  pub fn height2(&mut self, v : i32) -> i32 {
    self._height = v;
    v
  }

  //protected abstract void init();
  //protected abstract void close();
  //fn setIcon() {}

  // Reset a viewport when the screen is resized.
  fn screenResized1(&mut self) {
    glViewport(0, 0, self._width as f32, self._height as f32);
    glMatrixMode(GL_PROJECTION);
    self.setPerspective();
    glMatrixMode(GL_MODELVIEW);
  }

  fn setPerspective(mut self) {
    glLoadIdentity();
    //gluPerspective(45.0f, cast(GLfloat) width / cast(GLfloat) height, nearPlane, farPlane);
    glFrustum(-self._nearPlane as f64,
              self._nearPlane as f64,
              -self._nearPlane * (self._height as f32) / (self._width as f32) as f64,
              self._nearPlane * (self._height as f32) / (self._width as f32) as f64,
              0.1, self._farPlane as f64);
  }

  fn resized(&mut self, w : i32, h : i32) {
    self._width = w;
    self._height = h;
    self.screenResized();
  }

  fn handleError() {
    let error : GLenum  = glGetError();
    if error == GL_NO_ERROR {
      return;
    }
    Screen::closeSDL();
    panic!("OpenGL error({})", error);
  }

  fn setCaption(name : &str) {
    SDL_WM_SetCaption(name, "");
  }

  pub fn glVertex(v : Vector) {
    glVertex3f(v.x, v.y, 0.0);
  }

  pub fn glVertex3(v : Vector3) {
    glVertex3f(v.x, v.y, v.z);
  }

  pub fn glTranslate(v : Vector) {
    glTranslatef(v.x, v.y, 0.0);
  }

  pub fn glTranslate3(v : Vector3) {
    glTranslatef(v.x, v.y, v.z);
  }

  pub fn glRotate(d : f32, x : f32/*= 0*/, y : f32 /*= 0*/, z : f32 /*= 1*/) {
    glRotatef(d * 180.0 / PI, x, y, z);
  }

  pub fn setColor(r : f32, g : f32, b : f32, a : f32 /*= 1*/) {
    glColor4f(r * _brightness, g * _brightness, b * _brightness, a);
  }

  pub fn setClearColor(r : f32, g : f32, b : f32, a : f32 /*= 1*/) {
    glClearColor(r * _brightness, g * _brightness, b * _brightness, a);
  }

  pub fn brightness(&mut self, v : f32) -> f32 {
    _brightness = v;
    v
  }

  // inlined from util/sdl/screen3d.d
  fn setIcon() {
    SDL_WM_SetIcon(SDL_LoadBMP(ICON_FILE_NAME), "");
  }

  // inlined from util/sdl/screen3d.d
  fn init(&self) {
    self.setCaption(CAPTION);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
    glEnable(GL_BLEND);
    glEnable(GL_LINE_SMOOTH);
    glDisable(GL_TEXTURE_2D);
    glDisable(GL_COLOR_MATERIAL);
    glDisable(GL_LIGHTING);
    glDisable(GL_DEPTH_TEST);
    glDisable(GL_CULL_FACE);
    self.setClearColor(0, 0, 0, 1);
  }

  // inlined from util/sdl/screen3d.d
  fn setField(&mut self, field : *mut Field) {
    self.field = field;
    self.screenResized();
  }

  // inlined from util/sdl/screen3d.d
  fn close() {}

  // inlined from util/sdl/screen3d.d
  fn screenResized(&mut self) {
    self.screenResized1();
    let lw : f32 = ((self.width as f32) / 640.0 + (self._height as f32) / 480.0) / 2.0;
    if lw < 1.0 {
      lw = 1.0;
    }  else if lw > 4.0 {
      lw = 4.0;
    }
    glLineWidth(lw);
    glViewport(0, 0, self._width, self._height);
    if self.field {
      self.field.setLookAt();
    }
  }
}


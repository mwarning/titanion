/*
 * $Id: screen3d.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2005 Kenta Cho. Some rights reserved.
 */
 /*
module src.util.sdl.screen3d;


private import std.string;
private import core.stdc.string;
private import std.math;
private import std.conv;

private import derelict.sdl.sdl;
private import derelict.opengl.gl;
private import derelict.opengl.glu;

private import src.util.vector;
private import src.util.sdl.screen;
private import src.util.sdl.sdlexception;


string fromStringz(char* str) {
    return (str[0..strlen(str)]).idup;
}
*/

/**
 * SDL screen handler (3D, OpenGL).
 */

struct Screen3D {
  /*static*/ _brightness : f32,
  _farPlane : f32,
  _nearPlane : f32,
  _width : i32,
  _height : i32,
  _windowMode : bool;
}

impl Default for Screen3D {
  fn default() -> Screen3D {
    Screen3D{
      brightness :  1.0,
     _farPlane : 1000,
     _nearPlane : 0.1,
     _width : 640,
     _height : 480,
     _windowMode : true,
   }
}

impl Screen3D : Screen + SizableScreen {

  //protected abstract void init();
  //protected abstract void close();
  fn setIcon() {}

  fn initSDL(&mut self) {
    //derelict specific
    DerelictGL.load();
    DerelictGLU.load();
    DerelictSDL.load();
      
    // Initialize SDL.
    if SDL_Init(SDL_INIT_VIDEO) < 0 {
      panic!("Unable to initialize SDL: {}", SDL_GetError());
    }
    self.setIcon();
    // Create an OpenGL screen.
    let mut videoFlags : u32;
    if self._windowMode {
      videoFlags = SDL_OPENGL | SDL_RESIZABLE;
    } else {
      videoFlags = SDL_OPENGL | SDL_FULLSCREEN;
    } 
    if SDL_SetVideoMode(_width, _height, 0, videoFlags) == null {
      panic!("Unable to create SDL screen: []", SDL_GetError());
    }
    glViewport(0, 0, self.width, self.height);
    glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    self.resized(self._width, self._height);
    SDL_ShowCursor(SDL_DISABLE);
    self.init();
  }

  // Reset a viewport when the screen is resized.
  fn screenResized(&mut self) {
    glViewport(0, 0, self._width, self._height);
    glMatrixMode(GL_PROJECTION);
    self.setPerspective();
    glMatrixMode(GL_MODELVIEW);
  }

  fn setPerspective(mut self) {
    glLoadIdentity();
    //gluPerspective(45.0f, cast(GLfloat) width / cast(GLfloat) height, nearPlane, farPlane);
    glFrustum(-self._nearPlane,
              self._nearPlane,
              -self._nearPlane * (self._height as GLfloat) / (self._width as Glfloat),
              self._nearPlane * (self._height as GLfloat) / (self._width as Glfloat),
              0.1f, self._farPlane);
  }

  fn resized(&mut self, w : i32, h : i32) {
    self._width = w;
    self._height = h;
    self.screenResized();
  }

  fn closeSDL(&mut self) {
    self.close();
    SDL_ShowCursor(SDL_ENABLE);
  }

  fn flip(&mut self) {
    self.handleError();
    SDL_GL_SwapBuffers();
  }

  fn clear() {
    glClear(GL_COLOR_BUFFER_BIT);
  }

  fn handleError(&mut self) {
    let error : GLenum  = glGetError();
    if error == GL_NO_ERROR {
      return;
    }
    self.closeSDL();
    panic!("OpenGL error({})", error);
  }

  fn setCaption(name : &string) {
    SDL_WM_SetCaption(name, null);
  }

  fn windowMode(&mut self, v : bool) ->  bool {
    self._windowMode = v
    v
  }

  fn windowMode(&self) -> bool {
    self._windowMode
  }

  fn width(&mut self, v : i32) -> i32 {
    self._width = v;
    v
  }

  fn width(&self) -> i32 {
    self._width;
  }

  fn height(&mut self, v : i32) -> i32 {
    self._height = v;
    v
  }

  fn height(&self) -> i32 {
    self._height
  }

  fn glVertex(v : Vector) {
    glVertex3f(v.x, v.y, 0);
  }

  fn glVertex(v : Vector3) {
    glVertex3f(v.x, v.y, v.z);
  }

  fn glTranslate(v : Vector) {
    glTranslatef(v.x, v.y, 0);
  }

  fn glTranslate(v : Vector3) {
    glTranslatef(v.x, v.y, v.z);
  }

  fn glRotate(d : f32, x : f32/*= 0*/, y : f32 /*= 0*/, z : f32 /*= 1*/) {
    glRotatef(d * 180 / PI, x, y, z);
  }

  fn setColor(&self, r : f32, g : f32, b : f32, a : f32/*= 1*/) {
    glColor4f(r * self._brightness, g * self._brightness, b * self._brightness, a);
  }

  fn setClearColor(&self, r : f32, g : f32, b : f32, a : f32/*= 1*/) {
    glClearColor(r * self._brightness, g * self._brightness, b * self._brightness, a);
  }

  fn brightness(&self, v : f32) -> f32 {
    self._brightness = v;
    v
  }
}

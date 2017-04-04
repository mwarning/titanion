

use util::vector::*;
use util::rand::*;

pub enum PadStateDir {
  UP,
  DOWN,
}

pub struct SDL_Event {
  _type : usize,
  resize : SDL_ResizeEvent,
}

pub const SDL_VIDEORESIZE : usize = 0;
pub struct SDL_ResizeEvent {
  w : u32,
  h : u32,
}
pub const SDL_PRESSED : usize = 0;
pub const SDLK_ESCAPE : usize = 0;
pub const SDLK_p : usize = 0;

pub struct ReplayData;

pub const GL_TEXTURE_2D : usize = 0;
pub const GL_PROJECTION : usize = 0;
pub const GL_MODELVIEW : usize = 0;

pub const SDL_USEREVENT : usize = 0;
pub const SDL_QUIT : usize = 0;

pub fn SDL_Delay(d : u32) {}
pub fn SDL_GetTicks() -> i64 {}
pub fn SDL_Quit() {}
pub fn SDL_PollEvent(e : &SDL_Event) -> u32 {}

pub fn glMatrixMode(mode : usize) {}
pub fn gluLookAt(x : f32, y : f32, z : f32, a : f32, b : f32, c : f32, d : f32, e : f32, g : f32) {}
pub fn glOrtho(x : f32, y : f32, z : f32, a : f32, b : f32, c : f32) {}
pub fn glPushMatrix() {}
pub fn glPopMatrix() {}
pub fn glLoadIdentity() {}
pub fn glTexCoord2f(x : f32, y : f32) {}

pub struct Object;
pub struct RecordablePad;
pub struct Pad;
pub struct PadState;
pub struct ShotShape;
pub struct Sound;

impl Sound {
	pub fn fadeBgm() {}
	pub fn playSe(s : &str) {}
	pub fn playMarkedSes() {}
}

pub const TurretSpec_SPEED_RATIO : f32 = 0.0;

pub const GL_TRIANGLES : usize = 0;
pub const GL_TRIANGLE_FAN : usize = 0;
pub const GL_LINES : usize = 0;
pub const GL_LINE_STRIP : usize = 0;
pub const GL_SRC_ALPHA : usize = 0;
pub const GL_ONE_MINUS_SRC_ALPHA : usize = 0;
pub const GL_QUADS : usize = 0;
pub const GL_ONE : usize = 0;
pub const GL_LINE_LOOP : usize = 0;

pub fn glRotatef(angle : f32, x : f32, y : f32, z : f32) {}
pub fn glTranslatef(angle : f32, x : f32, y : f32) {}
pub fn glVertex3f(x : f32, y : f32, z : f32) {}
pub fn glBegin(mode : usize) {}
pub fn glEnd() {}
pub fn glEnable() {}
pub fn glDisable() {}
pub fn glBlendFunc(a : usize, b : usize) {}
pub fn glScalef(a : f32, b : f32, c : f32) {}

pub struct File;

impl File {
  fn new(name : &'static str, mode : usize) -> File {
    File{}
  }
}


pub struct Texture;

impl Texture {
  fn new(s : &'static str) -> Texture {
    Texture{}
  }
}

pub struct Screen;

impl Screen {
  pub fn setColor(r : f32, g : f32, b : f32, a : f32) {}
  pub fn glTranslate(v : Vector3) {}
  pub fn glRotate(deg : f32) {}
  pub fn glVertex(v : Vector3) {}
}

pub fn drawPillar(a : f32, b : f32, c : f32) {}

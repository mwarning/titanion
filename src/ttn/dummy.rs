

use util::vector::*;
use std::ptr;

pub enum PadStateDir {
  UP,
  DOWN,
}

pub struct SDL_Event {
  _type : usize,
  resize : SDL_ResizeEvent,
}


pub struct GLenum;
pub fn glGetError() -> GLenum { GLenum{} }

pub struct SDL_Joystick;
pub fn SDL_Init(flags : u32) {}
pub fn SDL_WM_SetCaption(title : &str, icon : &str) {}

pub struct SDL_Surface;
pub fn SDL_WM_SetIcon(icon : *const SDL_Surface, mask : *const u8) {}

pub fn SDL_LoadBMP(file : &str) {}
pub const SDL_INIT_VIDEO : u32 = 0;

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

pub const SDLK_RIGHT : usize = 0;
pub const SDLK_LEFT : usize = 0;
pub const SDLK_DOWN : usize = 0;
pub const SDLK_UP : usize = 0;
pub const SDLK_KP2 : usize = 0;
pub const SDLK_KP4 : usize = 0;
pub const SDLK_KP6 : usize = 0;
pub const SDLK_KP8 : usize = 0;
pub const SDLK_d : usize = 0;
pub const SDLK_l : usize = 0;
pub const SDLK_a : usize = 0;
pub const SDLK_j : usize = 0;
pub const SDLK_s : usize = 0;
pub const SDLK_k : usize = 0;
pub const SDLK_w : usize = 0;
pub const SDLK_i : usize = 0;
pub const SDLK_x : usize = 0;
pub const SDLK_z : usize = 0;
pub const SDLK_PERIOD : usize = 0;
pub const SDLK_LCTRL : usize = 0;
pub const SDLK_RCTRL : usize = 0;
pub const SDLK_SLASH : usize = 0;
pub const SDLK_RALT : usize = 0;
pub const SDLK_LALT : usize = 0;
pub const SDLK_LSHIFT : usize = 0;
pub const SDLK_RSHIFT : usize = 0;
pub const SDLK_RETURN : usize = 0;

pub const SDL_INIT_JOYSTICK : u32 = 0;

pub fn SDL_JoystickGetButton(stick : *const SDL_Joystick, n : u32) {}
pub fn SDL_JoystickGetAxis(stick : *const SDL_Joystick, n : u32) {}
pub fn SDL_InitSubSystem(n : u32) -> u32 { 0 }
pub fn SDL_JoystickOpen(n : u32) -> *const SDL_Joystick { ptr::null() }
pub fn SDL_GetKeyState(x :  *const SDL_Event) -> *const u8 {} //accepts a pointer...
pub fn SDL_Delay(d : u32) {}
pub fn SDL_GetTicks() -> i64 {}
pub fn SDL_Quit() {}
pub fn SDL_PollEvent(e : *const SDL_Event) -> u32 {}

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

pub struct DerelictGL;
impl DerelictGL {
  fn load() {}
}

pub struct DerelictGLU;
impl DerelictGLU {
  fn load() {}
}

pub struct DerelictSDL;
impl DerelictSDL {
  fn load() {}
}

pub fn SDL_GetError() -> &'static str { "" }

pub const SDL_OPENGL : usize = 0;
pub const SDL_RESIZABLE : usize = 0;
pub const SDL_FULLSCREEN : usize = 0;

pub fn SDL_SetVideoMode(_width : u32, _height : u32, n : u32, x : u32) {}
pub fn SDL_GL_SwapBuffers() {}
pub fn SDL_ShowCursor(n : usize) {}
pub fn glClear(d : u32) {}
pub fn glClearColor(a : f32, b : f32, c : f32, d : f32) {}

pub fn glColor4f(a : f32, b : f32, c : f32, d : f32) {}
pub fn glFrustum(left : f64, right : f64, bottom : f64, top : f64, nearVal : f64, farVal : f64) {}

pub const GL_NO_ERROR : usize = 0;
pub const GL_COLOR_BUFFER_BIT : usize = 0;
pub const SDL_ENABLE : usize = 0;
pub const SDL_DISABLE : usize = 0;
pub const GL_BLEND : usize = 0;
pub const GL_LINE_SMOOTH : usize = 0;
pub const GL_COLOR_MATERIAL : usize = 0;
pub const GL_LIGHTING : usize = 0;
pub const GL_DEPTH_TEST : usize = 0;
pub const GL_CULL_FACE : usize = 0;

pub fn glLineWidth(lw : f32) {}
pub fn glViewport(w : i32, h : i32, x : f32, y : f32) {}

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

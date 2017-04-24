

use util::vector::*;
use util::sdl::pad::*;
use std::ptr;

pub struct SDL_Event {
  pub _type : usize,
  pub resize : SDL_ResizeEvent,
}

impl SDL_Event {
  pub fn new() -> SDL_Event {
    SDL_Event {_type : 0, resize : SDL_ResizeEvent::new()}
  }
}


enum GLenum {}

pub fn glGetError() -> GLenum { GLenum{} }

pub struct SDL_Joystick;
pub fn SDL_Init(flags : u32) -> i32 { 0 }
pub fn SDL_WM_SetCaption(title : &str, icon : &str) {}

pub struct SDL_Surface;
pub fn SDL_WM_SetIcon(icon : *const SDL_Surface, mask : &'static str) {}

pub fn SDL_LoadBMP(file : &str) -> *const SDL_Surface {}
pub const SDL_INIT_VIDEO : u32 = 0;

pub const SDL_VIDEORESIZE : usize = 0;
pub struct SDL_ResizeEvent {
  pub w : u32,
  pub h : u32,
}

impl SDL_ResizeEvent {
 fn new() -> SDL_ResizeEvent {
  SDL_ResizeEvent {
    w : 0, h : 0,
  }
 }
}

pub struct Mix_Music;
pub struct Mix_Chunk;

pub const AUDIO_S16 : u16 = 0;
pub fn Mix_PlayChannel(chunkChannel : i32, chunk : *const Mix_Chunk, n : i32) {}
pub fn Mix_HaltChannel(chunkChannel : i32) {}
pub fn Mix_FreeChunk(chunk : *const Mix_Chunk) {}
pub fn Mix_LoadWAV(fileName : &str) -> *const Mix_Chunk { ptr::null() }
pub fn Mix_FadeOutMusic(speed : i32) {}
pub fn Mix_GetError() -> &'static str { "" }
pub fn Mix_OpenAudio(audio_rate : i32, audio_format : u16, audio_channels : i32, audio_buffers : i32) -> i32 { 0 }
pub fn Mix_QuerySpec(audio_rate : &i32, audio_format : &mut u16, audio_channels : &mut i32) {}
pub fn Mix_VolumeMusic(v : i32) {}
pub fn Mix_Volume(v : i32, seVol : i32) {}
pub fn Mix_PlayingMusic() -> i32 { 0 }
pub fn Mix_HaltMusic() {}
pub fn Mix_CloseAudio() {}
pub fn Mix_LoadMUS(filename : &str) -> *const Mix_Music { ptr::null() }
pub fn Mix_FreeMusic(m : *const Mix_Music) {}
pub fn Mix_PlayMusic(m : *const Mix_Music, n : i32) {}

pub const SDL_PRESSED : u8 = 0;
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
pub const SDL_INIT_AUDIO : u32 = 0;

pub fn SDL_JoystickGetButton(stick : *const SDL_Joystick, n : u32) -> i32 { 0 }
pub fn SDL_JoystickGetAxis(stick : *const SDL_Joystick, n : u32) -> i32 { 0 }
pub fn SDL_InitSubSystem(n : u32) -> u32 { 0 }
pub fn SDL_JoystickOpen(n : u32) -> *const SDL_Joystick { ptr::null() }

static keys : [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
pub fn SDL_GetKeyState(x :  *const SDL_Event) -> &'static [u8; 16] { &keys }
pub fn SDL_Delay(d : u32) {}
pub fn SDL_GetTicks() -> i64 { 0 }
pub fn SDL_Quit() {}
pub fn SDL_PollEvent(e : *const SDL_Event) -> u32 { 0 }

pub fn glMatrixMode(mode : usize) {}
pub fn gluLookAt(x : f32, y : f32, z : f32, a : f32, b : f32, c : f32, d : f32, e : f32, g : f32) {}
pub fn glOrtho(x : f32, y : f32, z : f32, a : f32, b : f32, c : f32) {}
pub fn glPushMatrix() {}
pub fn glPopMatrix() {}
pub fn glLoadIdentity() {}
pub fn glTexCoord2f(x : f32, y : f32) {}

pub struct Object;

pub struct DerelictSDLMixer;
impl DerelictSDLMixer {
  pub fn load() {}
}

pub struct DerelictGL;
impl DerelictGL {
  pub fn load() {}
}

pub struct DerelictGLU;
impl DerelictGLU {
  pub fn load() {}
}

pub struct DerelictSDL;
impl DerelictSDL {
  pub fn load() {}
}

pub fn SDL_GetError() -> &'static str { "" }

pub const SDL_OPENGL : i32 = 0;
pub const SDL_RESIZABLE : i32 = 0;
pub const SDL_FULLSCREEN : i32 = 0;

pub fn SDL_SetVideoMode(_width : i32, _height : i32, n : u32, x : i32) {}
pub fn SDL_GL_SwapBuffers() {}
pub fn SDL_ShowCursor(n : usize) {}
pub fn glClear(d : u32) {}
pub fn glClearColor(a : f32, b : f32, c : f32, d : f32) {}

pub fn glColor4f(a : f32, b : f32, c : f32, d : f32) {}
pub fn glFrustum(left : f64, right : f64, bottom : f64, top : f64, nearVal : f64, farVal : f64) {}

pub const GL_NO_ERROR : usize = 0;
pub const GL_COLOR_BUFFER_BIT : u32 = 0;
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
pub fn glEnable(n : usize) {}
pub fn glDisable(n : usize) {}
pub fn glBlendFunc(a : usize, b : usize) {}
pub fn glScalef(a : f32, b : f32, c : f32) {}

pub const FileReadExisting : usize = 0;
pub const FileWriteCreate : usize = 0;

pub struct File;

impl File {
  pub fn new(name : &'static str, mode : usize) -> File {
    File{}
  }

  pub fn write1(&self, n : i32) {}
  pub fn write2(&self, n : usize) {}
  pub fn read1(&self, n : &PadState) {}
  pub fn read2(&self, n : &i32) {}
  pub fn close(&self) {}
}

//TODO: remove
pub struct Texture;

impl Texture {
  fn new(s : &'static str) -> Texture {
    Texture{}
  }
}

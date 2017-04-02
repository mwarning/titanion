

use util::vector::*;
use util::rand::*;

/*
//dummy
pub enum GameStateMode {
  CLASSIC, BASIC, MODERN,
}*/

pub struct TriangleParticleSpec;
impl TriangleParticleSpec {
  fn new() -> Self { TriangleParticleSpec{} }
}

pub struct QuadParticleSpec;
impl QuadParticleSpec {
  fn new() -> Self { QuadParticleSpec{} }
}

pub struct LineParticleSpec;
impl LineParticleSpec {
  fn new() -> Self { LineParticleSpec{} }
}

pub struct BonusParticleSpec;
impl BonusParticleSpec {
  fn new() -> Self { BonusParticleSpec{} }
}

//dummy
pub enum ParticleShape {
  TRIANGLE, LINE, QUAD, BONUS,
}

pub enum PadStateDir {
  UP,
  DOWN,
}

pub struct SDL_Event {
  _type : usize,
  resize : usize,
}

pub const SDL_VIDEORESIZE : usize = 0;
pub struct SDL_ResizeEvent;
pub const SDL_PRESSED : usize = 0;
pub const SDLK_ESCAPE : usize = 0;
pub const SDLK_p : usize = 0;

pub struct ReplayData;

pub struct Preference;

impl Preference {
  pub const RANKING_NUM : usize = 10;
}

pub const GL_TEXTURE_2D : usize = 0;
pub const GL_PROJECTION : usize = 0;
pub const GL_MODELVIEW : usize = 0;

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
pub struct Particle;
pub trait ParticlePool {}

pub struct Sound;

impl Sound {
	pub fn fadeBgm() {}
	pub fn playSe(s : &str) {}
	pub fn playMarkedSes() {}
}

pub struct Letter;

impl Letter {
	pub fn drawString(name : &str, lx : f32, y : f32, s : f32,
                                d : i32 /*= Direction.TO_RIGHT*/,
                                rev : bool /*= false*/, od : f32 /*= 0*/,
                                r : f32 /*= 1*/, g : f32 /* = 1*/, b : f32 /*= 1*/) {
	}
}

pub const TurretSpec_SPEED_RATIO : f32 = 0.0;


pub const PlayerSpec_CAPTURED_ENEMIES_INTERVAL_LENGTH : f32 = 1.2;


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

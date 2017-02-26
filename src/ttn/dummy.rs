

use util::vector::*;


//dummy
pub enum GameStateMode {
  CLASSIC, BASIC, MODERN,
}

//dummy
pub enum ParticleShape {
  TRIANGLE, LINE, QUAD, BONUS,
}

pub const GL_PROJECTION : usize = 0;
pub const GL_MODELVIEW : usize = 0;

pub fn glMatrixMode(mode : usize) {}
pub fn gluLookAt(x : f32, y : f32, z : f32, a : f32, b : f32, c : f32, d : f32, e : f32, g : f32) {}
pub fn glOrtho(x : f32, y : f32, z : f32, a : f32, b : f32, c : f32) {}
pub fn glPushMatrix() {}
pub fn glPopMatrix() {}
pub fn glLoadIdentity() {}

//pub struct Field;
pub struct Object;
pub struct Frame;
pub struct Screen;
//pub trait BulletPool {  }
pub struct Player;
pub trait ParticlePool {}
pub struct Stage;
//pub struct BulletSpec;
pub struct GameState;
pub struct Particle;
//pub struct Bullet;
pub struct Rand {
}

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

impl Rand {
  pub fn nextFloat(&mut self, n : f32 /*n = 1*/) -> f32 {
    0.0
  }

  pub fn nextSignedFloat(&mut self, n : f32 /*= 1*/) -> f32 {
    0.0
  }
}

pub const TurretSpec_SPEED_RATIO : f32 = 0.0;


pub static rand : Rand = Rand{};


pub const PlayerSpec_CAPTURED_ENEMIES_INTERVAL_LENGTH : f32 = 1.2;

pub const Field_CIRCLE_RADIUS : f32 = 64.0;
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
pub fn glBegin(mode : usize) {}
pub fn glVertex3f(x : f32, y : f32, z : f32) {}
pub fn glEnd() {}
pub fn glBlendFunc(a : usize, b : usize) {}
pub fn glScalef(a : f32, b : f32, c : f32) {}
pub fn Screen_setColor(r : f32, g : f32, b : f32, a : f32) {}
pub fn drawPillar(a : f32, b : f32, c : f32) {}
pub fn Screen_glTranslate(v : Vector3) {}
pub fn Screen_glRotate(deg : f32) {}
pub fn Screen_glVertex(v : Vector3) {}


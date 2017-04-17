/*
 * $Id: bullet.d,v 1.4 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
/*
module src.ttn.bullet;


private import derelict.opengl.gl;
private import src.util.vector;
private import src.util.actor;
private import src.util.math;
private import src.ttn.token;
private import src.ttn.field;
private import src.ttn.shape;
private import src.ttn.player;
private import src.ttn.particle;
private import src.ttn.enemy;
private import src.ttn.screen;
private import src.ttn.frame;
*/

use std::f32::consts::PI;
use std::ptr;

use util::vector::*;
use util::actor::*;
use ttn::token::*;
use ttn::shape::*;
use ttn::dummy::*;
use ttn::enemy::*;
use ttn::field::*;
use ttn::frame::*;
use ttn::particle::*;
use ttn::player::*;

/*
 * Enemies' bullets.
 */

const BULLET_REMOVED_RANGE : f32 = 2.0;

pub struct BulletPool<'a> {
  ap : ActorPool<Bullet<'a>>,
}

impl<'a> BulletPool<'a> {
  fn new(n : i32) -> BulletPool<'a> {
    BulletPool{ap : ActorPool::<Bullet<'a>>::new(n)}
  }

  fn move1(&self) {
    self.ap.move1();
    /*
    super.move();
    BulletState.move();
  */
  }

  fn removeAround(&mut self, cnt : &i32, pos : Vector, particles : &ParticlePool, bonusParticles : &ParticlePool, player : &Player) {
    for b in self.actors {
      if b.exists {
        if b.pos.dist(pos) < BULLET_REMOVED_RANGE {
          b.remove();
          player.addScore(cnt);
          cnt += 1;
          let wc = if cnt <= 50 {
            cnt;
          } else {
            50 + ((cnt - 50) as f32).sqrt() as i32
          };
          let mut bp : &Particle = bonusParticles.getInstanceForced();
          bp.set(Particle::Shape::BONUS, b.state.pos.x, b.state.pos.y, 0, 0.2,
                 0.5, 1, 1, 1, 60, false, cnt, wc);
          let mut p : &Particle = particles.getInstanceForced();
          p.set(Particle::Shape::QUAD, b.state.pos.x, b.state.pos.y,
                b.state.deg, b.state.speed,
                1.5, 0.5, 0.75, 1.0, 60, false);
          self.removeAround(cnt, b.pos, particles, bonusParticles, player);
        }
      }
    }
  }
}

static mut colorCnt : i32 = 0;
static mut colorAlpha : f32 = 0.0;

struct BulletState {
  ts : TokenState,
  ppos : Vector,
  tailPos : Vector,
  cnt : i32,
  waitCnt : i32,
  speedRatio : f32,
}

impl BulletState {
  fn new() -> BulletState {
    BulletState {
      //colorCnt : 0,
      //colorAlpha : 0,
      ppos : Vector::new(0.0, 0.0),
      tailPos : Vector::new(0.0, 0.0),
      cnt : 0,
      waitCnt : 0,
      speedRatio : 0.0,
    }
  } 

  fn move1(&mut self) {
    self.colorCnt += 1;
    let c : i32 = self.colorCnt % 30;
    if c < 15  {
      self.colorAlpha = (c / 15) as f32;
    } else {
      self.colorAlpha = 1.0 - ((c - 15) / 15) as f32;
    }
  }

  fn clear(&mut self) {
    self.ppos.x = 0.0;
    self.ppos.y = 0;
    self.tailPos.x = 0;
    self.tailPos.y = 0;
    self.cnt = 0;
    self.waitCnt = 0;
    self.speedRatio = 0.0;
    self.clear();
  }
}

const DISAPPEAR_CNT : f32 = 300.0;

pub struct BulletSpec<'a> {
  //ts : TokenSpec<BulletState>, //inlined
  field : &'a mut Field<'a>,
  shape : &'a mut Shape,
  player : &'a Player<'a>,
  enemies : &'a EnemyPool<'a>,
  particles : &'a mut ParticlePool<'a>,
  lineShape : &'a mut Shape,
  gameState : &'a GameState<'a>,
}

impl<'a> TokenSpec<BulletState> for BulletSpec<'a> {
} 

impl<'a> BulletSpec<'a> {
  fn new(field : &'a mut Field, player : &'a Player, enemies : &'a EnemyPool<'a>, particles : &'a mut ParticlePool,
              shape : &'a mut Shape, lineShape : &'a mut Shape, gameState : &'a mut GameState) -> BulletSpec<'a> {
    BulletSpec{
      //ts : TokenSpec::<BulletState>::new(field, shape),
      shape : shape,
      field : field,
      player : player,
      enemies : enemies,
      particles : particles,
      shape : shape,
      lineShape : lineShape,
      gameState : gameState
    }
  }

  fn set(&mut self, bs : &mut BulletState) {
    //with bs {
      bs.ppos.x = self.ts.pos.x;
      bs.ppos.y = self.ts.pos.y;
      bs.tailPos.x = self.ts.pos.x;
      bs.tailPos.y = self.ts.pos.y;
      //assert(deg <>= 0);
    //}
  }

  fn move2(&mut self, bs : &mut BulletState) -> bool {
      if bs.waitCnt > 0 {
        bs.waitCnt -= 1;
        return true;
      }
      bs.ppos.x = bs.ts.pos.x;
      bs.ppos.y = bs.ts.pos.y;
      let mut sp = bs.ts.speed;
      if (self.gameState.mode != Frame::Mode::CLASSIC) && (bs.cnt < 40) {
        sp *= ((bs.cnt + 10) as f32) / 50;
      }
      bs.tailPos.x -= bs.ts.deg.cos() * sp * 0.7;
      bs.tailPos.y += bs.ts.deg.cos() * sp * 0.7;
      bs.ts.pos.pos.x -= bs.ts.deg.sin() * sp;
      bs.ts.pos.y += bs.ts.deg.cos() * sp;
      self.field.addSlowdownRatio(bs.ts.speed * 0.04);
      bs.ts.pos.x = self.field.normalizeX(bs.ts.pos.x);
      if !self.field.containsOuter(bs.ts.pos) {
        return false;
      }
      if !self.field.contains(bs.ts.pos) || bs.cnt >= (DISAPPEAR_CNT * 0.9) {
        bs.tailPos.x += (bs.ts.pos.x - bs.tailPos.x) * 0.1;
        bs.tailPos.y += (bs.ts.pos.y - bs.tailPos.y) * 0.1;
      }
      bs.tailPos.x = self.field.normalizeX(bs.tailPos.x);
      if self.player.enemiesHasCollision() {
        if self.enemies.checkBulletHit(bs.ts.pos, bs.ppos) {
          return false;
        }
      }
      if self.player.checkBulletHit(bs.ts.pos, bs.ppos) {
        return false;
      }
      bs.cnt += 1;
      
      (bs.cnt < (DISAPPEAR_CNT as i32))
  }

  fn draw(&mut self, bs : &BulletState) {
      if bs.waitCnt > 0 {
        return;
      }
      let p : Vector3;
      glBegin(GL_LINES);
      Screen::setColor(0.1, 0.4, 0.4, 0.5);
      p = self.field.calcCircularPos(bs.tailPos);
      Screen::glVertex(p);
      Screen::setColor(0.2 * colorAlpha, 0.8 * colorAlpha, 0.8 * colorAlpha);
      p = self.field.calcCircularPos(bs.ts.pos);
      Screen::glVertex(p);
      glEnd();
      p = self.field.calcCircularPos(bs.ts.pos);
      let d : f32 = match self.gameState.mode {
        Frame::Mode::CLASSIC => PI,
        Frame::Mode::BASIC => bs.ts.deg,
        Frame::Mode::MODERN => bs.ts.deg,
      };
      let cd = self.field.calcCircularDeg(bs.ts.pos.x);
      (self.ts.shape as &BulletShapeBase).draw(p, cd, d, bs.cnt * 3.0);
      Screen::setColor(0.6 * colorAlpha, 0.9 * colorAlpha, 0.9 * colorAlpha);
      (self.lineShape as &BulletShapeBase).draw(p, cd, d, bs.cnt * 3.0);
    }
}

pub struct Bullet<'a> {
  //tok : Token<BulletState, BulletSpec>, //inlined
  state : &'a mut BulletState,
  spec : &'a mut BulletSpec<'a>,
  _exists : bool, //inherited by Actor class
}

//we inline varaibles and methods into Bullet
//impl Token<BulletState, BulletSpec> for Bullet {
//}

impl<'a> Bullet<'a> {
  fn setWaitCnt(&mut self, c : i32) {
    self.tok.state.waitCnt = c;
  }
}

impl<'a> Actor for Bullet<'a> {
  fn getExists(&self) -> bool {
    self._exists
  }

  fn setExists(&mut self, v : bool)-> bool {
    self._exists = v;
    v
  }

  fn init(&mut self /*Object[] args*/) {
    self.state = BulletState::new();
  }

  fn move1(&self) {
    if !self.spec.move2(self.state) {
      self.remove();
    }
  }

  fn draw1(&self) {
    self.spec.draw(self.state);
  }
}

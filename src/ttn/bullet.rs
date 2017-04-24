/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
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
use ttn::screen::*;

/*
 * Enemies' bullets.
 */

const BULLET_REMOVED_RANGE : f32 = 2.0;

pub struct BulletPool<'a> {
  ap : ActorPoolData<Bullet<'a>>,
}

impl<'a> ActorPool<Bullet<'a>> for BulletPool<'a> {
  fn getActorPoolData(&mut self) -> &mut ActorPoolData<Bullet<'a>> {
    &mut self.ap
  }
}

impl<'a> BulletPool<'a> {
  pub fn new(n : i32) -> BulletPool<'a> {
    BulletPool{ap : ActorPool::<Bullet<'a>>::new(n)}
  }

  pub fn move1(&self) {
    self.ap.move1();
    /*
    super.move();
    BulletState.move();
  */
  }

  pub fn removeAround(&mut self, cnt : &mut i32, pos : Vector, particles : &ParticlePool, bonusParticles : &ParticlePool, player : &Player) {
    for b in self.actors {
      if b.exists {
        if b.pos.dist(pos) < BULLET_REMOVED_RANGE {
          b.remove();
          player.addScore(*cnt);
          *cnt += 1;
          let wc = if *cnt <= 50 {
            *cnt;
          } else {
            50 + ((*cnt - 50) as f32).sqrt() as i32
          };
          let mut bp : &Particle = bonusParticles.getInstanceForced();
          bp.set(ParticleShape::BONUS, b.state.pos.x, b.state.pos.y, 0.0, 0.2,
                 0.5, 1.0, 1.0, 1.0, 60, false, *cnt, wc);
          let mut p = particles.getInstanceForced();
          p.set(ParticleShape::QUAD, b.state.pos.x, b.state.pos.y,
                b.state.deg, b.state.speed,
                1.5, 0.5, 0.75, 1.0, 60, false);
          self.removeAround(*cnt, b.pos, particles, bonusParticles, player);
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
  fn set(&self, state : &BulletState) {}
  fn removed(&self, state : &BulletState) {}

  fn move2(&self, state : &BulletState) -> bool {
    true
  }

  fn draw(&self, state : &BulletState) {
    //with (state) {
      let p = self.field.calcCircularPos(state.ts.pos);
      let cd = self.field.calcCircularDeg(state.ts.pos.x);
      self.shape.draw(p, cd, state.ts.deg);
    //}
  }
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
      if (self.gameState.mode != Mode::CLASSIC) && (bs.cnt < 40) {
        sp *= ((bs.cnt + 10) as f32) / 50.0;
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
      Screen::setColor(0.2 * colorAlpha, 0.8 * colorAlpha, 0.8 * colorAlpha, 1.0);
      p = self.field.calcCircularPos(bs.ts.pos);
      Screen::glVertex(p);
      glEnd();
      p = self.field.calcCircularPos(bs.ts.pos);
      let d : f32 = match self.gameState.mode {
        Mode::CLASSIC => PI,
        Mode::BASIC => bs.ts.deg,
        Mode::MODERN => bs.ts.deg,
      };
      let cd = self.field.calcCircularDeg(bs.ts.pos.x);
      (self.ts.shape as &BulletShapeBase).draw(p, cd, d, bs.cnt * 3);
      Screen::setColor(0.6 * colorAlpha, 0.9 * colorAlpha, 0.9 * colorAlpha, 1.0);
      (self.lineShape as &BulletShapeBase).draw(p, cd, d, bs.cnt * 3);
    }
}

pub struct Bullet<'a> {
  //tok : Token<BulletState, BulletSpec>, //inlined
  state : BulletState,
  spec : &'a mut BulletSpec<'a>,
  _exists : bool, //inherited by Actor class
}

//we inline varaibles and methods into Bullet
//impl Token<BulletState, BulletSpec> for Bullet {
//}

impl<'a> Bullet<'a> {
  fn setWaitCnt(&mut self, c : i32) {
    self.state.waitCnt = c;
  }
}

impl<'a> Actor for Bullet<'a> {
  fn new() -> Bullet<'a> {
    Bullet {
      state : BulletState::new(),
      spec : BulletSpec::new(), //use generic spec or Option type?
    }
  }

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

impl<'a> Token<BulletState, BulletSpec<'a>> for Bullet<'a> {
  /*
  fn getExists(&self) -> bool {
    self._exists
  }

  fn setExists(&mut self, v : bool) -> bool {
    self._exists = v;
    v
  }*/

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

  fn set5Vec(&self, spec : &BulletSpec, pos : Vector, deg : f32, speed : f32) {
    self.spec = spec;
    self.set5(pos.x, pos.y, deg, speed);
  }

  fn set6(&self, spec : &BulletSpec, x : f32, y : f32, deg : f32, speed : f32) {
    self.spec = spec;
    self.set5(x, y, deg, speed);
  }

  fn set5(&self, x : f32, y : f32, deg : f32, speed : f32) {
    self.state.clear();
    self.state.pos.x = x;
    self.state.pos.y = y;
    self.state.deg = deg;
    self.state.speed = speed;
    self.spec.set(self.state);
    self.actor._exists = true;
  }

  fn remove(&self) {
    self._exists = false;
    self.spec.removed(self.state);
  }

  fn pos(&self) -> Vector {
    self.state.pos
  }
}

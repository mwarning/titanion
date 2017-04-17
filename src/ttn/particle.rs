/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::f32::consts::PI;

use util::vector::*;
use util::actor::*;
use util::rand::*;
use ttn::field::*;
use ttn::shape::*;
use ttn::token::*;
use ttn::player::*;
use ttn::letter::*;
use ttn::screen::*;
use ttn::dummy::*;

/**
 * Particles (Triangle / Line / Quad / Bonus).
 */
 //TODO:
//public class ParticlePool: ActorPool!(Particle) {
//}

pub struct ParticlePool<'a> {
  ap : ActorPool<Particle<'a>>,
}

impl<'a> ParticlePool<'a> {
}

struct ParticleState { //: TokenState {
  ts : TokenState, //"inherited"
  vel : Vector,
  tailPos : Vector,
  size : f32,
  cnt : i32,
  startCnt : i32,
  r: f32,
  g : f32,
  b : f32,
  a : f32,
  d1 : f32,
  d2 : f32,
  vd1 : f32,
  vd2 : f32,
  effectedByGravity : bool,
  num : f32,
  trgNum : f32,
  trgSize : f32,
  waitCnt : i32,
}

impl ParticleState {
/*
  invariant() {
    if (isInitialized) {
      assert(pos.x <>= 0);
      assert(pos.y <>= 0);
      assert(vel.x <>= 0);
      assert(vel.y <>= 0);
      assert(tailPos.x <>= 0);
      assert(tailPos.y <>= 0);
      assert(size > 0 && size < 20);
      assert(r >= 0 && r <= 1);
      assert(g >= 0 && g <= 1);
      assert(b >= 0 && b <= 1);
      assert(a >= 0 && a <= 1);
      assert(d1 <>= 0);
      assert(d2 <>= 0);
      assert(vd1 <>= 0);
      assert(vd2 <>= 0);
      assert(num <>= 0);
      assert(trgNum <>= 0);
      assert(trgSize > 0);
    }
  }
*/
  fn new(&mut self) -> ParticleState {
    //super();
    ParticleState {
      ts : TokenState::new(),
      vel : Vector::new(),
      tailPos : Vector::new(),
      size : 1.0,
      cnt : 0,
      startCnt : 0,
      r: 0.0,
      g : 0.0,
      b : 0.0,
      a : 1.0,
      d1 : 0.0,
      d2 : 0.0,
      vd1 : 0.0,
      vd2 : 0.0,
      effectedByGravity : false,
      num : 1.0,
      trgNum : 1.0,
      trgSize : 1.0,
      waitCnt : 0,
    }
  }

  fn clear(&mut self) {
    self = ParticleState::new();
    /*
    self.vel.x = 0.0;
    self.vel.y = 0.0;
    self.size = 1;
    self.cnt = 0;
    self.r = 0.0;
    self.g = 0.0;
    self.b = 0.0;
    self.a = 1.0;
    self.d1 = 0.0;
    self.d2 = 0.0;
    self.vd1 = 0.0;
    self.vd2 = 0.0;
    self.effectedByGravity = false;
    self.num = 1;
    self.trgNum = 1;
    self.trgSize = 1;
    self.waitCnt = 0;
    self.TokenState.clear(); // super.clear();
    */
  }
}

//public class ParticleSpec: TokenSpec!(ParticleState) {
pub struct ParticleSpec<'a> {
  //ts : TokenSpec<ParticleState>, //inlined
  field : &'a mut Field<'a>,
  shape : &'a mut Shape,
  //mixin StaticRandImpl;
  player : &'a mut Player<'a>,
  rand : Rand,
}

impl<'a> TokenSpec<ParticleSpec<'a>> for ParticleSpec<'a> {
}

impl<'a> ParticleSpec<'a> {
  fn new(field : &mut Field<'a>, shape : &mut Shape, player : &mut Player<'a>) -> ParticleSpec<'a> {
   ParticleSpec{ field : field, shape : shape, player : player, rand : Rand::new() }
  }
/*
  fn setPlayer(&mut self, player : &Player) {
    self.player = player;
  }
*/
  fn calcNearPlayerAlpha(&self, pos : Vector) -> f32 {
    if !self.player.isActive {
      return 1.0;
    }

    let pd = self.player.pos.dist(pos);
    if pd < 20.0 {
      pd / 20.0
    } else {
      1.0
    }
  }
}

const TPS_SLOW_DOWN_RATIO : f32 = 0.05;
const TPS_GRAVITY : f32 = 0.003;

pub struct TriangleParticleSpec<'a> {
  ps : &'a ParticleSpec<'a>, //inherited
  particleShape : &'a Shape,
  particles : &'a ParticlePool<'a>,
}

impl<'a> TriangleParticleSpec<'a> {
  fn new(field : &'a Field<'a>) -> TriangleParticleSpec<'a> {
    TriangleParticleSpec {
      field : field,
      particleShape : TriangleParticleShape::new(),
    }
  }

  fn setParticles(&mut self, particles : &ParticlePool) {
    self.particles = particles;
  }

  fn set(&mut self, ps : &ParticleState) {
    ps.d1 = self.rand.nextFloat(PI * 2.0);
    ps.d2 = self.rand.nextFloat(PI * 2.0);
    ps.vd1 = self.rand.nextSignedFloat(0.1);
    ps.vd2 = self.rand.nextSignedFloat(0.1);
  }

  fn move2(&mut self, ps : &ParticleState) -> bool {
    //with (ps) {
      ps.pos += ps.vel;
      ps.pos.x = self.field.normalizeX(ps.pos.x);
      if ps.effectedByGravity {
        ps.vel.y -= TPS_GRAVITY;
      }
      ps.vel *= 1.0 - TPS_SLOW_DOWN_RATIO;
      ps.d1 += ps.vd1;
      ps.d2 += ps.vd2;
      ps.vd1 *= 1.0 - TPS_SLOW_DOWN_RATIO * 0.2;
      ps.vd2 *= 1.0 - TPS_SLOW_DOWN_RATIO * 0.2;
      let cfr = 1.0 - (1.0 / (ps.startCnt as f32));
      if cfr < 0 {
        cfr = 0;
      }
      ps.r *= cfr;
      ps.g *= cfr;
      ps.b *= cfr;
      ps.a *= cfr;
      let fs = if (ps.size > 2.0) && (self.rand.nextInt(45) == 0) {
        0.5 - self.rand.nextFloat(0.2)
      } else if (ps.size > 0.5) && (self.rand.nextInt(10) == 0) {
        0.1 + self.rand.nextSignedFloat(0.05)
      };
      if fs > 0 {
        let vx = ps.vel.x * self.rand.nextSignedFloat(0.8);
        let vy = ps.vel.y * self.rand.nextSignedFloat(0.8);
        ps.vel.x -= vx * fs;
        ps.vel.y -= vy * fs;
        let cr = 1.0 - fs * 0.2;
        ps.vel /= cr;
        let p = self.particles.getInstanceForced();
        let nc = (ps.cnt * (0.8 + fs * 0.2)) as i32;
        if nc > 0 {
          p.setByVelocity(ps.pos.x, ps.pos.y, vx, vy, ps.size * fs, ps.r, ps.g, ps.b, ps.a, nc, ps.effectedByGravity);
        }
        ps.size *= 1 - fs;
        ps.cnt *= cr;
      };
      ps.cnt -= 1;
      ps.cnt > 0
    //}
  }

  fn draw(&self, ps : &ParticleState) {
    //with (ps) {
      let p = self.field.calcCircularPos(ps.pos);
      let aa = ps.a * ps.calcNearPlayerAlpha(ps.pos);
      Screen::setColor(ps.r, ps.g, ps.b, aa);
      self.particleShape.draw(p, ps.d1, ps.d2);
    //}
  }
}

const LPS_SLOW_DOWN_RATIO : f32 = 0.03;

pub struct LineParticleSpec<'a> {
  ps : &'a ParticleSpec<'a>,
}

impl<'a> LineParticleSpec<'a> {
  fn new(field : &Field) -> LineParticleSpec<'a> {
    LineParticleSpec{ ps : ParticleSpec::new(field) }
  }

  fn set(&self, ps : &ParticleState) {
    //with (ps) {
      ps.tailPos.x = ps.pos.x;
      ps.tailPos.y = ps.pos.y;
    //}
  }

  fn move2(&mut self, ps : &ParticleState) -> bool {
    //with (ps) {
      ps.stepForward();
      ps.tailPos.x += (ps.pos.x - ps.tailPos.x) * 0.05;
      ps.tailPos.y += (ps.pos.y - ps.tailPos.y) * 0.05;
      ps.speed *= 1 - LPS_SLOW_DOWN_RATIO;
      ps.pos.x = self.field.normalizeX(ps.pos.x);
      let cfr = 1.0 - (1.0 / (ps.startCnt as f32));
      if cfr < 0.0 {
        cfr = 0.0;
      };
      ps.r *= cfr;
      ps.g *= cfr;
      ps.b *= cfr;
      ps.a *= cfr;
      ps.cnt -= 1;
      ps.cnt > 0
    //}
  }

  fn draw(&self, ps : &ParticleState) {
    //with (ps) {
      glBegin(GL_LINES);
      //let aa : f32 = ps.a;// * calcNearPlayerAlpha(pos);
      Screen::setColor(ps.r, ps.g, ps.b, ps.a);
      let p1 = self.field.calcCircularPos(ps.pos);
      Screen::glVertex(p1);
      let p2 = self.field.calcCircularPos(ps.tailPos);
      Screen::glVertex(p2);
      glEnd();
    //}
  }
}


const QPS_SLOW_DOWN_RATIO : f32 = 0.07;
const QPS_GRAVITY : f32 = 0.002;

pub struct QuadParticleSpec<'a> {
  ps : &'a ParticleSpec<'a>,
}

impl<'a> QuadParticleSpec<'a> {
  fn new(field : &Field) -> QuadParticleSpec<'a> {
    QuadParticleSpec{ ps : ParticleSpec::new(field) }
  }

  fn move1(&mut self, ps : &ParticleState) -> bool {
    //with (ps) {
      ps.pos += ps.vel;
      ps.pos.x = self.field.normalizeX(ps.pos.x);
      if ps.effectedByGravity {
        ps.vel.y -= QPS_GRAVITY;
      }
      ps.vel *= 1 - QPS_SLOW_DOWN_RATIO;
      let mut cfr = 1.0 - (1.0 / (ps.startCnt as f32));
      if cfr < 0.0 {
        cfr = 0.0;
      }
      ps.pos.r *= cfr;
      ps.pos.g *= cfr;
      ps.pos.b *= cfr;
      ps.pos.a *= cfr;
      ps.size *= 1 - (1 - cfr) * 0.5;
      ps.cnt -= 1;
      ps.cnt > 0
    //}
  }

  fn draw(&mut self, ps : &ParticleState) {
   // with (ps) {
      let mut p : Vector3;
      let sz = ps.size * 0.5;
      let aa = ps.a * self.calcNearPlayerAlpha(ps.pos);
      Screen::setColor(ps.r, ps.g, ps.b, ps.aa);
      glBegin(GL_QUADS);
      p = self.field.calcCircularPos(ps.pos.x - sz, ps.pos.y - sz);
      Screen::glVertex(p);
      p = self.field.calcCircularPos(ps.pos.x + sz, ps.pos.y - sz);
      Screen::glVertex(p);
      p = self.field.calcCircularPos(ps.pos.x + sz, ps.pos.y + sz);
      Screen::glVertex(p);
      p = self.field.calcCircularPos(ps.pos.x - sz, ps.pos.y + sz);
      Screen::glVertex(p);
      glEnd();
      glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
      Screen::setColor(0, 0, 0, aa * 0.66);
      glBegin(GL_LINE_LOOP);
      p = self.field.calcCircularPos(ps.pos.x - sz, ps.pos.y - sz);
      Screen::glVertex(p);
      p = self.field.calcCircularPos(ps.pos.x + sz, ps.pos.y - sz);
      Screen::glVertex(p);
      p = self.field.calcCircularPos(ps.pos.x + sz, ps.pos.y + sz);
      Screen::glVertex(p);
      p = self.field.calcCircularPos(ps.pos.x - sz, ps.pos.y + sz);
      Screen::glVertex(p);
      glEnd();
      glBlendFunc(GL_SRC_ALPHA, GL_ONE);
   // }
  }
}

const BPS_SLOW_DOWN_RATIO : f32 = 0.04;

pub struct BonusParticleSpec<'a> {
 ps : &'a ParticleSpec<'a>,
}

impl<'a> BonusParticleSpec<'a> {
  fn mew(&mut self, field : Field) -> BonusParticleSpec<'a> {
    BonusParticleSpec{ ps : ParticleSpec::new(field), }
  }

  fn setSize(&mut self, ps : &ParticleState, sz : f32) {
    //with (ps) {
      ps.trgSize = sz;
      ps.size = 0.1;
    //}
  }

  fn move1(&mut self, ps : &ParticleState) -> bool {
    //with (ps) {
      if ps.waitCnt > 0 {
        ps.waitCnt -= 1;
        return true;
      }
      ps.stepForward();
      ps.speed *= 1 - BPS_SLOW_DOWN_RATIO;
      self.field.addSlowdownRatio(0.01);
      ps.pos.x = self.field.normalizeX(ps.pos.x);
      let mut cfr = 1.0 - (1.0 / (ps.startCnt as f32));
      if cfr < 0.0 {
        cfr = 0.0;
      }
      ps.a *= cfr;
      ps.num += (ps.trgNum - ps.num) * 0.2;
      if (ps.trgNum - ps.num).abs() < 0.5 {
        ps.num = ps.trgNum;
      }
      ps.size += (ps.trgSize - ps.size) * 0.1;
      ps.cnt -= 1;
      ps.cnt > 0
    //}
  }

  fn draw(&self, ps : &ParticleState) {
    //with (ps) {
      if ps.waitCnt > 0 {
        return;
      }
      glPushMatrix();
      let p : Vector3 = self.field.calcCircularPos(ps.pos);
      let aa = ps.a * self.calcNearPlayerAlpha(ps.pos);
      glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
      Screen::setColor(1, 1, 1, aa * 0.5);
      Screen::glTranslate(p);
      Letter::drawNumSign(ps.num as i32, 0, 0, ps.size, 33, 0, 1);
      Screen::setColor(1, 1, 1, aa);
      Letter::drawNumSign(ps.num as i32, 0, 0, ps.size, 33, 0, 2);
      glBlendFunc(GL_SRC_ALPHA, GL_ONE);
      glPopMatrix();
    //}
  }
}

enum Shape {
  TRIANGLE, LINE, QUAD, BONUS,
}

pub struct Particle<'a> {
  //tok : Token<ParticleState, ParticleSpec>,
  _exists : bool, // _exists : bool, //inherited by Actor class
  state : &'a ParticleState,
  spec : &'a ParticleSpec<'a>,

  triangleParticleSpec : &'a TriangleParticleSpec<'a>,
  lineParticleSpec : &'a LineParticleSpec<'a>,
  quadParticleSpec : &'a QuadParticleSpec<'a>,
  bonusParticleSpec : &'a BonusParticleSpec<'a>,
}

impl<'a> Actor for Particle<'a> {
  fn getExists(&self) -> bool {
    self._exists
  }
  fn setExists(&mut self, v : bool)-> bool {
    self._exists = v;
    v
  }

  fn init(&mut self) { //, args : &Vec<Object>) {
    /*
    self.tok.init(args);
    self.triangleParticleSpec = args[0] as &TriangleParticleSpec;
    self.lineParticleSpec = args[1] as &LineParticleSpec;
    self.quadParticleSpec = args[2] as &QuadParticleSpec;
    self.bonusParticleSpec = args[3] as &BonusParticleSpec;
    */
  }

  fn move1(&self) {
    self.tok.move1();
  }

  fn draw1(&self) {
    self.tok.draw1();
  }
}

impl<'a> Particle<'a> {
  //replacement for Particle::init()
  fn new() -> Particle<'a> {
    Particle {
      tok : Token::<ParticleState, ParticleSpec>::new(), //call new() instead of init()

      //field / player must be given on method call
      triangleParticleSpec  : TriangleParticleSpec::new(),
      lineParticleSpec : LineParticleSpec::new(),
      quadParticleSpec : QuadParticleSpec::new(),
      bonusParticleSpec : BonusParticleSpec::new(),
      _exists : false,
    }
  }

  fn set(&mut self, type_ : i32,
          x : f32, y : f32, deg : f32, speed : f32, sz : f32, r : f32, g : f32, b : f32,
          c : i32 /*= 60*/, ebg : bool /*= true*/, num : f32 /*= 0*/, waitCnt : i32 /*= 0*/) {
    self.tok.spec = match type_ {
      Shape::TRIANGLE => self.triangleParticleSpec,
      Shape::LINE => self.lineParticleSpec,
      Shape::QUAD => self.quadParticleSpec,
      Shape::BONUS => self.bonusParticleSpec,
    };
    self.tok.set(x, y, deg, speed);
    self.state.size = sz;
    self.state.vel.x = -deg.sin() * speed;
    self.state.vel.y = deg.cos() * speed;
    self.state.r = r;
    self.state.g = g;
    self.state.b = b;
    self.state.cnt = c;
    self.state.startCnt = c;
    self.state.effectedByGravity = ebg;
    self.state.trgNum = num;
    self.state.waitCnt = waitCnt;
    if type_ == Shape::BONUS {
      (self.tok.spec as &BonusParticleSpec).setSize(&self.state, sz);
    }
  }

  fn setByVelocity(&mut self, x : f32, y : f32, vx : f32, vy : f32,
                            sz : f32, r : f32, g : f32, b : f32, a : f32,
                            c : i32 /*= 60*/, ebg : bool /* = true*/) {
    self.tok.spec = &self.triangleParticleSpec;
    self.tol.set(x, y, 0.0, 0.0);
    self.state.vel.x = vx;
    self.state.vel.y = vy;
    self.state.size = sz;
    self.state.r = r;
    self.state.g = g;
    self.state.b = b;
    self.state.a = a;
    self.state.cnt = c;
    self.state.startCnt = c;
    self.state.effectedByGravity = ebg;
  }
}

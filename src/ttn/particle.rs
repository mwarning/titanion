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
  ap : ActorPoolData<Particle<'a>>,
}

impl<'a> ParticlePool<'a> {
  pub fn new() -> ParticlePool<'a> {
    ParticlePool { ap : ActorPoolData::<Particle<'a>>::new() }
  }
}

impl<'a> ActorPool<Particle<'a>> for ParticlePool<'a> {
  fn getActorPoolData(&mut self) -> &mut ActorPoolData<Particle<'a>> {
    &mut self.ap
  }
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
      vel : Vector::new(0.0, 0.0),
      tailPos : Vector::new(0.0 ,0.0),
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

impl<'a> TokenSpec<ParticleState> for ParticleSpec<'a> {
  fn set(&self, state : &ParticleState) {}
  fn removed(&self, state : &ParticleState) {}

  fn move2(&self, state : &ParticleState) -> bool {
    true
  }

  fn draw(&self, state : &ParticleState) {
    //with (state) {
      let p = self.field.calcCircularPos1(state.ts.pos);
      let cd = Field::calcCircularDeg(state.ts.pos.x);
      self.shape.draw(p, cd, state.ts.deg);
    //}
  }
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

    let pd = self.player.pos().dist(pos);
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
  pub fn new(field : &'a Field<'a>) -> TriangleParticleSpec<'a> {
    TriangleParticleSpec {
      field : field,
      particleShape : TriangleParticleShape::new(),
    }
  }

  pub fn setParticles(&mut self, particles : &ParticlePool) {
    self.particles = particles;
  }

  pub fn set(&mut self, ps : &ParticleState) {
    ps.d1 = self.ps.rand.nextFloat(PI * 2.0);
    ps.d2 = self.ps.rand.nextFloat(PI * 2.0);
    ps.vd1 = self.ps.rand.nextSignedFloat(0.1);
    ps.vd2 = self.ps.rand.nextSignedFloat(0.1);
  }

  pub fn move2(&mut self, ps : &ParticleState) -> bool {
    //with (ps) {
      ps.ts.pos += ps.vel;
      ps.ts.pos.x = Field::normalizeX(ps.ts.pos.x);
      if ps.effectedByGravity {
        ps.vel.y -= TPS_GRAVITY;
      }
      ps.vel *= 1.0 - TPS_SLOW_DOWN_RATIO;
      ps.d1 += ps.vd1;
      ps.d2 += ps.vd2;
      ps.vd1 *= 1.0 - TPS_SLOW_DOWN_RATIO * 0.2;
      ps.vd2 *= 1.0 - TPS_SLOW_DOWN_RATIO * 0.2;
      let cfr = 1.0 - (1.0 / (ps.startCnt as f32));
      if cfr < 0.0 {
        cfr = 0.0;
      }
      ps.r *= cfr;
      ps.g *= cfr;
      ps.b *= cfr;
      ps.a *= cfr;
      let rand = &mut self.ps.rand;
      let fs = if (ps.size > 2.0) && (rand.nextInt(45) == 0) {
        0.5 - rand.nextFloat(0.2)
      } else if (ps.size > 0.5) && (rand.nextInt(10) == 0) {
        0.1 + rand.nextSignedFloat(0.05)
      } else {
        0.0
      };
      if fs > 0.0 {
        let vx = ps.vel.x * rand.nextSignedFloat(0.8);
        let vy = ps.vel.y * rand.nextSignedFloat(0.8);
        ps.vel.x -= vx * fs;
        ps.vel.y -= vy * fs;
        let cr = 1.0 - fs * 0.2;
        ps.vel /= cr;
        let p = self.particles.getInstanceForced();
        let nc = ((ps.cnt as f32) * (0.8 + fs * 0.2)) as i32;
        if nc > 0 {
          p.setByVelocity(ps.ts.pos.x, ps.ts.pos.y, vx, vy, ps.size * fs, ps.r, ps.g, ps.b, ps.a, nc, ps.effectedByGravity);
        }
        ps.size *= 1.0 - fs;
        ps.cnt *= cr as i32;
      };
      ps.cnt -= 1;
      ps.cnt > 0
    //}
  }

  pub fn draw(&self, ps : &ParticleState) {
    //with (ps) {
      let p = self.field.calcCircularPos1(ps.ts.pos);
      let aa = ps.a * self.ps.calcNearPlayerAlpha(ps.ts.pos);
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
      ps.tailPos.x = ps.ts.pos.x;
      ps.tailPos.y = ps.ts.pos.y;
    //}
  }

  fn move2(&mut self, ps : &ParticleState) -> bool {
    //with (ps) {
      ps.ts.stepForward();
      ps.tailPos.x += (ps.ts.pos.x - ps.tailPos.x) * 0.05;
      ps.tailPos.y += (ps.ts.pos.y - ps.tailPos.y) * 0.05;
      ps.ts.speed *= 1.0 - LPS_SLOW_DOWN_RATIO;
      ps.ts.pos.x = Field::normalizeX(ps.ts.pos.x);
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
      //let aa : f32 = ps.a;// * calcNearPlayerAlpha(pos());
      Screen::setColor(ps.r, ps.g, ps.b, ps.a);
      let p1 = self.field.calcCircularPos1(ps.ts.pos);
      Screen::glVertex(p1);
      let p2 = self.field.calcCircularPos1(ps.tailPos);
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
      ps.ts.pos += ps.vel;
      ps.ts.pos.x = Field::normalizeX(ps.ts.pos.x);
      if ps.effectedByGravity {
        ps.vel.y -= QPS_GRAVITY;
      }
      ps.vel *= 1.0 - QPS_SLOW_DOWN_RATIO;
      let mut cfr = 1.0 - (1.0 / (ps.startCnt as f32));
      if cfr < 0.0 {
        cfr = 0.0;
      }
      ps.r *= cfr;
      ps.g *= cfr;
      ps.b *= cfr;
      ps.a *= cfr;
      ps.size *= 1.0 - (1.0 - cfr) * 0.5;
      ps.cnt -= 1;
      ps.cnt > 0
    //}
  }

  fn draw(&mut self, ps : &ParticleState) {
   // with (ps) {
      let mut p : Vector3;
      let sz = ps.size * 0.5;
      let pos = ps.ts.pos;
      let aa = ps.a * self.ps.calcNearPlayerAlpha(pos);
      Screen::setColor(ps.r, ps.g, ps.b, ps.aa);
      glBegin(GL_QUADS);
      p = self.field.calcCircularPos(pos.x - sz, pos.y - sz);
      Screen::glVertex3(p);
      p = self.field.calcCircularPos(pos.x + sz, pos.y - sz);
      Screen::glVertex3(p);
      p = self.field.calcCircularPos(pos.x + sz, pos.y + sz);
      Screen::glVertex3(p);
      p = self.field.calcCircularPos(pos.x - sz, pos.y + sz);
      Screen::glVertex3(p);
      glEnd();
      glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
      Screen::setColor(0.0, 0.0, 0.0, aa * 0.66);
      glBegin(GL_LINE_LOOP);
      p = self.field.calcCircularPos(pos.x - sz, pos.y - sz);
      Screen::glVertex3(p);
      p = self.field.calcCircularPos(pos.x + sz, pos.y - sz);
      Screen::glVertex3(p);
      p = self.field.calcCircularPos(pos.x + sz, pos.y + sz);
      Screen::glVertex3(p);
      p = self.field.calcCircularPos(pos.x - sz, pos.y + sz);
      Screen::glVertex3(p);
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

  pub fn move1(&mut self, ps : &ParticleState) -> bool {
    //with (ps) {
      if ps.waitCnt > 0 {
        ps.waitCnt -= 1;
        return true;
      }
      ps.ts.stepForward();
      ps.ts.speed *= 1.0 - BPS_SLOW_DOWN_RATIO;
      self.field.addSlowdownRatio(0.01);
      ps.ts.pos.x = Field::normalizeX(ps.ts.pos.x);
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

  pub fn draw(&self, ps : &ParticleState) {
    //with (ps) {
      if ps.waitCnt > 0 {
        return;
      }
      let letter = self.frame.letter.borrow();
      glPushMatrix();
      let p = self.field.calcCircularPos1(ps.ts.pos);
      let aa = ps.a * self.ps.calcNearPlayerAlpha(ps.ts.pos);
      glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
      Screen::setColor(1.0, 1.0, 1.0, aa * 0.5);
      Screen::glTranslate3(p);
      letter.drawNumSign(ps.num as i32, 0.0, 0.0, ps.size, 33, 0, 1);
      Screen::setColor(1.0, 1.0, 1.0, aa);
      letter.drawNumSign(ps.num as i32, 0.0, 0.0, ps.size, 33, 0, 2);
      glBlendFunc(GL_SRC_ALPHA, GL_ONE);
      glPopMatrix();
    //}
  }
}

// was enum Shape
#[derive(PartialEq, Eq)]
pub enum ParticleShape {
  TRIANGLE, LINE, QUAD, BONUS,
}

pub struct Particle<'a> {
  //tok : Token<ParticleState, ParticleSpec>,
  _exists : bool, // _exists : bool, //inherited by Actor class
  state : ParticleState,
  spec : &'a ParticleSpec<'a>,

  triangleParticleSpec : &'a TriangleParticleSpec<'a>,
  lineParticleSpec : &'a LineParticleSpec<'a>,
  quadParticleSpec : &'a QuadParticleSpec<'a>,
  bonusParticleSpec : &'a BonusParticleSpec<'a>,
}

// methods inlined from Token.
impl<'a> Token<ParticleState, ParticleSpec<'a>> for Particle<'a> {
/*
  fn getExists(&self) -> bool {
    self._exists
  }

  fn setExists(&mut self, v : bool) -> bool {
    self._exists = v;
    v
  }
*/

  fn set5Vec(&mut self, spec : &ParticleSpec, pos : Vector, deg : f32, speed : f32) {
    self.spec = spec;
    self.set5(pos.x, pos.y, deg, speed);
  }

  fn set6(&mut self, spec : &ParticleSpec, x : f32, y : f32, deg : f32, speed : f32) {
    self.spec = spec;
    self.set5(x, y, deg, speed);
  }

  fn set5(&mut self, x : f32, y : f32, deg : f32, speed : f32) {
    self.state.clear();
    self.state.ts.pos.x = x;
    self.state.ts.pos.y = y;
    self.state.ts.deg = deg;
    self.state.ts.speed = speed;
    self.spec.set(&self.state);
    self._exists = true;
  }

  fn remove(&mut self) {
    self._exists = false;
    self.spec.removed(&self.state);
  }

  fn pos(&self) -> Vector {
    self.state.ts.pos
  }
}

impl<'a> Actor for Particle<'a> {
  fn new() -> Particle<'a> {
    Particle {
      _exists : false,
      state : ParticleState::new(),
      spec : ParticleSpec::new(),  //use generic spec or Option type?
    }
  }

  fn getExists(&self) -> bool {
    self._exists
  }

  fn setExists(&mut self, v : bool) -> bool {
    self._exists = v;
    v
  }

  fn init(&mut self /*Object[] args*/) {
    self.state = ParticleState::new();
    /*
    //moved to new()
    self.init(args);
    self.triangleParticleSpec = args[0] as &TriangleParticleSpec;
    self.lineParticleSpec = args[1] as &LineParticleSpec;
    self.quadParticleSpec = args[2] as &QuadParticleSpec;
    self.bonusParticleSpec = args[3] as &BonusParticleSpec;
    */
  }

  fn move1(&self) {
    if !self.spec.move2(&self.state) {
      self.remove();
    }
  }

  fn draw1(&self) {
    self.spec.draw(&self.state);
  }
}

impl<'a> Particle<'a> {
  //replacement for Particle::init()
  pub fn new(state : &'a ParticleState, spec : &'a ParticleSpec<'a>) -> Particle<'a> {
    Particle {
      // inlined
      //tok : Token::<ParticleState, ParticleSpec>::new(), //call new() instead of init()
      spec : spec,
      state : state,

      //field / player must be given on method call
      triangleParticleSpec  : TriangleParticleSpec::new(),
      lineParticleSpec : LineParticleSpec::new(),
      quadParticleSpec : QuadParticleSpec::new(),
      bonusParticleSpec : BonusParticleSpec::new(),
      _exists : false,
    }
  }

  pub fn set(&mut self, type_ : ParticleShape /*i32*/,
          x : f32, y : f32, deg : f32, speed : f32, sz : f32, r : f32, g : f32, b : f32,
          c : i32 /*= 60*/, ebg : bool /*= true*/, num : f32 /*= 0*/, waitCnt : i32 /*= 0*/) {
    self.spec = match type_ {
      ParticleShape::TRIANGLE => self.triangleParticleSpec,
      ParticleShape::LINE => self.lineParticleSpec,
      ParticleShape::QUAD => self.quadParticleSpec,
      ParticleShape::BONUS => self.bonusParticleSpec,
    } as &ParticleSpec;
    self.set5(x, y, deg, speed);
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
    if type_ == ParticleShape::BONUS {
      (&mut self.spec as &mut BonusParticleSpec).setSize(&self.state, sz);
    }
  }

  pub fn setByVelocity(&mut self, x : f32, y : f32, vx : f32, vy : f32,
                            sz : f32, r : f32, g : f32, b : f32, a : f32,
                            c : i32 /*= 60*/, ebg : bool /* = true*/) {
    self.spec = self.triangleParticleSpec as &'a ParticleSpec;
    self.set5(x, y, 0.0, 0.0);
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

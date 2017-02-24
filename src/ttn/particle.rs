/*
 * $Id: particle.d,v 1.3 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.particle;


private import std.math;

private import derelict.opengl.gl;

private import src.util.vector;
private import src.util.math;
private import src.util.actor;
private import src.util.rand;
private import src.ttn.token;
private import src.ttn.field;
private import src.ttn.screen;
private import src.ttn.shape;
private import src.ttn.letter;
private import src.ttn.player;
*/

/**
 * Particles (Triangle / Line / Quad / Bonus).
 */
//public class ParticlePool: ActorPool!(Particle) {
//}

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
  fn this(&mut self) {
    //super();
    self.vel = Vector();
    self.tailPos = Vector();
  }

  fn clear(&mut self) {
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
  }
}

//public class ParticleSpec: TokenSpec!(ParticleState) {
struct ParticleSpec {
  ts : TokenSpec<ParticleState>, //inherited
  //mixin StaticRandImpl;

  player : Player;
}

impl ParticleSpec {
  fn setPlayer(&mut self, player : Player) {
    self.player = player;
  }

  fn calcNearPlayerAlpha(pos : Vector) -> f32 {
    if !self.player.isActive {
      return 1.0;
    }

    let pd : f32 = player.pos.dist(pos);
    if pd < 20.0 {
      pd / 20.0;
    } else {
      1.0
    }
  }
}

static SLOW_DOWN_RATIO : f32 = 0.05;
static GRAVITY : f32 = 0.003;

struct TriangleParticleSpec {
  ps ParticleSpec; //inherited
  particleShape : Shape;
  particles : ParticlePool;
}

impl TriangleParticleSpec
  fn this(&mut self, field : Field) {
    self.field = field;
    self.particleShape = TriangleParticleShape();
  }

  fn setParticles(&mut self, particles : ParticlePool) {
    self.particles = particles;
  }

  fn set(ps : &ParticleState) {
      ps.d1 = self.rand.nextFloat(PI * 2.0);
      ps.d2 = self.rand.nextFloat(PI * 2.0);
      ps.vd1 = self.rand.nextSignedFloat(0.1);
      ps.vd2 = self.rand.nextSignedFloat(0.1);
  }

  fn move(&mut self, ps : ParticleState) -> bool {
    //with (ps) {
      pos += vel;
      pos.x = self.field.normalizeX(pos.x);
      if effectedByGravity {
        vel.y -= GRAVITY;
      }
      vel *= (1.0 - SLOW_DOWN_RATIO);
      d1 += vd1;
      d2 += vd2;
      vd1 *= (1.0 - SLOW_DOWN_RATIO * 0.2);
      vd2 *= (1.0 - SLOW_DOWN_RATIO * 0.2);
      let cfr : f32 = 1.0 - (1.0 / (startCnt as f32);
      if cfr < 0 {
        cfr = 0;
      }
      r *= cfr;
      g *= cfr;
      b *= cfr;
      a *= cfr;
      let mut fs : f32 = 0.0;
      if (size > 2.0) && (self.rand.nextInt(45) == 0) {
        fs = 0.5f - self.rand.nextFloat(0.2);
      } else if (size > 0.5f && self.rand.nextInt(10) == 0) {
        fs = 0.1f + self.rand.nextSignedFloat(0.05);
      }
      if fs > 0 {
        let mut vx : f32 = vel.x * self.rand.nextSignedFloat(0.8f);
        let mut vy : f32 = vel.y * self.rand.nextSignedFloat(0.8f);
        vel.x -= vx * fs;
        vel.y -= vy * fs;
        let cr : f32 = 1.0 - fs * 0.2;
        vel /= cr;
        let p : Particle = particles.getInstanceForced();
        let nc : i32 = (cnt * (0.8 + fs * 0.2)) as i32;
        if nc > 0 {
          p.setByVelocity(pos.x, pos.y, vx, vy, size * fs, r, g, b, a,
                          nc, effectedByGravity);
        }
        size *= (1 - fs);
        cnt *= cr;
      }
      cnt -= 1;
      cnt > 0
      }
    }
  }

  fn draw(ps : ParticleState) {
    //with (ps) {
      let p : Vector3 = field.calcCircularPos(pos);
      let aa : f32 = a * calcNearPlayerAlpha(pos);
      Screen.setColor(r, g, b, aa);
      particleShape.draw(p, d1, d2);
    }
  }
}

static SLOW_DOWN_RATIO : f32 = 0.03;

struct LineParticleSpec {
  ps : ParticleSpec;
}

impl LineParticleSpec {
  fn this(&mut self, field : Field) {
    self.field = field;
  }

  fn set(&self, ps : ParticleState) {
    //with (ps) {
      ps.tailPos.x = self.pos.x;
      ps.tailPos.y = self.pos.y;
    //}
  }


  fn move(ps : ParticleState) -> bool {
    //with (ps) {
      ps.stepForward();
      tailPos.x += (pos.x - tailPos.x) * 0.05;
      tailPos.y += (pos.y - tailPos.y) * 0.05;
      speed *= (1 - SLOW_DOWN_RATIO);
      pos.x = field.normalizeX(pos.x);
      let cfr : f32 = 1.0 - (1.0 / (startCnt as f32);
      if (cfr < 0) {
        cfr = 0;
      }
      r *= cfr;
      g *= cfr;
      b *= cfr;
      a *= cfr;
      cnt -= 1;
      (cnt > 0)
    //}
  }

  fn draw(&self, ps : ParticleState) {
    //with (ps) {
      let mut p : Vector3;
      glBegin(GL_LINES);
      let aa : f32 = a;// * calcNearPlayerAlpha(pos);
      Screen.setColor(r, g, b, aa);
      p = self.field.calcCircularPos(pos);
      Screen.glVertex(p);
      p = self.field.calcCircularPos(tailPos);
      Screen.glVertex(p);
      glEnd();
    //}
  }
}


static SLOW_DOWN_RATIO : f32 = 0.07;
static GRAVITY : f32 = 0.002;

struct QuadParticleSpec {
  ps : ParticleSpec,
}

impl QuadParticleSpec {
  fn this(f&mut self, field : Field) {
    self.field = field;
  }

  fn move(ps : ParticleState) -> bool {
    //with (ps) {
      pos += vel;
      pos.x = field.normalizeX(pos.x);
      if effectedByGravity {
        vel.y -= GRAVITY;
      }
      vel *= (1 - SLOW_DOWN_RATIO);
      let cfr : f32 = 1.0 - (1.0 / (startCnt as f32);
      if (cfr < 0)
        cfr = 0;
      r *= cfr;
      g *= cfr;
      b *= cfr;
      a *= cfr;
      size *= (1 - (1 - cfr) * 0.5f);
      cnt -= 1;
      (cnt > 0)
    //}
  }

  fn draw(ps : ParticleState) {
   // with (ps) {
      let mut p : Vector3;
      let sz : f32 = size * 0.5f;
      let aa : f32 = a * calcNearPlayerAlpha(pos);
      Screen.setColor(r, g, b, aa);
      glBegin(GL_QUADS);
      p = self.field.calcCircularPos(pos.x - sz, pos.y - sz);
      Screen.glVertex(p);
      p = self.field.calcCircularPos(pos.x + sz, pos.y - sz);
      Screen.glVertex(p);
      p = self.field.calcCircularPos(pos.x + sz, pos.y + sz);
      Screen.glVertex(p);
      p = self.field.calcCircularPos(pos.x - sz, pos.y + sz);
      Screen.glVertex(p);
      glEnd();
      glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
      Screen.setColor(0, 0, 0, aa * 0.66f);
      glBegin(GL_LINE_LOOP);
      p = self.field.calcCircularPos(pos.x - sz, pos.y - sz);
      Screen.glVertex(p);
      p = self.field.calcCircularPos(pos.x + sz, pos.y - sz);
      Screen.glVertex(p);
      p = self.field.calcCircularPos(pos.x + sz, pos.y + sz);
      Screen.glVertex(p);
      p = self.field.calcCircularPos(pos.x - sz, pos.y + sz);
      Screen.glVertex(p);
      glEnd();
      glBlendFunc(GL_SRC_ALPHA, GL_ONE);
   // }
  }
}

static SLOW_DOWN_RATIO : f32 = 0.04;

struct BonusParticleSpec {
 ps : ParticleSpec,
}

impl BonusParticleSpec {
  fn this(&mut self, field : Field) {
    self.field = field;
  }

  fn setSize(&mut self, ps : ParticleState, sz : f32) {
    //with (ps) {
      trgSize = sz;
      size = 0.1;
    //}
  }

  fn move(ps : ParticleState) -> bool {
    //with (ps) {
      if waitCnt > 0 {
        waitCnt -= 1;
        return true;
      }
      ps.stepForward();
      speed *= (1 - SLOW_DOWN_RATIO);
      field.addSlowdownRatio(0.01);
      pos.x = field.normalizeX(pos.x);
      float cfr = 1.0f - (1.0f / (startCnt as f32));
      if cfr < 0 {
        cfr = 0;
      }
      a *= cfr;
      num += (trgNum - num) * 0.2;
      if (trgNum - num).abs() < 0.5 {
        num = trgNum;
      }
      size += (trgSize - size) * 0.1;
      cnt -= 1;
      /cnt > 0
    //}
  }

  fn draw(&self, ps : ParticleState) {
    //with (ps) {
      if waitCnt > 0 {
        return;
      }
      glPushMatrix();
      let p : Vector3 = field.calcCircularPos(pos);
      let aa : f32 = a * calcNearPlayerAlpha(pos);
      glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
      Screen.setColor(1, 1, 1, aa * 0.5);
      Screen.glTranslate(p);
      Letter.drawNumSign(num as i32, 0, 0, size, 33, 0, 1);
      Screen.setColor(1, 1, 1, aa);
      Letter.drawNumSign(num as i32, 0, 0, size, 33, 0, 2);
      glBlendFunc(GL_SRC_ALPHA, GL_ONE);
      glPopMatrix();
    //}
  }
}

enum Shape {
  TRIANGLE, LINE, QUAD, BONUS,
};

struct Particle {
  tok : Token<ParticleState, ParticleSpec>,
  triangleParticleSpec : TriangleParticleSpec,
  lineParticleSpec : LineParticleSpec,
  quadParticleSpec : QuadParticleSpec,
  bonusParticleSpec : BonusParticleSpec,
  _exists : bool, //inherited by Actor class
}

impl Actor for Particle {
  fn getExists(&self) -> bool {
    self._exists
  }
  fn setExists(&mut self, v : bool)-> bool {
    self._exists = v;
    v
  }
  /*
  fn init(&mut self) { //, args : &[Object]) {
    self.tok.init()
  }*/
  fn init(&mut self, args : &Vec<Object>) {
    self.tok.init(args);
    self.triangleParticleSpec = args[0] as TriangleParticleSpec;
    self.lineParticleSpec = args[1]as LineParticleSpec;
    self.quadParticleSpec = args[2] as QuadParticleSpec;
    self.bonusParticleSpec = args[3] as BonusParticleSpec;
  }

  fn move1(&self) {
    self.tok.move1();
  }

  fn draw1(&self) {
    self.tok.draw1();
  }
}

impl Particle {
  fn set(&mut self,type : i32,
          x : f32, y : f32, deg : f32, speed : f32, sz : f32, r : f32, g : f32, b : f32,
          c : i32 /*= 60*/, ebg : bool /*= true*/, num : f32 /*= 0*/, waitCnt : i32 /*= 0*/) {
    self.spec = match type {
      Shape.TRIANGLE => triangleParticleSpec,
      Shape.LINE => lineParticleSpec,
      Shape.QUAD => quadParticleSpec,
      Shape.BONUS =>bonusParticleSpec,
    }
    self.tok.set(x, y, deg, speed);
    self.state.size = sz;
    self.state.vel.x = -(deg.sin()) * speed;
    self.state.vel.y = deg.cos() * speed;
    self.state.r = r;
    self.state.g = g;
    self.state.b = b;
    self.state.cnt = state.startCnt = c;
    self.state.effectedByGravity = ebg;
    self.state.trgNum = num;
    self.state.waitCnt = waitCnt;
    if type == Shape.BONUS {
      (spec as BonusParticleSpec).setSize(state, sz);
    }
  }

  fn setByVelocity(&mut self, x : f32, y : f32, vx : f32, vy : f32,
                            sz : f32, r : f32, g : f32, b : f32, a : f32,
                            c : i32 /*= 60*/, ebg : bool /* = true*/) {
    self.tok.spec = triangleParticleSpec;
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

/*
 * $Id: enemy.d,v 1.7 2006/12/09 04:17:40 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.enemy;


private import derelict.opengl.gl;

private import src.util.rand;
private import src.util.vector;
private import src.util.actor;
private import src.util.math;
private import src.ttn.token;
private import src.ttn.field;
private import src.ttn.shape;
private import src.ttn.bullet;
private import src.ttn.player;
private import src.ttn.particle;
private import src.ttn.stage;
private import src.ttn.screen;
private import src.ttn.sound;
private import src.ttn.frame;
*/

use std::f32::consts::PI;

use util::vector::*;
use util::actor::*;
use ttn::token::*;
use ttn::shape::*;
use util::math::*;
use ttn::bullet::*;
use ttn::field::*;
use ttn::dummy::*;


//######################

/**
 * Enemies and turrets.
 */

static trailEffect : bool = false;

pub struct EnemyPool {
  ap : ActorPool<Enemy>,
  _field : Field,
}

impl EnemyPool {
  fn getNearestEnemy(&self, p : Vector) -> Option<&Enemy> {
    let dst : f32 = 99999.0;
    let ne : Option<&Enemy> = None;
    for e in self.ap.actors {
      if e.getExists() && !e.isBeingCaptured() {
        if self._field.calcCircularDist2(e.pos(), p) < dst {
          dst = self._field.calcCircularDist2(e.pos(), p);
          ne = Some(&e);
        }
      }
    }
    ne
  }

  fn getNearestMiddleEnemy(&self, p: Vector) -> Option<&Enemy> {
    let dst : f32 = 99999.0;
    let ne : Option<&Enemy> = None;
    for e in self.ap.actors {
      if e.getExists() && !e.isBeingCaptured() {
        if e.spec as MiddleEnemySpec {
          if self._field.calcCircularDist2(e.pos(), p) < dst {
            dst = self._field.calcCircularDist2(e.pos(), p);
            ne = Some(&e);
          }
        }
      }
    }
    ne
  }

  fn checkShotHit(&self, p : Vector, deg : f32, widthRatio : f32 /*= 1.0*/) -> bool {
    let e : Enemy = self.getNearestEnemy(p);
    if e {
      let ox : f32 = self.self._field.normalizeX(e.pos().x - p.x);
      let oy : f32 = e.pos().y - p.y;
      if (ox.abs() < 1.0) * e.state.size.x && oy.abs() < (1.0 * e.state.size.y * widthRatio) {
        e.hitShot(deg);
        return true
      }
    }
    false
  }

  fn checkBulletHit(&self, p : Vector, pp : Vector) -> bool {
    let hitf : bool = false;
    for e in self.actors {
      if e.exists && e.isCaptured {
        if self.self._field.checkHitDist(e.pos(), p, pp, EnemySpec::BULLET_HIT_WIDTH) {
          e.hitCaptured();
          hitf = true;
        }
      }
    }
    hitf
  }

  fn checkEnemyHit(&self, p : Vector, size : Vector) -> bool {
    let hitf : bool= false;
    for e in self.actors {
      if e.getExists() && e.isCaptured() {
        let ox = self.self._field.normalizeX(e.pos().x - p.x);
        let oy = e.pos().y - p.y;
        if ox.abs() < 0.5 * (e.state.size.x + size.x) &&
            oy.abs() < 0.5 * (e.state.size.y + size.y) {
          e.hitCaptured();
          hitf = true;
        }
      }
    }
    hitf
  }

  fn checkMiddleEnemyExists(&self, x : f32, px : f32) -> bool {
    for e in self.actors {
      if e.exists && !e.isBeingCaptured {
        if e.spec as MiddleEnemySpec {
          if ((e.pos().x - x) * (e.pos().x - px)) < 0.0 {
            return true
          }
        }
      }
    }
    false
  }

  fn num(&self) -> i32 {
    let mut n : i32 = 0;
    for e in self.actors {
      if e.exists && !e.isCaptured {
        n += 1;
      }
    }
    n
  }

  fn numInAttack(&self) -> i32 {
    let mut n = 0;
    for e in self.actors {
      if e.exists && e.isInAttack {
        n += 1;
      }
    }
    n
  }

  fn numInScreen(&self) -> i32 {
    let mut n = 0;
    for e in self.actors {
      if e.exists && e.isInScreen {
        n += 1;
      }
    }
    n
  }

  fn numBeforeAlign(&self) -> i32 {
    let mut n = 0;
    for e in self.actors {
      if e.exists && e.beforeAlign {
        n += 1;
      }
    }
    n
  }

  fn drawFront(&self) {
    if self.trailEffect {
      for a in self.actors {
        if a.exists && (a.state.pos.y <= (self._field.size.y * 1.5)) {
          a.drawTrails();
        }
      }
    }
    for a in self.actors {
      if a.exists && (a.state.pos.y <= (self._field.size.y * 1.5)) {
        a.draw();
      }
    }
  }

  fn drawBack(&self) {
    if self.trailEffect {
      for a in self.actors {
        if a.exists &&
            a.state.pos.y > self._field.size.y * 1.5 &&
            (a.state.pos.x <= self._field.circularDistance / 4 &&
             a.state.pos.x >= -self._field.circularDistance / 4) {
          a.drawTrails();
        }
      }
    }
    for a in self.actors {
      if a.exists &&
          a.state.pos.y > self._field.size.y * 1.5 &&
          (a.state.pos.x <= self._field.circularDistance / 4 &&
           a.state.pos.x >= -self._field.circularDistance / 4) {
        a.draw();
      }
    }
  }

  fn drawPillarBack(&self) {
    if trailEffect {
      for a in self.actors {
        if a.exists &&
            a.state.pos.y > self._field.size.y * 1.5 &&
            (a.state.pos.x > self._field.circularDistance / 4 ||
             a.state.pos.x < -self._field.circularDistance / 4) {
          a.drawTrails();
        }
      }
    }
    for a in self.actors {
      if a.exists &&
          a.state.pos.y > self._field.size.y * 1.5 &&
          (a.state.pos.x > self._field.circularDistance / 4 ||
           a.state.pos.x < -self._field.circularDistance / 4) {
        a.draw();
      }
    }
  }

  fn field(&self, v : Field) -> Field {
    self._field = v;
    v
  }
}

struct Enemy {
  tok : Token<EnemyState, EnemySpec>,
  _exists : bool, //inherited by Actor class
}

impl Actor for Enemy {
  fn getExists(&self) -> bool {
    self._exists
  }
  fn setExists(&mut self, v : bool)-> bool {
    self._exists = v;
    v
  }

  fn init(&mut self) { //, args : &[Object]) {
    self.tok.init()
  }

  fn move1(&self) {
    self.tok.move1();
  }

  fn draw1(&self) {
    self.tok.draw1();
  }
}

impl Enemy {
  /*
  //moved to Actor
  fn init(&mut self, args : &[Object]) {
    self.tok.init(args);
    self.tok.state.enemy = self;
  }
  */

  fn setSmallEnemyState(&mut self, baseSpeed : f32, angVel : f32, waitCnt : i32, appPattern : i32,
                                er : f32 /*= 0*/, ed : f32 /*= 0*/, gd : bool /*= false*/,
                                fireIntervalRatio : f32 /*= 0*/, firstEnemy : Enemy /*= null*/) {
    self.tok.state.baseBaseSpeed = baseSpeed;
    self.tok.state.baseSpeed = baseSpeed;
    self.tol.state.baseAngVel = angVel;
    self.tok.state.angVel = angVel;
    self.tok.state.waitCnt = waitCnt;
    self.tok.state.ellipseRatio = er;
    self.tok.state.ellipseDeg = ed;
    self.tok.state.isGoingDownBeforeStandBy = gd;
    match appPattern {
      0 => {
        self.tok.state.phase = -200;
      }
      1 => {
        self.tok.state.phase = -100;
      }
    }

    if firstEnemy {
      (self.tok.spec as SmallEnemySpec).init(self.tok.state, firstEnemy.state);
      self.tok.state.isFirstEnemy = false;
    } else {
      self.tok.spec.init(self.tok.state);
      self.tok.state.isFirstEnemy = true;
    }
  }

  fn setMiddleEnemyState(&mut self, baseSpeed : f32, angVel : f32,
                                er : f32 /* = 0*/, ed : f32 /*= 0*/) {
    self.tok.state.baseBaseSpeed = baseSpeed;
    self.tok.state.baseSpeed = baseSpeed;
    self.tok.state.baseAngVel = angVel;
    self.tok.state.angVel= angVel;
    self.tok.state.ellipseRatio = er;
    self.tok.state.ellipseDeg = ed;
    self.tok.spec.init(self.tok.state);
  }

  fn setGhostEnemyState(&mut self, x : f32, y : f32, deg : f32, cnt : i32) {
    self.tok.state.pos.x = x;
    self.tok.state.pos.y = y;
    self.tok.state.deg = deg;
    self.tok.state.cnt = cnt;
  }

  fn hitShot(&self, deg : f32 /*= 0*/) {
    if self.tok.spec.hitShot(self.tok.state, deg) {
      self.tok.remove();
    }
  }

  fn hitCaptured(&mut self) {
    let ses : SmallEnemySpec = self.tok.spec as SmallEnemySpec;
    if ses {
      ses.hitCaptured(self.state);
    }
  }

  fn destroyed(&mut self) {
    self.tok.spec.destroyed(self.tok.state);
    self.tok._exists = false;
  }

  fn isInAttack(&mut self) -> bool {
    if self.tok.spec.isBeingCaptured(self.tok.state) {
      return false;
    }
    self.tok.spec.isInAttack(self.tok.state)
  }

  fn isInScreen(&self) -> bool {
    if self.tok.spec.isBeingCaptured(self.tok.state) {
      return false;
    }
    self.tok.spec.isInScreen(self.tok.state);
  }

  fn isBeingCaptured(&self) -> bool {
    self.tok.spec.isBeingCaptured(self.tok.state)
  }

  fn isCaptured(&self) -> bool {
    let ges : GhostEnemySpec = self.tok.spec as GhostEnemySpec;
    if ges {
      return true;
    }
    let ses : SmallEnemySpec = self.tok.spec as SmallEnemySpec;
    if !ses {
      return false;
    }
    ses.isCaptured(self.tok.state)
  }

  fn beforeAlign(&self) -> bool {
    if self.tok.spec.isBeingCaptured(self.tok.state) {
      return false;
    }
    self.tok.spec.beforeAlign(self.tok.state)
  }

  fn drawTrails(&self) {
    self.tok.spec.drawTrails(self.tok.state);
  }

  fn pos(&self) -> Vector {
    self.tok.state.pos
  }
}

const TRAIL_NUM : usize = 64;
const TRAIL_INTERVAL : i32 = 8;
const TURRET_MAX_NUM2 : usize = 3;

struct EnemyState {
  ts : TokenState,
  turretStates : [TurretState; TURRET_MAX_NUM2],
  enemy : *mut Enemy,
  vel : Vector,
  centerPos : Vector,
  centerVel : Vector,
  standByPos : Vector,
  baseBaseSpeed : f32,
  baseSpeed : f32,
  baseAngVel : f32,
  angVel : f32,
  waitCnt : i32,
  cnt : i32,
  ellipseRatio : f32,
  ellipseDeg : f32, 
  shield : f32,
  phase : i32,
  phaseCnt : i32,
  nextPhaseCnt : i32,
  captureState : i32,
  captureIdx : i32,
  isGoingDownBeforeStandBy : bool,
  size : Vector,
  targetSize : Vector,
  sizeVel : Vector,
  trails : Vec<Trail>,
  trailIdx : i32,
  trailLooped : bool,
  isFirstEnemy : bool,
  anger : f32,
}
/*
  invariant() {
    if (isInitialized) {
      assert(baseBaseSpeed >= 0);
      assert(baseSpeed >= 0);
      assert(baseAngVel >= 0);
      assert(angVel >= 0);
      assert(centerPos.x <>= 0);
      assert(centerPos.y <>= 0);
      assert(centerVel.x <>= 0);
      assert(centerVel.y <>= 0);
      assert(shield <>= 0);
      assert(captureState >= 0);
      assert(size.x <>= 0);
      assert(size.y <>= 0);
      assert(targetSize.x <>= 0);
      assert(targetSize.y <>= 0);
      assert(sizeVel.x <>= 0);
      assert(sizeVel.y <>= 0);
      assert(anger <>= 0);
    }
  }
*/
impl EnemyState {
  fn new() -> EnemyState {
    let mut inst : Self = Default::default(); 
    //inst.super();
    /*
    for &ts in inst.turretStates {
      ts = TurretState::new();
    }
    */
    inst.turretStates = [TokenState::new(), TURRET_MAX_NUM2];
    inst.vel = Vector::new();
    inst.centerPos = Vector::new();
    inst.centerVel = Vector::new();
    inst.standByPos = Vector::new();
    inst.size = Vector::new();
    inst.targetSize = Vector::new();
    inst.sizeVel = Vector::new();
    inst.trails = [Trail::new(), TRAIL_NUM];
    /*
    for &t in self.trails {
      t = Trail::new();
    }*/
    inst
  }

  fn clear(&mut self) {
    for ts in self.turretStates {
      ts.clear();
    }
    self.vel.x = 0.0;
    self.vel.y = 0.0;
    self.centerPos.x = 0;
    self.centerPos.y = 0;
    self.centerVel.x = 0;
    self.centerVel.y = 0;
    self.standByPos.x = 0;
    self.standByPos.y = 0;
    self.baseBaseSpeed = 0;
    self.baseSpeed = 0;
    self.baseAngVel = 0;
    self.angVel = 0;
    self.waitCnt = 0;
    self.cnt = 0;
    self.ellipseRatio = 0;
    self.ellipseDeg = 0;
    self.shield = 0;
    self.phase = 0;
    self.phaseCnt = 0;
    self.nextPhaseCnt = 0;
    self.captureState = 0;
    self.captureIdx = 0;
    self.isGoingDownBeforeStandBy = false;
    self.size.x = 1;
    self.size.y = 1;
    self.targetSize.x = 1;
    self.targetSize.y = 1;
    self.sizeVel.x = 0;
    self.sizeVel.y = 0;
    self.trailIdx = 0;
    self.trailLooped = false;
    self.isFirstEnemy = false;
    self.anger = 0;
    self.ts.clear();
  }

  //was move()
  fn move1(&mut self) {
    self.cnt += 1;;
    self.anger *= 0.9995;
  }

  fn recordTrail(&mut self) {
    self.trails[self.trailIdx].set(self.pos.x, self.pos.y, self.deg, self.cnt);
    self.trailIdx += 1;
    if self.trailIdx >= TRAIL_NUM {
      self.trailIdx = 0;
      self.trailLooped = true;
    }
  }

  fn drawTrails(&self, s : &EnemyShape, r : f32, g : f32, b : f32, size : Vector, field : Field) {
    let mut ti : i32 = self.trailIdx;
    let mut a : f32 = 1.0;
    for i in 0..(TRAIL_NUM / TRAIL_INTERVAL) {
      ti -= TRAIL_INTERVAL;
      if ti < 0 {
        if self.trailLooped {
          ti += TRAIL_NUM;
        } else {
          break;
        }
      }
      let t = self.trails[ti];
      Screen_setColor(r * a, g * a, b * a, a * 0.66);
      let p : Vector3 = field.calcCircularPos(t.pos);
      let cd : f32 = field.calcCircularDeg(t.pos.x);
      s.draw(p, cd, t.deg, t.cnt, size);
      a *= 0.7;
    }
  }
}

const BULLET_HIT_WIDTH : f32 = 0.8;
const NEXT_PHASE_DIST : f32 = 5.0;
const TURRET_MAX_NUM1 : usize = 3;

struct EnemySpec {
  ts : TokenSpec<EnemyState>,
  //mixin StaticRandImpl;
  bullets : *mut BulletPool,
  player : *mut Player,
  particles : *mut ParticlePool,
  bonusParticles : *mut ParticlePool,
  enemies : *mut EnemyPool,
  stage : *mut Stage,
  trailShape : *mut EnemyShape,
  bulletSpec : *mut BulletSpec,
  counterBulletSpec : *mut BulletSpec,
  turretSpecs : [TurretSpec; TURRET_MAX_NUM1],
  turretNum : i32,
  turretWidth : f32, //= 0;
  gameState : *mut GameState,
  shield : f32, // = 1;
  rank : f32, //= 0;
  capturable : bool,
  score : i32,
  explosionSeName : String,
  removeBullets : bool,
}
/*
  invariant() {
    assert(shield > 0);
    assert(rank >= 0);
    assert(turretWidth >= 0);
  }
*/
  //public this() {}

impl EnemySpec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles : *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : *mut GameState) -> EnemySpec {
    EnemySpec {
      ts : TokenSpec::<EnemyState>::new(field, shape),
      bullets : bullets,
      player : player,
      particles : particles,
      bonusParticles : bonusParticles,
      enemies : enemies,
      stage : stage,
      trailShape : trailShape,
      bulletSpec : bulletSpec,
      counterBulletSpec : counterBulletSpec,
      gameState : gameState,
      turretWidth : 0,
      shield : 1,
      rank : 0,
    }
  }

  fn set(&mut self , es : &mut EnemyState) {
    es.shield = self.shield;
    for i in 0..self.turretNum {
      self.turretSpecs[i].set(es.turretStates[i]);
    }
  }

  fn move2(&mut self, es : &mut EnemyState) -> bool {
    //with (es) {
      es.move();
      if self.isInScreen(es) && es.isFirstEnemy {
        Sound_playSe("flying_down.wav");
        es.isFirstEnemy = false;
      }
      if es.captureState > 0 {
        es.moveCaptured(es);
        return true;
      }
      if self.player.enemiesHasCollision() {
        if self.enemies.checkEnemyHit(es.pos, es.size) {
          self.destroyed(es);
          return false;
        }
      }
      if self.player.checkEnemyHit(es.pos, es.vel, es.size) {
        self.destroyed(es);
        return false;
      }
      if self.capturable {
        self.checkCaptured(es);
      }
      let er: f32 = (1.0 - es.ellipseRatio) + (es.deg + es.ellipseDeg).sin().abs() * es.ellipseRatio * 2.0;
      let rk : f32 = self.rank;
      es.vel.x -= es.ts.deg.sin() * es.speed * er * 0.1 * rk;
      es.vel.y += es.ts.deg.cos() * es.speed * er * 0.1 * rk;
      es.vel *= 0.9;
      es.ts.pos += es.vel;
      if self.isInScreen(es) {
        self.field.addSlowdownRatio(es.speed * 0.04 * rk);
      }
      es.ts.pos.x = self.field.normalizeX(es.ts.pos.x);
      es.recordTrail();
      if (es.phase >= -50) && (es.phase < 0) && !self.field.containsIncludingPit(es.ts.pos) {
        return false;
      }
      if es.waitCnt > 0 {
        es.waitCnt -= 1;
      } else {
        let cp : Vector = es.centerPos;
        es.centerPos.x = self.field.normalizeX(es.centerPos.x);
        self.phaseCnt += 1;
        if self.field.calcCircularDist2(es.centerPos, es.ts.pos) < NEXT_PHASE_DIST {
          es.nextPhaseCnt -= 1;
          if es.nextPhaseCnt <= 0 {
            es.phase += 1;
            if !self.gotoNextPhase(es) {
              return false;
            }
          }
        }
        cp.x = self.field.normalizeX(cp.x);
        let dst : f32 = self.field.calcCircularDist2(cp, es.ts.pos);
        es.ts.speed += ((es.baseSpeed * (1 + dst * 0.1)) - es.ts.speed) * 0.05;
        let mut av : f32 = self.angVel * rk;
        let mut td : f32 = (self.field.normalizeX(-(cp.x - es.ts.pos.x)), cp.y - es.ts.pos.y).atan2();
        let mut ad : f32 = normalize_deg(td - es.ts.deg);
        av *= 2.5 - er;
        if (ad > av) || (ad < (-PI * 0.8)) {
          es.ts.deg += av;
        }
        else if ad < -av {
          es.ts.deg -= av;
        } else {
          es.ts.deg = td;
        }
        //assert(deg <>= 0);
        for i in 0..self.turretNum {
          let ts : TurretState = es.turretStates[i];
          let tx : f32 = es.ts.pos.x;
          let ty : f32 = es.ts.pos.y;
          match i {
          0 =>  {},
          1 => { tx -= self.turretWidth; },
          2 => { tx += self.turretWidth; },
          }
          let turretDeg : f32 = (self.field.normalizeX(-(self.player.pos.x - tx)), self.player.pos.y - ty).atan2();
          match self.gameState.mode {
            GameState::Mode::CLASSIC => {
              if (turretDeg >= 0) && (turretDeg < (PI - PI / 6.0)) {
                turretDeg = PI - PI / 6;
              } else if (turretDeg < 0) && turretDeg > (-PI + PI / 6.0) {
                turretDeg = -PI + PI / 6;
             }
             turretDeg = ((((turretDeg + PI / 64.0) / (PI / 32.0)) as i32) as f32) * (PI / 32.0);
            },
            GameState::Mode::BASIC => {
              if (turretDeg >= 0) && (turretDeg < (PI - PI / 4.0)) {
               turretDeg = PI - PI / 4.0;
              } else if (turretDeg < 0) && (turretDeg > (-PI + PI / 4.0)) {
               turretDeg = -PI + PI / 4.0;
              }
            },
            GameState::Mode::MODERN => {}
          };
          ts.update(tx, ty, turretDeg);
        }
        self.movePhase(es);
        es.sizeVel.x += (es.targetSize.x - es.size.x) * 0.2;
        es.sizeVel.y += (es.targetSize.y - es.size.y) * 0.2;
        es.size += es.sizeVel;
        es.sizeVel *= 0.95;
      }
      true
    //}
  }

  fn moveCaptured(&mut self, es : &mut EnemyState) {
    //with (es) {
      match es.captureState {
      1 => {
        es.vel.x += (self.player.pos.x - es.ts.pos.x) * 0.03;
        es.vel.y += (self.player.pos.y - es.ts.pos.y) * 0.03;
        es.ts.pos.x += (self.player.pos.x - es.ts.pos.x) * 0.03;
        es.ts.pos.y += (self.player.pos.y - es.ts.pos.y) * 0.03;
        es.ts.deg *= 0.95;
        if self.player.pos.dist(es.ts.pos) < 1 {
          es.captureState = 2;
        }
      },
      2 => {
        let cx : f32 = self.calcCapturePosX(es.captureIdx);
        es.vel.x += (self.player.pos.x + cx - es.ts.pos.x) * 0.03;
        es.ts.pos.x += (self.player.pos.x + cx - es.ts.pos.x) * 0.1;
        es.ts.pos.y += (self.player.pos.y - es.ts.pos.y) * 0.33;
        es.vel.y *= 0.6;
        es.ts.deg *= 0.95;
        if (self.player.pos.x + cx - es.ts.pos.x).abs() < 0.2 {
          es.captureState = 3;
        }
      },
      3 => {
        let cx : f32 = self.calcCapturePosX(es.captureIdx);
        es.ts.pos.x = self.player.pos.x + cx;
        es.ts.pos.y = self.player.pos.y;
        es.ts.deg = self.player.deg;
        }
      }
      es.vel *= 0.9;
      es.ts.pos += es.vel;
    //}
  }

  fn calcCapturePosX(&self, idx : i32) -> f32 {
    if (idx % 2) == 0 {
      ((idx / 2) + 0.5) * PlayerSpec_CAPTURED_ENEMIES_INTERVAL_LENGTH * self.player.capturedEnemyWidth
    } else {
      -((idx / 2) + 0.5) * PlayerSpec_CAPTURED_ENEMIES_INTERVAL_LENGTH * self.player.capturedEnemyWidth
    }
  }

  fn checkCaptured(&self, es : &mut EnemyState) {
    //with (es) {
      if self.player.isInTractorBeam(es.ts.pos) {
        if self.gameState.mode != GameState::Mode::MODERN {
          let idx : i32 = self.player.addCapturedEnemy(es.enemy);
          if idx >= 0 {
            es.captureState = 1;
            es.captureIdx = idx;
          }
        } else {
          self.provacated(es);
        }
      }
    //}
  }

  fn hitCaptured(&mut self, es : &EnemyState) {
    self.player.destroyCapturedEnemies(es.captureIdx);
  }

  fn isBeingCaptured(&self, es : &EnemyState) -> bool {
    (es.captureState > 0)
  }

  fn isCaptured(es : &EnemyState) -> bool {
    (es.captureState == 3)
  }

  fn beforeAlign(es : &EnemyState) -> bool {
    (es.phase < -10)
  }

  fn hitShot(&mut self, es : &mut EnemyState, dd : f32 /* = 0*/) -> bool {
    //with (es) {
      es.shield -= 1;
      let r : f32 = 0.5 + rand.nextFloat(0.5);
      let g : f32 = 0.1 + rand.nextFloat(0.5);
      let b : f32 = 0.5 + rand.nextFloat(0.5);
      for i in 0..10 {
        let p : Particle = self.particles.getInstanceForced();
        let d : f32 = dd + rand.nextSignedFloat(PI / 4);
        p.set(Particle.Shape.LINE, es.ts.pos.x, es.ts.pos.y, d, 0.1 + rand.nextFloat(0.5), 1,
              r, g, b, 30 + rand.nextInt(30));
        p = self.particles.getInstanceForced();
        d = dd + PI + rand.nextSignedFloat(PI / 4);
        p.set(Particle.Shape.LINE, es.ts.pos.x, es.ts.pos.y, d, 0.1 + rand.nextFloat(0.5), 1,
              r, g, b, 30 + rand.nextInt(30));
      }
      if es.shield <= 0 {
        self.destroyed(es, dd);
        return true;
      }
      match self.gameState.mode {
       GameState::Mode::CLASSIC => {
        es.ts.targetSize.x *= 1.3;
        es.ts.targetSize.y *= 1.3;
        },
      GameState::Mode::BASIC => {
        es.ts.targetSize.x *= 1.2;
        es.ts.targetSize.y *= 1.2;
        },
      GameState::Mode::MODERN => {
        es.ts.targetSize.x *= 1.01;
        es.ts.targetSize.y *= 1.01;
        },
      }
      es.sizeVel.x = 0.3;
      es.sizeVel.y = 0.3;
      return false;
    //}
  }

  fn destroyed(&mut self, es : &mut EnemyState, dd : f32 /*= 0*/) {
    //with (es) {
      let r : f32 = 0.5 + rand.nextFloat(0.5);
      let g : f32 = 0.1 + rand.nextFloat(0.5);
      let b : f32 = 0.5 + rand.nextFloat(0.5);
      let sz : f32 = (es.ts.targetSize.x + es.ts.targetSize.y) / 2;
      sz = (sz - 1.0) * 2.0 + 1.0;
      let mut n : i32 = 3 + rand.nextInt(2);
      n *= sz;
      for i  in 0..n {
        let p : Particle = self.particles.getInstanceForced();
        let d : f32 = dd + rand.nextSignedFloat(PI / 5.0);
        p.set(Particle.Shape.TRIANGLE, es.ts.pos.x, es.ts.pos.y, d, 0.5,
              (2.0 + rand.nextFloat(0.5)) * sz, r, g, b, 50 + rand.nextInt(100));
      }
      for i in 0..n {{
        let p : Particle = self.particles.getInstanceForced();
        let d : f32 = rand.nextFloat(PI * 2.0);
        p.set(Particle.Shape.QUAD, es.ts.pos.x, es.ts.pos.y, d, 0.1 + rand.nextFloat(0.1),
              (1 + rand.nextFloat(0.5)) * sz, r, g, b, 50 + rand.nextInt(100));
      }
      if !self.isBeingCaptured(es) {
        if self.removeBullets {
          let mut cnt : i32 = 1;
          self.bullets.removeAround(cnt, es.ts.pos, self.particles, self.bonusParticles, self.player);
          let p : Particle = self.bonusParticles.getInstanceForced();
          let mut wc : i32;
          if cnt <= 50 {
            wc = cnt;
          } else {
            wc = 50 + (((cnt - 50) as f32).sqrt() as i32);
          }
          p.set(Particle.Shape.BONUS, es.ts.pos.x, es.ts.pos.y, 0, 0.1,
                1.0 + (wc as f32) / 75.0, 1, 1, 1, 120, false, cnt, wc);
          self.player.addScore(self.score * cnt);
        } else {
          if self.gameState.mode == GameState::Mode::BASIC {
            let oy : f32 = es.ts.pos.y - self.player.pos.y;
            let mut pm : i32 = (18.0 - oy) as i32;
            if pm > 16 {
              pm = 16;
            } else if pm < 1 {
              pm = 1;
            }
            self.player.addScore(self.score * pm);
            let mut p : Particle = self.bonusParticles.getInstanceForced();
            p.set(Particle.Shape.BONUS, es.ts.pos.x, es.ts.pos.y, 0, 0.1,
                  0.5, 1, 1, 1, 60, false, pm);
            self.gameState.setProximityMultiplier(pm);
          } else {
            self.player.addScore(self.score);
          }
        }
        self.player.addMultiplier(0.1);
        if self.stage.existsCounterBullet {
          let blt : Bullet = self.bullets.getInstance();
          if blt {
            blt.set(self.counterBulletSpec, es.ts.pos,
                    self.turretStates[0].deg, self.turretSpecs[0].speed * TurretSpec_SPEED_RATIO);
          }
        }
      }
      Sound_playSe(self.explosionSeName);
    }
  }

  fn provacated(&mut self, es : &mut EnemyState) {
    //with (es) {
      es.anger += (1 - es.anger) * 0.05;
      if es.sizeVel.dist < 0.1 {
        es.sizeVel.x = 0.2;
        es.sizeVel.y = 0.2;
      }
      let mut p : Particle = self.particles.getInstanceForced();
      p.set(Particle.Shape.LINE, es.ts.pos.x, es.ts.pos.y, PI / 2.0 + rand.nextSignedFloat(PI / 4.0),
            0.1 + rand.nextFloat(0.2), 1,
            1, 0.5, 0.5, 30 + rand.nextInt(30));
      p = self.particles.getInstanceForced();
      p.set(Particle.Shape.LINE, es.ts.pos.x, es.ts.pos.y, -PI / 2.0 + rand.nextSignedFloat(PI / 4.0),
            0.1 + rand.nextFloat(0.2), 1,
            1, 0.5, 0.5, 30 + rand.nextInt(30));
      if self.removeBullets {
        self.player.midEnemyProvacated();
      }
    //}
  }

  fn gotoNextPhaseInAppearing(&mut self, es : &mut EnemyState) -> bool {
    //with (es) {
      match es.phase {
      -300 => {
        let mut cpw : f32;
        match self.gameState.mode {
          GameState::Mode::CLASSIC => { cpw = 0.2; },
          GameState::Mode::BASIC => { cpw = 0.2; },
          GameState::Mode::MODERN => { cpw = 0.4; },
        }
        es.centerPos.x = rand.nextSignedFloat(self.field.size.x * cpw);
        es.centerPos.y = self.field.size.y * 2.0;
        es.standByPos.x = rand.nextSignedFloat(self.field.size.x * cpw);
        es.standByPos.y = self.field.size.y * (0.7 + rand.nextFloat(0.1));
        es.nextPhaseCnt = 15;
        es.baseSpeed = es.baseBaseSpeed * 1.5;
        es.angVel = es.baseAngVel * 1.5;
        es.phase = -50;
        },

      -200 => {
        es.centerPos.x = rand.nextSignedFloat(self.field.size.x * 0.1);
        es.centerPos.y = self.field.size.y * 1.6;
        if es.centerPos.x < 0 {
          es.standByPos.x = self.field.size.x * (rand.nextSignedFloat(0.4) + 0.4);
        } else {
          es.standByPos.x = self.field.size.x * (rand.nextSignedFloat(0.4) - 0.4);
        }
        es.standByPos.y = self.field.size.y * (0.5 + rand.nextFloat(0.3));
        es.nextPhaseCnt = 60;
        es.baseSpeed = es.baseBaseSpeed * 1.0;
        es.angVel = es.baseAngVel * 1.0;
        },
      -199 => {
        if es.standByPos.x < 0 {
          es.centerPos.x = self.field.size.x * 0.75;
        } else {
          es.centerPos.x = -self.field.size.x * 0.75;
        }
        es.centerPos.y = 0;
        if self.isGoingDownBeforeStandBy {
          es.nextPhaseCnt = 20;
        } else {
          es.nextPhaseCnt = 60;
        }
        es.baseSpeed = es.baseBaseSpeed * 2;
        es.angVel = es.baseAngVel * 2;
        es.phase = -50;
       },
 
      -100 => {
        es.centerPos.x = self.field.size.x * 4.0;
        if rand.nextInt(2) == 0 {
          es.centerPos.x *= -1;
        }
        es.centerPos.y = self.field.size.y * 1.6;
        if es.centerPos.x < 0 {
          es.standByPos.x = self.field.size.x * (rand.nextSignedFloat(0.4) + 0.4);
        }
        else {
          es.standByPos.x = self.field.size.x * (rand.nextSignedFloat(0.4) - 0.4);
        }
        es.standByPos.y = self.field.size.y * (0.5 + rand.nextFloat(0.3));
        es.nextPhaseCnt = 20;
        es.baseSpeed = es.baseBaseSpeed * 2.0;
        es.angVel = es.baseAngVel * 2.0;
       },
      -99 => {
        if es.centerPos.x > 0 {
          es.centerPos.x = self.field.size.x * 2.0;
        } else {
          es.centerPos.x = -self.field.size.x * 2.0;
        }
        es.centerPos.y = -self.field.size.y * 1.2;
        es.nextPhaseCnt = 20;
        es.baseSpeed = es.baseBaseSpeed * 2;
        es.angVel = es.baseAngVel * 2;
      },
      -98 => {
        if es.centerPos.x > 0 {
          es.centerPos.x = self.field.size.x * 0.5;
        } else {
          es.centerPos.x = -self.field.size.x * 0.5;
        }
        es.centerPos.y = 0;
        es.nextPhaseCnt = 30;
        es.phase = -50;
      },
      -49 => {
        if self.isGoingDownBeforeStandBy {
          es.centerPos.x = es.centerPos.x / 2;
          es.centerPos.y = 0;
          es.phase = -30;
          es.nextPhaseCnt = 10;
        } else {
          es.centerPos.x = es.standByPos.x;
          es.centerPos.y = es.standByPos.y;
          es.nextPhaseCnt = es.calcStandByTime(es);
          es.baseSpeed = es.baseBaseSpeed;
          es.angVel = es.baseAngVel;
          es.phase = -10;
        }
      },
      -29 => {
        self.vcenterPos.x = (es.centerPos.x + self.player.pos.x * 2) / 3;
        es.centerPos.y = -self.field.size.y * 1.2;
        es.baseSpeed = es.baseBaseSpeed * 1.2;
        es.angVel = es.baseAngVel * 1.2;
        es.nextPhaseCnt = 5;
       },
      -28 => {
        es.centerPos.y = -self.field.size.y * 1.5;
        es.nextPhaseCnt = 10;
       },
      -9 => {
        es.phase = 0;
      },
      _ => {
        return false;
      }
      }
      es.nextPhaseCnt /= self.rank;
      es.phaseCnt = 0;
    //}
    true;
  }

  fn movePhase(&mut self, es : &mut EnemyState) {
    //with (es) {
      match es.phase {
      -200|-100 => {
        if es.ts.pos.y < (self.field.size.y * 1.5) {
          es.ts.pos.y = self.field.size.y * 1.5;
        }
      },
      -99 => {
        if (es.centerPos.x < 0) && (es.ts.pos.x > -self.field.size.x) {
          es.ts.pos.x += (-self.field.size.x - es.ts.pos.x) * 0.2;
        } else if (es.centerPos.x > 0) && (es.ts.pos.x < self.field.size.x) {
          es.ts.pos.x += (self.field.size.x - es.ts.pos.x) * 0.2;
        }
      },
      -50|-49|-10=> {
        if es.ts.pos.y < (-self.field.size.y * 0.5) {
          es.ts.pos.y += (-self.field.size.y * 0.5 - es.ts.pos.y) * 0.2;
        }
      },
      _ => {},
      };
      if self.isInAttack(es) {
        if (self.gameState.mode == GameState::Mode::MODERN) || (es.phase >= 0) || (rand.nextInt(5) == 0) {
          for i in 0..self.turretNum {
            self.turretSpecs[i].move(self.turretStates[i], es.rank, es.anger);
          }
        }
      }
    //}
  }

  fn isInScreen(&self, es : &EnemyState) -> bool {
    self.field.size.contains(es.pos);
  }
/*
  public abstract void setRank(float rank);
  public abstract void init(EnemyState es);
  public abstract bool gotoNextPhase(EnemyState es);
  public abstract bool isInAttack(EnemyState es);
  protected abstract int calcStandByTime(EnemyState es);
*/
  fn draw(&self, es : &EnemyState) {
    let mut p : Vector3 = self.field.calcCircularPos(es.ts.pos);
    let mut cd : f32 = self.field.calcCircularDeg(es.ts.pos.x);
    (self.ts.shape as EnemyShape).draw(p, cd, es.deg, es.cnt, es.size);
    for i in 1..self.turretNum {
      let x : f32 = es.ts.pos.x;
      match i {
      1 => {
        x -= self.turretWidth;
      },
      2 => {
        x += self.turretWidth;
      },
      }
      p = self.field.calcCircularPos(x, es.ts.pos.y);
      cd = self.field.calcCircularDeg(x);
      Screen_setColor(0.5, 0.5, 1);
      (self.trailShape as EnemyShape).draw(p, cd, es.deg, es.cnt, es.size.x * 0.5, es.size.y * 0.5);
    }
  }

  fn drawTrails(&self, es : &EnemyState) {
    if es.captureState > 0 {
      return;
    }
    es.drawTrails(self.trailShape, 0.2, 0.2, 0.8, es.size, self.field);
  }
}

struct Trail {
  pos : Vector,
  deg : f32,
  cnt : i32,
}

/*
  invariant() {
    assert(pos.x <>= 0);
    assert(pos.y <>= 0);
    assert(deg <>= 0);
  }
*/

impl Trail {
  fn new() -> Self {
    Trail{pos : Vector::new(), deg : 0.0, cnt : 0}
  }

  fn set(&mut self, x : f32, y : f32, d : f32, c : i32) {
    self.pos.x = x;
    self.pos.y = y;
    self.deg = d;
    self.cnt = c;
  }
}

struct GhostEnemySpec {
  es : EnemySpec,
}

impl GhostEnemySpec {
  fn new(&mut self, field : *mut Field, shape : *mut Shape) -> GhostEnemySpec {
    GhostEnemySpec{ field: field, shape: shape}
  }

  fn draw(&self, es : &EnemyState) {
    //with (es) {
      let p : Vector3 = self.field.calcCircularPos(es.ts.pos);
      let cd : f32 = self.field.calcCircularDeg(es.ts.pos.x);
      Screen_setColor(0.5, 0.5, 1, 0.8);
      (self.ts.shape as EnemyShape).draw(p, cd, es.ts.deg, self.cnt, es.size);
    //}
  }

  fn set(&mut self, es : &EnemyState) {}
  fn move2(&mut self, es : &EnemyState) -> bool { true }
  fn destroyed(&mut self, es : &EnemyState, dd : f32 /*= 0*/) {}
  fn setRank(&mut self, rank : f32) {}
  fn init(&mut self, es : &EnemyState) {}
  fn gotoNextPhase(&mut self, es : &EnemyState) -> bool { false }
  fn isInAttack(&mut self, ses : &EnemyState) -> bool { false }
  fn calcStandByTime(&mut self, es : &EnemyState) -> i32 { 0 }
  fn isBeingCaptured(&mut self, es : &EnemyState) -> bool { true }
  fn isCaptured(&mut self, es : &EnemyState) -> bool { true }
}


struct MiddleEnemySpec {
 es : EnemySpec,
}

impl MiddleEnemySpec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles :  *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : *mut GameState) -> MiddleEnemySpec {
    let mut inst = MiddleEnemySpec{es : EnemySpec {
      field : field, bullets: bullets, player : player, particles : particles, bonusParticles : bonusParticles, enemies : enemies, stage : stage,
          shape : shape, trailShape: trailShape, bulletSpec : bulletSpec, counterBulletSpec : counterBulletSpec, gameState : gameState}};
    for &ts in inst.turretSpecs {
      ts = TurretSpec::new(field, bullets, player, enemies, particles, stage, bulletSpec, gameState);
    }
    match gameState.mode {
    GameState::Mode::CLASSIC => {
      inst.shield = 2;
      inst.capturable = false;
      inst.removeBullets = false;
    },
    GameState::Mode::BASIC => {
      inst.shield = 3;
      inst.capturable = false;
      inst.removeBullets = false;
    },
    GameState::Mode::MODERN => {
      inst.shield = 32;
      inst.capturable = true;
      inst.removeBullets = true;
      },
    }
    inst.score = 100;
    inst.es.explosionSeName = "explosion3.wav";
    inst
  }

  fn init(&mut self, es : &mut EnemyState) {
    //with (es) {
      self.es.size.x = 1.33;
      self.es.size.y = 1.33;
      self.es.phase = -300;
      self.es.gotoNextPhaseInAppearing(es);
    //}
  }

  fn setRank(&mut self, r : f32) {
    self.es.rank = r.sqrt();
    let mut tr : f32;
    match self.es.gameState.mode {
    GameState::Mode::CLASSIC => {
      self.es.rank = self.es.rank.sqrt();
      tr = r * 2.0;
      },
    GameState::Mode::BASIC => {
      tr = r * 3.0;
      },
    GameState::Mode::MODERN => {
      self.es.rank = 1.0;
      tr = r * 15.0;
      },
    };
    if self.es.rank < 1.5 {
      self.es.rank = 1.5;
    }
    self.es.turretSpecs[0].setRankMiddle(tr);
    self.es.turretNum = 1;
    if self.es.gameState.mode == GameState::Mode::MODERN {
      let ts : TurretSpec = self.es.turretSpecs[0];
      let ptn : i32 = rand.nextInt(6);
      if ptn == 1 || ptn == 2 || ptn == 4 {
        self.es.turretSpecs[1].copy(self.es.turretSpecs[0]);
        self.es.turretSpecs[2].copy(self.es.turretSpecs[0]);
        if (ts.nway > 1) && (rand.nextInt(2) == 0) {
          let nsa : f32 = (ts.speed * (0.2 + ts.nway * 0.05 + rand.nextFloat(0.1))) / (ts.nway - 1);
          if rand.nextInt(2) == 0 {
            nsa *= -1.0;
          }
          self.es.turretSpecs[1].nwaySpeedAccel = nsa;
          self.es.turretSpecs[2].nwaySpeedAccel = -nsa;
        }
        self.es.turretWidth = 1.0 + rand.nextFloat(1.0);
        self.es.turretNum = 3;
        if ptn == 4 {
          self.es.turretSpecs[0].setRankMiddle(tr * 2);
          self.es.turretSpecs[1].interval *= 4;
          self.es.turretSpecs[2].interval *= 4;
          self.es.turretSpecs[0].interval = self.es.turretSpecs[1].interval;
          self.es.turretSpecs[2].fireIntervalRatio = 0.25;
          self.es.turretSpecs[0].fireIntervalRatio = 0.5;
        } else {
          self.es.turretSpecs[0].disabled = true;
          self.es.turretSpecs[1].interval *= 2;
          self.es.turretSpecs[2].interval *= 2;
          if rand.nextInt(2) == 0 {
            self.es.turretSpecs[2].fireIntervalRatio = 0.5;
          }
        }
      } else if ptn == 3 || ptn == 5 {
        self.es.turretSpecs[0].interval *= 2;
        if rand.nextInt(3) == 0 {
          self.es.turretSpecs[0].nwayAngle *= 0.1;
        }
        self.es.turretSpecs[1].setRankMiddle(tr);
        self.es.turretSpecs[1].interval *= 2;
        self.es.turretSpecs[2].copy(self.es.turretSpecs[1]);
        if (ts.nway > 1) && (rand.nextInt(2) == 0) {
          let nsa : f32 = (ts.speed * (0.2 + ts.nway * 0.05 + rand.nextFloat(0.1))) / (ts.nway - 1);
          if rand.nextInt(2) == 0 {
            nsa *= -1;
          }
          self.es.turretSpecs[1].nwaySpeedAccel = nsa;
          self.es.turretSpecs[2].nwaySpeedAccel = -nsa;
        }
        self.es.turretSpecs[1].nwayBaseDeg = -PI / 8.0 - rand.nextFloat(PI / 12.0);
        if self.es.turretSpecs[1].nway > 1 {
          self.es.turretSpecs[1].nwayBaseDeg -= self.es.turretSpecs[1].nwayAngle * (self.es.turretSpecs[1].nway - 1) / 2;
        }
        self.es.turretSpecs[2].nwayBaseDeg = -self.es.turretSpecs[1].nwayBaseDeg;
        self.es.turretWidth = 1.5 + rand.nextFloat(0.5);
        self.es.turretNum = 3;
      }
    }
  }

  fn gotoNextPhase(&mut self, es : &mut EnemyState) -> bool {
    //with (es) {
      if es.phase < 0 {
        return self.es.gotoNextPhaseInAppearing(es);
      }
      match es.phase {
      1 => {
        if (self.es.gameState.mode != GameState::Mode::MODERN) && !self.es.player.hasCollision {
          es.phase = 0;
          es.nextPhaseCnt = self.es.calcStandByTime(es);
        } else {
          Sound_playSe("flying_down.wav");
          if self.es.gameState.mode != GameState::Mode::MODERN {
            es.centerPos.x = self.field.size.x * (0.6 + rand.nextSignedFloat(0.1));
            if rand.nextInt(2) == 0 {
              es.centerPos.x *= -1;
            }
            es.centerPos.y = self.field.size.y * (0.2 + rand.nextFloat(0.2));
            es.nextPhaseCnt = 60;
          } else {
            es.centerPos.x = es.standByPos.x;
            es.centerPos.y = self.field.size.y * 0.95;
            es.baseSpeed = es.baseBaseSpeed * 0.3;
            es.nextPhaseCnt = 60;
          }
        }
      },
      2 => {
        if self.es.gameState.mode != GameState::Mode::MODERN {
          es.centerPos.x *= -0.9;
          es.centerPos.y = self.field.size.y * (0.2 + rand.nextFloat(0.2));
          es.nextPhaseCnt = 60;
        } else {
          es.centerPos.x = es.standByPos.x;
          es.centerPos.y = 0;
          es.baseSpeed = es.baseBaseSpeed * 0.1;
          es.nextPhaseCnt = 10;
        }
      },
      3 => {
        if self.es.gameState.mode != GameState::Mode::MODERN {
          es.centerPos.x = es.standByPos.x;
          es.centerPos.y = es.standByPos.y;
          es.phase = 0;
          es.nextPhaseCnt = self.es.calcStandByTime(es);
        } else {
          es.centerPos.x = es.standByPos.x;
          es.centerPos.y = -self.field.size.y * 1.5;
          es.baseSpeed = es.baseBaseSpeed * 0.5;
          es.nextPhaseCnt = 10;
        }
      },
      _ => {
        return false;
      },
      };
      es.nextPhaseCnt /= self.es.rank;
      es.phaseCnt = 0;
    //}
    true
  }

  fn isInAttack(&mut self, es : &EnemyState) -> bool {
    (es.phase == 1) || (es.phase == 2)
  }

  fn calcStandByTime(&mut self, es : &EnemyState) -> i32 {
    if (es.phase < 0) || (self.es.gameState.mode == GameState::Mode::MODERN) {
      return 30 + rand.nextInt(30);
    } else {
      return 200 + rand.nextInt(150);
    }
  }
}

struct SmallEnemySpec {
  es : EnemySpec,
}

impl SmallEnemySpec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles : *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : *mut GameState) -> SmallEnemySpec {
    let mut inst = SmallEnemySpec{ es : EnemySpec{
      field : field, bullets: bullets, player : player, particles : particles, bonusParticles : bonusParticles, enemies : enemies, stage : stage,
          shape : shape, trailShape : trailShape, bulletSpec : bulletSpec, counterBulletSpec : counterBulletSpec, gameState : gameState}};
    inst.turretSpecs[0] = TurretSpec::new(field, bullets, player, enemies, particles, stage, bulletSpec, gameState);
    inst.shild = match gameState.mode {
    GameState::Mode::CLASSIC => 1,
    GameState::Mode::BASIC => 1,
    GameState::Mode::MODERN => 2,
    };
    inst.capturable = true;
    inst.score = 10;
    inst.removeBullets = false;
    inst
  }

  fn init2(&mut self, es : &mut EnemyState) {
    self.es.gotoNextPhaseInAppearing(es);
  }

  fn init3(&mut self, es : &mut EnemyState, fes : &EnemyState) {
    //with (es) {
      es.centerPos.x = fes.centerPos.x;
      es.centerPos.y = fes.centerPos.y;
      es.standByPos.x = fes.standByPos.x;
      es.standByPos.y = fes.standByPos.y;
      es.nextPhaseCnt = fes.nextPhaseCnt;
      es.baseSpeed = fes.baseSpeed;
      es.angVel = fes.angVel;
      es.phase = fes.phase;
      es.size.x = 1.25;
      es.size.y = 1.25;
    //}
  }

fn setRank(&mut self, r : f32) {
    self.es.rank =(r * 0.5).sqrt();
    let mut tr : f32;
    match self.es.gameState.mode {
    GameState::Mode::CLASSIC => {
      self.es.rank = self.es.rank.sqrt();
      tr = r;
    },
    GameState::Mode::BASIC => {
      tr = r * 2;
    },
    GameState::Mode::MODERN => {
      self.es.rank = 1.0;
      tr = r;
    },
    };
    if self.es.rank < 1.0 {
      self.es.rank = 1;
    }
    self.es.turretSpecs[0].setRankNormal(tr);
    self.es.turretNum = 1;
  }

  fn calcStandByTime(es : EnemyState) -> i32 {
    60 + rand.nextInt(120)
  }
}

struct SE1Spec {
  ses : SmallEnemySpec,
}

impl SE1Spec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles : *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : *mut GameState) -> SE1Spec {
    let mut inst = SE1Spec {ses : SmallEnemySpec::new(field, bullets, player, particles, bonusParticles, enemies, stage,
          shape, trailShape, bulletSpec, counterBulletSpec, gameState)};
    inst.ses.es.explosionSeName = "explosion1.wav";
    inst
  }

  fn gotoNextPhase(&mut self, es : &mut EnemyState) -> bool {
    //with (es) {
      if es.phase < 0 {
        return self.ses.es.gotoNextPhaseInAppearing(es);
      }
      match es.phase {
      1 => {
        if !self.ses.player.hasCollision || (self.ses.enemies.numInAttack > self.ses.stage.attackSmallEnemyNum) {
          es.phase = 0;
          es.nextPhaseCnt = self.calcStandByTime(es);
        } else {
          Sound_playSe("flying_down.wav");
          es.centerPos.y = 0;
          es.centerPos.x = (es.standByPos.x + self.ses.player.pos.x) / 2;
          es.nextPhaseCnt = 60;
        }
      },
      2 => {
        es.centerPos.y = -self.ses.field.size.y * 0.7;
        es.centerPos.x = self.ses.player.pos.x;
        es.nextPhaseCnt = 30;
      },
      3 => {
        es.centerPos.x = es.standByPos.x;
        es.centerPos.y = es.standByPos.y;
        es.phase = 0;
        es.nextPhaseCnt = self.ses.calcStandByTime(es);
        },
      }
      es.nextPhaseCnt /= self.ses.rank;
      es.phaseCnt = 0;
    //}
    true
  }

  fn isInAttack(es : &EnemyState) -> bool {
    (es.phase < -10 || es.phase == 1 || es.phase == 2)
  }
}

struct SE2Spec {
 ses : SmallEnemySpec,
}

impl SE2Spec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles : *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : *mut GameState) -> SE2Spec {
    let mut inst = SE2Spec{ ses : SE2Spec::new(field, bullets, player, particles, bonusParticles, enemies, stage,
          shape, trailShape, bulletSpec, counterBulletSpec, gameState) };
    inst.ses.explosionSeName = "explosion2.wav";
    inst
  }

  fn gotoNextPhase(&mut self, es : &mut EnemyState) -> bool {
    //with (es) {
      if es.phase < 0 {
        return self.ses.gotoNextPhaseInAppearing(es);
      }
      match es.phase {
      1 => {
        if !self.ses.player.hasCollision || (self.ses.enemies.numInAttack > self.ses.stage.attackSmallEnemyNum) {
          es.phase = 0;
          es.nextPhaseCnt = self.ses.calcStandByTime(es);
        } else {
          Sound_playSe("flying_down.wav");
          es.centerPos.y = -self.ses.field.size.y * 0.3;
          es.centerPos.x = (es.standByPos.x + self.ses.player.pos.x) / 2;
          es.baseSpeed = es.baseBaseSpeed;
          es.angVel = es.baseAngVel;
          es.nextPhaseCnt = 30 + rand.nextInt(60);
        }
      },
      2 => {
        es.centerPos.y = -self.ses.field.size.y * 1.3;
        es.centerPos.x *= -1;
        es.nextPhaseCnt = 30;
      },
      3 => {
        es.centerPos.y = -self.ses.field.size.y * 1.0;
        if es.centerPos.x < 0 {
          es.centerPos.x = -self.ses.field.size.x * 1.5;
        } else {
          es.centerPos.x = self.ses.field.size.x * 1.5;
        }
        es.baseSpeed = es.baseBaseSpeed * 1.5;
        es.angVel = es.baseAngVel * 1.5;
        es.nextPhaseCnt = 30;
      },
      4 => {
        es.centerPos.x = es.standByPos.x;
        es.centerPos.y = es.standByPos.y;
        es.phase = 0;
        es.nextPhaseCnt = es.calcStandByTime(es);
      },
      }
      es.nextPhaseCnt /= self.ses.rank;
      es.phaseCnt = 0;
    //}
    true
  }

  fn movePhase(&mut self, es : &mut EnemyState) {
    self.ses.movePhase(es);
    //with (es) {
      if es.phase == 3 {
        if es.centerPos.x < 0 {
          if es.ts.pos.x > (-self.ses.field.size.x * 1.2) {
            es.ts.pos.x += (es.centerPos.x - es.ts.pos.x) * 0.2;
          }
        } else {
          if es.ts.pos.x < (self.field.size.x * 1.2) {
            es.ts.pos.x += (es.centerPos.x - es.ts.pos.x) * 0.2;
          }
        }
      }
    //}
  }
  
  fn isInAttack(&mut self, es : &EnemyState) -> bool {
    (es.phase < -10 || es.phase == 1 || es.phase == 2 || es.phase == 3)
  }
}

struct TurretState {
 ts : TokenState,
  fireCnt : f32,
  burstCnt : f32,
  burstNum : i32,
  nwaySpeedAccelDir : i32,
}
/*
  invariant() {
    if (isInitialized) {
      assert(fireCnt <>= 0);
      assert(burstCnt <>= 0);
    }
  }
*/

impl TurretState {
  fn clear(&mut self) {
    self.fireCnt = 0.0;
    self.burstCnt = 0.0;
    self.burstNum = 0.0;
    self.nwaySpeedAccelDir = 1;
    self.ts.clear();
  }

  fn update(&mut self, x : f32, y : f32, d : f32) {
    self.ts.pos.x = x;
    self.ts.pos.y = y;
    if self.ts.burstNum <= 0 {
      self.ts.deg = d;
    }
  }
}

const SPEED_RATIO : f32 = 5.0;
const INTERVAL_MAX : f32 = 90.0;

struct TurretSpec {
  ts : TokenSpec<TurretState>,
  //mixin StaticRandImpl;
  bulletSpec : BulletSpec,
  bullets : BulletPool,
  player : Player,
  stage : Stage,
  gameState : GameState,
  interval : i32,
  speed : f32,
  speedAccel : f32,
  burstNum : i32,
  burstInterval : i32,
  nway : i32,
  nwayAngle : f32,
  nwayBaseDeg : f32,
  nwaySpeedAccel : f32,
  fireingAtATime : bool,
  fireIntervalRatio : f32,
  _disabled : bool,
  minimumFireDist : f32,
}
/*
  invariant() {
    assert(interval > 0);
    assert(speed > 0);
    assert(speedAccel < 1 && speedAccel > -1);
    assert(burstNum >= 1);
    assert(burstInterval >= 0);
    assert(nway >= 1);
    assert(nwayAngle >= 0);
    assert(nwayBaseDeg <>= 0);
    assert(nwaySpeedAccel <>= 0);
    assert(fireIntervalRatio <>= 0);
    assert(minimumFireDist >= 0);
  }
*/

impl TurretSpec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              enemies : *mut EnemyPool, particles : *mut ParticlePool,
              stage : *mut Stage, bulletSpec : *mut BulletSpec, gameState : *mut GameState) {
    let mut inst = TurretSpec{bulletSpec : bulletSpec,
      field : field, bullets : bullets, player : player, stage : stage, gameState : gameState,};
    inst.initParam();
    inst
  }

  fn initParam(&mut self) {
    self.interval = 99999;
    self.speed = 1;
    self.speedAccel = 0;
    self.burstNum = 1;
    self.burstInterval = 99999;
    self.nway = 1;
    self.nwayAngle = 0;
    self.nwayBaseDeg = 0;
    self.nwaySpeedAccel = 0;
    self.fireingAtATime = false;
    self.fireIntervalRatio = 0;
    self.disabled = false;
    self.minimumFireDist = 0;
  }

  fn copy(&mut self, ts : &TurretSpec) {
    self.interval = ts.interval;
    self.speed = ts.speed;
    self.speedAccel = ts.speedAccel;
    self.burstNum = ts.burstNum;
    self.burstInterval = ts.burstInterval;
    self.nway = ts.nway;
    self.nwayAngle = ts.nwayAngle;
    self.nwayBaseDeg = ts.nwayBaseDeg;
    self.nwaySpeedAccel = ts.nwaySpeedAccel;
    self.fireingAtATime = ts.fireingAtATime;
  }

  fn set(&mut self, ts : &TurretState) {
    self.setFireIntervalRatio(ts, self.fireIntervalRatio);
  }

  fn setFireIntervalRatio(&mut self, ts : &TurretState, fir : f32) {
    ts.fireCnt = fir * self.interval;
  }

  fn setRankNormal(&mut self, rank : f32, isWide : bool /*= false*/) {
    self.initParam();
    let rr : f32 = rand.nextFloat(0.5);
    let nsr : f32 = 0.5 + rand.nextSignedFloat(0.3);
    let mut nr : f32;
    let mut br : f32;
    let mut ir : f32;
    let intervalMax : f32 = INTERVAL_MAX;
    match self.gameState.mode {
    GameState::Mode::CLASSIC => {
      nr = 0;
      br = 0;
      ir = (rank * nsr).sqrt() * 2.0;
      self.burstInterval = 3 + rand.nextInt(2);
    },
    GameState::Mode::BASIC => {
      if isWide {
        nr = rank * nsr * rr;
        br = 0;
        ir = rank * nsr * (1 - rr);
      } else {
        nr = 0;
        br = rank * nsr * rr;
        ir = rank * nsr * (1 - rr);
      }
      self.burstInterval = 3 + rand.nextInt(2);
    },
    GameState::Mode::MODERN => {
      if isWide {
        nr = rank * nsr * rr;
        br = 0;
        ir = rank * nsr * (1 - rr);
      } else {
        nr = 0;
        br = rank * nsr * rr;
        ir = rank * nsr * (1 - rr);
      }
      intervalMax = 120;
      self.burstInterval = 4 + rand.nextInt(4);
      },
    }
    self.burstNum = 1 + (br.sqrt() as i32);
    self.nway = 1 + (nr.sqrt() as i32);
    self.interval = ((intervalMax / (ir + 1)) as i32) + 1;
    let sr : f32 = rank - self.nway + 1 - self.burstNum + 1 - ir;
    if sr < 0.01 {
      sr = 0.01;
    }
    self.speed = (sr * 0.66).sqrt();
    //assert(speed > 0);
    self.speed *= 0.2;
    if self.speed < 0.1 {
      self.speed = 0.1;
    } else {
      self.speed = (self.speed * 10).sqrt() / 10;
    }
    //assert(speed > 0);
    match self.es.ts.gameState.mode {
    GameState::Mode::CLASSIC => {
      self.speed *= 0.36;
      if self.speed < 0.05 {
        self.speed = 0.05;
      } else {
        self.speed = (self.speed * 20).sqrt() / 20;
      }
    },
    GameState::Mode::BASIC => {
      self.speed *= 0.33;
    },
    GameState::Mode::MODERN => {
      self.speed *= 0.25;
      if self.speed < 0.04 {
        self.speed = 0.04;
      }
      if self.speed > 0.04 {
        self.speed = (self.speed * 25.0).sqrt() / 25.0;
      }
      },
    }
    self.nwayAngle = (1.66 + rand.nextFloat(0.33)) / (1.0 + self.nway * 0.7) * 0.06;
    self.fireingAtATime = true;
    self.minimumFireDist = 10;
  }

  fn setRankMiddle(&mut self, rank : f32) {
    self.initParam();
    let mut nr : f32;
    let mut br : f32;
    let mut ir : f32;
    let mut nwayDegRatio : f32;
    let intervalMax : f32 = INTERVAL_MAX;
    match self.gameState.mode {
      GameState::Mode::CLASSIC => {
      nr = 0;
      br = 0;
      ir = (rank * (0.5 + rand.nextSignedFloat(0.3))).sqrt() * 2;
      nwayDegRatio = 0;
      self.burstInterval = 3 + rand.nextInt(2);
      },
    GameState::Mode::BASIC => {
      if rand.nextInt(3) == 0 {
        nr = 0;
        br = (rank * 0.4) * (1.0 + rand.nextSignedFloat(0.2));
        ir = (rank * 0.5) * (1.0 + rand.nextSignedFloat(0.2));
      } else {
        rank *= 0.5;
        nr = (rank * 0.3) * (1.0 + rand.nextSignedFloat(0.2));
        br = (rank * 0.3) * (1.0 + rand.nextSignedFloat(0.2));
        ir = (rank * 0.3) * (1.0 + rand.nextSignedFloat(0.2));
      }
      ir *= 1.5;
      nwayDegRatio = 0.06;
      self.burstInterval = 3 + rand.nextInt(2);
    },
    GameState::Mode::MODERN => {
      let v = rand.nextInt(5);
      if v == 0 {
        rank *= 1.2;
        nr = 0;
        br = (rank * 0.7) * (1.0 + rand.nextSignedFloat(0.2));
        ir = (rank * 0.2) * (1.0 + rand.nextSignedFloat(0.2));
      } else if v == 1 || v == 2 {
        nr = (rank * 0.7) * (1.0 + rand.nextSignedFloat(0.2));
        br = 0;
        ir = (rank * 0.2) * (1.0 + rand.nextSignedFloat(0.2));
      } else if v == 3 || v == 4  {
        rank *= 0.75;
        nr = (rank * 0.3) * (1.0 + rand.nextSignedFloat(0.2));
        br = (rank * 0.3) * (1.0 + rand.nextSignedFloat(0.2));
        ir = (rank * 0.3) * (1.0 + rand.nextSignedFloat(0.2));
      }
      nwayDegRatio = 1;
      intervalMax = 120;
      self.burstInterval = 4 + rand.nextInt(8);
      },
    };
    let acf : bool = false;
    self.burstNum = (br.sqrt() as i32) + 1;
    if (self.burstNum > 1) && (rand.nextInt(3) > 0) {
      acf = true;
      nr *= 0.9;
      ir *= 0.9;
      rank *= 0.9;
    }
    self.nway = (nr.sqrt() as i32) + 1;
    self.interval = ((intervalMax / ((ir + 1).sqrt())) as i32) + 1;
    let sr : f32 = rank - self.burstNum + 1 - self.nway + 1 - ir;
    if sr < 0.01 {
      sr = 0.01;
    }
    self.speed = (sr * 0.66).sqrt();
    //assert(speed > 0);
    self.speed *= 0.2;
    if self.speed < 0.1 {
      self.speed = 0.1;
    } else {
      self.speed = (self.speed * 10.0).sqrt() / 10;
    }
    //assert(speed > 0);
    match self.gameState.mode {
    GameState::Mode::CLASSIC => { 
      self.speed *= 0.36;
      if self.speed < 0.05 {
        self.speed = 0.05;
      } else {
        self.speed = (self.speed * 20.0).sqrt / 20.0;
      }
    },
    GameState::Mode::BASIC => {
      self.speed *= 0.4;
    },
    GameState::Mode::MODERN => {
      self.speed *= 0.22;
      if self.speed < 0.04 {
        self.speed = 0.04;
      }
      if self.speed > 0.04 {
        self.speed = (self.speed * 25.0).sqrt() / 25.0;
      }
      },
    }
    if acf {
      self.speedAccel = (self.speed * (0.2 + self.burstNum * 0.05 + rand.nextFloat(0.1))) / (self.burstNum - 1);
      if rand.nextInt(2) == 0 {
        self.speedAccel *= -1;
      }
    }
    if (self.gameState.mode == GameState::Mode::BASIC) && (self.nway > 1) && (rand.nextInt(3) == 0) {
      self.speed *= 0.9;
      self.nwaySpeedAccel = (self.speed * (0.2 + self.nway * 0.05 + rand.nextFloat(0.1))) / (self.nway - 1);
      if rand.nextInt(2) == 0 {
        self.nwaySpeedAccel *= -1;
      }
    }
    if self.nway > 1 {
      self.nwayAngle = (1.66 + rand.nextFloat(0.33)) / (1 + self.nway * 0.7) * nwayDegRatio;
    }
    if rand.nextInt(3) == 0 {
      self.fireingAtATime = true;
    }
    self.minimumFireDist = 5;
  }

 //was move()
  fn move4(&mut self, ts : &TurretState, time : f32 /* = 1*/, anger : f32 /*= 0*/) -> bool {
    if self._disabled {
      return true;
    }
    let itv : f32 = self.interval * ((1 - anger) * 0.99 + 0.01);
    if itv < 3 {
      itv = 3;
    }
    if ts.fireCnt > itv {
      ts.fireCnt = itv;
    }
    let spd : f32 = self.speed * (1 + anger * 0.2);
    if self.fireingAtATime {
      ts.fireCnt -= time;
      if ts.fireCnt <= 0 {
        ts.fireCnt = itv;
        if ts.fireCnt < 3 {
          ts.fireCnt = 3;
        }
        if self.isAbleToFire(ts.pos) {
          let sp : f32 = spd - self.speedAccel * (self.burstNum - 1) / 2;
          for i in 0..self.burstNum {
            let d : f32 = ts.deg - self.nwayAngle * (self.nway - 1) / 2 + self.nwayBaseDeg;
            let nsp : f32 = sp - self.nwaySpeedAccel * ts.nwaySpeedAccelDir * (self.nway - 1) / 2;
            for j in 0..self.nway {
              let b : Bullet = self.bullets.getInstance();
              if !b {
                break;
              }
              b.set(self.bulletSpec, ts.pos, d, nsp * SPEED_RATIO);
              b.setWaitCnt(i * self.burstInterval);
              d += self.nwayAngle;
              nsp += self.nwaySpeedAccel * ts.nwaySpeedAccelDir;
            }
            sp += self.speedAccel;
          }
          ts.nwaySpeedAccelDir *= -1;
        }
      }
    } else {
      if ts.burstNum <= 0 {
        ts.fireCnt -= time;
        if ts.fireCnt <= 0 {
          ts.fireCnt = itv;
          if ts.fireCnt < 3 {
            ts.fireCnt = 3;
          }
          ts.burstNum = self.burstNum;
          ts.burstCnt = 0;
          ts.speed = spd - self.speedAccel * (ts.burstNum - 1) / 2;
        }
      }
      if ts.burstNum > 0 {
        ts.burstCnt -= time;
        if ts.burstCnt <= 0 {
          ts.burstCnt = self.burstInterval;
          ts.burstNum -= 1;
          if self.isAbleToFire(ts.pos) {
            let d : f32 = ts.deg - self.nwayAngle * (self.nway - 1) / 2 + self.nwayBaseDeg;
            let nsp : f32 = ts.speed - self.nwaySpeedAccel * ts.nwaySpeedAccelDir * (self.nway - 1) / 2;
            for i in 0..self.nway {
              let b : Bullet = self.bullets.getInstance();
              if !b {
                break;
              }
              b.set(self.bulletSpec, ts.pos, d, nsp * SPEED_RATIO);
              d += self.nwayAngle;
              nsp += self.nwaySpeedAccel * ts.nwaySpeedAccelDir;
            }
          }
          ts.speed += self.speedAccel;
        }
      }
    }
    true
  }

  fn isAbleToFire(&self, p : Vector) -> bool {
    if self.gameState.mode != GameState::Mode::MODERN {
      p.y > 0
    } else {
      (p.y > 0) && (p.dist(self.player.pos) > self.minimumFireDist)
    }
  }

  fn disabled(&mut self, v : bool) -> bool {
    self._disabled = v;
    v
  }
}

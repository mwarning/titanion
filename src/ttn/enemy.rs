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
use ttn::player::*;
use ttn::dummy::*;


//######################

/**
 * Enemies and turrets.
 */

static /*mut*/ trailEffect : bool = false;

pub struct EnemyPool<'a> {
  ap : ActorPool<Enemy<'a>>,
  _field : Field,
}

impl<'a> EnemyPool<'a> {
  fn getNearestEnemy(&self, p : Vector) -> Option<&Enemy> {
    let dst : f32 = 99999.0;
    let ne : Option<&Enemy> = None;
    for e in &self.ap.actors {
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
    for e in &self.ap.actors {
      if e.getExists() && !e.isBeingCaptured() {
        if e.tok.spec.get_type() == EnemySpecType::MiddleEnemySpec {
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
    if let Some(e) = self.getNearestEnemy(p) {
      let ox : f32 = self._field.normalizeX(e.pos().x - p.x);
      let oy : f32 = e.pos().y - p.y;
      if ox.abs() < (1.0 * e.tok.state.size.x) && oy.abs() < (1.0 * e.tok.state.size.y * widthRatio) {
        e.hitShot(deg);
        return true
      }
    }
    false
  }

  fn checkBulletHit(&self, p : Vector, pp : Vector) -> bool {
    let hitf : bool = false;
    for e in self.ap.actors {
      if e.getExists() && e.isCaptured() {
        if self._field.checkHitDist(e.pos(), p, pp, BULLET_HIT_WIDTH) {
          e.hitCaptured();
          hitf = true;
        }
      }
    }
    hitf
  }

  fn checkEnemyHit(&self, p : Vector, size : Vector) -> bool {
    let hitf = false;
    for e in &self.ap.actors {
      if e.getExists() && e.isCaptured() {
        let ox = self._field.normalizeX(e.pos().x - p.x);
        let oy = e.pos().y - p.y;
        if ox.abs() < 0.5 * (e.tok.state.size.x + size.x) &&
            oy.abs() < 0.5 * (e.tok.state.size.y + size.y) {
          e.hitCaptured();
          hitf = true;
        }
      }
    }
    hitf
  }

  fn checkMiddleEnemyExists(&self, x : f32, px : f32) -> bool {
    for e in &self.ap.actors {
      if e.getExists() && !e.isBeingCaptured() {
        if e.tok.spec.get_type() == EnemySpecType::MiddleEnemySpec {
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
    for e in &self.ap.actors {
      if e.getExists() && !e.isCaptured() {
        n += 1;
      }
    }
    n
  }

  fn numInAttack(&self) -> i32 {
    let mut n = 0;
    for e in &self.ap.actors {
      if e.getExists() && e.isInAttack() {
        n += 1;
      }
    }
    n
  }

  fn numInScreen(&self) -> i32 {
    let mut n = 0;
    for e in &self.ap.actors {
      if e.getExists() && e.isInScreen() {
        n += 1;
      }
    }
    n
  }

  fn numBeforeAlign(&self) -> i32 {
    let mut n = 0;
    for e in &self.ap.actors {
      if e.getExists() && e.beforeAlign() {
        n += 1;
      }
    }
    n
  }

  fn drawFront(&self) {
    if trailEffect {
      for a in &self.ap.actors {
        if a.getExists() && (a.tok.state.ts.pos.y <= (self._field.size().y * 1.5)) {
          a.drawTrails();
        }
      }
    }
    for a in &self.ap.actors {
      if a.getExists() && (a.tok.state.ts.pos.y <= (self._field.size().y * 1.5)) {
        a.draw1();
      }
    }
  }

  fn drawBack(&self) {
    if trailEffect {
      for a in &self.ap.actors {
        if a.getExists() &&
            a.tok.state.ts.pos.y > self._field.size().y * 1.5 &&
            (a.tok.state.ts.pos.x <= Field::circularDistance() / 4.0 &&
             a.tok.state.ts.pos.x >= -Field::circularDistance() / 4.0) {
          a.drawTrails();
        }
      }
    }
    for a in &self.ap.actors {
      if a.getExists() &&
          a.tok.state.ts.pos.y > self._field.size().y * 1.5 &&
          (a.tok.state.ts.pos.x <= Field::circularDistance() / 4.0 &&
           a.tok.state.ts.pos.x >= -Field::circularDistance() / 4.0) {
        a.draw1();
      }
    }
  }

  fn drawPillarBack(&self) {
    if trailEffect {
      for a in &self.ap.actors {
        if a.getExists() &&
            a.tok.state.ts.pos.y > self._field.size().y * 1.5 &&
            (a.tok.state.ts.pos.x > Field::circularDistance() / 4.0 ||
             a.tok.state.ts.pos.x < -Field::circularDistance() / 4.0) {
          a.drawTrails();
        }
      }
    }
    for a in &self.ap.actors {
      if a.getExists() &&
          a.tok.state.ts.pos.y > self._field.size().y * 1.5 &&
          (a.tok.state.ts.pos.x > Field::circularDistance() / 4.0 ||
           a.tok.state.ts.pos.x < -Field::circularDistance() / 4.0) {
        a.draw1();
      }
    }
  }

  fn field(&self, v : Field) -> Field {
    self._field = v;
    v
  }
}

pub struct Enemy<'a> {
  //tok : Token<EnemyState, EnemySpec>, //inlined
  state : &'a mut EnemyState,
  spec : &'a mut EnemySpec,
  _exists : bool, //inherited by Actor class
}

impl<'a> Actor for Enemy<'a> {
  fn getExists(&self) -> bool {
    self._exists
  }

  fn setExists(&mut self, v : bool)-> bool {
    self._exists = v;
    v
  }

  fn init(&mut self /*, args : &[Object]*/) {
    Token::<EnemyState, EnemySpec>::init(self /*, args*/);
    self.tok.state.enemy = self;
  }
}

impl<'a> Token<EnemyState, EnemySpec> for Enemy<'a> {
}

impl<'a> Enemy<'a> {

  fn setSmallEnemyState(&mut self, baseSpeed : f32, angVel : f32, waitCnt : i32, appPattern : i32,
                                er : f32 /*= 0*/, ed : f32 /*= 0*/, gd : bool /*= false*/,
                                fireIntervalRatio : f32 /*= 0*/, firstEnemy : Option<&Enemy> /*= null*/) {
    self.tok.state.baseBaseSpeed = baseSpeed;
    self.tok.state.baseSpeed = baseSpeed;
    self.tok.state.baseAngVel = angVel;
    self.tok.state.angVel = angVel;
    self.tok.state.waitCnt = waitCnt;
    self.tok.state.ellipseRatio = er;
    self.tok.state.ellipseDeg = ed;
    self.tok.state.isGoingDownBeforeStandBy = gd;
    self.tok.state.phase = match appPattern {
      0 =>  -200,
      _ /*1*/ => -100,
    };

    if let Some(e) = firstEnemy {
      // (self.tok.spec as SmallEnemySpec).init(self.tok.state, e.tok.state);
      if is_small_enemy_spec(&self.tok.spec) {
        SmallEnemySpec_init3(self.tok.state, e.tok.state);
      }
      self.tok.state.isFirstEnemy = false;
    } else {
      self.tok.spec.init(self.tok.state);
      self.tok.state.isFirstEnemy = true;
    }
  }

  fn setMiddleEnemyState(&mut self, baseSpeed : f32, angVel : f32, er : f32 /* = 0*/, ed : f32 /*= 0*/) {
    self.tok.state.baseBaseSpeed = baseSpeed;
    self.tok.state.baseSpeed = baseSpeed;
    self.tok.state.baseAngVel = angVel;
    self.tok.state.angVel= angVel;
    self.tok.state.ellipseRatio = er;
    self.tok.state.ellipseDeg = ed;
    self.tok.spec.init(self.tok.state);
  }

  fn setGhostEnemyState(&mut self, x : f32, y : f32, deg : f32, cnt : i32) {
    self.tok.state.ts.pos.x = x;
    self.tok.state.ts.pos.y = y;
    self.tok.state.ts.deg = deg;
    self.tok.state.cnt = cnt;
  }

  fn hitShot(&self, deg : f32 /*= 0*/) {
    if self.tok.spec.hitShot(&self.tok.state, deg) {
      self.tok.remove();
    }
  }

  fn hitCaptured(&mut self) {
    /*
    let ses : SmallEnemySpec = self.tok.spec as SmallEnemySpec;
    if ses {
      ses.hitCaptured(&self.tok.state);
    }*/
    if is_small_enemy_spec(&self.tok.spec) {
      self.tok.spec.hitCaptured(&self.tok.state);
    }
  }

  fn destroyed(&mut self) {
    self.tok.spec.destroyed(&self.tok.state, 0.0);
    self.tok._exists = false;
  }

  fn isInAttack(&self) -> bool {
    if self.tok.spec.isBeingCaptured(&self.tok.state) {
      return false;
    }
    self.tok.spec.isInAttack(&self.tok.state)
  }

  fn isInScreen(&self) -> bool {
    if self.tok.spec.isBeingCaptured(&self.tok.state) {
      return false;
    }
    self.tok.spec.isInScreen(&self.tok.state);
  }

  fn isBeingCaptured(&self) -> bool {
    self.tok.spec.isBeingCaptured(&self.tok.state)
  }

  fn isCaptured(&self) -> bool {
    match self.tok.spec.get_type() {
      EnemySpecType::GhostEnemySpec => true,
      EnemySpecType::MiddleEnemySpec => false,
      EnemySpecType::SE1Spec | EnemySpecType::SE2Spec => self.tok.spec.isCaptured(&self.tok.state),
    }
    /*
    let ges : GhostEnemySpec = self.tok.spec as GhostEnemySpec;
    if ges {
      return true;
    }
    let ses : SmallEnemySpec = self.tok.spec as SmallEnemySpec;
    if !ses {
      return false;
    }
    ses.isCaptured(&self.tok.state)
    */
  }

  fn beforeAlign(&self) -> bool {
    if self.tok.spec.isBeingCaptured(&self.tok.state) {
      return false;
    }
    self.tok.spec.beforeAlign(&self.tok.state)
  }

  fn drawTrails(&self) {
    self.tok.spec.drawTrails(&self.tok.state);
  }

  fn pos(&self) -> Vector {
    self.tok.state.ts.pos
  }
}

const TRAIL_NUM : usize = 64;
const TRAIL_INTERVAL : i32 = 8;
const TURRET_MAX_NUM2 : usize = 3;

struct EnemyState {
  ts : TokenState,
  turretStates : [TurretState; TURRET_MAX_NUM2],
  enemy : &'static Enemy<'static>,
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
      Screen::setColor(r * a, g * a, b * a, a * 0.66);
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

struct EnemySpecData {
  ts : TokenSpec<EnemyState>,
  //mixin StaticRandImpl;
  bullets : *mut BulletPool,
  player : &'static Player,
  particles : *mut ParticlePool,
  bonusParticles : *mut ParticlePool,
  enemies : &'static EnemyPool<'static>,
  stage : *mut Stage,
  trailShape : *mut EnemyShape,
  bulletSpec : *mut BulletSpec,
  counterBulletSpec : *mut BulletSpec,
  turretSpecs : [TurretSpec; TURRET_MAX_NUM1],
  turretNum : i32,
  turretWidth : f32, //= 0;
  gameState : GameState,
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

impl EnemySpecData {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles : *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : GameState) -> EnemySpec {
    EnemySpecData {
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
      turretWidth : 0.0,
      shield : 1.0,
      rank : 0.0,
    }
  }
}

// helper enum
#[derive(PartialEq)]
enum EnemySpecType {
  GhostEnemySpec,
  MiddleEnemySpec,
  //SmallEnemySpec,
  SE1Spec,
  SE2Spec,
}

//Helper
fn is_small_enemy_spec(es : &EnemySpec) -> bool {
  let ty = es.get_type();
  (ty == EnemySpecType::SE1Spec) || (ty == EnemySpecType::SE1Spec)
}

trait EnemySpec {
  // helpers
  fn get_enemyspec_data(&mut self) -> &mut EnemySpecData;
  fn get_type(&self) -> EnemySpecType;

  fn set(&mut self , es : &mut EnemyState) {
    let spec = self.get_enemyspec_data();
    es.shield = spec.shield;
    for i in 0..spec.turretNum {
      spec.turretSpecs[i as usize].set(&es.turretStates[i as usize]);
    }
  }

  fn move2(&mut self, es : &mut EnemyState) -> bool {
    let spec = self.get_enemyspec_data();
    //with (es) {
      es.move1();
      if self.isInScreen(es) && es.isFirstEnemy {
        Sound::playSe("flying_down.wav");
        es.isFirstEnemy = false;
      }
      if es.captureState > 0 {
        self.moveCaptured(es);
        return true;
      }
      if spec.player.enemiesHasCollision() {
        if spec.enemies.checkEnemyHit(es.ts.pos, es.size) {
          self.destroyed(es, 0.0);
          return false;
        }
      }
      if spec.player.checkEnemyHit(es.ts.pos, es.vel, es.size) {
        self.destroyed(es, 0.0);
        return false;
      }
      if spec.capturable {
        self.checkCaptured(es);
      }
      let er: f32 = (1.0 - es.ellipseRatio) + (es.ts.deg + es.ellipseDeg).sin().abs() * es.ellipseRatio * 2.0;
      let rk : f32 = spec.rank;
      es.vel.x -= es.ts.deg.sin() * es.ts.speed * er * 0.1 * rk;
      es.vel.y += es.ts.deg.cos() * es.ts.speed * er * 0.1 * rk;
      es.vel *= 0.9;
      es.ts.pos += es.vel;
      if self.isInScreen(es) {
        spec.ts.field.addSlowdownRatio(es.ts.speed * 0.04 * rk);
      }
      es.ts.pos.x = spec.ts.field.normalizeX(es.ts.pos.x);
      es.recordTrail();
      if (es.phase >= -50) && (es.phase < 0) && !spec.ts.field.containsIncludingPit(es.ts.pos) {
        return false;
      }
      if es.waitCnt > 0 {
        es.waitCnt -= 1;
      } else {
        let cp : Vector = es.centerPos;
        es.centerPos.x = spec.ts.field.normalizeX(es.centerPos.x);
        es.phaseCnt += 1;
        if spec.ts.field.calcCircularDist2(es.centerPos, es.ts.pos) < NEXT_PHASE_DIST {
          es.nextPhaseCnt -= 1;
          if es.nextPhaseCnt <= 0 {
            es.phase += 1;
            if !self.gotoNextPhase(es) {
              return false;
            }
          }
        }
        cp.x = spec.ts.field.normalizeX(cp.x);
        let dst : f32 = spec.ts.field.calcCircularDist2(cp, es.ts.pos);
        es.ts.speed += ((es.baseSpeed * (1.0 + dst * 0.1)) - es.ts.speed) * 0.05;
        let mut av : f32 = es.angVel * rk;
        let mut td : f32 = spec.ts.field.normalizeX(-(cp.x - es.ts.pos.x)).atan2(cp.y - es.ts.pos.y);
        let mut ad : f32 = normalize_deg(td - es.ts.deg);
        av *= 2.5 - er;
        if (ad > av) || (ad < (-PI * 0.8)) {
          es.ts.deg += av;
        } else if ad < -av {
          es.ts.deg -= av;
        } else {
          es.ts.deg = td;
        }
        //assert(deg <>= 0);
        for i in 0..spec.turretNum {
          let ts : TurretState = es.turretStates[i as usize];
          let tx : f32 = es.ts.pos.x;
          let ty : f32 = es.ts.pos.y;
          match i {
          _ /*0*/ =>  {},
          1 => { tx -= spec.turretWidth; },
          2 => { tx += spec.turretWidth; },
          }
          let turretDeg : f32 = spec.ts.field.normalizeX(-(spec.player.pos().x - tx)).atan2(spec.player.pos().y - ty);
          match spec.gameState.mode {
            GameStateMode::CLASSIC => {
              if (turretDeg >= 0.0) && (turretDeg < (PI - PI / 6.0)) {
                turretDeg = PI - PI / 6.0;
              } else if (turretDeg < 0.0) && turretDeg > (-PI + PI / 6.0) {
                turretDeg = -PI + PI / 6.0;
             }
             turretDeg = ((((turretDeg + PI / 64.0) / (PI / 32.0)) as i32) as f32) * (PI / 32.0);
            },
            GameStateMode::BASIC => {
              if (turretDeg >= 0.0) && (turretDeg < (PI - PI / 4.0)) {
               turretDeg = PI - PI / 4.0;
              } else if (turretDeg < 0.0) && (turretDeg > (-PI + PI / 4.0)) {
               turretDeg = -PI + PI / 4.0;
              }
            },
            GameStateMode::MODERN => {}
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
    let spec = self.get_enemyspec_data();
    //with (es) {
      match es.captureState {
      1 => {
        es.vel.x += (spec.player.pos().x - es.ts.pos.x) * 0.03;
        es.vel.y += (spec.player.pos().y - es.ts.pos.y) * 0.03;
        es.ts.pos.x += (spec.player.pos().x - es.ts.pos.x) * 0.03;
        es.ts.pos.y += (spec.player.pos().y - es.ts.pos.y) * 0.03;
        es.ts.deg *= 0.95;
        if spec.player.pos().dist2(es.ts.pos) < 1.0 {
          es.captureState = 2;
        }
      },
      2 => {
        let cx : f32 = self.calcCapturePosX(es.captureIdx);
        es.vel.x += (spec.player.pos().x + cx - es.ts.pos.x) * 0.03;
        es.ts.pos.x += (spec.player.pos().x + cx - es.ts.pos.x) * 0.1;
        es.ts.pos.y += (spec.player.pos().y - es.ts.pos.y) * 0.33;
        es.vel.y *= 0.6;
        es.ts.deg *= 0.95;
        if (spec.player.pos().x + cx - es.ts.pos.x).abs() < 0.2 {
          es.captureState = 3;
        }
      },
      3 => {
        let cx : f32 = self.calcCapturePosX(es.captureIdx);
        es.ts.pos.x = spec.player.pos().x + cx;
        es.ts.pos.y = spec.player.pos().y;
        es.ts.deg = spec.player.deg();
        }
      }
      es.vel *= 0.9;
      es.ts.pos += es.vel;
    //}
  }

  fn calcCapturePosX(&self, idx : i32) -> f32 {
    let spec = self.get_enemyspec_data();
    if (idx % 2) == 0 {
      ((idx as f32 / 2.0) + 0.5) * PlayerSpec_CAPTURED_ENEMIES_INTERVAL_LENGTH * spec.player.capturedEnemyWidth()
    } else {
      -((idx as f32 / 2.0) + 0.5) * PlayerSpec_CAPTURED_ENEMIES_INTERVAL_LENGTH * spec.player.capturedEnemyWidth()
    }
  }

  fn checkCaptured(&self, es : &mut EnemyState) {
    let spec = self.get_enemyspec_data();
    //with (es) {
      if spec.player.isInTractorBeam(es.ts.pos) {
        if spec.gameState.mode != GameStateMode::MODERN {
          let idx : i32 = spec.player.addCapturedEnemy(es.enemy);
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
    let spec = self.get_enemyspec_data();
    spec.player.destroyCapturedEnemies(es.captureIdx);
  }

  fn isBeingCaptured(&self, es : &EnemyState) -> bool {
    (es.captureState > 0)
  }

  fn isCaptured(&self, es : &EnemyState) -> bool {
    (es.captureState == 3)
  }

  fn beforeAlign(&self, es : &EnemyState) -> bool {
    (es.phase < -10)
  }

  fn hitShot(&mut self, es : &mut EnemyState, dd : f32 /* = 0*/) -> bool {
    let spec = self.get_enemyspec_data();
    //with (es) {
      es.shield -= 1;
      let r : f32 = 0.5 + rand.nextFloat(0.5);
      let g : f32 = 0.1 + rand.nextFloat(0.5);
      let b : f32 = 0.5 + rand.nextFloat(0.5);
      for i in 0..10 {
        let p : Particle = spec.particles.getInstanceForced();
        let d : f32 = dd + rand.nextSignedFloat(PI / 4.0);
        p.set(Particle.Shape.LINE, es.ts.pos.x, es.ts.pos.y, d, 0.1 + rand.nextFloat(0.5), 1,
              r, g, b, 30 + rand.nextInt(30));
        p = spec.particles.getInstanceForced();
        d = dd + PI + rand.nextSignedFloat(PI / 4.0);
        p.set(Particle.Shape.LINE, es.ts.pos.x, es.ts.pos.y, d, 0.1 + rand.nextFloat(0.5), 1,
              r, g, b, 30 + rand.nextInt(30));
      }
      if es.shield <= 0 {
        self.destroyed(es, dd);
        return true;
      }
      match spec.gameState.mode {
       GameStateMode::CLASSIC => {
        es.ts.targetSize.x *= 1.3;
        es.ts.targetSize.y *= 1.3;
        },
      GameStateMode::BASIC => {
        es.ts.targetSize.x *= 1.2;
        es.ts.targetSize.y *= 1.2;
        },
      GameStateMode::MODERN => {
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
    let spec = self.get_enemyspec_data();
    //with (es) {
      let r : f32 = 0.5 + rand.nextFloat(0.5);
      let g : f32 = 0.1 + rand.nextFloat(0.5);
      let b : f32 = 0.5 + rand.nextFloat(0.5);
      let sz : f32 = (es.ts.targetSize.x + es.ts.targetSize.y) / 2;
      sz = (sz - 1.0) * 2.0 + 1.0;
      let mut n : i32 = 3 + rand.nextInt(2);
      n *= sz;
      for i  in 0..n {
        let p : Particle = spec.particles.getInstanceForced();
        let d : f32 = dd + rand.nextSignedFloat(PI / 5.0);
        p.set(Particle.Shape.TRIANGLE, es.ts.pos.x, es.ts.pos.y, d, 0.5,
              (2.0 + rand.nextFloat(0.5)) * sz, r, g, b, 50 + rand.nextInt(100));
      }
      for i in 0..n {{
        let p : Particle = spec.particles.getInstanceForced();
        let d : f32 = rand.nextFloat(PI * 2.0);
        p.set(Particle.Shape.QUAD, es.ts.pos.x, es.ts.pos.y, d, 0.1 + rand.nextFloat(0.1),
              (1 + rand.nextFloat(0.5)) * sz, r, g, b, 50 + rand.nextInt(100));
      }
      if !self.isBeingCaptured(es) {
        if spec.removeBullets {
          let mut cnt : i32 = 1;
          spec.bullets.removeAround(cnt, es.ts.pos, spec.particles, spec.bonusParticles, spec.player);
          let p : Particle = spec.bonusParticles.getInstanceForced();
          let mut wc : i32;
          if cnt <= 50 {
            wc = cnt;
          } else {
            wc = 50 + (((cnt - 50) as f32).sqrt() as i32);
          }
          p.set(Particle.Shape.BONUS, es.ts.pos.x, es.ts.pos.y, 0, 0.1,
                1.0 + (wc as f32) / 75.0, 1, 1, 1, 120, false, cnt, wc);
          spec.player.addScore(spec.score * cnt);
        } else {
          if spec.gameState.mode == GameStateMode::BASIC {
            let oy : f32 = es.ts.pos.y - spec.player.pos.y;
            let mut pm : i32 = (18.0 - oy) as i32;
            if pm > 16 {
              pm = 16;
            } else if pm < 1 {
              pm = 1;
            }
            spec.player.addScore(spec.score * pm);
            let mut p : Particle = spec.bonusParticles.getInstanceForced();
            p.set(Particle.Shape.BONUS, es.ts.pos.x, es.ts.pos.y, 0, 0.1,
                  0.5, 1, 1, 1, 60, false, pm);
            spec.gameState.setProximityMultiplier(pm);
          } else {
            spec.player.addScore(spec.score);
          }
        }
        spec.player.addMultiplier(0.1);
        if spec.stage.existsCounterBullet {
          if let Some(blt) = spec.bullets.getInstance() {
            blt.set(spec.counterBulletSpec, es.ts.pos,
                    spec.turretStates[0].deg, spec.turretSpecs[0].speed * TurretSpec_SPEED_RATIO);
          }
        }
      }
      Sound::playSe(spec.explosionSeName);
    }
  }

  fn provacated(&mut self, es : &mut EnemyState) {
    let spec = self.get_enemyspec_data();
    //with (es) {
      es.anger += (1 - es.anger) * 0.05;
      if es.sizeVel.dist < 0.1 {
        es.sizeVel.x = 0.2;
        es.sizeVel.y = 0.2;
      }
      let mut p : Particle = spec.particles.getInstanceForced();
      p.set(Particle.Shape.LINE, es.ts.pos.x, es.ts.pos.y, PI / 2.0 + rand.nextSignedFloat(PI / 4.0),
            0.1 + rand.nextFloat(0.2), 1,
            1, 0.5, 0.5, 30 + rand.nextInt(30));
      p = spec.particles.getInstanceForced();
      p.set(Particle.Shape.LINE, es.ts.pos.x, es.ts.pos.y, -PI / 2.0 + rand.nextSignedFloat(PI / 4.0),
            0.1 + rand.nextFloat(0.2), 1,
            1, 0.5, 0.5, 30 + rand.nextInt(30));
      if spec.removeBullets {
        spec.player.midEnemyProvacated();
      }
    //}
  }

  fn gotoNextPhaseInAppearing(&mut self, es : &mut EnemyState) -> bool {
    let spec = self.get_enemyspec_data();
    //with (es) {
      match es.phase {
      -300 => {
        let mut cpw : f32;
        match spec.gameState.mode {
          GameStateMode::CLASSIC => { cpw = 0.2; },
          GameStateMode::BASIC => { cpw = 0.2; },
          GameStateMode::MODERN => { cpw = 0.4; },
        }
        es.centerPos.x = rand.nextSignedFloat(spec.field.size.x * cpw);
        es.centerPos.y = spec.field.size.y * 2.0;
        es.standByPos.x = rand.nextSignedFloat(spec.field.size.x * cpw);
        es.standByPos.y = spec.field.size.y * (0.7 + rand.nextFloat(0.1));
        es.nextPhaseCnt = 15;
        es.baseSpeed = es.baseBaseSpeed * 1.5;
        es.angVel = es.baseAngVel * 1.5;
        es.phase = -50;
        },

      -200 => {
        es.centerPos.x = rand.nextSignedFloat(spec.field.size.x * 0.1);
        es.centerPos.y = spec.field.size.y * 1.6;
        if es.centerPos.x < 0 {
          es.standByPos.x = spec.field.size.x * (rand.nextSignedFloat(0.4) + 0.4);
        } else {
          es.standByPos.x = spec.field.size.x * (rand.nextSignedFloat(0.4) - 0.4);
        }
        es.standByPos.y = spec.field.size.y * (0.5 + rand.nextFloat(0.3));
        es.nextPhaseCnt = 60;
        es.baseSpeed = es.baseBaseSpeed * 1.0;
        es.angVel = es.baseAngVel * 1.0;
        },
      -199 => {
        if es.standByPos.x < 0 {
          es.centerPos.x = spec.field.size.x * 0.75;
        } else {
          es.centerPos.x = -spec.field.size.x * 0.75;
        }
        es.centerPos.y = 0;
        if spec.isGoingDownBeforeStandBy {
          es.nextPhaseCnt = 20;
        } else {
          es.nextPhaseCnt = 60;
        }
        es.baseSpeed = es.baseBaseSpeed * 2.0;
        es.angVel = es.baseAngVel * 2.0;
        es.phase = -50;
       },
 
      -100 => {
        es.centerPos.x = spec.field.size.x * 4.0;
        if rand.nextInt(2) == 0 {
          es.centerPos.x *= -1;
        }
        es.centerPos.y = spec.field.size.y * 1.6;
        if es.centerPos.x < 0 {
          es.standByPos.x = spec.field.size.x * (rand.nextSignedFloat(0.4) + 0.4);
        }
        else {
          es.standByPos.x = spec.field.size.x * (rand.nextSignedFloat(0.4) - 0.4);
        }
        es.standByPos.y = spec.field.size.y * (0.5 + rand.nextFloat(0.3));
        es.nextPhaseCnt = 20;
        es.baseSpeed = es.baseBaseSpeed * 2.0;
        es.angVel = es.baseAngVel * 2.0;
       },
      -99 => {
        if es.centerPos.x > 0 {
          es.centerPos.x = spec.field.size.x * 2.0;
        } else {
          es.centerPos.x = -spec.field.size.x * 2.0;
        }
        es.centerPos.y = -spec.field.size.y * 1.2;
        es.nextPhaseCnt = 20;
        es.baseSpeed = es.baseBaseSpeed * 2;
        es.angVel = es.baseAngVel * 2;
      },
      -98 => {
        if es.centerPos.x > 0 {
          es.centerPos.x = spec.field.size.x * 0.5;
        } else {
          es.centerPos.x = -spec.field.size.x * 0.5;
        }
        es.centerPos.y = 0;
        es.nextPhaseCnt = 30;
        es.phase = -50;
      },
      -49 => {
        if spec.isGoingDownBeforeStandBy {
          es.centerPos.x = es.centerPos.x / 2.0;
          es.centerPos.y = 0.0;
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
        spec.vcenterPos.x = (es.centerPos.x + spec.player.pos.x * 2.0) / 3.0;
        es.centerPos.y = -spec.field.size.y * 1.2;
        es.baseSpeed = es.baseBaseSpeed * 1.2;
        es.angVel = es.baseAngVel * 1.2;
        es.nextPhaseCnt = 5;
       },
      -28 => {
        es.centerPos.y = -spec.field.size.y * 1.5;
        es.nextPhaseCnt = 10;
       },
      -9 => {
        es.phase = 0;
      },
      _ => {
        return false;
      }
      }
      es.nextPhaseCnt /= spec.rank;
      es.phaseCnt = 0;
    //}
    true;
  }

  fn movePhase(&mut self, es : &mut EnemyState) {
    let spec = self.get_enemyspec_data();
    //with (es) {
      match es.phase {
      -200|-100 => {
        if es.ts.pos.y < (spec.field.size.y * 1.5) {
          es.ts.pos.y = spec.field.size.y * 1.5;
        }
      },
      -99 => {
        if (es.centerPos.x < 0) && (es.ts.pos.x > -spec.field.size.x) {
          es.ts.pos.x += (-spec.field.size.x - es.ts.pos.x) * 0.2;
        } else if (es.centerPos.x > 0) && (es.ts.pos.x < spec.field.size.x) {
          es.ts.pos.x += (spec.field.size.x - es.ts.pos.x) * 0.2;
        }
      },
      -50|-49|-10=> {
        if es.ts.pos.y < (-spec.field.size.y * 0.5) {
          es.ts.pos.y += (-spec.field.size.y * 0.5 - es.ts.pos.y) * 0.2;
        }
      },
      _ => {},
      };
      if self.isInAttack(es) {
        if (spec.gameState.mode == GameStateMode::MODERN) || (es.phase >= 0) || (rand.nextInt(5) == 0) {
          for i in 0..spec.turretNum {
            spec.turretSpecs[i].move(spec.turretStates[i], es.rank, es.anger);
          }
        }
      }
    //}
  }

  fn isInScreen(&self, es : &EnemyState) -> bool {
    let spec = self.get_enemyspec_data();
    spec.field.size.contains(es.pos);
  }

  fn setRank(&mut self, rank : f32);
  fn init(&mut self, es : &mut EnemyState);
  fn gotoNextPhase(&self, es : &mut EnemyState) -> bool;
  fn isInAttack(&self, es : &EnemyState) -> bool;
  fn calcStandByTime(&self, es : &EnemyState) -> i32;

  fn draw(&self, es : &EnemyState) {
    let spec = self.get_enemyspec_data();
    let mut p : Vector3 = spec.field.calcCircularPos(es.ts.pos);
    let mut cd : f32 = spec.field.calcCircularDeg(es.ts.pos.x);
    (spec.ts.shape as EnemyShape).draw(p, cd, es.deg, es.cnt, es.size);
    for i in 1..spec.turretNum {
      let x : f32 = es.ts.pos.x;
      match i {
      1 => {
        x -= spec.turretWidth;
      },
      2 => {
        x += spec.turretWidth;
      },
      }
      p = spec.field.calcCircularPos(x, es.ts.pos.y);
      cd = spec.field.calcCircularDeg(x);
      Screen::setColor(0.5, 0.5, 1);
      (spec.trailShape as EnemyShape).draw(p, cd, es.deg, es.cnt, es.size.x * 0.5, es.size.y * 0.5);
    }
  }

  fn drawTrails(&self, es : &EnemyState) {
    let spec = self.get_enemyspec_data();
    if es.captureState > 0 {
      return;
    }
    es.drawTrails(spec.trailShape, 0.2, 0.2, 0.8, es.size, spec.field);
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

pub struct GhostEnemySpec {
  es : EnemySpecData,
}

impl GhostEnemySpec {
  fn new(&mut self, field : *mut Field, shape : *mut Shape) -> GhostEnemySpec {
    GhostEnemySpec{ field: field, shape: shape}
  }
}

impl EnemySpec for GhostEnemySpec {
  fn get_enemyspec_data(&mut self) -> &mut EnemySpecData {
    &mut self.es
  }

  fn get_type(&self) -> EnemySpecType {
    EnemySpecType::GhostEnemySpec
  }

  fn draw(&self, es : &EnemyState) {
    //with (es) {
      let p : Vector3 = self.field.calcCircularPos(es.ts.pos);
      let cd : f32 = self.field.calcCircularDeg(es.ts.pos.x);
      Screen::setColor(0.5, 0.5, 1, 0.8);
      (self.ts.shape as EnemyShape).draw(p, cd, es.ts.deg, self.cnt, es.size);
    //}
  }

  fn set(&mut self, es : &mut EnemyState) {}
  fn move2(&mut self, es : &mut EnemyState) -> bool { true }
  fn destroyed(&mut self, es : &mut EnemyState, dd : f32 /*= 0*/) {}
  fn setRank(&mut self, rank : f32) {}
  fn init(&mut self, es : &mut EnemyState) {}
  fn gotoNextPhase(&self, es : &mut EnemyState) -> bool { false }
  fn isInAttack(&self, ses : &EnemyState) -> bool { false }
  fn calcStandByTime(&self, es : &EnemyState) -> i32 { 0 }
  fn isBeingCaptured(&self, es : &EnemyState) -> bool { true }
  fn isCaptured(&self, es : &EnemyState) -> bool { true }
}

struct MiddleEnemySpec {
 es : EnemySpecData,
}

impl MiddleEnemySpec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles :  *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : *mut GameState) -> MiddleEnemySpec {
    let mut inst = MiddleEnemySpec{es : EnemySpecData {
      field : field, bullets: bullets, player : player, particles : particles, bonusParticles : bonusParticles, enemies : enemies, stage : stage,
          shape : shape, trailShape: trailShape, bulletSpec : bulletSpec, counterBulletSpec : counterBulletSpec, gameState : gameState}};
    for &ts in inst.turretSpecs {
      ts = TurretSpec::new(field, bullets, player, enemies, particles, stage, bulletSpec, gameState);
    }
    match gameState.mode {
    GameStateMode::CLASSIC => {
      inst.shield = 2;
      inst.capturable = false;
      inst.removeBullets = false;
    },
    GameStateMode::BASIC => {
      inst.shield = 3;
      inst.capturable = false;
      inst.removeBullets = false;
    },
    GameStateMode::MODERN => {
      inst.shield = 32;
      inst.capturable = true;
      inst.removeBullets = true;
      },
    }
    inst.score = 100;
    inst.es.explosionSeName = "explosion3.wav";
    inst
  }
}

impl EnemySpec for MiddleEnemySpec {
  fn get_enemyspec_data(&mut self) -> &mut EnemySpecData {
    &mut self.es
  }

  fn get_type(&self) -> EnemySpecType {
    EnemySpecType::MiddleEnemySpec
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
    GameStateMode::CLASSIC => {
      self.es.rank = self.es.rank.sqrt();
      tr = r * 2.0;
      },
    GameStateMode::BASIC => {
      tr = r * 3.0;
      },
    GameStateMode::MODERN => {
      self.es.rank = 1.0;
      tr = r * 15.0;
      },
    };
    if self.es.rank < 1.5 {
      self.es.rank = 1.5;
    }
    self.es.turretSpecs[0].setRankMiddle(tr);
    self.es.turretNum = 1;
    if self.es.gameState.mode == GameStateMode::MODERN {
      let ts : TurretSpec = self.es.turretSpecs[0];
      let ptn : i32 = rand.nextInt(6);
      if ptn == 1 || ptn == 2 || ptn == 4 {
        self.es.turretSpecs[1].copy(self.es.turretSpecs[0]);
        self.es.turretSpecs[2].copy(self.es.turretSpecs[0]);
        if (ts.nway > 1) && (rand.nextInt(2) == 0) {
          let nsa : f32 = (ts.speed * (0.2 + (ts.nway as f32) * 0.05 + rand.nextFloat(0.1))) / (ts.nway - 1);
          if rand.nextInt(2) == 0 {
            nsa *= -1.0;
          }
          self.es.turretSpecs[1].nwaySpeedAccel = nsa;
          self.es.turretSpecs[2].nwaySpeedAccel = -nsa;
        }
        self.es.turretWidth = 1.0 + rand.nextFloat(1.0);
        self.es.turretNum = 3;
        if ptn == 4 {
          self.es.turretSpecs[0].setRankMiddle(tr * 2.0);
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
          let nsa : f32 = (ts.speed * (0.2 + ts.nway * 0.05 + rand.nextFloat(0.1))) / ((ts.nway - 1) as f32);
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

  fn gotoNextPhase(&self, es : &mut EnemyState) -> bool {
    //with (es) {
      if es.phase < 0 {
        return self.es.gotoNextPhaseInAppearing(es);
      }
      match es.phase {
      1 => {
        if (self.es.gameState.mode != GameStateMode::MODERN) && !self.es.player.hasCollision {
          es.phase = 0;
          es.nextPhaseCnt = self.es.calcStandByTime(es);
        } else {
          Sound::playSe("flying_down.wav");
          if self.es.gameState.mode != GameStateMode::MODERN {
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
        if self.es.gameState.mode != GameStateMode::MODERN {
          es.centerPos.x *= -0.9;
          es.centerPos.y = self.field.size.y * (0.2 + rand.nextFloat(0.2));
          es.nextPhaseCnt = 60;
        } else {
          es.centerPos.x = es.standByPos.x;
          es.centerPos.y = 0.0;
          es.baseSpeed = es.baseBaseSpeed * 0.1;
          es.nextPhaseCnt = 10;
        }
      },
      3 => {
        if self.es.gameState.mode != GameStateMode::MODERN {
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

  fn isInAttack(&self, es : &EnemyState) -> bool {
    (es.phase == 1) || (es.phase == 2)
  }

  fn calcStandByTime(&self, es : &EnemyState) -> i32 {
    if (es.phase < 0) || (self.es.gameState.mode == GameStateMode::MODERN) {
      30 + rand.nextInt(30)
    } else {
      200 + rand.nextInt(150)
    }
  }
}

/*
//TODO: finish transform into trait, what about new?
trait SmallEnemySpec { //: EnemySpec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles : *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : *mut GameState) -> SmallEnemySpec {
    let mut inst = SmallEnemySpec{ es : EnemySpecData{
      field : field, bullets: bullets, player : player, particles : particles, bonusParticles : bonusParticles, enemies : enemies, stage : stage,
          shape : shape, trailShape : trailShape, bulletSpec : bulletSpec, counterBulletSpec : counterBulletSpec, gameState : gameState}};
    inst.turretSpecs[0] = TurretSpec::new(field, bullets, player, enemies, particles, stage, bulletSpec, gameState);
    inst.shild = match gameState.mode {
      GameStateMode::CLASSIC => 1,
      GameStateMode::BASIC => 1,
      GameStateMode::MODERN => 2,
    };
    inst.capturable = true;
    inst.score = 10;
    inst.removeBullets = false;
    inst
  }
*/
  fn SmallEnemySpec_init3(self_ : &mut EnemySpec, es : &mut EnemyState, fes : &EnemyState) {
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

  fn SmallEnemySpec_init(self_ : &mut EnemySpec, es : &mut EnemyState) {
    self_.gotoNextPhaseInAppearing(es);
  }

  fn SmallEnemySpec_setRank(self_ : &mut EnemySpec, r : f32) {
    let es = self_.get_enemyspec_data();
    es.rank = (r * 0.5).sqrt();
    let mut tr : f32;
    match es.gameState.mode {
    GameStateMode::CLASSIC => {
      es.rank = es.rank.sqrt();
      tr = r;
    },
    GameStateMode::BASIC => {
      tr = r * 2.0;
    },
    GameStateMode::MODERN => {
      es.rank = 1.0;
      tr = r;
    },
    };
    if es.rank < 1.0 {
      es.rank = 1.0;
    }
    es.turretSpecs[0].setRankNormal(tr, false);
    es.turretNum = 1;
  }

  fn SmallEnemySpec_calcStandByTime(self_ : &EnemySpec, es : &EnemyState) -> i32 {
    60 + rand.nextInt(120)
  }
//}

struct SE1Spec {
  es : EnemySpecData,
}

impl SE1Spec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles : *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : *mut GameState) -> SE1Spec {
    let mut inst = SE1Spec{ es : EnemySpecData{
      field : field, bullets: bullets, player : player, particles : particles, bonusParticles : bonusParticles, enemies : enemies, stage : stage,
      shape : shape, trailShape : trailShape, bulletSpec : bulletSpec, counterBulletSpec : counterBulletSpec, gameState : gameState}};

    inst.turretSpecs[0] = TurretSpec::new(field, bullets, player, enemies, particles, stage, bulletSpec, gameState);
    inst.shild = match gameState.mode {
      GameStateMode::CLASSIC => 1,
      GameStateMode::BASIC => 1,
      GameStateMode::MODERN => 2,
    };
    inst.capturable = true;
    inst.score = 10;
    inst.removeBullets = false;

    inst.ses.es.explosionSeName = "explosion1.wav";
    inst
  }
}

impl EnemySpec for SE1Spec {
  fn get_enemyspec_data(&mut self) -> &mut EnemySpecData {
    self.es.get_enemyspec_data()
  }

  fn get_type(&self) -> EnemySpecType {
    EnemySpecType::SE1Spec
  }
/*
  //we use SmallEnemySpec_init3 directly
  fn init3(&mut self, es : &mut EnemyState, fes : &EnemyState) {
    SmallEnemySpec_init3(self, es, fes);
  }
*/
  fn init(&mut self, es : &mut EnemyState) {
    SmallEnemySpec_init(self, es);
  }

  fn setRank(&mut self, r : f32) {
    SmallEnemySpec_setRank(self, r);
  }

  fn calcStandByTime(&self, es : &EnemyState) -> i32 {
    SmallEnemySpec_calcStandByTime(self, es)
  }

  fn gotoNextPhase(&self, es : &mut EnemyState) -> bool {
    //with (es) {
      if es.phase < 0 {
        return self.es.gotoNextPhaseInAppearing(es);
      }
      match es.phase {
      1 => {
        if !self.es.player.hasCollision || (self.es.enemies.numInAttack > self.es.stage.attackSmallEnemyNum) {
          es.phase = 0;
          es.nextPhaseCnt = self.calcStandByTime(es);
        } else {
          Sound::playSe("flying_down.wav");
          es.centerPos.y = 0.0;
          es.centerPos.x = (es.standByPos.x + self.es.player.pos.x) / 2.0;
          es.nextPhaseCnt = 60;
        }
      },
      2 => {
        es.centerPos.y = -self.es.field.size.y * 0.7;
        es.centerPos.x = self.es.player.pos.x;
        es.nextPhaseCnt = 30;
      },
      3 => {
        es.centerPos.x = es.standByPos.x;
        es.centerPos.y = es.standByPos.y;
        es.phase = 0;
        es.nextPhaseCnt = self.es.calcStandByTime(es);
        },
      }
      es.nextPhaseCnt /= self.es.rank;
      es.phaseCnt = 0;
    //}
    true
  }

  fn isInAttack(&self, es : &EnemyState) -> bool {
    (es.phase < -10 || es.phase == 1 || es.phase == 2)
  }
}

struct SE2Spec {
  es : EnemySpecData,
}

impl SE2Spec {
  fn new(field : *mut Field, bullets : *mut BulletPool, player : *mut Player,
              particles : *mut ParticlePool, bonusParticles : *mut ParticlePool,
              enemies : *mut EnemyPool, stage : *mut Stage,
              shape : *mut Shape, trailShape : *mut EnemyShape,
              bulletSpec : *mut BulletSpec, counterBulletSpec : *mut BulletSpec,
              gameState : *mut GameState) -> SE2Spec {
    let mut inst = SE2Spec{ es : EnemySpecData{
      field : field, bullets: bullets, player : player, particles : particles, bonusParticles : bonusParticles, enemies : enemies, stage : stage,
      shape : shape, trailShape : trailShape, bulletSpec : bulletSpec, counterBulletSpec : counterBulletSpec, gameState : gameState}};

    inst.turretSpecs[0] = TurretSpec::new(field, bullets, player, enemies, particles, stage, bulletSpec, gameState);
    inst.shield = match gameState.mode {
      GameStateMode::CLASSIC => 1,
      GameStateMode::BASIC => 1,
      GameStateMode::MODERN => 2,
    };
    inst.capturable = true;
    inst.score = 10;
    inst.removeBullets = false;

    inst.ses.explosionSeName = "explosion2.wav";
    inst
  }
}

impl EnemySpec for SE2Spec {
  fn get_enemyspec_data(&mut self) -> &mut EnemySpecData {
    self.es.get_enemyspec_data()
  }

  fn get_type(&self) -> EnemySpecType {
    EnemySpecType::SE2Spec
  }
/*
//we use SmallEnemySpec_init3 directly
  fn init3(&mut self, es : &mut EnemyState, fes : &EnemyState) {
    SmallEnemySpec_init3(self, es, fes);
  }
*/
  fn init(&mut self, es : &mut EnemyState) {
    SmallEnemySpec_init(self, es);
  }

  fn setRank(&mut self, r : f32) {
    SmallEnemySpec_setRank(self, r);
  }

  fn calcStandByTime(&self, es : &EnemyState) -> i32 {
    SmallEnemySpec_calcStandByTime(self, es)
  }

  fn gotoNextPhase(&self, es : &mut EnemyState) -> bool {
    //with (es) {
      if es.phase < 0 {
        return self.es.gotoNextPhaseInAppearing(es);
      }
      match es.phase {
      1 => {
        if !self.es.player.hasCollision || (self.es.enemies.numInAttack > self.es.stage.attackSmallEnemyNum) {
          es.phase = 0;
          es.nextPhaseCnt = self.es.calcStandByTime(es);
        } else {
          Sound::playSe("flying_down.wav");
          es.centerPos.y = -self.es.field.size.y * 0.3;
          es.centerPos.x = (es.standByPos.x + self.es.player.pos.x) / 2;
          es.baseSpeed = es.baseBaseSpeed;
          es.angVel = es.baseAngVel;
          es.nextPhaseCnt = 30 + rand.nextInt(60);
        }
      },
      2 => {
        es.centerPos.y = -self.es.field.size.y * 1.3;
        es.centerPos.x *= -1.0;
        es.nextPhaseCnt = 30;
      },
      3 => {
        es.centerPos.y = -self.es.field.size.y * 1.0;
        if es.centerPos.x < 0 {
          es.centerPos.x = -self.es.field.size.x * 1.5;
        } else {
          es.centerPos.x = self.es.field.size.x * 1.5;
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
      es.nextPhaseCnt /= self.es.rank;
      es.phaseCnt = 0;
    //}
    true
  }

  fn movePhase(&mut self, es : &mut EnemyState) {
    self.es.movePhase(es);
    //with (es) {
      if es.phase == 3 {
        if es.centerPos.x < 0.0 {
          if es.ts.pos.x > (-self.es.field.size.x * 1.2) {
            es.ts.pos.x += (es.centerPos.x - es.ts.pos.x) * 0.2;
          }
        } else {
          if es.ts.pos.x < (self.es.field.size.x * 1.2) {
            es.ts.pos.x += (es.centerPos.x - es.ts.pos.x) * 0.2;
          }
        }
      }
    //}
  }
  
  fn isInAttack(&self, es : &EnemyState) -> bool {
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
    self.speed = 1.0;
    self.speedAccel = 0.0;
    self.burstNum = 1;
    self.burstInterval = 99999;
    self.nway = 1;
    self.nwayAngle = 0.0;
    self.nwayBaseDeg = 0.0;
    self.nwaySpeedAccel = 0.0;
    self.fireingAtATime = false;
    self.fireIntervalRatio = 0.0;
    self.disabled = false;
    self.minimumFireDist = 0.0;
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
    GameStateMode::CLASSIC => {
      nr = 0.0;
      br = 0.0;
      ir = (rank * nsr).sqrt() * 2.0;
      self.burstInterval = 3 + rand.nextInt(2);
    },
    GameStateMode::BASIC => {
      if isWide {
        nr = rank * nsr * rr;
        br = 0.0;
        ir = rank * nsr * (1.0 - rr);
      } else {
        nr = 0.0;
        br = rank * nsr * rr;
        ir = rank * nsr * (1.0 - rr);
      }
      self.burstInterval = 3 + rand.nextInt(2);
    },
    GameStateMode::MODERN => {
      if isWide {
        nr = rank * nsr * rr;
        br = 0.0;
        ir = rank * nsr * (1.0 - rr);
      } else {
        nr = 0.0;
        br = rank * nsr * rr;
        ir = rank * nsr * (1.0 - rr);
      }
      intervalMax = 120;
      self.burstInterval = 4 + rand.nextInt(4);
      },
    }
    self.burstNum = 1 + (br.sqrt() as i32);
    self.nway = 1 + (nr.sqrt() as i32);
    self.interval = ((intervalMax / (ir + 1)) as i32) + 1;
    let sr : f32 = (rank - self.nway + 1 - self.burstNum + 1 - ir) as f32;
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
    GameStateMode::CLASSIC => {
      self.speed *= 0.36;
      if self.speed < 0.05 {
        self.speed = 0.05;
      } else {
        self.speed = (self.speed * 20).sqrt() / 20;
      }
    },
    GameStateMode::BASIC => {
      self.speed *= 0.33;
    },
    GameStateMode::MODERN => {
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
      GameStateMode::CLASSIC => {
      nr = 0.0;
      br = 0.0;
      ir = (rank * (0.5 + rand.nextSignedFloat(0.3))).sqrt() * 2;
      nwayDegRatio = 0.0;
      self.burstInterval = 3 + rand.nextInt(2);
      },
    GameStateMode::BASIC => {
      if rand.nextInt(3) == 0 {
        nr = 0.0;
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
    GameStateMode::MODERN => {
      let v = rand.nextInt(5);
      if v == 0 {
        rank *= 1.2;
        nr = 0.0;
        br = (rank * 0.7) * (1.0 + rand.nextSignedFloat(0.2));
        ir = (rank * 0.2) * (1.0 + rand.nextSignedFloat(0.2));
      } else if v == 1 || v == 2 {
        nr = (rank * 0.7) * (1.0 + rand.nextSignedFloat(0.2));
        br = 0.0;
        ir = (rank * 0.2) * (1.0 + rand.nextSignedFloat(0.2));
      } else if v == 3 || v == 4  {
        rank *= 0.75;
        nr = (rank * 0.3) * (1.0 + rand.nextSignedFloat(0.2));
        br = (rank * 0.3) * (1.0 + rand.nextSignedFloat(0.2));
        ir = (rank * 0.3) * (1.0 + rand.nextSignedFloat(0.2));
      }
      nwayDegRatio = 1.0;
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
    self.interval = ((intervalMax / ((ir + 1.0).sqrt())) as i32) + 1;
    let sr : f32 = (rank - self.burstNum + 1 - self.nway + 1) as f32 - ir;
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
    GameStateMode::CLASSIC => { 
      self.speed *= 0.36;
      if self.speed < 0.05 {
        self.speed = 0.05;
      } else {
        self.speed = (self.speed * 20.0).sqrt / 20.0;
      }
    },
    GameStateMode::BASIC => {
      self.speed *= 0.4;
    },
    GameStateMode::MODERN => {
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
        self.speedAccel *= -1.0;
      }
    }
    if (self.gameState.mode == GameStateMode::BASIC) && (self.nway > 1) && (rand.nextInt(3) == 0) {
      self.speed *= 0.9;
      self.nwaySpeedAccel = (self.speed * (0.2 + (self.nway as f32) * 0.05 + rand.nextFloat(0.1))) / ((self.nway - 1) as f32);
      if rand.nextInt(2) == 0 {
        self.nwaySpeedAccel *= -1.0;
      }
    }
    if self.nway > 1 {
      self.nwayAngle = (1.66 + rand.nextFloat(0.33)) / (1.0 + (self.nway as f32) * 0.7) * nwayDegRatio;
    }
    if rand.nextInt(3) == 0 {
      self.fireingAtATime = true;
    }
    self.minimumFireDist = 5.0;
  }

 //was move()
  fn move4(&mut self, ts : &TurretState, time : f32 /* = 1*/, anger : f32 /*= 0*/) -> bool {
    if self._disabled {
      return true;
    }
    let itv : f32 = (self.interval as f32) * ((1.0 - anger) * 0.99 + 0.01);
    if itv < 3.0 {
      itv = 3.0;
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
          let sp : f32 = spd - self.speedAccel * (self.burstNum - 1.0) / 2.0;
          for i in 0..self.burstNum {
            let d : f32 = ts.deg - self.nwayAngle * ((self.nway as f32) - 1.0) / 2.0 + self.nwayBaseDeg;
            let nsp : f32 = sp - self.nwaySpeedAccel * ts.nwaySpeedAccelDir * ((self.nway as f32) - 1.0) / 2.0;
            for j in 0..self.nway {
              if let Some(b) = self.bullets.getInstance() {
                b.set(self.bulletSpec, ts.pos, d, nsp * SPEED_RATIO);
                b.setWaitCnt(i * self.burstInterval);
                d += self.nwayAngle;
                nsp += self.nwaySpeedAccel * ts.nwaySpeedAccelDir;
              } else {
                break;
              }
            }
            sp += self.speedAccel;
          }
          ts.nwaySpeedAccelDir *= -1.0;
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
          ts.speed = spd - self.speedAccel * ((ts.burstNum as f32) - 1.0) / 2.0;
        }
      }
      if ts.burstNum > 0 {
        ts.burstCnt -= time;
        if ts.burstCnt <= 0 {
          ts.burstCnt = self.burstInterval;
          ts.burstNum -= 1;
          if self.isAbleToFire(ts.pos) {
            let d : f32 = ts.deg - self.nwayAngle * ((self.nway as f32) - 1.0) / 2.0 + self.nwayBaseDeg;
            let nsp : f32 = ts.speed - self.nwaySpeedAccel * ts.nwaySpeedAccelDir * ((self.nway as f32) - 1.0) / 2.0;
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
    if self.gameState.mode != GameStateMode::MODERN {
      p.y > 0.0
    } else {
      (p.y > 0.0) && (p.dist(self.player.pos) > self.minimumFireDist)
    }
  }

  fn disabled(&mut self, v : bool) -> bool {
    self._disabled = v;
    v
  }
}

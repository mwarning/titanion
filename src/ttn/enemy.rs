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

/**
 * Enemies and turrets.
 */

static trailEffect : bool = false;

struct EnemyPool {
  ap : ActorPool<Enemy>,
  _field : Field,
}

impl EnemyPool {
  fn getNearestEnemy(&self, p : Vector) -> Enemy {
    let dst : f32 = 99999.0;
    let ne : Enemy = null;
    for e in self.ap.actors {
      if e.exists && !e.isBeingCaptured {
        if self.self._field.calcCircularDist(e.pos, p) < dst {
          dst = self._field.calcCircularDist(e.pos, p);
          ne = e;
        }
      }
    }
    ne
  }

  fn getNearestMiddleEnemy(&self, p: Vector) -> Enemy {
    let dst : f32 = 99999.0;
    let ne : Enemy = null;
    for e in self.actors {
      if e.exists && !e.isBeingCaptured {
        if e.spec as MiddleEnemySpec {
          if self.self._field.calcCircularDist(e.pos, p) < dst {
            dst = self.self._field.calcCircularDist(e.pos, p);
            ne = e;
          }
        }
      }
    }
    ne
  }

  fn checkShotHit(&self, p : Vector, deg : f32, widthRatio : f32 /*= 1.0*/) -> bool {
    let e : Enemy = self.getNearestEnemy(p);
    if e {
      let ox : f32 = self.self._field.normalizeX(e.pos.x - p.x);
      let oy : f32 = e.pos.y - p.y;
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
        if self.self._field.checkHitDist(e.pos, p, pp, EnemySpec.BULLET_HIT_WIDTH) {
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
      if e.exists && e.isCaptured {
        let ox = self.self._field.normalizeX(e.pos.x - p.x);
        let oy = e.pos.y - p.y;
        if (ox.abs() < 0.5 * (e.state.size.x + size.x) &&
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
          if ((e.pos.x - x) * (e.pos.x - px)) < 0.0 {
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

  fn drawPillarBack() {
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
}

impl Enemy {
  fn init(&mut self, args : &[Object]) {
    self.tok.init(args);
    self.tok.state.enemy = this;
  }

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
      (spec as SmallEnemySpec).init(self.tok.state, firstEnemy.state);
      self.tok.state.isFirstEnemy = false;
    } else {
      self.tok.spec.init(state);
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
    self.tok.spec.init(state);
  }

  fn setGhostEnemyState(&mut self, x : f32, y : f32, deg : f32, cnt : i32) {
    self.tok.state.pos.x = x;
    self.tok.state.pos.y = y;
    self.tok.state.deg = deg;
    self.tok.state.cnt = cnt;
  }

  fn hitShot(&self, deg : f32 /*= 0*/) {
    if self.tok.spec.hitShot(state, deg) {
      remove();
    }
  }

  fn hitCaptured(&mut self) {
    let ses : SmallEnemySpec = spec as SmallEnemySpec;
    if ses {
      ses.hitCaptured(self.state);
    }
  }

  fn destroyed(&mut self) {
    self.tok.spec.destroyed(state);
    self.tok._exists = false;
  }

  fn isInAttack(&mut self) -> bool {
    if self.tok.spec.isBeingCaptured(state) {
      return false;
    }
    self.tok.spec.isInAttack(state)
  }

  fn isInScreen(&self) -> bool {
    if self.tok.spec.isBeingCaptured(state) {
      return false;
    }
    self.tok.spec.isInScreen(state);
  }

  fn isBeingCaptured(&self) -> bool {
    self.tok.spec.isBeingCaptured(self.tok.state)
  }

  fn isCaptured(&self) -> bool {
    let ges : GhostEnemySpec = spec as GhostEnemySpec;
    if ges {
      return true;
    }
    let ses : SmallEnemySpec = spec as SmallEnemySpec;
    if !ses {
      return false;
    }
    ses.isCaptured(state)
  }

  fn beforeAlign(&self) -> bool {
    if self.tok.spec.isBeingCaptured(state) {
      return false;
    }
    self.tok.spec.beforeAlign(state)
  }

  fn drawTrails(&self) {
    self.tok.spec.drawTrails(self.tok.state);
  }

  fn pos(&self) -> Vector {
    self.tok.state.pos
  }
}

const TRAIL_NUM : i32 = 64;
const TRAIL_INTERVAL : i32 = 8;
const TURRET_MAX_NUM2 : i32 = 3;

struct EnemyState {
  ts : TokenState,
  turretStates : [TurretState; TURRET_MAX_NUM2],
  enemy : Enemy,
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
    for &ts in inst.turretStates {
      ts = TurretState::new();
    }
    inst.vel = Vector::new();
    inst.centerPos = Vector::new();
    inst.centerVel = Vector::new();
    inst.standByPos = Vector::new();
    inst.size = Vector::new();
    inst.targetSize = Vector::new();
    inst.sizeVel = Vector::new();
    inst.trails = Trail[TRAIL_NUM];
    for &t in self.trails {
      t = Trail::new();
    }
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
    self.phaseCnt = nextPhaseCnt = 0;
    self.captureState = 0;
    self.captureIdx = 0;
    self.isGoingDownBeforeStandBy = false;
    self.size.x = size.y = 1;
    self.targetSize.x = targetSize.y = 1;
    self.sizeVel.x = sizeVel.y = 0;
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
    self.trails[trailIdx].set(pos.x, pos.y, deg, cnt);
    self.trailIdx += 1;
    if self.trailIdx >= TRAIL_NUM {
      self.trailIdx = 0;
      self.trailLooped = true;
    }
  }

  fn fndrawTrails(s : EnemyShape, r : f32, g : f32, b : f32, size : Vector, field : Field) {
    let mut ti : i32 = trailIdx;
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
      let t = trails[ti];
      Screen.setColor(r * a, g * a, b * a, a * 0.66);
      let p : Vector3 = field.calcCircularPos(t.pos);
      let cd : f32 = field.calcCircularDeg(t.pos.x);
      s.draw(p, cd, t.deg, t.cnt, size);
      a *= 0.7;
    }
  }
}

const BULLET_HIT_WIDTH : f32 = 0.8;
const NEXT_PHASE_DIST : f32 = 5.0;
const TURRET_MAX_NUM1 : i32 = 3;

struct EnemySpec {
  ts : TokenSpec<EnemyState>,
  //mixin StaticRandImpl;
  bullets : BulletPool,
  player : Player,
  particles : ParticlePool,
  bonusParticles : ParticlePool,
  enemies : EnemyPool,
  stage : Stage,
  trailShape : EnemyShape,
  bulletSpec : BulletSpec,
  counterBulletSpec : BulletSpec,
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

impl EnemySpec {
  fn new(field : Field, bullets : BulletPool, player : Player,
              particles : ParticlePool, bonusParticles : ParticlePool,
              enemies : EnemyPool, stage : Stage,
              shape : Shape, trailShape : EnemyShape,
              bulletSpec : BulletSpec, counterBulletSpec : BulletSpec,
              gameState : GameState) -> EnemySpec {
    EnemySpec {
      field : field,
      bullets : bullets,
      player : player,
      particles : particles,
      bonusParticles : bonusParticles,
      enemies : enemies,
      stage : stage,
      shape : shape,
      trailShape : trailShape,
      bulletSpec : bulletSpec,
      counterBulletSpec : counterBulletSpec,
      gameState : gameState,
    }
  }

  fn set(&mut self , es : EnemyState) {
    es.shield = shield;
    for i in 0..self.turretNum {
      self.turretSpecs[i].set(es.turretStates[i]);
    }
  }

  fn move2(&mut self, es : EnemyState) -> bool {
    //with (es) {
      es.move();
      if self.isInScreen(es) && self.isFirstEnemy {
        Sound.playSe("flying_down.wav");
        self.isFirstEnemy = false;
      }
      if captureState > 0 {
        moveCaptured(es);
        return true;
      }
      if player.enemiesHasCollision() {
        if enemies.checkEnemyHit(es.pos, size) {
          destroyed(es);
          return false;
        }
      }
      if player.checkEnemyHit(es.pos, es.vel, size) {
        destroyed(es);
        return false;
      }
      if capturable {
        checkCaptured(es);
      }
      let er: f32 = (1.0 - ellipseRatio) + (deg + ellipseDeg).sin().abs() * ellipseRatio * 2.0;
      let rk : f32 = rank;
      vel.x -= sin(deg) * speed * er * 0.1 * rk;
      vel.y += cos(deg) * speed * er * 0.1 * rk;
      vel *= 0.9;
      pos += vel;
      if isInScreen(es) {
        field.addSlowdownRatio(speed * 0.04 * rk);
      }
      pos.x = field.normalizeX(pos.x);
      recordTrail();
      if (phase >= -50) && (phase < 0) && !field.containsIncludingPit(pos) {
        return false;
      }
      if waitCnt > 0 {
        waitCnt -= 1;
      } else {
        let cp : Vector = centerPos;
        centerPos.x = field.normalizeX(centerPos.x);
        phaseCnt += 1;
        if field.calcCircularDist(centerPos, pos) < NEXT_PHASE_DIST {
          nextPhaseCnt -= 1;
          if nextPhaseCnt <= 0 {
            phase += 1;
            if !gotoNextPhase(es) {
              return false;
            }
          }
        }
        cp.x = field.normalizeX(cp.x);
        let dst : f32 = field.calcCircularDist(cp, pos);
        speed += ((baseSpeed * (1 + dst * 0.1)) - speed) * 0.05;
        let mut av : f32 = angVel * rk;
        let mut td : f32 = (field.normalizeX(-(cp.x - pos.x)), cp.y - pos.y).atan2();
        let mut ad : f32 = Math.normalizeDeg(td - deg);
        av *= 2.5 - er;
        if (ad > av) || (ad < (-PI * 0.8)) {
          deg += av;
        }
        else if ad < -av {
          deg -= av;
        } else {
          deg = td;
        }
        //assert(deg <>= 0);
        for i in 0..turretNum {
          let ts : TurretState = turretStates[i];
          let tx : f32 = pos.x;
          let ty : f32 = pos.y;
          match i {
          0 =>  {},
          1 => { tx -= turretWidth; },
          2 => { tx += turretWidth; },
          }
          let turretDeg : f32 = (field.normalizeX(-(player.pos.x - tx)), player.pos.y - ty).atan2();
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
              if (turretDeg >= 0) && (turretDeg < (PI - PI / 4.0) {
               turretDeg = PI - PI / 4.0;
              } else if (turretDeg < 0) && (turretDeg > (-PI + PI / 4.0)) {
               turretDeg = -PI + PI / 4.0;
              }
            },
            GameState::Mode::MODERN => {}
          }
          ts.update(tx, ty, turretDeg);
        }
        self.movePhase(es);
        sizeVel.x += (targetSize.x - size.x) * 0.2;
        sizeVel.y += (targetSize.y - size.y) * 0.2;
        size += sizeVel;
        sizeVel *= 0.95;
      }
      true
    //}
  }

  fn moveCaptured(&mut self, es : EnemyState) {
    //with (es) {
      match captureState {
      1 => {
        vel.x += (player.pos.x - pos.x) * 0.03;
        vel.y += (player.pos.y - pos.y) * 0.03;
        pos.x += (player.pos.x - pos.x) * 0.03;
        pos.y += (player.pos.y - pos.y) * 0.03;
        deg *= 0.95;
        if player.pos.dist(pos) < 1 {
          captureState = 2;
        }
      },
      2 => {
        let cx : f32 = calcCapturePosX(captureIdx);
        vel.x += (player.pos.x + cx - pos.x) * 0.03;
        pos.x += (player.pos.x + cx - pos.x) * 0.1;
        pos.y += (player.pos.y - pos.y) * 0.33;
        vel.y *= 0.6;
        deg *= 0.95;
        if (player.pos.x + cx - pos.x).abs() < 0.2 {
          captureState = 3;
        }
      },
      3 => {
        let cx : f32 = calcCapturePosX(captureIdx);
        pos.x = player.pos.x + cx;
        pos.y = player.pos.y;
        deg = player.deg;
        }
      }
      vel *= 0.9;
      pos += vel;
    //}
  }

  fn calcCapturePosX(idx : i32) -> f32 {
    if (idx % 2) == 0 {
      return ((idx / 2) + 0.5) * PlayerSpec::CAPTURED_ENEMIES_INTERVAL_LENGTH * player.capturedEnemyWidth;
    } else {
      return -((idx / 2) + 0.5) * PlayerSpec::CAPTURED_ENEMIES_INTERVAL_LENGTH * player.capturedEnemyWidth;
    }
  }

  fn checkCaptured(&self, es : EnemyState) {
    //with (es) {
      if player.isInTractorBeam(pos) {
        if gameState.mode != GameState::Mode::MODERN {
          let idx : i32 = player.addCapturedEnemy(es.enemy);
          if idx >= 0 {
            captureState = 1;
            captureIdx = idx;
          }
        } else {
          provacated(es);
        }
      }
    //}
  }

  fn hitCaptured(&mut self, es : EnemyState) {
    player.destroyCapturedEnemies(es.captureIdx);
  }

  fn isBeingCaptured(&self, es : EnemyState) -> bool {
    (es.captureState > 0)
  }

  fn isCaptured(es : EnemyState) -> bool {
    (es.captureState == 3)
  }

  fn beforeAlign(es : EnemyState) -> bool {
    (es.phase < -10)
  }

  fn hitShot(&mut self, es : EnemyState, dd : f32 /* = 0*/) -> bool {
    //with (es) {
      shield -= 1;
      let r : f32 = 0.5 + rand.nextFloat(0.5);
      let g : f32 = 0.1 + rand.nextFloat(0.5);
      let b : f32 = 0.5 + rand.nextFloat(0.5);
      for i in 0..10 {
        let p : Particle = particles.getInstanceForced();
        let d : f32 = dd + rand.nextSignedFloat(PI / 4);
        p.set(Particle.Shape.LINE, pos.x, pos.y, d, 0.1 + rand.nextFloat(0.5), 1,
              r, g, b, 30 + rand.nextInt(30));
        p = particles.getInstanceForced();
        d = dd + PI + rand.nextSignedFloat(PI / 4);
        p.set(Particle.Shape.LINE, pos.x, pos.y, d, 0.1 + rand.nextFloat(0.5), 1,
              r, g, b, 30 + rand.nextInt(30));
      }
      if shield <= 0 {
        destroyed(es, dd);
        return true;
      }
      match gameState.mode {
       GameState::Mode::CLASSIC => {
        targetSize.x *= 1.3;
        targetSize.y *= 1.3;
        },
      GameState::Mode::BASIC => {
        targetSize.x *= 1.2;
        targetSize.y *= 1.2;
        },
      GameState::Mode::MODERN => {
        targetSize.x *= 1.01;
        targetSize.y *= 1.01;
        },
      }
      sizeVel.x = 0.3;
      sizeVel.y = 0.3;
      return false;
    //}
  }

  fn destroyed(&mut self, es : EnemyState, dd : f32 /*= 0*/) {
    //with (es) {
      let r : f32 = 0.5 + rand.nextFloat(0.5);
      let g : f32 = 0.1 + rand.nextFloat(0.5);
      let b : f32 = 0.5 + rand.nextFloat(0.5);
      let sz : f32 = (targetSize.x + targetSize.y) / 2;
      sz = (sz - 1.0) * 2.0 + 1.0;
      let mut n : i32 = 3 + rand.nextInt(2);
      n *= sz;
      for i  in 0..n {
        let p : Particle = particles.getInstanceForced();
        let d : f32 = dd + rand.nextSignedFloat(PI / 5.0);
        p.set(Particle.Shape.TRIANGLE, pos.x, pos.y, d, 0.5,
              (2.0 + rand.nextFloat(0.5)) * sz, r, g, b, 50 + rand.nextInt(100));
      }
      for i in 0..n {{
        let p : Particle = particles.getInstanceForced();
        let d : f32 = rand.nextFloat(PI * 2.0);
        p.set(Particle.Shape.QUAD, pos.x, pos.y, d, 0.1 + rand.nextFloat(0.1),
              (1 + rand.nextFloat(0.5)) * sz, r, g, b, 50 + rand.nextInt(100));
      }
      if !self.isBeingCaptured(es) {
        if removeBullets {
          let mut cnt : i32 = 1;
          bullets.removeAround(cnt, pos, particles, bonusParticles, player);
          let p : Particle = bonusParticles.getInstanceForced();
          let mut wc : i32;
          if cnt <= 50 {
            wc = cnt;
          } else {
            wc = 50 + (((cnt - 50) as f32).sqrt() as i32);
          }
          p.set(Particle.Shape.BONUS, pos.x, pos.y, 0, 0.1,
                1.0 + (wc as f32) / 75.0, 1, 1, 1, 120, false, cnt, wc);
          player.addScore(score * cnt);
        } else {
          if gameState.mode == GameState::Mode::BASIC {
            let oy : f32 = pos.y - player.pos.y;
            let mut pm : i32 = (18.0 - oy) as i32;
            if pm > 16 {
              pm = 16;
            } else if pm < 1 {
              pm = 1;
            }
            player.addScore(score * pm);
            let mut p : Particle = bonusParticles.getInstanceForced();
            p.set(Particle.Shape.BONUS, pos.x, pos.y, 0, 0.1,
                  0.5, 1, 1, 1, 60, false, pm);
            gameState.setProximityMultiplier(pm);
          } else {
            player.addScore(score);
          }
        }
        player.addMultiplier(0.1);
        if stage.existsCounterBullet {
          let blt : Bullet = bullets.getInstance();
          if blt {
            blt.set(counterBulletSpec, pos,
                    turretStates[0].deg, turretSpecs[0].speed * TurretSpec.SPEED_RATIO);
          }
        }
      }
      Sound.playSe(explosionSeName);
    }
  }

  fn provacated(&mut self, es : EnemyState) {
    //with (es) {
      anger += (1 - anger) * 0.05;
      if sizeVel.dist < 0.1 {
        sizeVel.x = 0.2;
        sizeVel.y = 0.2;
      }
      let mut p : Particle = particles.getInstanceForced();
      p.set(Particle.Shape.LINE, pos.x, pos.y, PI / 2.0 + rand.nextSignedFloat(PI / 4.0),
            0.1 + rand.nextFloat(0.2), 1,
            1, 0.5, 0.5, 30 + rand.nextInt(30));
      p = particles.getInstanceForced();
      p.set(Particle.Shape.LINE, pos.x, pos.y, -PI / 2.0 + rand.nextSignedFloat(PI / 4.0),
            0.1 + rand.nextFloat(0.2), 1,
            1, 0.5, 0.5, 30 + rand.nextInt(30));
      if removeBullets {
        player.midEnemyProvacated();
      }
    //}
  }

  fn gotoNextPhaseInAppearing(&mut self, es : EnemyState) -> bool {
    //with (es) {
      match phase {
      -300 => {
        let mut cpw : f32;
        match gameState.mode {
          GameState::Mode::CLASSIC => { cpw = 0.2; },
          GameState::Mode::BASIC => { cpw = 0.2; },
          GameState::Mode::MODERN => { cpw = 0.4; },
        }
        centerPos.x = rand.nextSignedFloat(field.size.x * cpw);
        centerPos.y = field.size.y * 2.0;
        standByPos.x = rand.nextSignedFloat(field.size.x * cpw);
        standByPos.y = field.size.y * (0.7 + rand.nextFloat(0.1));
        nextPhaseCnt = 15;
        baseSpeed = baseBaseSpeed * 1.5;
        angVel = baseAngVel * 1.5;
        phase = -50;
        },

      -200 => {
        centerPos.x = rand.nextSignedFloat(field.size.x * 0.1);
        centerPos.y = field.size.y * 1.6;
        if centerPos.x < 0 {
          standByPos.x = field.size.x * (rand.nextSignedFloat(0.4) + 0.4);
        } else {
          standByPos.x = field.size.x * (rand.nextSignedFloat(0.4) - 0.4);
        }
        standByPos.y = field.size.y * (0.5 + rand.nextFloat(0.3));
        nextPhaseCnt = 60;
        baseSpeed = baseBaseSpeed * 1.0;
        angVel = baseAngVel * 1.0;
        },
      -199 => {
        if standByPos.x < 0 {
          centerPos.x = field.size.x * 0.75;
        } else {
          centerPos.x = -field.size.x * 0.75;
        }
        centerPos.y = 0;
        if isGoingDownBeforeStandBy {
          nextPhaseCnt = 20;
        } else {
          nextPhaseCnt = 60;
        }
        baseSpeed = baseBaseSpeed * 2;
        angVel = baseAngVel * 2;
        phase = -50;
       },
 
      -100 => {
        centerPos.x = field.size.x * 4.0;
        if rand.nextInt(2) == 0 {
          centerPos.x *= -1;
        }
        centerPos.y = field.size.y * 1.6;
        if centerPos.x < 0 {
          standByPos.x = field.size.x * (rand.nextSignedFloat(0.4) + 0.4);
        }
        else {
          standByPos.x = field.size.x * (rand.nextSignedFloat(0.4) - 0.4);
        }
        standByPos.y = field.size.y * (0.5 + rand.nextFloat(0.3));
        nextPhaseCnt = 20;
        baseSpeed = baseBaseSpeed * 2.0;
        angVel = baseAngVel * 2.0;
       },
      -99 => {
        if centerPos.x > 0 {
          centerPos.x = field.size.x * 2.0;
        } else {
          centerPos.x = -field.size.x * 2.0;
        }
        centerPos.y = -field.size.y * 1.2;
        nextPhaseCnt = 20;
        baseSpeed = baseBaseSpeed * 2;
        angVel = baseAngVel * 2;
      },
      -98 => {
        if centerPos.x > 0 {
          centerPos.x = field.size.x * 0.5;
        } else {
          centerPos.x = -field.size.x * 0.5;
        }
        centerPos.y = 0;
        nextPhaseCnt = 30;
        phase = -50;
      },
      -49 => {
        if isGoingDownBeforeStandBy {
          centerPos.x = centerPos.x / 2;
          centerPos.y = 0;
          phase = -30;
          nextPhaseCnt = 10;
          break;
        }
        centerPos.x = standByPos.x;
        centerPos.y = standByPos.y;
        nextPhaseCnt = calcStandByTime(es);
        baseSpeed = baseBaseSpeed;
        angVel = baseAngVel;
        phase = -10;
      },
      -29 => {
        centerPos.x = (centerPos.x + player.pos.x * 2) / 3;
        centerPos.y = -field.size.y * 1.2;
        baseSpeed = baseBaseSpeed * 1.2;
        angVel = baseAngVel * 1.2;
        nextPhaseCnt = 5;
       },
      -28 => {
        centerPos.y = -field.size.y * 1.5;
        nextPhaseCnt = 10;
       },
      -9 => {
        phase = 0;
      },
      _ => {
        return false;
      }
      }
      nextPhaseCnt /= rank;
      phaseCnt = 0;
    //}
    true;
  }

  fn movePhase(&mut self, es : EnemyState) {
    //with (es) {
      match phase {
      -200|-100 => {
        if pos.y < (field.size.y * 1.5) {
          pos.y = field.size.y * 1.5;
        }
      },
      -99 => {
        if (centerPos.x < 0) && (pos.x > -field.size.x) {
          pos.x += (-field.size.x - pos.x) * 0.2;
        } else if (centerPos.x > 0) && (pos.x < field.size.x) {
          pos.x += (field.size.x - pos.x) * 0.2;
        }
      },
      -50|-49|-10=> {
        if pos.y < (-field.size.y * 0.5) {
          pos.y += (-field.size.y * 0.5 - pos.y) * 0.2;
        }
      },
      _ => {},
      };
      if isInAttack(es) {
        if (gameState.mode == GameState::Mode::MODERN) || (phase >= 0) || (rand.nextInt(5) == 0) {
          for i in 0..turretNum {
            turretSpecs[i].move(turretStates[i], rank, anger);
          }
        }
      }
    //}
  }

  fn isInScreen(&self, es : EnemyState) -> bool {
    self.field.size.contains(es.pos);
  }
/*
  public abstract void setRank(float rank);
  public abstract void init(EnemyState es);
  public abstract bool gotoNextPhase(EnemyState es);
  public abstract bool isInAttack(EnemyState es);
  protected abstract int calcStandByTime(EnemyState es);
*/
  fn draw(&self, es : EnemyState) {
    let mut p : Vector3 = self.field.calcCircularPos(es.pos);
    let mut cd : f32 = self.field.calcCircularDeg(es.pos.x);
    (shape as EnemyShape).draw(p, cd, es.deg, es.cnt, es.size);
    for i in 1..turretNum {
      let x : f32 = es.pos.x;
      match i {
      1 => {
        x -= turretWidth;
      },
      2 => {
        x += turretWidth;
      },
      }
      p = field.calcCircularPos(x, es.pos.y);
      cd = field.calcCircularDeg(x);
      Screen.setColor(0.5, 0.5, 1);
      (trailShape as EnemyShape).draw(p, cd, es.deg, es.cnt, es.size.x * 0.5, es.size.y * 0.5);
    }
  }

  fn drawTrails(&self, es : EnemyState) {
    if es.captureState > 0 {
      return;
    }
    es.drawTrails(trailShape, 0.2, 0.2, 0.8, es.size, field);
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
  fn this(&mut self, field : Field, shape : Shape) {
    self.field = field;
    self.shape = shape;
  }

  fn draw(&self, es : EnemyState) {
    //with (es) {
      let p : Vector3 = field.calcCircularPos(pos);
      let cd : f32 = field.calcCircularDeg(pos.x);
      Screen.setColor(0.5, 0.5, 1, 0.8);
      (shape as EnemyShape).draw(p, cd, deg, cnt, size);
    //}
  }

  fn set(&mut self, es : EnemyState) {}
  fn move2(&mut self, es : EnemyState) -> bool { true }
  fn destroyed(&mut self, es : EnemyState, dd : f32 /*= 0*/) {}
  fn setRank(&mut self, rank : f32) {}
  fn init(&mut self, es : EnemyState) {}
  fn gotoNextPhase(&mut self, es : EnemyState) -> bool { false }
  fn isInAttack(&mut self, ses : EnemyState) -> bool { false }
  fn calcStandByTime(&mut self, es : EnemyState) -> i32 { 0 }
  fn isBeingCaptured(&mut self, es : EnemyState) -> bool { true }
  fn isCaptured(&mut self, es : EnemyState) -> bool { true }
}


struct MiddleEnemySpec {
 es : EnemySpec,
}

impl MiddleEnemySpec {
  fn new(field : Field, bullets : BulletPool, player : Player,
              particles :  ParticlePool, bonusParticles : ParticlePool,
              enemies : EnemyPool, stage : Stage,
              shape : Shape, trailShape : EnemyShape,
              bulletSpec : BulletSpec, counterBulletSpec : BulletSpec,
              gameState : GameState) -> MiddleEnemySpec {
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
    explosionSeName = "explosion3.wav";
    inst
  }

  fn init(&mut self, es : EnemyState) {
    //with (es) {
      self.es.size.x = 1.33;
      self.es.size.y = 1.33;
      self.es.phase = -300;
      self.es.gotoNextPhaseInAppearing(es);
    //}
  }

  fn setRank(&mut self, r : f32) {
    rank = r.sqrt();
    let mut tr : f32;
    match gameState.mode {
    GameState::Mode::CLASSIC => {
      rank = sqrt(rank);
      tr = r * 2.0;
      },
    GameState::Mode::BASIC => {
      tr = r * 3.0;
      },
    GameState::Mode::MODERN => {
      rank = 1.0;
      tr = r * 15.0;
      },
    };
    if rank < 1.5 {
      rank = 1.5;
    }
    turretSpecs[0].setRankMiddle(tr);
    turretNum = 1;
    if gameState.mode == GameState::Mode::MODERN {
      let ts : TurretSpec = turretSpecs[0];
      let ptn : i32 = rand.nextInt(6);
      if ptn == 1 || ptn == 2 || ptn == 4 {
        turretSpecs[1].copy(turretSpecs[0]);
        turretSpecs[2].copy(turretSpecs[0]);
        if (ts.nway > 1) && (rand.nextInt(2) == 0) {
          let nsa : f32 = (ts.speed * (0.2 + ts.nway * 0.05 + rand.nextFloat(0.1))) / (ts.nway - 1);
          if rand.nextInt(2) == 0 {
            nsa *= -1.0;
          }
          turretSpecs[1].nwaySpeedAccel = nsa;
          turretSpecs[2].nwaySpeedAccel = -nsa;
        }
        turretWidth = 1.0 + rand.nextFloat(1.0);
        turretNum = 3;
        if ptn == 4 {
          turretSpecs[0].setRankMiddle(tr * 2);
          turretSpecs[1].interval *= 4;
          turretSpecs[2].interval *= 4;
          turretSpecs[0].interval = turretSpecs[1].interval;
          turretSpecs[2].fireIntervalRatio = 0.25;
          turretSpecs[0].fireIntervalRatio = 0.5;
        } else {
          turretSpecs[0].disabled = true;
          turretSpecs[1].interval *= 2;
          turretSpecs[2].interval *= 2;
          if rand.nextInt(2) == 0 {
            turretSpecs[2].fireIntervalRatio = 0.5;
          }
        }
      } else if ptn == 3 || ptn == 5 {
        turretSpecs[0].interval *= 2;
        if rand.nextInt(3) == 0 {
          turretSpecs[0].nwayAngle *= 0.1;
        }
        turretSpecs[1].setRankMiddle(tr);
        turretSpecs[1].interval *= 2;
        turretSpecs[2].copy(turretSpecs[1]);
        if (ts.nway > 1) && r(and.nextInt(2) == 0) {
          let nsa : f32 = (ts.speed * (0.2 + ts.nway * 0.05 + rand.nextFloat(0.1))) / (ts.nway - 1);
          if rand.nextInt(2) == 0 {
            nsa *= -1;
          }
          turretSpecs[1].nwaySpeedAccel = nsa;
          turretSpecs[2].nwaySpeedAccel = -nsa;
        }
        turretSpecs[1].nwayBaseDeg = -PI / 8.0 - rand.nextFloat(PI / 12.0);
        if turretSpecs[1].nway > 1 {
          turretSpecs[1].nwayBaseDeg -= turretSpecs[1].nwayAngle * (turretSpecs[1].nway - 1) / 2;
        }
        turretSpecs[2].nwayBaseDeg = -turretSpecs[1].nwayBaseDeg;
        turretWidth = 1.5 + rand.nextFloat(0.5);
        turretNum = 3;
      }
    }
  }

  fn gotoNextPhase(&mut self, es : EnemyState) -> bool {
    //with (es) {
      if phase < 0 {
        return gotoNextPhaseInAppearing(es);
      }
      match phase {
      1 => {
        if (gameState.mode != GameState::Mode::MODERN) && !player.hasCollision {
          phase = 0;
          nextPhaseCnt = calcStandByTime(es);
          break;
        }
        Sound.playSe("flying_down.wav");
        if gameState.mode != GameState::Mode::MODERN {
          centerPos.x = field.size.x * (0.6 + rand.nextSignedFloat(0.1));
          if rand.nextInt(2) == 0 {
            centerPos.x *= -1;
          }
          centerPos.y = field.size.y * (0.2 + rand.nextFloat(0.2));
          nextPhaseCnt = 60;
        } else {
          centerPos.x = standByPos.x;
          centerPos.y = field.size.y * 0.95;
          baseSpeed = baseBaseSpeed * 0.3;
          nextPhaseCnt = 60;
        }
      },
      2 => {
        if gameState.mode != GameState::Mode::MODERN {
          centerPos.x *= -0.9;
          centerPos.y = field.size.y * (0.2 + rand.nextFloat(0.2));
          nextPhaseCnt = 60;
        } else {
          centerPos.x = standByPos.x;
          centerPos.y = 0;
          baseSpeed = baseBaseSpeed * 0.1;
          nextPhaseCnt = 10;
        }
      },
      3 => {
        if gameState.mode != GameState::Mode::MODERN {
          centerPos.x = standByPos.x;
          centerPos.y = standByPos.y;
          phase = 0;
          nextPhaseCnt = calcStandByTime(es);
        } else {
          centerPos.x = standByPos.x;
          centerPos.y = -field.size.y * 1.5;
          baseSpeed = baseBaseSpeed * 0.5;
          nextPhaseCnt = 10;
        }
      },
      _ => {
        return false;
      },
      };
      nextPhaseCnt /= rank;
      phaseCnt = 0;
    //}
    true
  }

  fn isInAttack(&mut self, es : EnemyState) -> bool {
    (es.phase == 1) || (es.phase == 2)
  }

  fn calcStandByTime(&mut self, es : EnemyState) -> i32 {
    if (es.phase < 0) || (gameState.mode == GameState::Mode::MODERN) {
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
  fn new(field : Field, bullets : BulletPool, player : Player,
              particles : ParticlePool, bonusParticles : ParticlePool,
              enemies :  EnemyPool, stage : Stage,
              shape : Shape, trailShape : EnemyShape,
              bulletSpec :  BulletSpec, counterBulletSpec : BulletSpec,
              gameState : GameState) -> SmallEnemySpec {
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

  fn init(&mut self, es : EnemyState) {
    gotoNextPhaseInAppearing(es);
  }

  fn init(&mut self, es : EnemyState, fes : EnemyState) {
    //with (es) {
      centerPos.x = fes.centerPos.x;
      centerPos.y = fes.centerPos.y;
      standByPos.x = fes.standByPos.x;
      standByPos.y = fes.standByPos.y;
      nextPhaseCnt = fes.nextPhaseCnt;
      baseSpeed = fes.baseSpeed;
      angVel = fes.angVel;
      phase = fes.phase;
      size.x = size.y = 1.25;
    //}
  }

fn setRank(&mut self, r : f32) {
    rank =(r * 0.5).sqrt();
    let mut tr : f32;
    match gameState.mode {
    GameState::Mode::CLASSIC => {
      rank = sqrt(rank);
      tr = r;
    },
    GameState::Mode::BASIC => {
      tr = r * 2;
    },
    GameState::Mode::MODERN => {
      rank = 1;
      tr = r;
    },
    };
    if rank < 1 {
      rank = 1;
    }
    turretSpecs[0].setRankNormal(tr);
    turretNum = 1;
  }

  fn calcStandByTime(es : EnemyState) -> i32 {
    60 + rand.nextInt(120)
  }
}

struct SE1Spec {
  ses : SmallEnemySpec,
}

impl SE1Spec {
  fn this(&mut self, field : Field, bullets : BulletPool, player : Player,
              particles : ParticlePool, bonusParticles : ParticlePool,
              enemies : EnemyPool, stage : Stage,
              shape : Shape, trailShape : EnemyShape,
              bulletSpec : BulletSpec, counterBulletSpec : BulletSpec,
              gameState : GameState) {
    super(field, bullets, player, particles, bonusParticles, enemies, stage,
          shape, trailShape, bulletSpec, counterBulletSpec, gameState);
    explosionSeName = "explosion1.wav";
  }

  fn gotoNextPhase(es : EnemyState) -> bool {
    //with (es) {
      if phase < 0 {
        return gotoNextPhaseInAppearing(es);
      }
      match phase {
      1 => {
        if !player.hasCollision || (enemies.numInAttack > stage.attackSmallEnemyNum) {
          phase = 0;
          nextPhaseCnt = calcStandByTime(es);
          break;
        }
        Sound.playSe("flying_down.wav");
        centerPos.y = 0;
        centerPos.x = (standByPos.x + player.pos.x) / 2;
        nextPhaseCnt = 60;
      },
      2 => {
        centerPos.y = -field.size.y * 0.7;
        centerPos.x = player.pos.x;
        nextPhaseCnt = 30;
      },
      3 => {
        centerPos.x = standByPos.x;
        centerPos.y = standByPos.y;
        phase = 0;
        nextPhaseCnt = calcStandByTime(es);
        },
      }
      nextPhaseCnt /= rank;
      phaseCnt = 0;
    //}
    true
  }

  fn isInAttack(es : EnemyState) -> bool {
    (es.phase < -10 || es.phase == 1 || es.phase == 2)
  }
}

struct SE2Spec {
 ses : SmallEnemySpec,
}

impl SE2Spec {
  fn this(&mut self, field : Field, bullets : BulletPool, player : Player,
              particles : particlePool, bonusParticles : ParticlePool,
              enemies : EnemyPool, stage : Stage,
              shape : Shape, trailShape : EnemyShape,
              bulletSpec : BulletSpec, counterBulletSpec : BulletSpec,
              gameState : GameState) {
    super(field, bullets, player, particles, bonusParticles, enemies, stage,
          shape, trailShape, bulletSpec, counterBulletSpec, gameState);
    explosionSeName = "explosion2.wav";
  }

  fn gotoNextPhase(&mut self, es : EnemyState) -> bool {
    //with (es) {
      if phase < 0 {
        return gotoNextPhaseInAppearing(es);
      }
      match phase {
      1 => {
        if !player.hasCollision || (enemies.numInAttack > stage.attackSmallEnemyNum) {
          phase = 0;
          nextPhaseCnt = calcStandByTime(es);
        } else {
          Sound.playSe("flying_down.wav");
          centerPos.y = -field.size.y * 0.3;
          centerPos.x = (standByPos.x + player.pos.x) / 2;
          baseSpeed = baseBaseSpeed;
          angVel = baseAngVel;
          nextPhaseCnt = 30 + rand.nextInt(60);
        }
      },
      2 => {
        centerPos.y = -field.size.y * 1.3;
        centerPos.x *= -1;
        nextPhaseCnt = 30;
      },
      3 => {
        centerPos.y = -field.size.y * 1.0;
        if centerPos.x < 0 {
          centerPos.x = -field.size.x * 1.5;
        } else {
          centerPos.x = field.size.x * 1.5;
        }
        baseSpeed = baseBaseSpeed * 1.5;
        angVel = baseAngVel * 1.5;
        nextPhaseCnt = 30;
      },
      4 => {
        centerPos.x = standByPos.x;
        centerPos.y = standByPos.y;
        phase = 0;
        nextPhaseCnt = calcStandByTime(es);
      },
      }
      nextPhaseCnt /= rank;
      phaseCnt = 0;
    //}
    true
  }

  fn movePhase(&mut self, es : EnemyState) {
    self.super.movePhase(es);
    //with (es) {
      if phase == 3 {
        if centerPos.x < 0 {
          if pos.x > (-field.size.x * 1.2) {
            pos.x += (centerPos.x - pos.x) * 0.2;
          }
        } else {
          if pos.x < (field.size.x * 1.2) {
            pos.x += (centerPos.x - pos.x) * 0.2;
          }
        }
      }
    //}
  }
  
  fn isInAttack(&mut self, es : EnemyState) -> bool {
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
    fireCnt = 0;
    burstCnt = 0;
    burstNum = 0;
    nwaySpeedAccelDir = 1;
    super.clear();
  }

  fn update(x : f32, y : f32, d : f32) {
    pos.x = x;
    pos.y = y;
    if burstNum <= 0 {
      deg = d;
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
  fn this(&mut self, field : Field, bullets : BulletPool, player : Player,
              enemies : EnemyPool, particles : ParticlePool,
              stage : Stage, bulletSpec : BulletSpec, gameState : GameState) {
    self.bulletSpec = bulletSpec;
    self.field = field;
    self.bullets = bullets;
    self.player = player;
    self.stage = stage;
    self.gameState = gameState;
    self.initParam();
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

  fn copy(&mut self, ts : TurretSpec) {
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

  fn set(&mut self, ts : TurretState) {
    self.setFireIntervalRatio(ts, fireIntervalRatio);
  }

  fn setFireIntervalRatio(&mut self, ts : TurretState, fir : f32) {
    ts.fireCnt = fir * interval;
  }

  fn setRankNormal(&mut self, rank : f32, isWide : bool /*= false*/) {
    self.initParam();
    let rr : f32 = rand.nextFloat(0.5);
    let nsr : f32 = 0.5 + rand.nextSignedFloat(0.3);
    let mut nr : f32;
    let mut br : f32;
    let mut ir : f32;
    let intervalMax : f32 = INTERVAL_MAX;
    match gameState.mode {
    GameState::Mode::CLASSIC => {
      nr = 0;
      br = 0;
      ir = (rank * nsr).sqrt() * 2.0;
      burstInterval = 3 + rand.nextInt(2);
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
      burstInterval = 3 + rand.nextInt(2);
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
      burstInterval = 4 + rand.nextInt(4);
      },
    }
    burstNum = 1 + (br.sqrt() as i32);
    nway = 1 + (nr.sqrt() as i32);
    interval = ((intervalMax / (ir + 1)) as i32) + 1;
    let sr : f32 = rank - nway + 1 - burstNum + 1 - ir;
    if sr < 0.01 {
      sr = 0.01;
    }
    speed = (sr * 0.66).sqrt();
    //assert(speed > 0);
    speed *= 0.2;
    if speed < 0.1 {
      speed = 0.1;
    } else {
      speed = (speed * 10).sqrt() / 10;
    }
    //assert(speed > 0);
    match gameState.mode {
    GameState::Mode::CLASSIC => {
      speed *= 0.36;
      if speed < 0.05 {
        speed = 0.05;
      } else {
        speed = (speed * 20).sqrt() / 20;
      }
    },
    GameState::Mode::BASIC => {
      speed *= 0.33;
    },
    GameState::Mode::MODERN => {
      speed *= 0.25;
      if speed < 0.04 {
        speed = 0.04;
      }
      if speed > 0.04 {
        speed = (speed * 25).sqrt() / 25;
      }
      },
    }
    nwayAngle = (1.66 + rand.nextFloat(0.33)) / (1 + nway * 0.7) * 0.06;
    fireingAtATime = true;
    minimumFireDist = 10;
  }

  fn setRankMiddle(&mut self, rank : f32) {
    self.initParam();
    let mut nr : f32;
    let mut br : f32;
    let mut ir : f32;
    let mut nwayDegRatio : f32;
    let intervalMax : f32 = INTERVAL_MAX;
    match gameState.mode {
      GameState::Mode::CLASSIC => {
      nr = 0;
      br = 0;
      ir = (rank * (0.5 + rand.nextSignedFloat(0.3))).sqrt() * 2;
      nwayDegRatio = 0;
      burstInterval = 3 + rand.nextInt(2);
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
      burstInterval = 3 + rand.nextInt(2);
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
      burstInterval = 4 + rand.nextInt(8);
      },
    };
    let acf : bool = false;
    burstNum = (br.sqrt() as i32) + 1;
    if (burstNum > 1) && (rand.nextInt(3) > 0) {
      acf = true;
      nr *= 0.9;
      ir *= 0.9;
      rank *= 0.9;
    }
    nway = (nr.sqrt() as i32) + 1;
    interval = ((intervalMax / ((ir + 1).sqrt())) as i32) + 1;
    let sr : f32 = rank - burstNum + 1 - nway + 1 - ir;
    if sr < 0.01 {
      sr = 0.01;
    }
    speed = (sr * 0.66).sqrt();
    //assert(speed > 0);
    speed *= 0.2;
    if speed < 0.1 {
      speed = 0.1;
    } else {
      speed = (speed * 10.0).sqrt() / 10;
    }
    //assert(speed > 0);
    match gameState.mode {
    GameState::Mode::CLASSIC => { 
      speed *= 0.36;
      if speed < 0.05 {
        speed = 0.05;
      } else {
        speed = (speed * 20.0).sqrt / 20.0;
      }
    },
    GameState::Mode::BASIC => {
      speed *= 0.4;
    },
    GameState::Mode::MODERN => {
      speed *= 0.22;
      if speed < 0.04 {
        speed = 0.04;
      }
      if speed > 0.04 {
        speed = (speed * 25.0).sqrt() / 25.0;
      }
      },
    }
    if acf {
      speedAccel = (speed * (0.2 + burstNum * 0.05 + rand.nextFloat(0.1))) / (burstNum - 1);
      if rand.nextInt(2) == 0 {
        speedAccel *= -1;
      }
    }
    if (gameState.mode == GameState::Mode::BASIC) && (nway > 1) && (rand.nextInt(3) == 0) {
      speed *= 0.9;
      nwaySpeedAccel = (speed * (0.2 + nway * 0.05 + rand.nextFloat(0.1))) / (nway - 1);
      if rand.nextInt(2) == 0 {
        nwaySpeedAccel *= -1;
      }
    }
    if nway > 1 {
      nwayAngle = (1.66 + rand.nextFloat(0.33)) / (1 + nway * 0.7) * nwayDegRatio;
    }
    if rand.nextInt(3) == 0 {
      fireingAtATime = true;
    }
    minimumFireDist = 5;
  }

 //was move()
  fn move4(&mut self, ts : TurretState, time : f32 /* = 1*/, anger : f32 /*= 0*/) -> bool {
    if self._disabled {
      return true;
    }
    let itv : f32 = interval * ((1 - anger) * 0.99 + 0.01);
    if itv < 3 {
      itv = 3;
    }
    if ts.fireCnt > itv {
      ts.fireCnt = itv;
    }
    let spd : f32 = speed * (1 + anger * 0.2);
    if fireingAtATime {
      ts.fireCnt -= time;
      if ts.fireCnt <= 0 {
        ts.fireCnt = itv;
        if ts.fireCnt < 3 {
          ts.fireCnt = 3;
        }
        if isAbleToFire(ts.pos) {
          let sp : f32 = spd - speedAccel * (burstNum - 1) / 2;
          for i in 0..burstNum {
            let d : f32 = ts.deg - nwayAngle * (nway - 1) / 2 + nwayBaseDeg;
            let nsp : f32 = sp - nwaySpeedAccel * ts.nwaySpeedAccelDir * (nway - 1) / 2;
            for j in 0..nway {
              let b : Bullet = bullets.getInstance();
              if !b {
                break;
              }
              b.set(bulletSpec, ts.pos, d, nsp * SPEED_RATIO);
              b.setWaitCnt(i * burstInterval);
              d += nwayAngle;
              nsp += nwaySpeedAccel * ts.nwaySpeedAccelDir;
            }
            sp += speedAccel;
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
          ts.burstNum = burstNum;
          ts.burstCnt = 0;
          ts.speed = spd - speedAccel * (ts.burstNum - 1) / 2;
        }
      }
      if ts.burstNum > 0 {
        ts.burstCnt -= time;
        if ts.burstCnt <= 0 {
          ts.burstCnt = burstInterval;
          ts.burstNum -= 1;
          if isAbleToFire(ts.pos) {
            let d : f32 = ts.deg - nwayAngle * (nway - 1) / 2 + nwayBaseDeg;
            let nsp : f32 = ts.speed - nwaySpeedAccel * ts.nwaySpeedAccelDir * (nway - 1) / 2;
            for i in 0..nway {
              let b : Bullet = bullets.getInstance();
              if !b {
                break;
              }
              b.set(bulletSpec, ts.pos, d, nsp * SPEED_RATIO);
              d += nwayAngle;
              nsp += nwaySpeedAccel * ts.nwaySpeedAccelDir;
            }
          }
          ts.speed += speedAccel;
        }
      }
    }
    true
  }

  fn isAbleToFire(&self, p : Vector) -> bool {
    if gameState.mode != GameState::Mode::MODERN {
      p.y > 0
    } else {
      (p.y > 0 && p.dist(player.pos) > minimumFireDist)
    }
  }

  fn disabled(&mut self, v : bool) -> bool {
    self._disabled = v;
    v
  }
}

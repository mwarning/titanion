/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::f32::consts::PI;

use util::vector::*;
use util::actor::*;
use util::rand::*;
use ttn::token::*;
use ttn::shape::*;
use ttn::bullet::*;
use ttn::field::*;
use ttn::player::*;
use ttn::enemy::*;
use ttn::pillar::*;
use ttn::frame::*;
use ttn::letter::*;
use ttn::particle::*;
use ttn::sound::*;
use ttn::dummy::*;


/*
 * Enemy appearance pattern handler.
 */

const PHASE_RESULT_SHOW_CNT : i32 = 150;
const PHASE_START_SHOW_CNT : i32 = 90;

pub struct Stage<'a> {
  randomized : bool,
  field : &'a Field<'a>,
  enemies : &'a EnemyPool<'a>,
  bullets : &'a BulletPool<'a>,
  player : &'a Player<'a>,
  particles : &'a ParticlePool<'a>,
  bonusParticles : &'a ParticlePool<'a>,
  pillars : &'a PillarPool<'a>,
  gameState : &'a GameState<'a>,
  rand : Rand,
  appCnt : i32,
  middleEnemySpec : MiddleEnemySpec<'a>,
  smallEnemy1Spec : SE1Spec<'a>,
  smallEnemy2Spec : SE2Spec<'a>,
  enemy1Shape : Enemy1Shape,
  enemy2Shape : Enemy2Shape,
  enemy3Shape : Enemy3Shape,
  enemy1TrailShape : Enemy1TrailShape,
  enemy2TrailShape : Enemy2TrailShape,
  enemy3TrailShape : Enemy3TrailShape,
  bulletSpec : BulletSpec<'a>,
  middleBulletSpec : BulletSpec<'a>,
  counterBulletSpec : BulletSpec<'a>,
  bulletShape : BulletShape,
  bulletLineShape : BulletLineShape,
  middleBulletShape : MiddleBulletShape,
  middleBulletLineShape : MiddleBulletLineShape,
  counterBulletShape : CounterBulletShape,
  counterBulletLineShape : CounterBulletLineShape,
  pillarSpec : PillarSpec<'a>,
  pillarShapes : Vec<&'a PillarShape>,
  outsidePillarShape : &'a PillarShape,
  smallEnemyNum : i32,
  smallEnemyFormationNum : i32,
  rank : f32,
  phaseTime : i32,
  stageStarted : bool,
  waitNextFormationPhase : bool,
  middleEnemyAppInterval : i32,
  _attackSmallEnemyNum : i32,
  goingDownBeforeStandByRatio : f32,
  appCntInterval : i32,
  formationIdx : i32,
  cnt : i32,
  rankTrg : f32,
  phaseNum : i32,
  shotFiredNum : i32,
  shotHitNum : i32,
  shotFiredNumRsl : i32,
  shotHitNumRsl : i32,
  shotFiredNumTotal : i32,
  shotHitNumTotal : i32,
  hitRatio : f32,
  hitRatioBonus : i32,
  _existsCounterBullet : bool,
}

impl<'a> Stage<'a> {
  pub fn new(field : &'a Field, enemies : &'a EnemyPool, bullets : &'a BulletPool,
            player : &'a Player, particles : &'a ParticlePool,
            bonusParticles : &'a ParticlePool, pillars : &'a PillarPool, gameState : &'a GameState) -> Stage<'a> {
    let bulletShape = BulletShape::new();
    let bulletLineShape = BulletLineShape::new();
    let middleBulletShape = MiddleBulletShape::new();
    let middleBulletLineShape = MiddleBulletLineShape::new();
    let counterBulletShape = CounterBulletShape::new();
    let counterBulletLineShape = CounterBulletLineShape::new();
    let enemy1Shape = Enemy1Shape::new();
    let enemy2Shape = Enemy2Shape::new();
    let enemy3Shape = Enemy3Shape::new();
    let enemy1TrailShape = Enemy1TrailShape::new();
    let enemy2TrailShape = Enemy2TrailShape::new();
    let enemy3TrailShape = Enemy3TrailShape::new();
    let middleBulletSpec = BulletSpec::new(field, player, enemies, particles, middleBulletShape, middleBulletLineShape, gameState);
    let counterBulletSpec = BulletSpec::new(field, player, enemies, particles, counterBulletShape, counterBulletLineShape, gameState);
    let bulletSpec = BulletSpec::new(field, player, enemies, particles, bulletShape, bulletLineShape, gameState);

    Stage{
      field : field,
      enemies : enemies,
      bullets : bullets,
      player : player,
      particles : particles,
      bonusParticles : bonusParticles,
      pillars : pillars,
      gameState : gameState,
      rand : Rand::new(),

      // moved here from setEnemySpecs(), some arguments were moved
      middleEnemySpec : MiddleEnemySpec::new(enemy3Shape, enemy3TrailShape, middleBulletSpec, counterBulletSpec),
      smallEnemy1Spec : SE1Spec::new(enemy1Shape, enemy1TrailShape, bulletSpec, counterBulletSpec),
      smallEnemy2Spec : SE2Spec::new(enemy2Shape, enemy2TrailShape, bulletSpec, counterBulletSpec),

      enemy1Shape : Enemy1Shape::new(),
      enemy2Shape : Enemy2Shape::new(),
      enemy3Shape : Enemy3Shape::new(),
      enemy1TrailShape : Enemy1TrailShape::new(),
      enemy2TrailShape : Enemy2TrailShape::new(),
      enemy3TrailShape : Enemy3TrailShape::new(),
      bulletShape : bulletShape,
      bulletLineShape : bulletLineShape,
      middleBulletShape : middleBulletShape,
      middleBulletLineShape : middleBulletLineShape,
      counterBulletShape : counterBulletShape,
      counterBulletLineShape : counterBulletLineShape,
      bulletSpec : bulletSpec,
      middleBulletSpec : middleBulletSpec,
      counterBulletSpec : counterBulletSpec,
      pillarSpec : PillarSpec::new(field),
      pillarShapes : [Pillar1Shape::new(), Pillar2Shape::new(), Pillar3Shape::new(), Pillar4Shape::new()],
      outsidePillarShape : OutsidePillarShape::new(),

      // from clear()
      smallEnemyNum : 0,
      smallEnemyFormationNum : 0,
      rank : 0.0,
      phaseTime : 0,
      stageStarted : false,
      waitNextFormationPhase : false,
      middleEnemyAppInterval : 0,
      _attackSmallEnemyNum : 0,
      goingDownBeforeStandByRatio : 0.0,
      appCntInterval : 0,
      formationIdx : 0,
      cnt : 0,
      rankTrg : 0.0,
      phaseNum : 0,
      shotFiredNum : 0,
      shotHitNum : 0,
      shotFiredNumRsl : 0,
      shotHitNumRsl : 0,
      shotFiredNumTotal : 0,
      shotHitNumTotal : 0,
      hitRatio : 0,
      hitRatioBonus : 0,
      _existsCounterBullet : false,
    }
  }

  pub fn close(&mut self) {
    self.enemy1Shape.close();
    self.enemy2Shape.close();
    self.enemy3Shape.close();
    self.enemy1TrailShape.close();
    self.enemy2TrailShape.close();
    self.enemy3TrailShape.close();
    self.bulletShape.close();
    self.bulletLineShape.close();
    self.middleBulletShape.close();
    self.middleBulletLineShape.close();
    self.counterBulletShape.close();
    self.counterBulletLineShape.close();
    for ps in self.pillarShapes {
      ps.close();
    }
    self.outsidePillarShape.close();
  }

  pub fn start(&mut self, randSeed : i64) {
    self.clear();
    self.rand.setSeed(randSeed);
    self.gameState.enemy_spec_rand.setRandSeed(randSeed);
    self.gameState.turret_spec_rand.setRandSeed(randSeed);
    self.gameState.player_spec_rand.setRandSeed(randSeed);
    self.gameState.particle_spec_rand.setRandSeed(randSeed);
    self.gameState.sound_rand.setRandSeed(randSeed);
    self.rank = 0;
    self.rankTrg = 0;
    self.phaseNum = 0;
    self.cnt = 0;
    self.shotFiredNumTotal = 0;
    self.shotHitNumTotal = 0;
    for i in 0..1000 {
      self.cnt += 1;
      self.moveOutsidePillars();
      self.pillars.move();
    }
    self.startPhase();
  }

  pub fn clear(&mut self) {
    self.smallEnemyNum = 0;
    self.smallEnemyFormationNum = 0;
    self.rank = 0.0;
    self.phaseTime = 0;
    self.stageStarted = false;
    self.waitNextFormationPhase = false;
    self.middleEnemyAppInterval = 0;
    self._attackSmallEnemyNum = 0;
    self.goingDownBeforeStandByRatio = 0.0;
    self.appCntInterval = 0;
    self.formationIdx = 0;
    self.cnt = 0;
    self.rankTrg = 0.0;
    self.phaseNum = 0;
    self.shotFiredNum = 0;
    self.shotHitNum = 0;
    self.shotFiredNumRsl = 0;
    self.shotHitNumRsl = 0;
    self.shotFiredNumTotal = 0;
    self.shotHitNumTotal = 0;
    self.hitRatio = 0.0;
    self.hitRatioBonus = 0;
    self._existsCounterBullet = false;
  }

  pub fn startPhase(&mut self) {
    self.phaseTime = 0;
    self.phaseNum += 0;
    if self.phaseNum > 1 {
      self.calcHitRatioBonus();
    }
    if (self.phaseNum % 10) == 0 {
      Sound::fadeBgm();
    }
    self.setEnemySpecs();
    self.initPillars();
  }

  fn calcHitRatioBonus(&mut self) {
    self.shotFiredNumRsl = self.shotFiredNum;
    self.shotHitNumRsl = self.shotHitNum;
    self.shotFiredNum = 0;
    self.shotHitNum = 0;
    if self.shotFiredNumRsl <= 0 {
      self.hitRatio = 0;
    } else {
      self.hitRatio = (self.shotHitNumRsl as f32) / (self.shotFiredNumRsl as f32);
    }
    let mut r : f32 = (((self.hitRatio * 100.0) as i32) / 100) as f32;
    if r > 1.0 {
      r = 1.0;
    }
    self.hitRatioBonus = (10000.0 * r * r * r * r) as i32;
    if self.gameState.mode() == Mode::MODERN {
      return;
    }
    if self.gameState.mode() == Mode::BASIC {
      self.hitRatioBonus *= 10;
    }
    self.gameState.addScore(self.hitRatioBonus, true);
  }

  fn setEnemySpecs(&mut self) {
    let rand = &self.gameState.player_spec_rand;

    self.rankTrg += 1.25;
    self.rank += (self.rankTrg - self.rank) * 0.33;
    if (self.phaseNum % 10) == 0 {
      self.rank *= 0.1;
    }
    if !self.randomized {
      let rs : i64 = self.phaseNum;
      match self.gameState.mode() {
        Mode::CLASSIC => { rs *= 2; },
        Mode::BASIC => {},
        Mode::MODERN => { rs *= 3; },
      }
      rand.setSeed(rs);
      self.gameState.enemy_spec_rand.setRandSeed(rs);
      self.gameState.turret_spec_rand.setRandSeed(rs);
    }
    self._existsCounterBullet = false;
    let en : i32;
    match self.gameState.mode() {
      Mode::CLASSIC => {
        en = 24 + ((50 + rand.nextInt(10)) * self.rank.sqrt() * 0.2) as i32;
        self.smallEnemyNum = 4 + rand.nextInt(2);
        if self.rank > 10 {
          self._existsCounterBullet = true;
        }
        self.middleEnemyAppInterval = 6 + rand.nextInt(2);
      },
      Mode::BASIC => {
        en = 32 + ((50 + rand.nextInt(10)) * self.rank.sqrt() * 0.33) as i32;
        self.smallEnemyNum = 7 + rand.nextInt(4);
        self.middleEnemyAppInterval = 5 + rand.nextInt(2);
      },
      Mode::MODERN => {
        en = 24 + ((50 + rand.nextInt(10)) * self.rank.sqrt() * 0.5) as i32;
        self.smallEnemyNum = 4 + rand.nextInt(2);
        self.middleEnemyAppInterval = 7 + rand.nextInt(3);
      }
    }
    self.smallEnemyFormationNum = ((en / self.smallEnemyNum) + 1) as i32;
    //moved to ctor
    /*
    self.middleEnemySpec = MiddleEnemySpec::new
      (self.field, self.bullets, self.player, self.particles, self.bonusParticles, self.enemies,
       self, self.enemy3Shape, self.enemy3TrailShape,
       self.middleBulletSpec, self.counterBulletSpec, self.gameState);
    */
    self.middleEnemySpec.setRank(self.rank * 0.15);
    //moved to ctor
    /*
    self.smallEnemy1Spec = SE1Spec::new
      (self.field, self.bullets, self.player, self.particles, self.bonusParticles, self.enemies,
       self, self.enemy1Shape, self.enemy1TrailShape,
       self.bulletSpec, self.counterBulletSpec, self.gameState);
    */
    self.smallEnemy1Spec.setRank(self.rank * 0.22);
    /*
    //moved to ctor
    self.smallEnemy2Spec = SE2Spec::new
      (self.field, self.bullets, self.player, self.particles, self.bonusParticles, self.enemies,
       self, self.enemy2Shape, self.enemy2TrailShape,
       self.bulletSpec, self.counterBulletSpec, self.gameState);
      */
    self.smallEnemy2Spec.setRank(self.rank * 0.22);
    self._attackSmallEnemyNum = (self.rank + 2.0).sqrt() as i32;
    self.goingDownBeforeStandByRatio = 0.0;
    if self.rand.nextFloat(self.rank + 1.0) > 2.0 {
      self.goingDownBeforeStandByRatio = rand.nextFloat(0.2) + 0.1;
    }
    self.appCntInterval = (48.0 + rand.nextSignedInt(10)) as f32;
    self.appCntInterval *= 0.5 + 0.5 / self.rank.sqrt();
    if self.gameState.mode() == Mode::MODERN {
      self.appCntInterval *= 0.75;
      self._attackSmallEnemyNum *= 2;
    }
    self.appCnt = 0;
    self.formationIdx = 0;
    self.stageStarted = false;
    self.waitNextFormationPhase = false;
  }

  pub fn initPillars(&mut self) {
    self.pillars.setEnd();
    let mut pp : Option(&Pillar) = None;
    let mut pln : i32 = 0;
    let pn = self.phaseNum;
    let mut pshapes = Vec::new();
    while true {
      if pn <= 0 {
        break;
      }
      if pn >= 20 {
        pshapes.push(3);
        pn -= 20;
      } else if pn >= 10 {
        pshapes.push(2);
        pn -= 10;
      } else if pn >= 5 {
        pshapes.push(1);
        pn -= 5;
      } else {
        pshapes.push(0);
        pn -= 1;
      }
      pln += 1;
    }
    let maxY : f32 = -15.0 + pln * 8.0;
    for i in 0..pln {
      let p = self.pillars.getInstance();
      if !p {
        break;
      }
      p.set(self.pillarSpec, -80 - i * 10, maxY, pp, self.pillarShapes[pshapes[i]], (pln - i as f32) * 0.03);
      pp = p;
    }
  }

  pub fn move1(&mut self) {
    let rand = &self.gameState.player_spec_rand;

    if self.appCnt <= 0 {
      if (self.formationIdx % self.middleEnemyAppInterval) == (self.middleEnemyAppInterval - 1) {
        let me = self.enemies.getInstance();
        if !me {
          return;
        }
        let mut x = rand.nextFloat(self.field.circularDistance);
        x = self.field.normalizeX(x);
        let sp = 0.1 + rand.nextSignedFloat(0.01);
        let av = sp * 0.4 + rand.nextSignedFloat(0.005);
        let er = rand.nextFloat(0.5);
        let ed = rand.nextFloat(PI * 2.0);
        me.set(self.middleEnemySpec, x, self.field.size.y * /*Field.*/ PIT_SIZE_Y_RATIO, PI, sp);
        me.setMiddleEnemyState(sp, av, er, ed);
      }
      let mut x = rand.nextFloat(self.field.circularDistance);
      x = self.field.normalizeX(x);
      let sp = 0.15 + rand.nextSignedFloat(0.01);
      let av = sp * 0.5 + rand.nextSignedFloat(0.005);
      let dst = sp * 6.0;
      let er = rand.nextFloat(0.8);
      let ed = rand.nextFloat(PI * 2.0);
      let fe : Option<&Enemy> = None;
      let fir = 0.0;
      for i in 0..self.smallEnemyNum {
        if let Some(e) = self.enemies.getInstance() {
          let appPattern : i32 = self.formationIdx % 2;
          let ses = match self.formationIdx % 3 {
            0 => (self.smallEnemy1Spec as &SmallEnemySpec),
            1 => (self.smallEnemy1Spec as &SmallEnemySpec),
            2 => (self.smallEnemy2Spec as &SmallEnemySpec),
          };
          e.set(ses, x, self.field.size.y * /*Field.*/ PIT_SIZE_Y_RATIO + (i as f32) * dst, PI, sp);
          let gd = rand.nextFloat(1) < self.goingDownBeforeStandByRatio;
          if fe == None {
            e.setSmallEnemyState(sp, av, (i * (dst / sp)) as i32, self.appPattern, er, ed, gd, 0.0, None);
            fe = Some(e);
          } else {
            e.setSmallEnemyState(sp, av, (i * (dst / sp)) as i32, self.appPattern, er, ed, gd, fir, fe);
          }
          fir += 1.0 / self.smallEnemyNum;
        } else {
          break;
        }
      }
      self.smallEnemyFormationNum -= 1;
      self.formationIdx += 1;
      if self.smallEnemyFormationNum <= 0 {
        self.stageStarted = true;
        self.appCnt = 9999999;
      } else {
        self.appCnt += (self.appCntInterval * (1.0 - 1.0 / (self.enemies.num as f32 + 1.0))) as i32;
      }
    }
    self.appCnt -= 1;
    self.phaseTime += 1;
    if (self.phaseNum >= 10) && ((self.phaseNum % 10) == 0) && (self.phaseTime == 120) &&
      self.gameState.isInGameAndNotGameOver {
      Sound::nextBgm();
    }
    self.cnt += 1;
    self.moveOutsidePillars();
    if self.enemies.numInScreen() > 0 {
      self.gameState.mulMultiplier(0.999);
    }
    if self.stageStarted && self.enemies.num <= 0 {
      self.startPhase();
    }
  }

  pub fn moveOutsidePillars(&mut self) {
    if (self.cnt % 120) == 0 {
      if let Some(p) = self.pillars.getInstance() {
        p.set(self.pillarSpec, 180, 0, None, self.outsidePillarShape, (((self.cnt / 120) % 2 * 2 - 1) as f32)  * 0.003, true);
      }
    }
  }

  pub fn countShotFired(&mut self) {
    if self.phaseTime >= PHASE_RESULT_SHOW_CNT {
      self.shotFiredNum += 1;
      self.shotFiredNumTotal += 1;
    }
  }

  pub fn countShotHit(&mut self) {
    if self.phaseTime >= PHASE_RESULT_SHOW_CNT {
      self.shotHitNum += 1;
      self.shotHitNumTotal += 1;
    }
  }

  pub fn draw(&mut self) {
    if (self.gameState.mode() != Mode::MODERN) && (self.phaseTime < PHASE_RESULT_SHOW_CNT) && (self.phaseNum > 1) {
      Letter::drawString11("SHOTS FIRED", 152.0, 250.0, 6.0, Letter::Direction::TO_RIGHT, false, 0.0, 1.0, 1.0, 0.33);
      Letter::drawNum(self.shotFiredNumRsl, 480.0, 250.0, 6);
      Letter::drawString11("NUMBER OF HITS", 152.0, 280.0, 6.0, Letter::Direction::TO_RIGHT, false, 0.0, 1.0, 1.0, 0.33);
      Letter::drawNum(self.shotHitNumRsl, 480.0, 280.0, 6);
      Letter::drawString("HIT-MISS RATIO", 152.0, 310.0, 6.0);
      Letter::drawNum7((self.hitRatio * 10000.0) as i32, 480.0, 310.0, 6.0, 3, -1.0, 2);
      Letter::drawString11("BONUS", 200.0, 350.0, 6.0, Letter::Direction::TO_RIGHT, false, 0.0, 1.0, 0.33, 0.33);
      Letter::drawNum(self.hitRatioBonus, 440.0, 350.0, 6);
    } else if self.phaseTime < (PHASE_RESULT_SHOW_CNT + PHASE_START_SHOW_CNT) {
      Letter::drawNum(self.phaseNum, 392.0, 200.0, 10);
      Letter::drawString("PHASE", 232.0, 200.0, 10.0);
    }
  }

  pub fn drawPhaseNum(&self) {
    Letter::drawNum(self.phaseNum, 622, 448, 10);
  }

  pub fn drawGameover(&self) {
    let hr = if self.shotFiredNumTotal > 0 {
      (self.shotHitNumTotal as f32) / (self.shotFiredNumTotal as f32)
    } else {
      0.0
    };
    Letter::drawString11("SHOTS FIRED", 152.0, 250.0, 6.0, Letter::Direction::TO_RIGHT, false, 0.0, 1.0, 1.0, 0.33);
    Letter::drawNum(self.shotFiredNumTotal, 480.0, 250.0, 6);
    Letter::drawString("NUMBER OF HITS", 152.0, 280.0, 6.0, Letter::Direction::TO_RIGHT, false, 0.0, 1.0, 1.0, 0.33);
    Letter::drawNum(self.shotHitNumTotal, 480.0, 280.0, 6);
    Letter::drawString("HIT-MISS RATIO", 152.0, 310.0, 6.0);
    Letter::drawNum7((hr * 10000.0) as i32, 480.0, 310.0, 6.0, 3, -1.0, 2);
  }

  pub fn attackSmallEnemyNum(&self) -> i32 {
    self._attackSmallEnemyNum
  }

  pub fn existsCounterBullet(&self) -> bool {
    self._existsCounterBullet && self.stageStarted && (self.enemies.numBeforeAlign <= 0)
  }
}

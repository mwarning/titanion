/*
 * $Id: stage.d,v 1.6 2006/12/09 04:17:40 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.stage;


private import src.util.rand;
private import src.util.vector;
private import src.ttn.field;
private import src.ttn.token;
private import src.ttn.enemy;
private import src.ttn.bullet;
private import src.ttn.player;
private import src.ttn.particle;
private import src.ttn.pillar;
private import src.ttn.shape;
private import src.ttn.sound;
private import src.ttn.frame;
private import src.ttn.letter;
*/

/**
 * Enemy appearance pattern handler.
 */

let PHASE_RESULT_SHOW_CNT : i32 = 150;
let PHASE_START_SHOW_CNT : i32 = 90;

struct Stage {
  randomized : bool,
  field : &Field,
  enemies : &EnemyPool,
  bullets : &BulletPool,
  player : &Player,
  particles : &ParticlePool,
  bonusParticles : &ParticlePool,
  pillars : &PillarPool,
  gameState : &GameState,
  rand : &Rand,
  appCnt : i32,
  middleEnemySpec : &EnemySpec,
  smallEnemy1Spec : &EnemySpec,
  SmallEnemySpec : &EnemySpec,
  enemy1Shape : &EnemySpec,
  enemy2Shape : &EnemySpec,
  enemy3Shape : &EnemySpec,
  enemy1TrailShape : &EnemySpec,
  enemy2TrailShape : &EnemySpec,
  enemy3TrailShape : &EnemySpec,
  bulletSpec : &BulletSpec,
  middleBulletSpec : &BulletSpec,
  counterBulletSpec : &BulletSpec,
  bulletShape : &BulletShapeBase,
  bulletLineShape : &BulletShapeBase,
  middleBulletShape : &BulletShapeBase,
  middleBulletLineShape : &BulletShapeBase,
  counterBulletShape : &RollBulletShapeBase,
  counterBulletLineShape : &RollBulletShapeBase,
  pillarSpec : PillarSpec,
  pillarShapes : Vec<PillarShape>,
  outsidePillarShape : &PillarShape,
  smallEnemyNum : i32,
  smallEnemyFormationNum : i32,
  rank : f32
  phaseTime : i32
  stageStarted : bool,
  waitNextFormationPhase : bool,
  middleEnemyAppInterval : i32,
  _attackSmallEnemyNum : i32
  goingDownBeforeStandByRatio : f32,
  appCntInterval : i32,
  formationIdx : i32,
  cnt : i32,
  rankTrg : f32,
  phaseNum : i32,
  shotFiredNum : i32,
  shotHitNum : i32,
  shotFiredNumRsl : i32
  shotHitNumRsl : i32
  shotFiredNumTotal : i32,
  shotHitNumTotal : i32
  hitRatio : f32,
  hitRatioBonus : i32
  _existsCounterBullet : bool,
}

impl Default for Stage {
  fn default(field : &Field, enemies : &EnemyPool, bullets : &BulletPool,
            player : &Player, particles : &ParticlePool,
            bonusParticles : ParticlePool, pillars : &PillarPool, gameState : &GameState) -> Stage {
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
      enemy1Shape : Enemy1Shape::new(),
      enemy2Shape : Enemy2Shape::new(),
      enemy3Shape : Enemy3Shape::new(),
      enemy1TrailShape : Enemy1TrailShape::new(),
      enemy2TrailShape : Enemy2TrailShape::new(),
      enemy3TrailShape : Enemy3TrailShape::new(),
      bulletShape : BulletShape::new(),
      bulletLineShape : BulletLineShape::new(),
      middleBulletShape : MiddleBulletShape::new(),
      middleBulletLineShape : MiddleBulletLineShape::new(),
      counterBulletShape : CounterBulletShape::new(),
      counterBulletLineShape : CounterBulletLineShape::new(),
      bulletSpec : BulletSpec::new(field, player, enemies, particles,
                                  bulletShape, bulletLineShape, gameState),
      middleBulletSpec : BulletSpec::new(field, player, enemies, particles,
                                        middleBulletShape, middleBulletLineShape, gameState),
      counterBulletSpec : BulletSpec::new(field, player, enemies, particles,
                                         counterBulletShape, counterBulletLineShape, gameState),
      pillarSpec : PillarSpec::new(field),
      pillarShapes :  [Pillar1Shape::new(), Pillar2Shape::new(), Pillar3Shape::new(), Pillar4Shape::new()],
      outsidePillarShape : OutsidePillarShape::new(),
    }
  }

  fn close(&mut self) {
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

  fn start(&mut self, randSeed : i64) {
    self.clear();
    self.rand.setSeed(randSeed);
    EnemySpec.setRandSeed(randSeed);
    TurretSpec.setRandSeed(randSeed);
    PlayerSpec.setRandSeed(randSeed);
    ParticleSpec.setRandSeed(randSeed);
    Sound.setRandSeed(randSeed);
    self.rank = 0;
    self.rankTrg = 0;
    self.phaseNum = 0;
    self.cnt = 0;
    self.shotFiredNumTotal = 0;
    self.shotHitNumTotal = 0;
    for i in 0..1000 {
      cnt += 1;
      self.moveOutsidePillars();
      self.pillars.move();
    }
    self.startPhase();
  }

  fn clear(&mut self) {
    self.smallEnemyNum = 0;
    self.smallEnemyFormationNum = 0;
    self.rank = 0;
    self.phaseTime = 0;
    self.stageStarted = false;
    self.waitNextFormationPhase = false;
    self.middleEnemyAppInterval = 0;
    self._attackSmallEnemyNum = 0;
    self.goingDownBeforeStandByRatio = 0;
    self.appCntInterval = 0;
    self.formationIdx = 0;
    self.cnt = 0;
    self.rankTrg = 0;
    self.phaseNum = 0;
    self.shotFiredNum = 0;
    self.shotHitNum = 0;
    self.shotFiredNumRsl = 0;
    self.shotHitNumRsl = 0;
    self.shotFiredNumTotal = 0;
    self.shotHitNumTotal = 0;
    self.hitRatio = 0;
    self.hitRatioBonus = 0;
    self._existsCounterBullet = false;
  }

  fn startPhase(&mut self) {
    self.phaseTime = 0;
    self.phaseNum += 0;
    if self.phaseNum > 1 {
      self.calcHitRatioBonus();
    }
    if (self.phaseNum % 10) == 0 {
      Sound.fadeBgm();
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
      self.hitRatio = (shotHitNumRsl / shotFiredNumRsl) as f32;
    }
    let mut r : f32 = ((self.hitRatio * 100) as i32) / 100) as f32;
    if r > 1.0 {
      r = 1.0;
    }
    self.hitRatioBonus = cast(int) (10000.0f * r * r * r * r);
    if (self.gameState.mode == GameState.Mode.MODERN) {
      return;
    }
    if (self.gameState.mode == GameState.Mode.BASIC){
      self.hitRatioBonus *= 10;
    }
    self.gameState.addScore(hitRatioBonus, true);
  }

  fn setEnemySpecs(&mut self) {
    self.rankTrg += 1.25;
    self.rank += (self.rankTrg - self.rank) * 0.33;
    if (self.phaseNum % 10) == 0 {
      self.rank *= 0.1f;
    }
    if !self.randomized {
      let rs : i64 = self.phaseNum;
      match self.gameState.mode {
        GameState.Mode.CLASSIC => { rs *= 2; },
        GameState.Mode.BASIC => {},
        GameState.Mode.MODERN => { rs *= 3; },
      }
      rand.setSeed(rs);
      EnemySpec.setRandSeed(rs);
      TurretSpec.setRandSeed(rs);
    }
    self._existsCounterBullet = false;
    let en : i32;
    match self.gameState.mode {
      GameState.Mode.CLASSIC => {
        en = 24 + cast(int) ((50 + rand.nextInt(10)) * rank.sqrt() * 0.2f);
        self.smallEnemyNum = 4 + rand.nextInt(2);
        if (rank > 10)
          _self.existsCounterBullet = true;
        self.middleEnemyAppInterval = 6 + rand.nextInt(2);
      },
      GameState.Mode.BASIC => {
        en = 32 + cast(int) ((50 + rand.nextInt(10)) * rank.sqrt() * 0.33f);
        self.smallEnemyNum = 7 + rand.nextInt(4);
        self.middleEnemyAppInterval = 5 + rand.nextInt(2);
      },
      GameState.Mode.MODERN >= {
        en = 24 + cast(int) ((50 + rand.nextInt(10)) * rank.sqrt() * 0.5);
        self.smallEnemyNum = 4 + rand.nextInt(2);
        self.middleEnemyAppInterval = 7 + rand.nextInt(3);
      }
    }
    smallEnemyFormationNum = ((en / smallEnemyNum) + 1) as i32;
    self.middleEnemySpec = MiddleEnemySpec::new
      (field, bullets, player, particles, bonusParticles, enemies,
       self, enemy3Shape, enemy3TrailShape,
       middleBulletSpec, counterBulletSpec, gameState);
    middleEnemySpec.setRank(rank * 0.15);
    smallEnemy1Spec = SE1Spec::new
      (field, bullets, player, particles, bonusParticles, enemies,
       self, enemy1Shape, enemy1TrailShape,
       bulletSpec, counterBulletSpec, gameState);
    (smallEnemy1Spec as SE1Spec).setRank(rank * 0.22);
    smallEnemy2Spec = SE2Spec::new
      (field, bullets, player, particles, bonusParticles, enemies,
       self, enemy2Shape, enemy2TrailShape,
       bulletSpec, counterBulletSpec, gameState);
    (smallEnemy2Spec as SE2Spec).setRank(rank * 0.22);
    _attackSmallEnemyNum = cast(int) sqrt(rank + 2);
    goingDownBeforeStandByRatio = 0;
    if rand.nextFloat(rank + 1) > 2 {
      goingDownBeforeStandByRatio = rand.nextFloat(0.2) + 0.1;
    }
    appCntInterval = 48 + rand.nextSignedInt(10);
    appCntInterval *= (0.5 + 0.5 / rank.sqrt());
    if gameState.mode == GameState.Mode.MODERN {
      appCntInterval *= 0.75;
      _attackSmallEnemyNum *= 2;
    }
    self.appCnt = 0;
    self.formationIdx = 0;
    self.stageStarted = false;
    waitNextFormationPhase = false;
  }

  fn initPillars(&mut self) {
    self.pillars.setEnd();
    let pp : Pillar = null;
    let mut pln : i32 = 0;
    let pn = phaseNum;
    int[] pshapes;
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
      let p = pillars.getInstance();
      if !p {
        break;
      }
      p.set(pillarSpec, -80 - i * 10, maxY, pp, self.pillarShapes[pshapes[i]], (pln - i) * 0.03f);
      pp = p;
    }
  }

  fn move(&mut self) {
    if self.appCnt <= 0 {
      if ((self.formationIdx % self.middleEnemyAppInterval) == self.middleEnemyAppInterval - 1) {
        let me = self.enemies.getInstance();
        if !me {
          return;
        }
        let mut x : f32 = rand.nextFloat(self.field.circularDistance);
        x = self.field.normalizeX(x);
        let sp : f32 = 0.1 + rand.nextSignedFloat(0.01);
        let av : f32 = sp * 0.4 + rand.nextSignedFloat(0.005);
        let er : f32 = rand.nextFloat(0.5);
        let ed : f32 = rand.nextFloat(PI * 2.0);
        me.set(self.middleEnemySpec, x, self.field.size.y * Field.PIT_SIZE_Y_RATIO, PI, sp);
        me.setMiddleEnemyState(sp, av, er, ed);
      }
      let mut x : f32 = rand.nextFloat(self.field.circularDistance);
      x = self.field.normalizeX(x);
      let sp : f32 = 0.15 + rand.nextSignedFloat(0.01);
      let av : f32 = sp * 0.5 + rand.nextSignedFloat(0.005);
      let dst : f32 = sp * 6.0;
      let er : f32 = rand.nextFloat(0.8);
      let ed : f32 = rand.nextFloat(PI * 2.0);
      let fe : &Enemy = null;
      let fir : f32 = 0.0;
      for i in 0..self.smallEnemyNum {
        let e = self.enemies.getInstance();
        if !e {
          break;
        }

        let appPattern : i32 = self.formationIdx % 2;
        let ses : &SmallEnemySpec = match self.formationIdx % 3 {
          0 => { ses = smallEnemy1Spec as SmallEnemySpec; },
          1 => { ses = smallEnemy1Spec as SmallEnemySpec; },
          2 => { ses = smallEnemy2Spec as SmallEnemySpec; },
        }
        e.set(ses, x, field.size.y * Field.PIT_SIZE_Y_RATIO + i * dst, PI, sp);
        let mut gd : bool = false;
        if rand.nextFloat(1) < goingDownBeforeStandByRatio {
          gd = true;
        }
        if !fe {
          e.setSmallEnemyState(sp, av, (i * (dst / sp)) as i32, self.appPattern,
                               er, ed, gd);
          fe = e;
        } else {
          e.setSmallEnemyState(sp, av, (i * (dst / sp)) as i32, self.appPattern,
                               er, ed, gd, fir, fe);
        }
        fir += (1.0 / self.smallEnemyNum);
      }
      self.smallEnemyFormationNum -= 1;
      self.formationIdx += 1;
      if self.smallEnemyFormationNum <= 0 {
        self.stageStarted = true;
        self.appCnt = 9999999;
      } else {
        self.appCnt += self.appCntInterval * (1 - 1 / (self.enemies.num + 1));
      }
    }
    self.appCnt -= 1;
    self.phaseTime += 1;
    if (self.phaseNum >= 10) && ((self.phaseNum % 10) == 0) && (self.phaseTime == 120) &&
        self.gameState.isInGameAndNotGameOver {
      Sound.nextBgm();
    }
    self.cnt += 1;
    self.moveOutsidePillars();
    if enemies.numInScreen() > 0 {
      self.gameState.mulMultiplier(0.999);
    }
    if self.stageStarted && self.enemies.num <= 0 {
      self.startPhase();
    }
  }

  fn moveOutsidePillars(&mut self) {
    if (self.cnt % 120) == 0 {
      let p : &Pillar = self.pillars.getInstance();
      if p {
        p.set(self.pillarSpec, 180, 0, null, self.outsidePillarShape, (((cnt / 120) as i32) % 2 * 2 - 1) * 0.003, true);
      }
    }
  }

  fn countShotFired(&mut self) {
    if self.phaseTime >= PHASE_RESULT_SHOW_CNT {
      self.shotFiredNum += 1;
      self.shotFiredNumTotal += 1;
    }
  }

  fn countShotHit(&mut self) {
    if self.phaseTime >= PHASE_RESULT_SHOW_CNT {
      self.shotHitNum += 1;
      self.shotHitNumTotal += 1;
    }
  }

  fn draw(&mut self) {
    if (self.gameState.mode != GameState.Mode.MODERN) &&
        (self.phaseTime < PHASE_RESULT_SHOW_CNT) && (self.phaseNum > 1) {
      Letter.drawString("SHOTS FIRED", 152, 250, 6, Letter.Direction.TO_RIGHT,
                        false, 0, 1, 1, 0.33f);
      Letter.drawNum(self.shotFiredNumRsl, 480, 250, 6);
      Letter.drawString("NUMBER OF HITS", 152, 280, 6, Letter.Direction.TO_RIGHT,
                        false, 0, 1, 1, 0.33f);
      Letter.drawNum(self.shotHitNumRsl, 480, 280, 6);
      Letter.drawString("HIT-MISS RATIO", 152, 310, 6);
      Letter.drawNum((self.hitRatio * 10000) as i32, 480, 310, 6, 3, -1, 2);
      Letter.drawString("BONUS", 200, 350, 6, Letter.Direction.TO_RIGHT,
                        false, 0, 1, 0.33f, 0.33f);
      Letter.drawNum(self.hitRatioBonus, 440, 350, 6);
    } else if (phaseTime < PHASE_RESULT_SHOW_CNT + PHASE_START_SHOW_CNT) {
      Letter.drawNum(self.phaseNum, 392, 200, 10);
      Letter.drawString("PHASE", 232, 200, 10);
    }
  }

  fn drawPhaseNum(&self) {
    Letter.drawNum(self.phaseNum, 622, 448, 10);
  }

  fn drawGameover(&self) {
  let hr : f32 = 0;
    if self.shotFiredNumTotal > 0 {
      hr = (self.shotHitNumTotal as f32) / (self.shotFiredNumTotal as f32);
    }
    Letter.drawString("SHOTS FIRED", 152, 250, 6, Letter.Direction.TO_RIGHT,
                      false, 0, 1, 1, 0.33f);
    Letter.drawNum(self.shotFiredNumTotal, 480, 250, 6);
    Letter.drawString("NUMBER OF HITS", 152, 280, 6, Letter.Direction.TO_RIGHT,
                      false, 0, 1, 1, 0.33f);
    Letter.drawNum(self.shotHitNumTotal, 480, 280, 6);
    Letter.drawString("HIT-MISS RATIO", 152, 310, 6);
    Letter.drawNum((hr * 10000) as i32, 480, 310, 6, 3, -1, 2);
  }

  fn attackSmallEnemyNum(&self) -> i32 {
   self._attackSmallEnemyNum
  }

  fn existsCounterBullet(&self) -> bool {
    (self._existsCounterBullet && self.stageStarted && (self.enemies.numBeforeAlign <= 0))
  }
}

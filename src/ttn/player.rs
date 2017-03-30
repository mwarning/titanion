/*
 * $Id: player.d,v 1.4 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.player;


private import std.math;

private import derelict.opengl.gl;

private import src.util.vector;
private import src.util.rand;
private import src.util.math;
private import src.util.actor;
private import src.util.sdl.pad;
private import src.util.sdl.recordableinput;
private import src.ttn.field;
private import src.ttn.frame;
private import src.ttn.screen;
private import src.ttn.shape;
private import src.ttn.token;
private import src.ttn.enemy;
private import src.ttn.bullet;
private import src.ttn.particle;
private import src.ttn.sound;
private import src.ttn.letter;
*/

use std::f32::consts::PI;

use util::vector::*;
use util::actor::*;
use ttn::token::*;
use ttn::enemy::*;
use ttn::bullet::*;
use ttn::shape::*;
use ttn::field::*;
use ttn::dummy::*;


/**
 * Player and shots.
 */
pub struct Player<'a> {
  //tok : Token!(PlayerState, PlayerSpec),
  pub _exists : bool, //from Actor
  pub state : &'a mut PlayerState,
  pub spec : &'a mut PlayerSpec,

  hitOffset : Vector,
}

impl<'a> Actor for Player<'a> {
  fn getExists(&self) -> bool {
    self._exists
  }

  fn setExists(&mut self, v : bool)-> bool {
    self._exists = v;
    v
  }

  fn init(&mut self /*Object[] args*/) {
    self.state = PlayerState::new();
  }
}

impl<'a> Token<PlayerState, PlayerSpec> for Player<'a> {
}

impl Player {
  fn new(spec : &PlayerSpec) -> Player {
    let mut ins = Player {
      state : PlayerState::new(),
      spec : spec,
      _exists : false, //correct init?
      hitOffset : Vector::new(),
    };
    ins.spec.setState(ins.state);
    ins.state.setSpec(ins.spec);
    ins
  }

  fn replayMode(&mut self, v : bool) -> bool {
    self.tok.state.replayMode = v;
    v
  }

  fn set(&mut self) {
    self.tok.state.set();
    self.tok.spec.start();
    self.hitOffset.x = 0.0;
    self.hitOffset.y = 0.0;
    self.tok.spec.field.setEyePos(self.pos());
  }

  fn checkBulletHit(&self, p : Vector, pp : Vector) -> bool {
    //with (state) {
      if !self.tok.state.hasCollision {
        return false;
      }
      if self.tok.spec.field.checkHitDist(self.state.pos, p, pp, self.spec.bulletHitWidth) {
        self.destroy();
        return true;
      }
      false
    //}
  }

  fn checkEnemyHit(&mut self, p : Vector, v : Vector, size : Vector) -> bool {
    if self.tok.spec.gameState.mode == GameStateMode::MODERN {
      return false;
    }
    //with (state) {
      if !self.state.hasCollision {
        return false;
      }
      if ((self.state.pos.x - p.x).abs() < size.x) && ((self.state.pos.y - p.y).abs() < size.y) {
        match self.spec.gameState.mode {
          GameStateMode::CLASSIC => { self.destroy(); },
          GameStateMode::BASIC => {
          self.hitOffset.x = self.pos().x - p.x;
          self.hitOffset.y = self.pos().y - p.y;
          self.tok.spec.addVelocity(self.tok.state, v, self.hitOffset);
          },
        _ => {},
        }
        return true;
      }
      false
    //}
  }

  fn destroy(&mut self) {
    self.remove();
    self.tok.spec.destroyed(self.tok.state);
  }

  fn drawState(&mut self) {
    if self.tok.spec.gameState.mode == GameStateMode::CLASSIC {
      self.tok.spec.drawState(self.state);
    }
  }

  fn destroyCapturedEnemies(&mut self, idx : i32) {
    self.tok.state.destroyCapturedEnemies(idx);
  }

  fn isInTractorBeam(&self, p : Vector) -> bool{
    self.tok.spec.tractorBeam.contains(p)
  }

  fn addCapturedEnemy(&mut self, e : &Enemy) {
    self.tok.state.addCapturedEnemy(e)
  }

  fn capturedEnemyWidth(&mut self) -> f32 {
    self.tok.state.capturedEnemyWidth
  }

  fn midEnemyProvacated(&mut self) {
    self.tok.state.midEnemyProvacated = true;
  }

  fn addScore(&mut self, sc : i32) {
    self.tok.spec.addScore(sc);
  }

  fn addMultiplier(&mut self, mp : f32) {
    self.tok.spec.addMultiplier(mp);
  }

  fn multiplier(&self) -> f32 {
    self.tok.spec.multiplier
  }

  fn deg(&self) -> f32 {
    self.tok.state.deg
  }

  fn isActive(&self) -> bool {
    self.tok.state.isActive
  }

  fn hasCollision(&self) -> bool {
    self.state.hasCollision
  }

  fn enemiesHasCollision(&self) -> bool {
    match self.tok.spec.gameState.mode {
      GameStateMode::CLASSIC => self.tok.state.hasCollision,
      GameStateMode::BASIC => true,
      GameStateMode::MODERN => false
    }
  }
}

const RESPAWN_INTERVAL : i32 = 72;
const INVINCIBLE_INTERVAL_RESPAWN : i32 = 240;
const MAX_CAPTURED_ENEMIES_NUM : i32 = 10;

struct PlayerState {
  ts : TokenState,
  replayMode : bool,
  spec : PlayerSpec,
  capturedEnemies : Vec<Enemy>,
  capturedEnemyNum : i32,
  respawnCnt : i32,
  isInRespawn : bool,
  invincibleCnt : i32,
  isInvincible : bool,
  shotCnt : i32,
  capturedEnemyShotCnt : i32,
  aPressed : bool,
  bPressed : bool,
  vel : Vector,
  capturedEnemyWidth : f32,
  colorCnt : i32,
  isFirstShot : bool,
  captureBeamEnergy : f32,
  captureBeamReleased : bool,
  ghostCnt : i32,
  ghostShotCnt : i32,
  midEnemyProvacated : bool,
}
/*
  invariant() {
    if (isInitialized) {
      assert(vel.x <>= 0);
      assert(vel.y <>= 0);
      assert(capturedEnemyWidth >= 0);
      assert(captureBeamEnergy <>= 0);
    }
  }
*/

impl PlayerState {
  fn new() -> PlayerState {
      PlayerState{
        ts : TokenState::new(),
        replayMode : false,
        spec : PlayerSpec::new(),
        capturedEnemies : [],
        capturedEnemyNum : 0,
        respawnCnt : 0,
        isInRespawn : false,
        invincibleCnt : 0,
        isInvincible : false,
        shotCnt : 0,
        capturedEnemyShotCnt : 0,
        aPressed : false,
        bPressed : false,
        vel : Vector::new(),
        capturedEnemyWidth : 0.0,
        colorCnt : 0,
        isFirstShot : false,
        captureBeamEnergy : 0.0,
        captureBeamReleased : false,
        ghostCnt : 0,
        ghostShotCnt : 0,
        midEnemyProvacated : false,
    }
  }

  fn setSpec(&mut self, spec : PlayerSpec) {
    self.spec = spec;
  }

  fn set(&mut self) {
    self.reset();
    self.pos.x = 0.0;
    self.respawnCnt = 0;
    self.isInRespawn = false;
    self.aPressed = true;
    self.bPressed = true;
    self.shotCnt = 60;
  }

  fn clear(&mut self) {
    self.capturedEnemyNum = 0;
    self.respawnCnt = 0;
    self.invincibleCnt = 0;
    self.isInRespawn = false;
    self.isInvincible = false;
    self.shotCnt = 0;
    self.capturedEnemyShotCnt = 0;
    self.vel.x = 0.0;
    self.vel.y = 0.0;
    self.capturedEnemyWidth = 1.0;
    self.colorCnt = 0;
    self.isFirstShot = false;
    self.captureBeamEnergy = 0;
    self.captureBeamReleased = false;
    self.ghostCnt = 0;
    self.ghostShotCnt = 0;
    self.midEnemyProvacated = false;
    self.ts.clear();
  }

  fn reset(&mut self) {
    let x : f32 = self.pos.x;
    self.clear();
    self.ts.pos.x = x;
    self.ts.pos.y = -10.0;
    self.ts.speed = PlayerSpec::BASE_SPEED;
    self.invincibleCnt = INVINCIBLE_INTERVAL_RESPAWN;
    self.isInvincible = true;
    self.isFirstShot = true;
    self.captureBeamEnergy = 1;
    self.spec.respawn(self);
  }

  fn move1(&mut self) {
    self.colorCnt += 1;
    self.ghostCnt += 1;
    if self.isInRespawn {
      self.respawnCnt -= 1;
      if self.respawnCnt <= 0 {
        self.reset();
        self.isInRespawn = false;
      }
    } else if self.isInvincible {
      self.invincibleCnt -= 1;
      if self.invincibleCnt <= 0 {
        self.isInvincible = false;
      }
    }
    self.midEnemyProvacated = false;
  }

  fn isActive(&self) -> bool {
    !self.isInRespawn
  }

  fn hasCollision(&self) -> bool {
    !self.isInRespawn && !self.isInvincible
  }

  fn hasShape(&self) -> bool {
    if self.isInRespawn {
      return false;
    }
    if !self.isInvincibl {
      return true;
    }
    
    (self.invincibleCnt % 60) >= 30
  }

  fn destroyed(&mut self) {
    self.respawnCnt = RESPAWN_INTERVAL;
    self.destroyCapturedEnemies(0);
    self.isInRespawn = true;
  }

  fn addCapturedEnemy(&mut self, e : &Enemy) -> f32 {
    if self.isInRespawn || (self.capturedEnemyNum >= MAX_CAPTURED_ENEMIES_NUM) {
      return -1;
    }
    self.capturedEnemies[self.capturedEnemyNum as usize] = e;
    self.capturedEnemyNum += 1;
    (self.capturedEnemyNum - 1)
  }

  fn destroyCapturedEnemies(&mut self, idx : i32) {
    for i in idx..self.capturedEnemyNum {
      if self.capturedEnemies[i as usize].exists() {
        self.capturedEnemies[i as usize].destroyed();
      }
    }
    self.capturedEnemyNum = idx;
  }

  fn countShotHit(&mut self) {
    self.captureBeamEnergy += 0.02 / ((self.capturedEnemyNum as f32) + 1.0);
    if self.captureBeamEnergy > 1 {
      self.captureBeamEnergy = 1;
    }
  }
}

const BASE_SPEED : f32 = 0.15;
const BASE_VELOCITY : f32 = 0.03;
const CAPTURED_ENEMIES_INTERVAL_LENGTH : f32 = 1.2;
const TILT_DEG : f32 = 1.0;
const SHOT_INTERVAL : f32 = 3;
const FIRST_SHOT_INTERVAL : f32 = 6;
const TWIN_SHOT_MAX_NUM : i32 = 2;

struct PlayerSpec {
  ts : TokenSpec<PlayerState>,
  //mixin StaticRandImpl;
  shots : &ShotPool,
  capturedEnemiesShots : &ShotPool,
  shotSpec : &ShotSpec,
  enemies : &EnemyPool,
  bullets : &BulletPool,
  particles : &ParticlePool,
  pad : &RecordablePad,
  gameState : &GameState,
  playerState : Option(&PlayerState),
  tractorBeam : Option(&TractorBeam),
  lineShape : &Shape,
  bulletHitWidth : f32,
  ghostEnemySpec : &GhostEnemySpec,
  ghostEnemyShape : &EnemyShape,
  shotMaxNum : i32,
}

impl PlayerSpec {
  fn new(pad : &Pad, gameState : &GameState,  field : &Field, enemies : &EnemyPool, bullets : &BulletPool, particles : &ParticlePool) {
    let mut ins = PlayerSpec {
      ts : TokenSpec::<PlayerState>::new(field, PlayerShape::new()),
      //mixin StaticRandImpl;
      shots : ShotPool::new(),
      capturedEnemiesShots : ShotPool::new(),
      shotSpec : ShotSpec::new(field, enemies, bullets, gameState),
      enemies : enemies,
      bullets : bullets,
      particles : particles,
      pad : (pad as &RecordablePad),
      gameState : gameState,
      playerState : None,
      tractorBeam : None,
      lineShape : PlayerLineShape::new(),
      bulletHitWidth : 0.0,
      ghostEnemySpec : GhostEnemySpec::new(field, ghostEnemyShape),
      ghostEnemyShape : Enemy1TrailShape::new(),
      shotMaxNum : 0,
    };
    ins.shots.init(16);
    ins.capturedEnemiesShots.init(64);
    ins
  }

  fn setState(&mut self, ps : &PlayerState) {
    self.playerState = Some(ps);
    self.shotSpec.setPlayerState(ps);
    self.tractorBeam = TractorBeam::new(self.field, ps, self.gameState);
  }

  fn close(&mut self) {
    self.ghostEnemyShape.close();
    (self.shape as &PlayerShape).close();
    self.shotSpec.close();
  }

  fn start(&mut self) {
    self.clear();
    match self.gameState.mode {
      GameStateMode::CLASSIC => {
        self.bulletHitWidth = 0.4;
        self.shotMaxNum = 3;
      },
      GameStateMode::BASIC => {
        self.bulletHitWidth = 0.2;
        self.shotMaxNum = 3;
      },
      GameStateMode::MODERN => {
        self.bulletHitWidth = 0.1;
        self.shotMaxNum = 16;
      },
    }
  }

  fn respawn(&mut self, ps : &PlayerState) {
    if self.gameState.mode == GameStateMode::MODERN {
      for i in 0..4 {
        if let Some(e) = self.enemies.getInstance() {
          e.set(self.ghostEnemySpec, ps.pos.x, ps.pos.y, 0, 0);
          self.playerState.addCapturedEnemy(e);
        } else {
          break;
        }
      }
    }
  }

  fn clear(&mut self) {
    self.tractorBeam.clear();
    self.shots.clear();
    self.capturedEnemiesShots.clear();
  }

  fn move2(&mut self, ps : &PlayerState) -> bool {
    //with (ps) {
      let mut input : PadState;
      if !self.replayMode {
        input = self.pad.getState();
      } else {
        //try {
          input = self.pad.replay();
        /*} catch (NoRecordDataException e) {
          gameState.startGameOverWithoutRecording();
          input = pad.getNullState();
        }*/
      }
      self.shots.move1();
      self.capturedEnemiesShots.move1();
      self.capturedEnemiesShots.checkParent();
      if self.gameState.isGameOver {
        if self.input.button & PadState.Button.A {
          if !self.aPressed {
            self.aPressed = true;
            if !self.replayMode {
              self.gameState.backToTitle();
            }
          }
        } else {
          self.aPressed = false;
        }
        return true;
      }
      ps.move1();
      if !self.isActive() {
        return true;
      }
      let mut vx : f32 = 0.0;
      let mut vy : f32 = 0.0;

      if self.input.dir & PadState.Dir.RIGHT {
        vx = 1.0;
      } else if self.input.dir & PadState.Dir.LEFT {
        vx = -1.0;
      }

      if self.input.dir & PadState.Dir.UP {
        vy = 1.0;
      } else if self.input.dir & PadState.Dir.DOWN {
        vy = -1.0;
      }

      if (vx != 0.0) && (vy != 0.0) {
        vx *= 0.7;
        vy *= 0.7;
      }

      let mut px : f32 = ps.pos.x;
      ps.pos.x += vx * ps.speed;
      if self.gameState.mode == GameStateMode::CLASSIC {
        vy *= 0.5;
      }
      ps.pos.y += vy * ps.speed;
      if !(input.button & PadState.Button.B) {
        ps.deg += (-TILT_DEG * (vx * ps.speed) - ps.deg) * 0.1;
      }
      //assert(deg <>= 0);
      ps.pos += ps.vel;
      ps.vel *= 0.9;
      if self.gameState.mode == GameStateMode::MODERN {
        let mut d : f32 = ps.ghostCnt * 0.05;
        for i in 0..self.capturedEnemyNum {
          let e : Enemy = self.capturedEnemies[i];
          e.setGhostEnemyState(ps.pos.x + d.sin() * ps.capturedEnemyWidth * 2.0, ps.pos.y, ps.deg, (d * 180.0 / PI / 3.0) as i32);
          d += PI / 2.0;
        }
      }
      match self.gameState.mode {
       GameStateMode::CLASSIC => {
        /*if (input.button & PadState.Button.A) {
          if (!aPressed) {
            aPressed = true;
            if (!captureBeamReleased)
              fireShot(ps);
          }
        } else {
          aPressed = false;
        }*/
        if (self.input.button & PadState.Button.A) && !self.captureBeamReleased {
          if self.shotCnt <= 0 {
            self.fireShot(ps);
          }
        } else {
          self.isFirstShot = true;
        }
      },
      GameStateMode::BASIC => {
        if (self.input.button & PadState.Button.A) && !(self.input.button & PadState.Button.B) {
          if self.shotCnt <= 0 {
            self.fireShot(ps);
          }
        } else {
          self.isFirstShot = true;
        }
      },
      GameStateMode::MODERN => {
        if self.input.button & PadState.Button.A {
          if self.shotCnt <= 0 {
            self.fireShot(ps);
          }
        } else {
          self.isFirstShot = true;
        }
      },
      }
      if self.input.button & PadState.Button.B {
        ps.speed += (BASE_SPEED * 1.2 - ps.speed) * 0.33;
        ps.deg *= 0.9;
        if self.gameState.mode == GameStateMode::MODERN {
          ps.capturedEnemyWidth -= 0.05;
          if ps.capturedEnemyWidth < 0.2 {
            ps.capturedEnemyWidth = 0.2;
          }
        }
      } else {
        ps.speed += (BASE_SPEED * 2.0 - ps.speed) * 0.33;
        if ps.gameState.mode == GameStateMode::MODERN {
          ps.capturedEnemyWidth += 0.05;
          if ps.capturedEnemyWidth > 1.0 {
            ps.capturedEnemyWidth = 1.0;
          }
        }
      }
      match self.gameState.mode {
        GameStateMode::CLASSIC => {
        if (self.input.button & PadState.Button.B) &&
            !self.captureBeamReleased && (self.captureBeamEnergy >= 1.0) &&
            (self.capturedEnemyNum < MAX_CAPTURED_ENEMIES_NUM) {
          self.captureBeamReleased = true;
          self.isInvincible = true;
          self.invincibleCnt = 99999;
        }
        if self.captureBeamReleased {
          if (self.captureBeamEnergy <= 0.0) || (self.capturedEnemyNum >= MAX_CAPTURED_ENEMIES_NUM) {
            self.captureBeamEnergy = 0.0;
            if self.tractorBeam.reduceLength(0.5) {
              self.captureBeamReleased = false;
              self.invincibleCnt = 120;
            }
          } else {
            self.tractorBeam.extendLength(0.5);
            self.captureBeamEnergy -= 0.005;
          }
        }
        },
      GameStateMode::BASIC => {
        if (self.input.button & PadState.Button.B) &&
            (self.capturedEnemyNum < MAX_CAPTURED_ENEMIES_NUM) {
          self.tractorBeam.extendLength();
        } else {
          self.tractorBeam.reduceLength();
        }
      },
      GameStateMode::MODERN => {
        if (self.input.button & PadState.Button.B) &&
            !(self.input.button & PadState.Button.A) {
          self.tractorBeam.extendLength();
        } else {
          self.tractorBeam.reduceLength();
        }
      },
      }
      self.tractorBeam.move1();
      if self.shotCnt > 0 {
        self.shotCnt -= 1;
      }
      if self.capturedEnemyShotCnt > 0 {
        self.capturedEnemyShotCnt -= 1;
      }
      match self.gameState.mode {
      GameStateMode::CLASSIC => {
        if self.pos.y > 0.0 {
          self.pos.y = 0.0;
        }
      },
      GameStateMode::BASIC => {
        if self.pos.y > 0 {
          self.pos.y = 0;
        }
      },
      GameStateMode::MODERN => {
        if self.pos.y > self.field.size.y {
          self.pos.y = self.field.size.y;
        }
      },
      }
      if self.pos.y < -self.field.size.y {
        self.pos.y = -self.field.size.y;
      }
      if self.pos.x > self.field.size.x {
        self.pos.x = self.field.size.x;
      }
      else if self.pos.x < -self.field.size.x {
        self.pos.x = -self.field.size.x;
      }
      self.pos.x = self.field.normalizeX(self.pos.x);
      self.field.setEyePos(self.pos);
      true
    //}
  }

  fn fireShot(&mut self, ps : &PlayerState) {
    //with (ps) {
      if self.shots.num >= self.shotMaxNum {
        return;
      }
      if let Some(s) = self.shots.getInstance() {
        s.set(self.shotSpec, ps.pos, ps.deg, 0.66);
        if self.isFirstShot {
          self.isFirstShot = false;
          self.shotCnt += FIRST_SHOT_INTERVAL;
        } else {
          self.shotCnt += SHOT_INTERVAL;
        }
        self.gameState.countShotFired();
        self.addShotParticle(ps.pos, ps.deg);
        Sound.playSe("shot.wav");
        for i in 0..self.capturedEnemyNum {
          if (self.gameState.mode == GameStateMode::MODERN) && ((i + self.ghostShotCnt) % 4 == 0) {
            continue;
          }
          if self.capturedEnemies[i as usize].isCaptured {
            let ces : Shot = self.capturedEnemiesShots.getInstance();
            if !ces {
              break;
            }
            let mut d : f32 = ps.deg;
            if self.gameState.mode == GameStateMode::MODERN {
              d -= (self.capturedEnemies[i].pos.x - self.pos.x) * 0.3;
            }
            ces.set(self.shotSpec, self.capturedEnemies[i].pos, d, 0.66);
            if self.gameState.mode != GameStateMode::MODERN {
              self.ces.setParent(s);
            }
            else {
              self.gameState.countShotFired();
            }
            self.addShotParticle(self.capturedEnemies[i].pos, ps.deg);
          }
        }
        if self.gameState.mode == GameStateMode::MODERN {
          self.ghostShotCnt += 1;
        }
      }
    //}
  }

  fn addShotParticle(&mut self, p : Vector, d : f32) {
    for i in 0..5  {
      let mut pt : Particle;
      pt = self.particles.getInstanceForced();
      pt.set(Particle.Shape.LINE, p.x - 0.5, p.y,
             -d + rand.nextSignedFloat(0.5), 0.25 + rand.nextFloat(0.75),
             1, 1.0, 0.25, 0.5, 10);
      pt = self.particles.getInstanceForced();
      pt.set(Particle.Shape.LINE, p.x + 0.5, p.y,
             -d + rand.nextSignedFloat(0.5), 0.25 + rand.nextFloat(0.75),
             1, 1.0, 0.25, 0.5, 10);
    }
  }

  fn addVelocity(&mut self, ps : &PlayerState, v : Vector, o : Vector) {
    let mut rv : Vector = v.getElement(o, 0.05, 0.25);
    rv *= 5.0;
    ps.vel += rv;
    let d : f32 = (rv.x, -rv.y).atan2();
    let sp : f32 = rv.vctSize();
    for i in 0..36 {
      let mut pt : Particle = self.particles.getInstanceForced();
      let mut r : f32 = 0.5 + rand.nextFloat(0.5);
      let mut g : f32 = 0.3 + rand.nextFloat(0.3);
      let mut b : f32 = 0.8 + rand.nextFloat(0.2);
      pt.set(Particle.Shape.LINE, ps.pos.x, ps.pos.y,
             d + rand.nextSignedFloat(0.3), sp * (1 + rand.nextFloat(2)),
             1, r, g, b, 30 + rand.nextInt(30));
    }
    Sound.playSe("flick.wav");
  }

  fn destroyed(&mut self, ps : &PlayerState) {
    //with (ps) {
      if !self.isActive {
        return;
      }
      self.ps.destroyed();
      self.tractorBeam.clear();
      self.gameState.destroyedPlayer();
      let mut r : f32 = 0.5 + rand.nextFloat(0.5);
      let mut g : f32= 0.3 + rand.nextFloat(0.3);
      let mut b : f32 = 0.8 + rand.nextFloat(0.2);
      for i in 0..100 {
        let mut p : Particle = self.particles.getInstanceForced();
        p.set(Particle.Shape.QUAD, ps.pos.x, ps.pos.y, rand.nextFloat(PI * 2.0), 0.01 + rand.nextFloat(1.0),
              1 + rand.nextFloat(4), r, g, b, 10 + rand.nextInt(200));
      }
      r = 0.5 + rand.nextFloat(0.5);
      g = 0.3 + rand.nextFloat(0.3);
      b = 0.8 + rand.nextFloat(0.2);
      for i in 0..30 {
        let mut p : Particle = self.particles.getInstanceForced();
        p.set(Particle.Shape.TRIANGLE, ps.pos.x, ps.pos.y, rand.nextFloat(PI * 2.0), 0.03 + rand.nextFloat(0.3),
              3, r, g, b, 50 + rand.nextInt(150));
      }
      r = 0.5 + rand.nextFloat(0.5);
      g = 0.3 + rand.nextFloat(0.3);
      b = 0.8 + rand.nextFloat(0.2);
      for i in 0..300 {
        let mut p : Particle = self.particles.getInstanceForced();
        p.set(Particle.Shape.LINE, ps.pos.x, ps.pos.y, rand.nextFloat(PI * 2.0), 0.07 + rand.nextFloat(0.7),
              1, r, g, b, 100 + rand.nextInt(100));
      }
      Sound.playSe("player_explosion.wav");
    //}
  }

  fn addScore(&mut self, sc : i32) {
    self.gameState.addScore(sc);
  }

  fn addMultiplier(&mut self, mp : f32) {
    self.gameState.addMultiplier(mp);
  }

  fn multiplier(&self) -> f32 {
    self.gameState.multiplier
  }

  fn draw(&mut self, ps : &PlayerState) {
    //with (ps) {
      self.shots.draw();
      self.capturedEnemiesShots.draw();
      self.tractorBeam.draw();
      if !self.isActive {
        return;
      }
      let p : Vector3 = self.field.calcCircularPos(ps.pos);
      let cd : f32 = self.field.calcCircularDeg(ps.pos.x);
      if self.hasShape {
        self.shape.draw(p, cd, ps.deg);
      }
      let c : i32 = ps.colorCnt % 60;
      let mut a : f32;
      if c < 30 {
        a = (c as f32) / 30.0;
      }
      else {
        a = 1.0 - ((c - 30) as f32) / 30.0;
      }
      Screen.setColor(a, a, a);
      self.lineShape.draw(p, cd, ps.deg);
    //}
  }

  fn drawState(&mut self, ps : &PlayerState) {
    //with (ps) {
      Screen.setColor(1, 1, 1, 0.5);
      glBegin(GL_TRIANGLE_FAN);
      glVertex3f(15, 400, 0);
      glVertex3f(15 + self.captureBeamEnergy * 100, 400, 0);
      glVertex3f(25 + self.captureBeamEnergy * 100, 420, 0);
      glVertex3f(25, 420, 0);
      glEnd();
      glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
      let mut a : f32;
      if self.captureBeamEnergy < 1 {
        a = self.captureBeamEnergy;
      } else {
        let c  : i32 = ps.colorCnt % 60;
        if c < 30 {
          a = (c / 30) as f32;
        }
        else {
          a = 1.0 - ((c - 30) as f32) / 30.0;
        }
      }
      Screen.setColor(1, 1, 1, a);
      glBegin(GL_LINE_LOOP);
      glVertex3f(15, 400, 0);
      glVertex3f(115, 400, 0);
      glVertex3f(125, 420, 0);
      glVertex3f(25, 420, 0);
      glEnd();
      glBlendFunc(GL_SRC_ALPHA, GL_ONE);
      if self.captureBeamEnergy >= 1.0 {
        Letter.drawString("READY", 50, 390, 4);
      }
    //}
  }
}

struct ShotPool {
  ap : ActorPool<Shot>,
}

impl ShotPool {
  fn checkParent(&mut self) {
    for a in &self.actors {
      if a.exists() {
        if !a.spec.checkParent(a.state) {
          a.remove();
        }
      }
    }
  }

  fn num() -> f32 {
    let mut n = 0;
    for a in &self.actors {
      if a.exists() {
        n += 0;
      }
    }
    n as u32
  }
}

struct Shot {
  tok : Token<ShotState, ShotSpec>,
  _exists : bool, //inherited by Actor class
}

impl Actor for Shot {
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

impl Shot {
  fn setParent(&mut self, s : &Shot) {
    self.tok.spec.setParent(self.tok.state, s);
  }
}

struct ShotState {
  ts : TokenState,
  parent : Shot,
  cnt : i32,
}

impl ShotState {
  fn clear(&mut self) {
    self.parent = None;
    self.cnt = 0;
    self.tok.clear();
  }
}

struct ShotSpec {
  ts : TokenSpec<ShotState>,
  enemies : EnemyPool,
  bullets : BulletPool,
  playerState : PlayerState,
  gameState : GameState,
}

impl ShotSpec {
  fn this(&mut self, field : Field, enemies : EnemyPool, bullets : BulletPool, gameState : GameState) {
    self.field = field;
    self.enemies = enemies;
    self.bullets = bullets;
    self.gameState = gameState;
    self.ts.shape = ShotShape::new();
  }

  fn setPlayerState(&mut self, ps : PlayerState) {
    self.playerState = ps;
  }

  fn close(&mut self) {
    (self.tok.shape as ShotShape).close();
  }

  fn set(ss : ShotState) {
    ss.parent = None;
    ss.cnt = 0;
  }

  fn setParent(ss : &ShotState, s : &Shot) {
    ss.parent = s;
  }

  fn move2(&mut self, ss : &ShotState) -> bool {
    //with (ss) {
      if ss.parent {
        if ss.parent.exists == false {
          return false;
        }
      }
      self.stepForward();
      self.tok.pos.x = self.tok.field.normalizeX(self.tok.pos.x);
      if !self.tok.field.containsOuterY(self.tok.pos.y) {
        return false;
      }
      if self.enemies.checkShotHit(pos, deg, 2.0) {
        if self.parent {
          self.parent.remove();
        }
        self.gameState.countShotHit();
        self.playerState.countShotHit();
        return false;
      }
      self.cnt += 1;
      true;
    //}
  }

  fn checkParent(ss : ShotState) -> bool {
    if ss.parent {
      if ss.parent.exists == false {
        return false;
      }
    }
    true
  }
}

const MAX_LENGTH : f32 = 10.0;
const WIDTH : f32 = 3.0;
const SHAPE_INTERVAL_TIME : f32 = 10.0;
const SHAPE_INTERVAL_LENGTH : f32 = 0.5;

struct TractorBeam {
  field : Field,
  playerState : PlayerState,
  gameState : GameState,
  shapes : Vec<Box<TractorBeamShape>>,
  length : f32, //= 0;
  cnt : i32,
  isExtending : bool,
}
/*
  invariant() {
    assert(length <>= 0);
  }
*/

impl TractorBeam {
  fn new(field : &Field, playerState : &PlayerState, gameState : &GameState) {
    TractorBeam {
      field : field,
      playerState : playerState,
      gameState : gameState,
      shapes : vec![
        Box::new(TractorBeamShapeRed::new()),
        Box::new(TractorBeamShapeBlue::new()),
        Box::new(TractorBeamShapePurple::new()),
        Box::new(TractorBeamShapeDarkRed::new()),
        Box::new(TractorBeamShapeDarkBlue::new()),
        Box::new(TractorBeamShapeDarkPurple::new())
      ],
      length : 0,
      cnt : 0,
      isExtending : false,
    }

    //ins.clear(); //not needed anymore
    //ins
  }

  fn clear(&mut self) {
    self.length = 0;
    self.cnt = 0;
    self.isExtending = false;
  }

  fn move1(&mut self) {
    if self.length <= 0 {
      return;
    }
    self.cnt += 1;
    if (self.cnt % 12) == 0 && self.isExtending {
      Sound.playSe("tractor.wav");
    }
  }

  fn extendLength(&mut self, ratio : f32 /* = 1*/) {
    self.length += (MAX_LENGTH - self.length) * 0.05 * ratio;
    self.isExtending = true;
  }

  fn reduceLength(&mut self, ratio : f32 /*= 1*/) -> bool {
    self.length += (0.0 - self.length) * 0.1 * ratio;
    if self.length < 0.33 {
      self.length = 0;
      return true;
    }
    self.isExtending = false;
    false
  }

  fn contains(&mut self, p : Vector) -> bool {
    if self.length <= 0.0 {
      return false;
    }
    p.x > (self.playerState.pos.x - WIDTH / 2.0) &&
            p.x < (self.playerState.pos.x + WIDTH / 2.0) &&
            p.y > self.playerState.pos.y && p.y < (self.playerState.pos.y + self.length + WIDTH);
  }

  fn draw(&mut self) {
    if self.length <= 0.0 {
      return;
    }
    let y : f32 = SHAPE_INTERVAL_LENGTH - (self.cnt % SHAPE_INTERVAL_TIME) * SHAPE_INTERVAL_LENGTH / SHAPE_INTERVAL_TIME;
    let c : i32 = (self.cnt / SHAPE_INTERVAL_TIME) as i32;
    loop {
      if y > self.length {
        break;
      }
      glPushMatrix();
      let p : Vector3 = self.field.calcCircularPos(self.playerState.pos.x, self.playerState.pos.y + y);
      Screen::glTranslate(p);
      let mut s : f32 = y;
      if s > 1.0 {
        s = 1.0;
      }
      glScalef(s, s, s);
      match self.gameState.mode {
      GameStateMode::CLASSIC => { self.shapes[c % 3].draw(); },
      GameStateMode::BASIC => { self.shapes[c % 3].draw(); },
      GameStateMode::MODERN => {
        if playerState.midEnemyProvacated {
          self.shapes[c % 3].draw();
        }
        else {
          self.shapes[c % 3 + 3].draw();
        }
      },
      }
      c += 1;
      glPopMatrix();
      y += SHAPE_INTERVAL_LENGTH;
    }
  }
}

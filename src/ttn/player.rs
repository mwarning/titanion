/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::f32::consts::PI;

use util::sdl::pad::*;
use util::vector::*;
use util::actor::*;
use ttn::shape::*;
use ttn::token::*;
use ttn::enemy::*;
use ttn::bullet::*;
use ttn::shape::*;
use ttn::field::*;
use ttn::frame::*;
use ttn::particle::*;
use ttn::letter::*;
use ttn::screen::*;
use ttn::dummy::*;


/**
 * Player and shots.
 */
pub struct Player<'a> {
  //tok : Token!(PlayerState, PlayerSpec),
  pub _exists : bool, //from Actor
  pub state : PlayerState<'a>,
  pub spec : &'a mut PlayerSpec<'a>,

  hitOffset : Vector,
}

impl<'a> Actor for Player<'a> {
  fn new() -> Player<'a> {
    Player {
      state : PlayerState::new(),
      spec : PlayerSpec::new(),  //use generic spec or Option type?
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
    self.state = PlayerState::new();
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

impl<'a> Token<PlayerState<'a>, PlayerSpec<'a>> for Player<'a> {
/*
  fn getExists(&self) -> bool {
    self._exists
  }

  fn setExists(&mut self, v : bool) -> bool {
    self._exists = v;
    v
  }
*/
  fn set5Vec(&mut self, spec : &PlayerSpec, pos : Vector, deg : f32, speed : f32) {
    self.spec = spec;
    self.set5(pos.x, pos.y, deg, speed);
  }

  fn set6(&mut self, spec : &PlayerSpec, x : f32, y : f32, deg : f32, speed : f32) {
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
    self.spec.removed(self.state);
  }

  fn pos(&self) -> Vector {
    self.state.ts.pos
  }
}

impl<'a> Player<'a> {
  pub fn new(spec : &PlayerSpec<'a>) -> Player<'a> {
    let mut ins = Player {
      state : PlayerState::new(),
      spec : spec,
      _exists : false, //correct init?
      hitOffset : Vector::new(0.0, 0.0),
    };
    ins.spec.setState(&ins.state);
    ins.state.setSpec(ins.spec);
    ins
  }

  pub fn replayMode(&mut self, v : bool) -> bool {
    self.state.replayMode = v;
    v
  }

  pub fn set(&mut self) {
    self.state.set();
    self.spec.start();
    self.hitOffset.x = 0.0;
    self.hitOffset.y = 0.0;
    self.spec.field.setEyePos(self.state.ts.pos);
  }

  pub fn checkBulletHit(&self, p : Vector, pp : Vector) -> bool {
    //with (state) {
      if !self.state.hasCollision() {
        return false;
      }
      if Field::checkHitDist(self.state.ts.pos, p, pp, self.spec.bulletHitWidth) {
        self.destroy();
        return true;
      }
      false
    //}
  }

  pub fn checkEnemyHit(&mut self, p : Vector, v : Vector, size : Vector) -> bool {
    if self.spec.gameState.mode() == Mode::MODERN {
      return false;
    }
    //with (state) {
      if !self.state.hasCollision() {
        return false;
      }
      if ((self.state.ts.pos.x - p.x).abs() < size.x) && ((self.state.ts.pos.y - p.y).abs() < size.y) {
        match self.spec.gameState.mode() {
          Mode::CLASSIC => { self.destroy(); },
          Mode::BASIC => {
          self.hitOffset.x = self.pos().x - p.x;
          self.hitOffset.y = self.pos().y - p.y;
          self.spec.addVelocity(&self.state, v, self.hitOffset);
          },
        _ => {},
        }
        return true;
      }
      false
    //}
  }

  pub fn destroy(&mut self) {
    self.remove();
    self.spec.destroyed(&self.state);
  }

  pub fn drawState(&mut self) {
    if self.spec.gameState.mode() == Mode::CLASSIC {
      self.spec.drawState(&self.state);
    }
  }

  pub fn destroyCapturedEnemies(&mut self, idx : i32) {
    self.state.destroyCapturedEnemies(idx);
  }

  pub fn isInTractorBeam(&self, p : Vector) -> bool{
    self.spec.tractorBeam.contains(p)
  }

  pub fn addCapturedEnemy(&mut self, e : &Enemy) -> i32 {
    self.state.addCapturedEnemy(e) as i32
  }

  pub fn capturedEnemyWidth(&mut self) -> f32 {
    self.state.capturedEnemyWidth
  }

  pub fn midEnemyProvacated(&mut self) {
    self.state.midEnemyProvacated = true;
  }

  pub fn addScore(&mut self, sc : i32) {
    self.spec.addScore(sc);
  }

  pub fn addMultiplier(&mut self, mp : f32) {
    self.spec.addMultiplier(mp);
  }

  pub fn multiplier(&self) -> f32 {
    self.spec.multiplier()
  }

  pub fn deg(&self) -> f32 {
    self.state.ts.deg
  }

  pub fn isActive(&self) -> bool {
    self.state.isActive()
  }

  pub fn hasCollision(&self) -> bool {
    self.state.hasCollision()
  }

  pub fn enemiesHasCollision(&self) -> bool {
    match self.spec.gameState.mode() {
      Mode::CLASSIC => self.state.hasCollision(),
      Mode::BASIC => true,
      Mode::MODERN => false
    }
  }
}

const RESPAWN_INTERVAL : i32 = 72;
const INVINCIBLE_INTERVAL_RESPAWN : i32 = 240;
const MAX_CAPTURED_ENEMIES_NUM : i32 = 10;

struct PlayerState<'a> {
  ts : TokenState,
  replayMode : bool,
  spec : PlayerSpec<'a>,
  capturedEnemies : Vec<&'a Enemy<'a>>,
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

impl<'a> PlayerState<'a> {
  pub fn new() -> PlayerState<'a> {
      PlayerState{
        ts : TokenState::new(),
        replayMode : false,
        spec : PlayerSpec::new(),
        capturedEnemies : Vec<&'a Enemy<'a>>::new(),
        capturedEnemyNum : 0,
        respawnCnt : 0,
        isInRespawn : false,
        invincibleCnt : 0,
        isInvincible : false,
        shotCnt : 0,
        capturedEnemyShotCnt : 0,
        aPressed : false,
        bPressed : false,
        vel : Vector::new(0.0, 0.0),
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

  pub fn setSpec(&mut self, spec : &PlayerSpec) {
    self.spec = spec;
  }

  pub fn set(&mut self) {
    self.reset();
    self.ts.pos.x = 0.0;
    self.respawnCnt = 0;
    self.isInRespawn = false;
    self.aPressed = true;
    self.bPressed = true;
    self.shotCnt = 60;
  }

  pub fn clear(&mut self) {
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
    self.captureBeamEnergy = 0.0;
    self.captureBeamReleased = false;
    self.ghostCnt = 0;
    self.ghostShotCnt = 0;
    self.midEnemyProvacated = false;
    self.ts.clear();
  }

  pub fn reset(&mut self) {
    let x : f32 = self.pos.x;
    self.clear();
    self.ts.pos.x = x;
    self.ts.pos.y = -10.0;
    self.ts.speed = BASE_SPEED;
    self.invincibleCnt = INVINCIBLE_INTERVAL_RESPAWN;
    self.isInvincible = true;
    self.isFirstShot = true;
    self.captureBeamEnergy = 1.0;
    self.spec.respawn(self);
  }

  pub fn move1(&mut self) {
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

  pub fn isActive(&self) -> bool {
    !self.isInRespawn
  }

  pub fn hasCollision(&self) -> bool {
    !self.isInRespawn && !self.isInvincible
  }

  pub fn hasShape(&self) -> bool {
    if self.isInRespawn {
      return false;
    }
    if !self.isInvincible {
      return true;
    }
    
    (self.invincibleCnt % 60) >= 30
  }

  pub fn destroyed(&mut self) {
    self.respawnCnt = RESPAWN_INTERVAL;
    self.destroyCapturedEnemies(0);
    self.isInRespawn = true;
  }

  pub fn addCapturedEnemy(&mut self, e : &Enemy) -> f32 {
    if self.isInRespawn || (self.capturedEnemyNum >= MAX_CAPTURED_ENEMIES_NUM) {
      return -1.0;
    }
    self.capturedEnemies[self.capturedEnemyNum as usize] = e;
    self.capturedEnemyNum += 1;
    (self.capturedEnemyNum - 1) as f32
  }

  pub fn destroyCapturedEnemies(&mut self, idx : i32) {
    for i in idx..self.capturedEnemyNum {
      if self.capturedEnemies[i as usize].getExists() {
        self.capturedEnemies[i as usize].destroyed();
      }
    }
    self.capturedEnemyNum = idx;
  }

  pub fn countShotHit(&mut self) {
    self.captureBeamEnergy += 0.02 / ((self.capturedEnemyNum as f32) + 1.0);
    if self.captureBeamEnergy > 1.0 {
      self.captureBeamEnergy = 1.0;
    }
  }
}

const BASE_SPEED : f32 = 0.15;
const BASE_VELOCITY : f32 = 0.03;
pub const CAPTURED_ENEMIES_INTERVAL_LENGTH : f32 = 1.2;
const TILT_DEG : f32 = 1.0;
const SHOT_INTERVAL : i32 = 3;
const FIRST_SHOT_INTERVAL : i32 = 6;
const TWIN_SHOT_MAX_NUM : i32 = 2;

pub struct PlayerSpec<'a> {
  //ts : TokenSpec<PlayerState>, //inlined
  field : &'a mut Field<'a>,
  shape : &'a mut Shape,
  frame : &'a Frame<'a>, //reference to access other objects
  //mixin StaticRandImpl;
  shots : &'a ShotPool<'a>,
  capturedEnemiesShots : &'a ShotPool<'a>,
  shotSpec : &'a ShotSpec<'a>,
  enemies : &'a EnemyPool<'a>,
  bullets : &'a BulletPool<'a>,
  particles : &'a ParticlePool<'a>,
  pad : &'a RecordablePad,
  gameState : &'a GameState<'a>,
  playerState : Option<&'a PlayerState<'a>>,
  tractorBeam : Option<&'a TractorBeam<'a>>,
  lineShape : &'a Shape,
  bulletHitWidth : f32,
  ghostEnemySpec : &'a GhostEnemySpec<'a>,
  ghostEnemyShape : &'a EnemyShape,
  shotMaxNum : i32,
}

impl<'a> TokenSpec<PlayerState<'a>> for PlayerSpec<'a> {
  fn set(&self, state : &PlayerState) {}
  fn removed(&self, state : &PlayerState) {}

  fn move2(&self, state : &PlayerState) -> bool {
    true
  }

  fn draw(&self, state : &PlayerState) {
    //with (state) {
      let p = self.field.calcCircularPos1(state.ts.pos);
      let cd = Field::calcCircularDeg(state.ts.pos.x);
      self.shape.draw4(p, cd, state.ts.deg);
    //}
  }
}


impl<'a> PlayerSpec<'a> {
  pub fn new(pad : &RecordablePad /*Pad*/, gameState : &GameState, field : &Field, enemies : &EnemyPool, bullets : &BulletPool, particles : &ParticlePool)
    -> PlayerSpec<'a>
  {
    let ghostEnemyShape = Enemy1TrailShape::new();
    let mut ins = PlayerSpec {
      //ts : TokenSpec::<PlayerState>::new(field, PlayerShape::new()),
      field : field,
      shape : PlayerShape::new(),
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
      ghostEnemyShape : ghostEnemyShape,
      shotMaxNum : 0,
    };
    ins.shots.init(16);
    ins.capturedEnemiesShots.init(64);
    ins
  }

  pub fn setState(&mut self, ps : &PlayerState) {
    self.playerState = Some(ps);
    self.shotSpec.setPlayerState(&mut ps);
    self.tractorBeam = TractorBeam::new(self.field, ps, self.gameState);
  }

  pub fn close(&mut self) {
    self.ghostEnemyShape.close();
    (self.shape as &PlayerShape).close();
    self.shotSpec.close();
  }

  pub fn start(&mut self) {
    self.clear();
    match self.gameState.mode() {
      Mode::CLASSIC => {
        self.bulletHitWidth = 0.4;
        self.shotMaxNum = 3;
      },
      Mode::BASIC => {
        self.bulletHitWidth = 0.2;
        self.shotMaxNum = 3;
      },
      Mode::MODERN => {
        self.bulletHitWidth = 0.1;
        self.shotMaxNum = 16;
      },
    }
  }

  pub fn respawn(&mut self, ps : &PlayerState) {
    if self.gameState.mode() == Mode::MODERN {
      for _ in 0..4 {
        if let Some(e) = self.enemies.getInstance() {
          e.set(self.ghostEnemySpec, ps.ts.pos.x, ps.ts.pos.y, 0, 0);
          self.playerState.addCapturedEnemy(e);
        } else {
          break;
        }
      }
    }
  }

  pub fn clear(&mut self) {
    self.tractorBeam.clear();
    self.shots.clear();
    self.capturedEnemiesShots.clear();
  }

  pub fn move2(&mut self, ps : &PlayerState) -> bool {
    //with (ps) {
      let mut input : PadState;
      if !ps.replayMode {
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
      if self.gameState.isGameOver() {
        if self.input.button & BUTTON_A {
          if !ps.aPressed {
            ps.aPressed = true;
            if !ps.replayMode {
              self.gameState.backToTitle();
            }
          }
        } else {
          ps.aPressed = false;
        }
        return true;
      }
      ps.move1();
      if !self.isActive() {
        return true;
      }
      let mut vx = 0.0;
      let mut vy = 0.0;

      if self.input.dir & DIR_RIGHT {
        vx = 1.0;
      } else if self.input.dir & DIR_LEFT {
        vx = -1.0;
      }

      if self.input.dir & DIR_UP {
        vy = 1.0;
      } else if self.input.dir & DIR_DOWN {
        vy = -1.0;
      }

      if (vx != 0.0) && (vy != 0.0) {
        vx *= 0.7;
        vy *= 0.7;
      }

      let mut px = ps.ts.pos.x;
      ps.ts.pos.x += vx * ps.ts.speed;
      if self.gameState.mode() == Mode::CLASSIC {
        vy *= 0.5;
      }
      ps.ts.pos.y += vy * ps.ts.speed;
      if !(input.button & BUTTON_B) {
        ps.deg += (-TILT_DEG * (vx * ps.ts.speed) - ps.ts.deg) * 0.1;
      }
      //assert(deg <>= 0);
      ps.ts.pos += ps.vel;
      ps.vel *= 0.9;
      if self.gameState.mode() == Mode::MODERN {
        let mut d = (ps.ghostCnt as f32) * 0.05;
        for i in 0..ps.capturedEnemyNum {
          let e : Enemy = self.capturedEnemies[i];
          e.setGhostEnemyState(ps.ts.pos.x + d.sin() * ps.capturedEnemyWidth * 2.0, ps.ts.pos.y, ps.deg, (d * 180.0 / PI / 3.0) as i32);
          d += PI / 2.0;
        }
      }
      match self.gameState.mode() {
       Mode::CLASSIC => {
        /*if (input.button & BUTTON_A) {
          if (!aPressed) {
            aPressed = true;
            if (!ps.captureBeamReleased)
              fireShot(ps);
          }
        } else {
          aPressed = false;
        }*/
        if (self.input.button & BUTTON_A) && !ps.captureBeamReleased {
          if ps.shotCnt <= 0 {
            self.fireShot(ps);
          }
        } else {
          ps.isFirstShot = true;
        }
      },
      Mode::BASIC => {
        if (self.input.button & BUTTON_A) && !(self.input.button & BUTTON_B) {
          if ps.shotCnt <= 0 {
            self.fireShot(ps);
          }
        } else {
          ps.isFirstShot = true;
        }
      },
      Mode::MODERN => {
        if self.input.button & BUTTON_A {
          if ps.shotCnt <= 0 {
            self.fireShot(ps);
          }
        } else {
          ps.isFirstShot = true;
        }
      },
      }
      if self.input.button & BUTTON_B {
        ps.ts.speed += (BASE_SPEED * 1.2 - ps.ts.speed) * 0.33;
        ps.ts.deg *= 0.9;
        if self.gameState.mode() == Mode::MODERN {
          ps.capturedEnemyWidth -= 0.05;
          if ps.capturedEnemyWidth < 0.2 {
            ps.capturedEnemyWidth = 0.2;
          }
        }
      } else {
        ps.ts.speed += (BASE_SPEED * 2.0 - ps.ts.speed) * 0.33;
        if ps.gameState.mode() == Mode::MODERN {
          ps.capturedEnemyWidth += 0.05;
          if ps.capturedEnemyWidth > 1.0 {
            ps.capturedEnemyWidth = 1.0;
          }
        }
      }
      match self.gameState.mode() {
        Mode::CLASSIC => {
        if (self.input.button & BUTTON_B) &&
            !ps.captureBeamReleased && (ps.captureBeamEnergy >= 1.0) &&
            (self.capturedEnemyNum < MAX_CAPTURED_ENEMIES_NUM) {
          ps.captureBeamReleased = true;
          ps.isInvincible = true;
          ps.invincibleCnt = 99999;
        }
        if ps.captureBeamReleased {
          if (ps.captureBeamEnergy <= 0.0) || (ps.capturedEnemyNum >= MAX_CAPTURED_ENEMIES_NUM) {
            ps.captureBeamEnergy = 0.0;
            if self.tractorBeam.reduceLength(0.5) {
              ps.captureBeamReleased = false;
              ps.invincibleCnt = 120;
            }
          } else {
            self.tractorBeam.extendLength(0.5);
            ps.captureBeamEnergy -= 0.005;
          }
        }
        },
      Mode::BASIC => {
        if (self.input.button & BUTTON_B) &&
            (self.capturedEnemyNum < MAX_CAPTURED_ENEMIES_NUM) {
          self.tractorBeam.extendLength(1.0);
        } else {
          self.tractorBeam.reduceLength(1.0);
        }
      },
      Mode::MODERN => {
        if (self.input.button & BUTTON_B) &&
            !(self.input.button & BUTTON_A) {
          self.tractorBeam.extendLength(1.0);
        } else {
          self.tractorBeam.reduceLength(1.0);
        }
      },
      }
      self.tractorBeam.move1();
      if ps.shotCnt > 0 {
        ps.shotCnt -= 1;
      }
      if ps.capturedEnemyShotCnt > 0 {
        ps.capturedEnemyShotCnt -= 1;
      }
      match self.gameState.mode() {
      Mode::CLASSIC => {
        if ps.ts.pos.y > 0.0 {
          ps.ts.pos.y = 0.0;
        }
      },
      Mode::BASIC => {
        if ps.ts.pos.y > 0.0 {
          ps.ts.pos.y = 0.0;
        }
      },
      Mode::MODERN => {
        if ps.ts.pos.y > self.field.size().y {
          ps.ts.pos.y = self.field.size().y;
        }
      },
      }
      if ps.ts.pos.y < -self.field.size().y {
        ps.ts.pos.y = -self.field.size().y;
      }
      if ps.ts.pos.x > self.field.size().x {
        ps.ts.pos.x = self.field.size().x;
      }
      else if ps.ts.pos.x < -self.field.size().x {
        ps.ts.pos.x = -self.field.size().x;
      }
      ps.ts.pos.x = Field::normalizeX(ps.ts.pos.x);
      self.field.setEyePos(ps.ts.pos);
      true
    //}
  }

  pub fn fireShot(&mut self, ps : &PlayerState) {
    //with (ps) {
      if (self.shots.num() as i32) >= self.shotMaxNum {
        return;
      }
      if let Some(s) = self.shots.getInstance() {
        s.set(self.shotSpec, ps.ts.pos, ps.ts.deg, 0.66);
        if ps.isFirstShot {
          ps.isFirstShot = false;
          ps.shotCnt += FIRST_SHOT_INTERVAL;
        } else {
          ps.shotCnt += SHOT_INTERVAL;
        }
        self.gameState.countShotFired();
        self.addShotParticle(ps.ts.pos, ps.ts.deg);
        self.frame.sound.borrow().playSe("shot.wav");
        for i in 0..self.capturedEnemyNum {
          if (self.gameState.mode() == Mode::MODERN) && ((i + ps.ghostShotCnt) % 4 == 0) {
            continue;
          }
          if ps.capturedEnemies[i as usize].isCaptured() {
            if let Some(ces) = self.capturedEnemiesShots.getInstance() {
              let mut d : f32 = ps.ts.deg;
              if self.gameState.mode() == Mode::MODERN {
                d -= (ps.capturedEnemies[i].pos().x - self.pos.x) * 0.3;
              }
              ces.set(self.shotSpec, ps.capturedEnemies[i].pos(), d, 0.66);
              if self.gameState.mode() != Mode::MODERN {
                ces.setParent(s);
              }
              else {
                self.gameState.countShotFired();
              }
              self.addShotParticle(ps.capturedEnemies[i as usize].pos(), ps.deg);
            } else {
              break;
            }
          }
        }
        if self.gameState.mode() == Mode::MODERN {
          ps.ghostShotCnt += 1;
        }
      }
    //}
  }

  pub fn addShotParticle(&mut self, p : Vector, d : f32) {
    let rand = &self.gameState.player_spec_rand;

    for i in 0..5  {
      let mut pt : Particle;
      pt = self.particles.getInstanceForced();
      pt.set(ParticleShape::LINE, p.x - 0.5, p.y,
             -d + rand.nextSignedFloat(0.5), 0.25 + rand.nextFloat(0.75),
             1, 1.0, 0.25, 0.5, 10);
      pt = self.particles.getInstanceForced();
      pt.set(ParticleShape::LINE, p.x + 0.5, p.y,
             -d + rand.nextSignedFloat(0.5), 0.25 + rand.nextFloat(0.75),
             1, 1.0, 0.25, 0.5, 10);
    }
  }

  pub fn addVelocity(&mut self, ps : &PlayerState, v : Vector, o : Vector) {
    let rand = &self.gameState.player_spec_rand;

    let mut rv = v.getElementMinMax(o, 0.05, 0.25);
    rv *= 5.0;
    ps.vel += rv;
    let d = (rv.x, -rv.y).atan2();
    let sp = rv.vctSize();
    for i in 0..36 {
      let mut pt = self.particles.getInstanceForced();
      let mut r = 0.5 + rand.nextFloat(0.5);
      let mut g = 0.3 + rand.nextFloat(0.3);
      let mut b = 0.8 + rand.nextFloat(0.2);
      pt.set(ParticleShape::LINE, ps.ts.pos.x, ps.ts.pos.y,
             d + rand.nextSignedFloat(0.3), sp * (1.0 + rand.nextFloat(2.0)),
             1, r, g, b, 30 + rand.nextInt(30));
    }
    self.frame.sound.borrow().playSe("flick.wav");
  }

  pub fn destroyed(&mut self, ps : &PlayerState) {
    let rand = &self.gameState.player_spec_rand;

    //with (ps) {
      if !ps.isActive() {
        return;
      }
      ps.destroyed();
      self.tractorBeam.clear();
      self.gameState.destroyedPlayer();
      let mut r = 0.5 + rand.nextFloat(0.5);
      let mut g = 0.3 + rand.nextFloat(0.3);
      let mut b = 0.8 + rand.nextFloat(0.2);
      for i in 0..100 {
        let mut p = self.particles.getInstanceForced();
        p.set(ParticleShape::QUAD, ps.ts.pos.x, ps.ts.pos.y, rand.nextFloat(PI * 2.0), 0.01 + rand.nextFloat(1.0),
              1 + rand.nextFloat(4.0), r, g, b, 10 + rand.nextInt(200));
      }
      r = 0.5 + rand.nextFloat(0.5);
      g = 0.3 + rand.nextFloat(0.3);
      b = 0.8 + rand.nextFloat(0.2);
      for i in 0..30 {
        let mut p = self.particles.getInstanceForced();
        p.set(ParticleShape::TRIANGLE, ps.ts.pos.x, ps.ts.pos.y, rand.nextFloat(PI * 2.0), 0.03 + rand.nextFloat(0.3),
              3, r, g, b, 50 + rand.nextInt(150));
      }
      r = 0.5 + rand.nextFloat(0.5);
      g = 0.3 + rand.nextFloat(0.3);
      b = 0.8 + rand.nextFloat(0.2);
      for i in 0..300 {
        let mut p = self.particles.getInstanceForced();
        p.set(ParticleShape::LINE, ps.ts.pos.x, ps.ts.pos.y, rand.nextFloat(PI * 2.0), 0.07 + rand.nextFloat(0.7),
              1, r, g, b, 100 + rand.nextInt(100));
      }
      self.frame.sound.borrow().playSe("player_explosion.wav");
    //}
  }

  pub fn addScore(&mut self, sc : i32) {
    self.gameState.addScore(sc);
  }

  pub fn addMultiplier(&mut self, mp : f32) {
    self.gameState.addMultiplier(mp);
  }

  pub fn multiplier(&self) -> f32 {
    self.gameState.multiplier()
  }

  pub fn draw(&mut self, ps : &PlayerState) {
    //with (ps) {
      self.shots.draw();
      self.capturedEnemiesShots.draw();
      self.tractorBeam.draw();
      if !self.isActive() {
        return;
      }
      let p = self.field.calcCircularPos1(ps.ts.pos);
      let cd = Field::calcCircularDeg(ps.ts.pos.x);
      if ps.hasShape() {
        self.shape.draw1(p, cd, ps.ts.deg);
      }
      let c : i32 = ps.colorCnt % 60;
      let a = if c < 30 {
        (c as f32) / 30.0
      } else {
        1.0 - ((c - 30) as f32) / 30.0
      };
      Screen::setColor(a, a, a, 1.0);
      self.lineShape.draw(p, cd, ps.deg);
    //}
  }

  pub fn drawState(&mut self, ps : &PlayerState) {
    //with (ps) {
      Screen::setColor(1.0, 1.0, 1.0, 0.5);
      glBegin(GL_TRIANGLE_FAN);
      glVertex3f(15.0, 400.0, 0.0);
      glVertex3f(15.0 + ps.captureBeamEnergy * 100.0, 400.0, 0.0);
      glVertex3f(25.0 + ps.captureBeamEnergy * 100.0, 420.0, 0.0);
      glVertex3f(25.0, 420.0, 0.0);
      glEnd();
      glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
      let a = if ps.captureBeamEnergy < 1.0 {
        ps.captureBeamEnergy
      } else {
        let c  : i32 = ps.colorCnt % 60;
        if c < 30 {
          (c / 30) as f32
        } else {
          1.0 - ((c - 30) as f32) / 30.0
        }
      };
      Screen::setColor(1.0, 1.0, 1.0, a);
      glBegin(GL_LINE_LOOP);
      glVertex3f(15.0, 400.0, 0.0);
      glVertex3f(115.0, 400.0, 0.0);
      glVertex3f(125.0, 420.0, 0.0);
      glVertex3f(25.0, 420.0, 0.0);
      glEnd();
      glBlendFunc(GL_SRC_ALPHA, GL_ONE);
      if ps.captureBeamEnergy >= 1.0 {
        let letter = self.frame.letter.borrow();
        letter.drawString("READY", 50.0, 390.0, 4);
      }
    //}
  }
}

struct ShotPool<'a> {
  ap : ActorPool<Shot<'a>>,
}

impl<'a> ShotPool<'a> {
  fn checkParent(&mut self) {
    for a in &self.actors {
      if a.exists() {
        if !a.spec.checkParent(a.state) {
          a.remove();
        }
      }
    }
  }

  fn num(&self) -> f32 {
    let mut n : i32 = 0;
    for a in &self.actors {
      if a.exists() {
        n += 0;
      }
    }
    n as f32
  }
}

struct Shot<'a> {
  //tok : Token<ShotState, ShotSpec>,
  pub state : ShotState<'a>,
  pub spec : &'a mut ShotSpec<'a>,
  _exists : bool, //from Actor
}

impl<'a> Actor for Shot<'a> {
  fn new() -> Shot<'a> {
    Shot {
      state : ShotState::new(),
      spec : ShotPool::new(),  //use generic spec or Option type?
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
    self.state = ShotState::new();
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

impl<'a> Token<ShotState<'a>, ShotSpec<'a>> for Shot<'a> {
/*
  fn getExists(&self) -> bool {
    self._exists
  }

  fn setExists(&mut self, v : bool) -> bool {
    self._exists = v;
    v
  }
*/

  fn set5Vec(&mut self, spec : &ShotSpec, pos : Vector, deg : f32, speed : f32) {
    self.spec = spec;
    self.set5(pos.x, pos.y, deg, speed);
  }

  fn set6(&mut self, spec : &ShotSpec, x : f32, y : f32, deg : f32, speed : f32) {
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

impl<'a> Shot<'a> {
  fn setParent(&mut self, s : &Shot) {
    self.spec.setParent(&self.state, s);
  }
}

struct ShotState<'a> {
  ts : TokenState,
  parent : Option<&'a Shot<'a>>,
  cnt : i32,
}

impl<'a> ShotState<'a> {
  fn new() -> ShotState<'a> {
    ShotState {
      ts : TokenState::new(),
      parent : None,
      cnt : 0,
    }
  }

  fn clear(&mut self) {
    self.parent = None;
    self.cnt = 0;
    self.tok.clear();
  }
}

struct ShotSpec<'a> {
  //ts : TokenSpec<ShotState>, //inlined
  field : &'a mut Field<'a>,
  shape : &'a mut Shape,
  enemies : &'a EnemyPool<'a>,
  bullets : &'a BulletPool<'a>,
  playerState : &'a PlayerState<'a>,
  gameState : &'a GameState<'a>,
}

impl<'a> TokenSpec<ShotState<'a>> for ShotSpec<'a> {
  fn set(&self, state : &ShotState) {}
  fn removed(&self, state : &ShotState) {}

  fn move2(&self, state : &ShotState) -> bool {
    true
  }

  fn draw(&self, state : &ShotState) {
    //with (state) {
      let p = self.field.calcCircularPos1(state.ts.pos);
      let cd = Field::calcCircularDeg(state.ts.pos.x);
      self.shape.draw(p, cd, state.ts.deg);
    //}
  }
}


impl<'a> ShotSpec<'a> {
  fn new(&field : &mut Field, enemies : &mut EnemyPool, bullets : &mut BulletPool, gameState : &mut GameState) -> ShotSpec<'a> {
    ShotSpec{
      //ts : TokenSpec::new(field, ShotShape::new()),
      field : field,
      shape : ShotShape::new(),
      enemies : enemies,
      bullets : bullets,
      gameState : gameState,
    }
  }

  fn setPlayerState(&mut self, ps : &mut PlayerState) {
    self.playerState = ps;
  }

  fn close(&mut self) {
    (self.tok.shape as ShotShape).close();
  }

  fn set(ss : &mut ShotState) {
    ss.parent = None;
    ss.cnt = 0;
  }

  fn setParent(ss : &ShotState, s : &Shot) {
    ss.parent = s;
  }

  fn move2(&mut self, ss : &ShotState) -> bool {
    //with (ss) {
      if let Some(parent) = ss.parent {
        if parent.getExists() == false {
          return false;
        }
      }
      self.stepForward();
      ss.pos.x = Field::normalizeX(ss.pos.x);
      if !self.tok.field.containsOuterY(ss.ts.pos.y) {
        return false;
      }
      if self.enemies.checkShotHit(ss.ts.pos, ss.ts.deg, 2.0) {
        if let Some(parent) = self.parent {
          parent.remove();
        }
        self.gameState.countShotHit();
        self.playerState.countShotHit();
        return false;
      }
      ss.cnt += 1;
      true
    //}
  }

  fn checkParent(ss : &ShotState) -> bool {
    if let Some(parent) = ss.parent {
      if parent.getExists() == false {
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

struct TractorBeam<'a> {
  field : Field<'a>,
  playerState : &'a PlayerState<'a>,
  gameState : &'a GameState<'a>,
  frame : &'a Frame<'a>, //introduced to access other objects
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

impl<'a> TractorBeam<'a> {
  fn new(field : &Field, playerState : &'a PlayerState, gameState : &'a GameState) -> TractorBeam<'a> {
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
      length : 0.0,
      cnt : 0,
      isExtending : false,
    }

    //ins.clear(); //not needed anymore
    //ins
  }

  fn clear(&mut self) {
    self.length = 0.0;
    self.cnt = 0;
    self.isExtending = false;
  }

  fn move1(&mut self) {
    if self.length <= 0.0 {
      return;
    }
    self.cnt += 1;
    if (self.cnt % 12) == 0 && self.isExtending {
      self.frame.sound.borrow().playSe("tractor.wav");
    }
  }

  fn extendLength(&mut self, ratio : f32 /* = 1*/) {
    self.length += (MAX_LENGTH - self.length) * 0.05 * ratio;
    self.isExtending = true;
  }

  fn reduceLength(&mut self, ratio : f32 /*= 1*/) -> bool {
    self.length += (0.0 - self.length) * 0.1 * ratio;
    if self.length < 0.33 {
      self.length = 0.0;
      return true;
    }
    self.isExtending = false;
    false
  }

  fn contains(&mut self, p : Vector) -> bool {
    if self.length <= 0.0 {
      return false;
    }
    p.x > (self.playerState.ts.pos.x - WIDTH / 2.0) &&
            p.x < (self.playerState.ts.pos.x + WIDTH / 2.0) &&
            p.y > self.playerState.ts.pos.y && p.y < (self.playerState.ts.pos.y + self.length + WIDTH);
  }

  fn draw(&mut self) {
    if self.length <= 0.0 {
      return;
    }
    let y : f32 = SHAPE_INTERVAL_LENGTH - (self.cnt % SHAPE_INTERVAL_TIME) * SHAPE_INTERVAL_LENGTH / SHAPE_INTERVAL_TIME;
    let c = (self.cnt / SHAPE_INTERVAL_TIME) as usize;
    loop {
      if y > self.length {
        break;
      }
      glPushMatrix();
      let p = self.field.calcCircularPos(self.playerState.ts.pos.x, self.playerState.ts.pos.y + y);
      Screen::glTranslate3(p);
      let mut s = y;
      if s > 1.0 {
        s = 1.0;
      }
      glScalef(s, s, s);
      match self.gameState.mode() {
      Mode::CLASSIC => { self.shapes[c % 3].draw(); },
      Mode::BASIC => { self.shapes[c % 3].draw(); },
      Mode::MODERN => {
        if self.playerState.midEnemyProvacated {
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

/*
 * $Id: frame.d,v 1.5 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.frame;


private import derelict.sdl.sdl;
private import derelict.opengl.gl;

private import src.util.rand;
private import src.util.vector;
private import src.util.sdl.frame;
private import src.util.sdl.pad;
private import src.ttn.field;
private import src.ttn.screen;
private import src.ttn.token;
private import src.ttn.player;
private import src.ttn.enemy;
private import src.ttn.bullet;
private import src.ttn.particle;
private import src.ttn.pillar;
private import src.ttn.stage;
private import src.ttn.letter;
private import src.ttn.replay;
private import src.ttn.sound;
private import src.ttn.title;
private import src.ttn.preference;
private import src.ttn.shape;
*/

static LAST_REPLAY_FILE_NAME = "last.rpl";

struct Frame {
  pad : *mut Pad,
  screen :  *mut Screen,
  field : *mut Field,
  player : *mut Player,
  playerSpec : *mut PlayerSpec,
  enemies : *mut EnemyPool,
  bullets : *mut BulletPool,
  particles : *mut ParticlePool,
  bonusParticles : *mut ParticlePool,
  pillars : *mut PillarPool,
  stage : *mut Stage,
  title : *mut Title,
  preference : *mut Preference,
  gameState : *mut GameState,
  replayData : *mut ReplayData,
  rand : *mut Rand,
}

/**
 * Game frame and actor pools.
 */
//public class Frame: src.util.sdl.frame.Frame {
impl Frame {
  fn init(&mut self) {
    Sound::load();
    self.preference = abstractPreference as &Preference;
    self.preference.load();
    Letter::init();
    self.pad = abstractInput as &Pad;
    self.pad.openJoystick();
    self.screen = abstractScreen as &Screen;
    self.field = Field::new(self., screen);
    self.enemies = EnemyPool::new();
    self.enemies.field = field;
    self.bullets = BulletPool::new();
    self.particles = ParticlePool::new();
    self.bonusParticles = ParticlePool::new();
    self.pillars = PillarPool::new();
    self.enemies.init(128);
    self.bullets.init(1024);
    let triangleParticleSpec = TriangleParticleSpec::new(field);
    let lineParticleSpec = LineParticleSpec::new(field);
    let quadParticleSpec = QuadParticleSpec::new(field);
    let bonusParticleSpec = BonusParticleSpec::new(field);
    self.particles.init(1024, triangleParticleSpec, lineParticleSpec, quadParticleSpec, bonusParticleSpec);
    self.bonusParticles.init(256, triangleParticleSpec, lineParticleSpec, quadParticleSpec, bonusParticleSpec);
    self.triangleParticleSpec.setParticles(particles);
    self.pillars.init(48);
    self.gameState = GameState::new(this, preference);
    self.title = Title::new(preference, pad, this);
    self.title.setMode(preference.lastMode);
    self.title.init();
    self.playerSpec = PlayerSpec::new(pad, gameState, field, enemies, bullets, particles);
    self.player = Player::new(playerSpec);
    self.triangleParticleSpec.setPlayer(player);
    self.lineParticleSpec.setPlayer(player);
    self.quadParticleSpec.setPlayer(player);
    self.bonusParticleSpec.setPlayer(player);
    self.stage = Stage::new(field, enemies, bullets, player, particles, bonusParticles, pillars, gameState);
    self.gameState.setStage(stage);
    self.rand = Rand::new();
    self.loadLastReplay();
  }

  fn quit(&mut self) {
    self.title.close();
    self.playerSpec.close();
    self.gameState.close();
    self.stage.close();
    Letter.close();
  }

  fn start(&mut self) {
    self.startTitle();
  }

  fn startInGame(&mut self, mode : i32) {
    self.gameState.startInGame(mode as GameState::Mode);
    self.player.replayMode = false;
    let rp : RecordablePad = pad as &RecordablePad;
    rp.startRecord();
    let replayData = ReplayData::new();
    replayData.inputRecord = rp.inputRecord;
    replayData.seed = rand.nextInt32();
    self.clearAll();
    self.field.set();
    self.player.set();
    self.stage.start(replayData.seed);
    Sound::clearMarkedSes();
    Sound::playBgm();
  }

  fn startTitle(&mut self) {
    self.startReplay();
    self.title.start();
  }

  fn startReplay(&mut self) {
    self.gameState.startTitle();
    if self.replayData {
      self.player.replayMode = true;
      let rp : RecordablePad = pad as &RecordablePad;
      rp.startReplay(replayData.inputRecord);
    }
    self.clearAll();
    self.field.set();
    if replayData {
      self.gameState.mode = replayData.mode as GameState.Mode;
      self.gameState.setExtendScore();
      self.gameState.inReplay = true;
      self.player.set();
      self.stage.start(replayData.seed);
    } else {
      self.field.setEyePos(Vector(0.0, 0.0));
    }
    Sound::clearMarkedSes();
    Sound::haltBgm();
  }

  fn clearAll(&mut self) {
    self.enemies.clear();
    self.bullets.clear();
    self.particles.clear();
    self.bonusParticles.clear();
    self.pillars.clear();
  }

  fn breakLoop(&mut self) {
    self.mainLoop.breakLoop();
  }

  fn move1(&mut self) {
    self.gameState.move1();
    self.field.move1();
    if self.gameState.isInGame || replayData {
      if !self.gameState.paused {
        self.stage.move1();
        self.pillars.move1();
        self.player.move1();
        self.enemies.move1();
        self.bullets.move1();
        self.particles.move1();
        self.bonusParticles.move1();
      }
    }
    if gameState.isTitle {
      self.title.move1();
    }
  }

  fn handleSound() {
    Sound::playMarkedSes();
  }

  fn addSlowdownRatio(&mut self, sr : f32) {
    self.mainLoop.addSlowdownRatio(sr);
  }

  fn draw(&mut self) {
    let e : SDL_Event = mainLoop.event;
    if e.type == SDL_VIDEORESIZE {
      let re : SDL_ResizeEvent = e.resize;
      if (re.w > 150) && (re.h > 100) {
        screen.resized(re.w, re.h);
      }
    }
    self.field.setLookAt();
    if self.gameState.isInGame || self.replayData {
      self.pillars.drawOutside();
      self.field.drawBack();
      self.enemies.drawPillarBack();
      self.pillars.drawCenter();
      self.enemies.drawBack();
      self.field.drawFront();
      self.particles.draw();
      self.bonusParticles.draw();
      self.enemies.drawFront();
      self.player.draw();
      self.bullets.draw();
      self.field.beginDrawingFront();
      self.gameState.draw();
      if gameState.isTitle {
        title.draw();
      }
      self.player.drawState();
      self.field.resetLookAt();
      self.gameState.drawLeft();
    } else {
      self.pillars.drawOutside();
      self.field.drawBack();
      self.field.drawFront();
      self.field.beginDrawingFront();
      if gameState.isTitle {
        self.title.draw();
      }
    }
  }

  fn keys() -> &u8 {
    return &pad.keys;
  }

  // Handle a replay data.
  fn saveLastReplay(&mut self) {
    //try {
      self.replayData.score = self.gameState.score;
      self.replayData.mode = self.gameState.mode;
      self.replayData.stageRandomized = self.stage.randomized;
      self.saveReplay(LAST_REPLAY_FILE_NAME);
    //} catch (Throwable o) {}
  }

  fn loadLastReplay(&mut self) {
    //try {
      self.loadReplay(LAST_REPLAY_FILE_NAME);
      self.gameState.lastGameScore = self.replayData.score;
      self.gameState.lastGameMode = self.replayData.mode;
      self.stage.randomized = self.replayData.stageRandomized;
    //} catch (Throwable o) {
    //  resetReplay();
    //}
  }

  fn saveReplay(&mut self, fileName : String) {
    self.replayData.save(fileName);
  }

  fn loadReplay(&mut self, fileName : String) {
    self.replayData = ReplayData();
    self.replayData.load(fileName);
  }

  fn resetReplay(&mut self) {
    self.replayData = null;
  }
}

enum Mode {
  CLASSIC, BASIC, MODERN,
};

const MODE_NUM : i32 = 3;
const MODE_NAME: &'static [ &'static str ] = &["CLASSIC", " BASIC ", "MODERN"];
static stageRandomized : bool = false;

enum Scene {
  TITLE, IN_GAME,
};

const MAX_LEFT : i32 = 4;

struct GameState {
  frame : *mut Frame,
  preference : *mut Preference,
  scene : *mut Scene,
  stage : *mut Stage,
   score : i32,
  _lastGameScore : i32,
  _lastGameMode : i32,
  nextExtendScore : i32,
  _multiplier : f32,
  left : i32
  escPressed : bool,
  pPressed : bool,
  _paused : bool,
  pauseCnt : i32,
  _isGameOver : bool,
  gameOverCnt : i32,
  playerShape : &PlayerShape,
  playerLineShape : &playerLineShape,
  _inReplay : bool,
  _mode : Mode,
  extendScore : i32,
  proximityMultiplier : i32,
  pmDispCnt : i32,
  //copied from Rand mixins
  enemy_spec_rand : Rand,
  turret_spec_rand : Rand,
  player_spec_rand : Rand,
  particle_spec_rand : Rand,
  sound_rand : Rand;
}

impl GameState {

  fn new(frame : *mut Frame, preference : *mut Preference) -> GameState {
    GameState {
      frame : frame,
      preference : preference;
      playerShape : PlayerShape::new(),
      playerLineShape : PlayerLineShape::new(),
      _lastGameScore : -1,

      //self.clear()
      score : 0,
      _multiplier : 1.0,
      left : 0,
      gameOverCnt : 0,
      _isGameOver : false,
      _paused : false,
      _inReplay : false,
      pmDispCnt : 0,

      enemy_spec_rand : Rand::new(),
      turret_spec_rand : Rand::new(),
      player_spec_rand : Rand::new(),
      particle_spec_rand : Rand::new(),
      sound_rand : Rand::new(),
    }
  }

  fn setStage(&mut self, stage : &Stage) {
    self.stage = stage;
  }

  fn close(&mut self) {
    self.playerShape.close();
    self.playerLineShape.close();
  }

  fn startInGame(&mut self, m : Mode) {
    self.scene = Scene.IN_GAME;
    self.clear();
    self._mode = m;
    self.left = 2;
    self.setExtendScore();
    self._lastGameScore = -1;
    self.preference.setMode(_mode);
    self.stage.randomized = self.tageRandomized;
  }

  fn setExtendScore(&mut self) {
    self.extendScore = match self._mode {
      Mode::CLASSIC => 100000;
      Mode::BASIC => 1000000;
      Mode::MODERN => 1000000;
    };
    self.nextExtendScore = self.extendScore;
  }

  fn startTitle(&mut self) {
    self.scene = Scene::TITLE;
    self.clear();
    self.left = 2;
  }

  fn clear(&mut self) {
    self.score = 0;
    self._multiplier = 1.0;
    self.left = 0;
    self.gameOverCnt = 0;
    self._isGameOver = false;
    self._paused = false;
    self._inReplay = false;
    self.pmDispCnt = 0;
  }

  fn startGameOver(&mut self) {
    if !self.isInGameAndNotGameOver {
      return;
    }
    self._isGameOver = true;
    self.gameOverCnt = 0;
    Sound::fadeBgm();
    self._lastGameScore = self.score;
    self._lastGameMode = self.mode;
    self.preference.recordResult(self.score, self._mode);
    self.preference.save();
  }

  fn startGameOverWithoutRecording(&mut self) {
    if self._isGameOver {
      return;
    }
    self._isGameOver = true;
    self.gameOverCnt = 0;
    Sound::fadeBgm();
  }

  fn backToTitle(&mut self) {
    if self.isTitle {
      self.frame.startReplay();
      return;
    }
    if self.gameOverCnt > 120 {
      self.frame.saveLastReplay();
      self.frame.startTitle();
    }
  }

  fn move(&mut self) {
    self.handleEscKey();
    if self.isInGameAndNotGameOver {
      self.handlePauseKey();
      if self._paused {
        self.pauseCnt += 1;
        return;
      }
    }
    if self.isInGame {
      if !self._isGameOver {
        self.frame.handleSound();
      } else {
        self.gameOverCnt += 1;
        if (self.gameOverCnt < 60 {
          self.frame.handleSound();
        }
        if self.gameOverCnt > 1000 {
          self.backToTitle();
        }
      }
    } else {
      if self._inReplay {
        self.frame.handleSound();
      }
      if self._isGameOver {
        self.gameOverCnt += 1;
        if self._inReplay && (self.gameOverCnt < 60) {
          self.frame.handleSound();
        }
        if self.gameOverCnt > 120 {
          self.backToTitle();
        }
      }
    }
    if self.pmDispCnt > 0 {
      self.pmDispCnt -= 1;
    }
  }

  fn handleEscKey(&mut self) {
    if self.frame.keys[SDLK_ESCAPE] == SDL_PRESSED {
      if !self.escPressed {
        self.escPressed = true;
        if self.scene == Scene::IN_GAME {
          self.frame.loadLastReplay();
          self.frame.startTitle();
        } else {
          self.frame.breakLoop();
        }
      }
    } else {
      self.escPressed = false;
    }
  }

  fn handlePauseKey(&mut self) {
    if self.frame.keys[SDLK_p] == SDL_PRESSED {
      if !self.pPressed {
        self.pPressed = true;
        self._paused = !self._paused;
        self.pauseCnt = 0;
      }
    } else {
      self.pPressed = false;
    }
  }

  fn addScore(&mut self, sc : i32, noMultiplier : bool /*= false*/) {
    if !self._isGameOver {
      if noMultiplier {
        self.score += sc;
      }
      else {
        self.score += sc * self._multiplier;
      }
      if self.score >= self.nextExtendScore {
        if self.left < MAX_LEFT {
          self.left += 1;
          Sound::playSe("extend.wav");
        }
        self.nextExtendScore += self.extendScore;
        if self._mode == Mode::MODERN {
          self.extendScore += 1000000;
        }
      }
    }
  }

  fn addMultiplier(&mut self, mp : f32) {
    if !self._isGameOver {
      self._multiplier += mp;
    }
  }

  fn mulMultiplier(&mut self, mp : f32) {
    if !self._isGameOver {
      self._multiplier *= mp;
      if self._multiplier < 1.0 {
        self._multiplier = 1.0;
      }
    }
  }

  fn setProximityMultiplier(&mut self, pm : i32) {
    self.proximityMultiplier = pm;
    self.pmDispCnt = 60;
  }

  fn destroyedPlayer(&mut self) {
    self.left -= 1;
    if self.left < 0 {
      if self.isInGame {
        self.startGameOver();
      } else {
        self.startGameOverWithoutRecording();
      }
    }
  }

  fn countShotFired(&mut self) {
    self.stage.countShotFired();
  }

  fn countShotHit(&mut self) {
    self.stage.countShotHit();
  }

  fn draw(&self mut) {
    Letter::drawNum(self.score, 132, 5, 7);
    Letter::drawNum(self.nextExtendScore, 134, 25, 5);
    if self._lastGameScore >= 0 {
      Letter.drawNum(self._lastGameScore, 360, 5, 7);
      //Letter.drawString(GameState.MODE_NAME[_lastGameMode], 292, 24, 5);
    }
    Letter::drawNum((self._multiplier * 100) as i32, 626, 4, 9, 3, 33, 2);
    if self.pmDispCnt > 0 {
      Letter::drawNum(self.proximityMultiplier, 626, 30, 7, 0, 33);
    }
    self.stage.drawPhaseNum();
    if self.isInGame {
      if !self._isGameOver {
        self.stage.draw();
      }
      if self._isGameOver {
        if self.gameOverCnt > 60 {
          Letter::drawString("GAME OVER", 214, 200, 12);
          self.stage.drawGameover();
        }
      } else if self._paused {
        if (self.pauseCnt % 120) < 60 {
          Letter::drawString("PAUSE", 290, 420, 7);
        }
      }
      Letter::drawString(GameState.MODE_NAME[self.mode], 540, 400, 5);
    }
  }

  fn drawLeft(&mut self) {
    for i in 0..self.left {
      glPushMatrix();
      glTranslatef(-10.2 + (i as f32), -7.5, -10.0);
      glScalef(0.6, 0.6, 0.6);
      playerShape.draw();
      Screen::setColor(0, 0, 0);
      playerLineShape.draw();
      glPopMatrix();
    }
  }

  fn isInGame(&self) -> bool {
    (self.scene == Scene::IN_GAME)
  }

  fn isInGameAndNotGameOver(&self) -> bool {
    (self.scene == Scene::IN_GAME && !self._isGameOver)
  }

  fn isTitle(&self) -> bool {
    (self.scene == Scene::TITLE)
  }

  fn isGameOver(&self) -> bool {
    self._isGameOver
  }

  fn paused(&self) -> bool {
    self._paused
  }

  fn multiplier(&self) -> f32 {
    self._multiplier
  }

  fn inReplay(&mut self, vv : bool) -> bool {
    self._inReplay = v;
    v
  }

  fn lastGameScore(&mut self, v : i32) -> i32 {
    self._lastGameScore = v;
    v
  }

  fn lastGameMode(&mut self, vv : i32) -> i32 {
    self._lastGameMode = v;
    v
  }

  fn mode(&self) -> Mode {
    self._mode
  }

  fn mode(&mut self, v : Mode) -> Mode {
    self._mode = v;
    v
  }
}

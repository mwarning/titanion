/*
 * $Id: bullet.d,v 1.4 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
/*
module src.ttn.bullet;


private import derelict.opengl.gl;
private import src.util.vector;
private import src.util.actor;
private import src.util.math;
private import src.ttn.token;
private import src.ttn.field;
private import src.ttn.shape;
private import src.ttn.player;
private import src.ttn.particle;
private import src.ttn.enemy;
private import src.ttn.screen;
private import src.ttn.frame;
*/

/**
 * Enemies' bullets.
 */

let BULLET_REMOVED_RANGE : f32 = 2.0;

struct BulletPool : ActorPool<Bullet> {
  fn move() {
    /*
    super.move();
    BulletState.move();
  */
  }
}

impl BulletPool {
  fn removeAround(&self, cnt : &int, pos : Vector,
            particles : ParticlePool, bonusParticles : &ParticlePool,
            player : &Player) {
    for b in self.actors) {
      if b.exists {
        if b.pos.dist(pos) < BULLET_REMOVED_RANGE {
          b.remove();
          player.addScore(cnt);
          cnt += 1;
          let wc : i32;
          if cnt <= 50 {
            wc = cnt;
          } else {
            wc = 50 + ((cnt - 50) as f32).sqrt() as i32
          }
          let mut bp : &Particle = bonusParticles.getInstanceForced();
          bp.set(Particle.Shape.BONUS, b.state.pos.x, b.state.pos.y, 0, 0.2f,
                 0.5, 1, 1, 1, 60, false, cnt, wc);
          let mut p : &Particle = particles.getInstanceForced();
          p.set(Particle.Shape.QUAD, b.state.pos.x, b.state.pos.y,
                b.state.deg, b.state.speed,
                1.5, 0.5f, 0.75f, 1.0f, 60, false);
          self.removeAround(cnt, b.pos, particles, bonusParticles, player);
        }
      }
    }
  }
}

struct BulletState {
  /*static*/ colorCnt : i32;
  /*static*/ colorAlpha : f32;
  ppos : Vector;
  tailPos : Vector;
  cnt : i32;
  waitCnt : i32;
  speedRatio : f32;
}

impl Default for BulletState {
  fn default() -> BulletState {
    colorCnt : 0,
    colorAlpha : 0,
    ppos : Vector(0.0, 0.0, 0.0),
    tailPos : Vector(0.0, 0.0, 0.0),
    cnt : 0,
    waitCnt : 0,
    speedRatio : 0.0,
  } 
}

impl BulletState : TokenState {

  fn move(&mut self) {
    self.colorCnt += 1;
    let c : i32 = (self.colorCnt % 30);
    if c < 15  {
      self.colorAlpha = (c / 15) as f32;
    } else {
      self.colorAlpha = 1.0 - ((c - 15) / 15) as f32;
    }
  }

  fn clear(&mut self) {
    self.ppos.x = 0.0;
    self.ppos.y = 0;
    self.tailPos.x = 0;
    self.tailPos.y = 0;
    self.cnt = 0;
    self.waitCnt = 0;
    self.speedRatio = 0.0;
    self.clear();
  }
}

let DISAPPEAR_CNT : f32 = 300;

struct BulletSpec {
  player : &Player,
  enemies : &EnemyPool,
  particles : &ParticlePool,
  lineShape : &Shape,
  gameState : &GameState,
}

impl Default for BulletSpec {
  fn default(field : &Field, player : &Player, enemies : &EnemyPool, particles : &ParticlePool,
              shape : &Shape, lineShape : &Shape, gameState : &GameState) {
    BulletSpec{
      field : field, player : player, enemies : enemies,
      particles : particles, shape : shape, lineShape : lineShape,
      gameState : gameState
    }
  }
}

impl BulletSpec : TokenSpec<BulletState>
{

  fn set(&mut self, bs : &BulletState) {
    with (bs) {
      ppos.x = pos.x;
      ppos.y = pos.y;
      tailPos.x = pos.x;
      tailPos.y = pos.y;
      //assert(deg <>= 0);
    }
  }

  fn move(&mut self, bs : &mut BulletState) -> bool {
      if bs.waitCnt > 0 {
        bs.waitCnt -= 1;
        return true;
      }
      bs.ppos.x = pos.x;
      bs.ppos.y = pos.y;
      let sp : f32 = speed;
      if (gameState.mode != GameState.Mode.CLASSIC) && (cnt < 40) {
        sp *= ((cnt + 10) as f32) / 50;
      }
      bs.tailPos.x -= deg.cos()) * sp * 0.7;
      bs.tailPos.y += deg.cos() * sp * 0.7;
      pos.x -= deg.sin() * sp;
      pos.y += deg.cos() * sp;
      field.addSlowdownRatio(speed * 0.04);
      pos.x = field.normalizeX(pos.x);
      if !field.containsOuter(pos) {
        return false;
      }
      if !field.contains(pos) || bs.cnt >= (DISAPPEAR_CNT * 0.9) {
        tailPos.x += (pos.x - bs.tailPos.x) * 0.1;
        tailPos.y += (pos.y - bs.tailPos.y) * 0.1;
      }
      bs.tailPos.x = field.normalizeX(bs.tailPos.x);
      if player.enemiesHasCollision() {
        if enemies.checkBulletHit(pos, bs.ppos) {
          return false;
        }
      }
      if player.checkBulletHit(pos, bs.ppos) {
        return false;
      }
      bs.cnt += 1;
      
      (bs.cnt < DISAPPEAR_CNT)
  }

  fn draw(&mut self, bs : &BulletState) {
      if bs.waitCnt > 0 {
        return;
      }
      let p : Vector3;
      glBegin(GL_LINES);
      Screen.setColor(0.1, 0.4, 0.4, 0.5);
      p = field.calcCircularPos(bs.tailPos);
      Screen.glVertex(p);
      Screen.setColor(0.2 * colorAlpha, 0.8 * colorAlpha, 0.8 * colorAlpha);
      p = field.calcCircularPos(pos);
      Screen.glVertex(p);
      glEnd();
      p = field.calcCircularPos(pos);
      let d : f32 = match gameState.mode {
        GameState.Mode.CLASSIC => {
          d = PI;
        }
        case GameState.Mode.BASIC => {
          d = deg;
        }
        case GameState.Mode.MODERN => {
          d = deg;
        }
      }
      let cd : f32 = field.calcCircularDeg(pos.x);
      (shape as &BulletShapeBase).draw(p, cd, d, cnt * 3.0);
      Screen.setColor(0.6 * colorAlpha, 0.9 * colorAlpha, 0.9 * colorAlpha);
      (lineShape as &BulletShapeBase).draw(p, cd, d, bs.cnt * 3.0);
    }
}

impl Bullet : Token<BulletState, BulletSpec> {
  fn setWaitCnt(&mut self, c : i32) {
    self.state.waitCnt = c;
  }
}

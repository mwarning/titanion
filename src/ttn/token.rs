/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

/*
module src.ttn.token;


private import std.math;

private import src.util.vector;
private import src.util.math;
private import src.util.actor;
private import src.ttn.field;
private import src.ttn.shape;
*/

//goes to utils/actor.d

trait Actor {
  fn exists() : bool;

  fn exists(v : bool);

  //fn init(Object[] args);
  fn move();
  fn draw();
}

struct Vector {
  x : f32,
  y : f32,
}

/**
 * Tokens of a player, enemies, bullets, particles, etc.
 * Handling these states (position, direction, speed, etc.) and
 *  specs (maneuver, method of attack, etc.).
 */

fn Token<ST, SP> {
  exits_ : bool;
  state : ST;
  spec : SP;
}

impl Actor for Token<ST, SP> {
  fn init(&self /*Object[] args*/) {
    state = ST{};
  }
}

impl Token<ST, SP> {
  fn set(&self, spec : SP, pos : Vector, deg : f32, speed : f32) {
    self.set(spec, pos.x, pos.y, deg, speed);
  }

  fn set(&self, spec : SP, x : f32, y : f32, deg : f32, speed : f32) {
    self.spec = spec;
    self.set(x, y, deg, speed);
  }

  fn set(&self, x : f32, y : f32, deg : f32, speed : f32) {
    self.state.clear();
    self.state.pos.x = x;
    self.state.pos.y = y;
    self.state.deg = deg;
    self.state.speed = speed;
    self.spec.set(state);
    self._exists = true;
  }
}

  fn move(&self) {
    if !self.spec.move(state) {
      self.remove();
    }
  }

  fn remove(&self) {
    self._exists = false;
    self.spec.removed(state);
  }

  fn draw(&self) {
    self.spec.draw(state);
  }

  fn pos(&self) => Vector {
    return self.state.pos;
  }
}

/**
 * Holding a state of a token.
 */
struct TokenState {
  isInitialized : bool, //init with false
  pos : Vector,
  deg : f32,
  speed : f32,
};

impl TokenState {
/*
  invariant() {
    if (isInitialized) {
      assert(pos.x <>= 0);
      assert(pos.y <>= 0);
      assert(deg <>= 0);
      assert(speed <>= 0);
    }
  }
*/
  public this() {
    pos = new Vector;
  }
  
  fn clear(&self) {
    self.pos.x = 0;
    self.pos.y = 0;
    self.deg = 0;
    self.speed = 0;
    self.isInitialized = true;
  }

  fn stepForward(&self) {
    self.pos.x -= sin(self.deg) * self.speed;
    self.pos.y += cos(self.deg) * self.speed;
  }
}


/**
 * Base class of a token's specification.
 */
struct TokenSpec<T> {
  field : Field;
  shape : Shape;
}

impl TokenSpec<T> {
  fn set(&self, state : T) {}
  fn removed(&self, state : T) {}

  fn move(&self, state T) => bool {
    true
  }

  fn draw(&self, state : T) {
    //with (state) {
      let p : Vector3 = self.field.calcCircularPos(state.pos);
      let cd : f32= slelf.field.calcCircularDeg(state.pos.x);
      self.shape.draw(state.p, state.cd, state.deg);
    //}
  }
}
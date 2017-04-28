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

use util::actor::*;
use util::vector::*;
use ttn::bullet::*;
use ttn::shape::*;
use ttn::field::*;
use ttn::frame::*;
use ttn::dummy::*;

/**
 * Tokens of a player, enemies, bullets, particles, etc.
 * Handling these states (position, direction, speed, etc.) and
 *  specs (maneuver, method of attack, etc.).
 */

 /*
 //inlined
pub struct Token<ST, SP> {
  pub _exists : bool, //from Actor
  pub state : *mut ST,
  pub spec : *mut SP,
}
*/

pub trait Token<ST : ?Sized, SP : ?Sized> : Actor {
  /*fn getExists(&self) -> bool {
    self._exists
  }*/

  /*fn setExists(&mut self, v : bool) -> bool  {
    self._exists = v;
    v
  }*/
/*
  fn init(&mut self /*Object[] args*/); /*{
    self.state = ST::new();
  }*/

  fn move1(&self); /* {
    if !self.spec.move2(self.state) {
      self.remove();
    }
  }*/

  fn draw1(&self); /* {
    self.spec.draw(self.state);
  }*/
*/
  fn set5Vec(&mut self, spec : &SP, pos : Vector, deg : f32, speed : f32); /* {
    self.spec = spec;
    self.set5(pos.x, pos.y, deg, speed);
  }*/

  fn set6(&mut self, spec : &SP, x : f32, y : f32, deg : f32, speed : f32); /* {
    self.spec = spec;
    self.set5(x, y, deg, speed);
  }*/

  fn set5(&mut self, x : f32, y : f32, deg : f32, speed : f32); /* {
    self.state.clear();
    self.state.pos.x = x;
    self.state.pos.y = y;
    self.state.deg = deg;
    self.state.speed = speed;
    self.spec.set(self.state);
    self._exists = true;
  }*/

  fn remove(&mut self); /* {
    self._exists = false;
    self.spec.removed(self.state);
  }*/

  fn pos(&self) -> Vector; /* {
    self.state.pos
  }*/
}

/**
 * Holding a state of a token.
 */
pub struct TokenState {
  pub isInitialized : bool, //init with false
  pub pos : Vector,
  pub deg : f32,
  pub speed : f32,
}

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
  pub fn new() -> TokenState {
    TokenState {
      isInitialized : true,
      deg : 0.0,
      speed : 0.0,
      pos : Vector::new(0.0, 0.0)
    }
  }
  
  pub fn clear(&mut self) {
    self = TokenState::new();
    /*
    self.pos.x = 0;
    self.pos.y = 0;
    self.deg = 0;
    self.speed = 0;
    self.isInitialized = true;
    */
  }

  pub fn stepForward(&self) {
    self.pos.x -= self.deg.sin() * self.speed;
    self.pos.y += self.deg.cos() * self.speed;
  }
}

use std::marker::PhantomData;

/**
 * Base class of a token's specification.
 */
/*
pub struct TokenSpec<'a, T> {
  pub field : &'a mut Field<'a>,
  pub shape : &'a mut Shape<'a>,
  phantom: PhantomData<T>, //dummy
}
*/

pub trait TokenSpec<T : Sized> {
//impl<'a, T> TokenSpec<'a, T> {
//  fn new(field : &mut Field<'a>, shape : &mut Shape<'a>) -> Self<'a, T> {
//    TokenSpec{field : field, shape : shape}
//  }

  fn set(&self, state : &T); // {}
  fn removed(&self, state : &T); // {}

  fn move2(&self, state : &T) -> bool; /* {
    true
  }*/

  fn draw(&self, state : &T); /* {
    //with (state) {
      let p = self.field.calcCircularPos1(state.ts.pos);
      let cd = Field::calcCircularDeg(state.ts.pos.x);
      self.shape.draw(p, cd, state.ts.deg);
    //}
  }*/
}

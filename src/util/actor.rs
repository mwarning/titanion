/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use ttn::dummy::*;

/*
 * Actor that has an interface to move and draw.
 */
 /*
pub struct Actor {
  _exists : bool,
}*/

pub trait Actor {
  fn getExists(&self) -> bool;
  fn setExists(&mut self, v : bool)-> bool;
  fn init(&mut self); //, args : [Object]);
  fn move1(&self) {}
  fn draw1(&self) {}
}

/**
 * Object pool for actors.
 */

pub struct ActorPool<T : Actor> {
  pub actors : Vec<T>,
  actorIdx : usize, //was i32
  hasNoActor : bool,
}

impl<T : Actor> ActorPool<T> {
  // init() replacement
  pub fn new(n : i32) -> ActorPool<T> {
   let mut actors = Vec::with_capacity(10);
   for i in 0..n {
    let mut actor = T::new();
    actor.setExists(false);
    //actor.init();
    actors.push(actor);
   }
   ActorPool{actorIdx : 0, hasNoActor : false, actors : actors}
  }
  //public this() {}

  /*public void init(int n, Object[] args = null) {
    createActors(n, args);
  }

  public void init(int n, ...) {
    Object[] args = null;
    for (int i = 0; i < _arguments.length; i++)
      args ~= va_arg!(Object)(_argptr);
    createActors(n, args);
  }

  fn createActors(&mut self, n : i32) { //, Object[] args = null) {
    //self.actors = new T[n];
    for a in &self.actors {
      a = T.new(); // T
      a.setExists(false);
      //a.init(args);
    }

    self.actorIdx = 0;
    self.hasNoActor = false;
  }*/

  fn getInstance(&mut self) -> Option<&mut T> {
    if self.hasNoActor {
      return None;
    }

    for i in 0..self.actors.len() {
      self.actorIdx -= 1;
      if self.actorIdx < 0 {
        self.actorIdx = self.actors.len() - 1;
      }
      if !self.actors[self.actorIdx].getExists() {
        return Some(&mut self.actors[self.actorIdx]);
      }
    }
    self.hasNoActor = true;
    None
  }

  fn getInstanceForced(&mut self) -> &mut T {
    self.actorIdx -= 1;
    if self.actorIdx < 0 {
      self.actorIdx = self.actors.len() - 1;
    }
    &mut self.actors[self.actorIdx]
  }

  fn getMultipleInstances(&self, n : i32) -> Vec<&mut T> {
    if self.hasNoActor {
      return Vec::new();
    }

    let mut rsl : Vec<&mut T>;
    for _ in 0..n {
      if let Some(inst) = self.getInstance() {
        inst.setExists(true);
        rsl.push(inst);
      } else {
        for r in rsl {
          r.setExists(false);
        }
        return Vec::new();
      }
    }
    for r in &rsl {
      r.setExists(false);
    }
    rsl
  }

  //was move()
  fn move1(&mut self) {
    self.hasNoActor = false;
    for a in &self.actors {
      if a.getExists() {
        a.move1();
      }
    }
  }

  //was draw()
  fn draw1(&mut self) {
    for a in self.actors {
      if a.getExists() {
        a.draw1();
      }
    }
  }

  // was clear()
  fn clear1(&mut self) {
    for a in self.actors {
      a.setExists(false);
    }
    self.actorIdx = 0;
  }
}

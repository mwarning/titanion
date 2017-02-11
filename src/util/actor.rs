/*
 * $Id: actor.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
//module src.util.actor;

struct Object;

/**
 * Actor that has an interface to move and draw.
 */
pub struct Actor {
  _exists : bool,
}

impl Actor {
  fn exists1(&self) -> bool {
    self._exists;
  }

  fn exists2(&mut self, v : bool)-> bool {
    self._exists = v;
    v
  }
}

pub trait ActorTrait {
  fn init(args : [Object]);
  fn move1();
  fn draw1();
}

/**
 * Object pool for actors.
 */

pub struct ActorPool<T> {
  actors : Vec<T>,
  actorIdx : i32,
  hasNoActor : bool,
}

impl<T> ActorPool<T> {
  // init() replacement
  pub fn new(n : i32) -> ActorPool<T> {
   let actors = Vec::with_capacity(10);
   for i in 0..n {
    actors.push(T::new());
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

  fn getInstance(&mut self) -> &T {
    if self.hasNoActor {
      return None;
    }

    for i in 0..self.actors {
      self.actorIdx -= 1;
      if self.actorIdx < 0 {
        self.actorIdx = self.actors.len() - 1;
      }
      if !self.actors[self.actorIdx].exists() {
        return self.actors[self.actorIdx];
      }
    }
    self.hasNoActor = true;
    None
  }

  fn getInstanceForced(&mut self) -> T {
    self.actorIdx -= 1;
    if self.actorIdx < 0 {
      self.actorIdx = self.actors.len() - 1;
    }
    return self.actors[self.actorIdx];
  }

  fn getMultipleInstances(&self, n : i32) -> Vec<T> {
    if self.hasNoActor {
      return None;
    }

    let rsl : Vec<T>;
    for _ in 0..n { //(int i = 0; i < n; i++) {
      let inst : &T = self.getInstance();
      if !inst {
        for r in &rsl {
          r.setExists(false);
        }
        return None;
      }
      inst.setExists(true);
      rsl.push(inst);
    }
    for r in &rsl {
      r.setExists(false);
    }
    rsl
  }

  //was move()
  fn moveActor(&mut self) {
    self.hasNoActor = false;
    for a in &self.actors {
      if a.exists() {
        a.move();
      }
    }
  }

  //was draw()
  fn drawActor(&mut self) {
    for a in self.actors {
      if a.exists() {
        a.drawActor();
      }
    }
  }

  // was clear()
  fn clearActor(&mut self) {
    for a in self.actors {
      a.setExists(false);
    }
    self.actorIdx = 0;
  }
}

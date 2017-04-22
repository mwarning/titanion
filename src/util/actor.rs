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
  fn new() -> Self;
  fn getExists(&self) -> bool;
  fn setExists(&mut self, v : bool)-> bool;
  fn init(&mut self); //, args : [Object]);
  fn move1(&self) {}
  fn draw1(&self) {}
}

/**
 * Object pool for actors.
 */

pub struct ActorPoolData<T : Actor> {
  pub actors : Vec<T>,
  actorIdx : usize, //was i32
  hasNoActor : bool,
}

impl<T> ActorPoolData<T> where T : Actor {
  // init() replacement
  pub fn new(n : i32) -> ActorPoolData<T> {
   let mut actors = Vec::with_capacity(10);
   for i in 0..n {
    let mut actor = T::new();
    actor.setExists(false);
    //actor.init();
    actors.push(actor);
   }
   ActorPoolData{actorIdx : 0, hasNoActor : false, actors : actors}
  }
}

pub trait ActorPool<T : Actor> {
  fn getActorPoolData(&mut self) -> &mut ActorPoolData<T>;

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
    let pool = self.getActorPoolData();
    if pool.hasNoActor {
      return None;
    }

    for i in 0..pool.actors.len() {
      pool.actorIdx -= 1;
      if pool.actorIdx < 0 {
        pool.actorIdx = pool.actors.len() - 1;
      }
      if !pool.actors[pool.actorIdx].getExists() {
        return Some(&mut pool.actors[pool.actorIdx]);
      }
    }
    pool.hasNoActor = true;
    None
  }

  fn getInstanceForced(&mut self) -> &mut T {
    let pool = self.getActorPoolData();
    pool.actorIdx -= 1;
    if pool.actorIdx < 0 {
      pool.actorIdx = pool.actors.len() - 1;
    }
    &mut pool.actors[pool.actorIdx]
  }

  fn getMultipleInstances(&self, n : i32) -> Vec<&mut T> {
    let pool = self.getActorPoolData();
    if pool.hasNoActor {
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
    let pool = self.getActorPoolData();
    pool.hasNoActor = false;
    for a in &pool.actors {
      if a.getExists() {
        a.move1();
      }
    }
  }

  //was draw()
  fn draw1(&mut self) {
    let pool = self.getActorPoolData();
    for a in pool.actors {
      if a.getExists() {
        a.draw1();
      }
    }
  }

  // was clear()
  fn clear1(&mut self) {
    let pool = self.getActorPoolData();
    for a in pool.actors {
      a.setExists(false);
    }
    pool.actorIdx = 0;
  }
}

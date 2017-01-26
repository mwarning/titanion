/*
 * $Id: actor.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
//module src.util.actor;

fn main() {

}

trait Actor {

  fn exists() -> bool;
  fn setExists(v : bool); // -> bool;

  //fn init(Vec<> args);
  //was move()
  fn moveActor();
  // was draw()
  fn drawActor();
}
/**
 * Actor that has an interface to move and draw.
 */
 /*
public class Actor {
 protected:
  bool _exists;
 private:

  public bool exists() {
    return _exists;
  }

  public bool exists(bool v) {
    return _exists = v;
  }

  public abstract void init(Object[] args);
  public abstract void move();
  public abstract void draw();
}
*/

/**
 * Object pool for actors.
 */

struct ActorPool<T> {
  actors : Vec<T>,
  actorIdx : i32,
  hasNoActor : bool,
}

impl<T> ActorPool<T> {
  // init() replacement
  pub fn new(n : i32) -> ActorPool<T> {
   let actors = Vec<T>::with_capacity(10);
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
      return null;
    }

    for i in 0..actors {
      self.actorIdx -= 1;
      if actorIdx < 0 {
        actorIdx = actors.len() - 1;
      }
      if !actors[actorIdx].exists() {
        return actors[actorIdx];
      }
    }
    self.hasNoActor = true;
    return null;
  }

  fn getInstanceForced(&mut self) -> T {
    self.actorIdx -= 1;
    if self.actorIdx < 0 {
      self.actorIdx = self.actors.len() - 1;
    }
    return self.actors[actorIdx];
  }

  fn getMultipleInstances(&self, n : i32) -> Vec<T> {
    if self.hasNoActor {
      return null;
    }

    let rsl : Vec<T>;
    for i in 0..n { //(int i = 0; i < n; i++) {
      let inst : &T = self.getInstance();
      if !ins {
        for r in &rsl {
          r.setExists(false);
        }
        return null;
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
    for a in actors {
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

/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use util::vector::*;
use util::actor::*;
use ttn::token::*;
use ttn::field::*;
use ttn::shape::*;
use ttn::dummy::*;
use ttn::bullet::*;
use ttn::enemy::*;

/**
 * Pillars at the center and on the background.
 */

pub struct PillarPool<'a> {
  ap : ActorPoolData<Pillar<'a>>,
}

impl<'a> ActorPool<Pillar<'a>> for PillarPool<'a> {
  fn getActorPoolData(&mut self) -> &mut ActorPoolData<Pillar<'a>> {
    &mut self.ap
  }
}

impl<'a> PillarPool<'a> {
  pub fn new(n : i32) -> PillarPool<'a> {
    PillarPool{ap : PillarPool::<Pillar<'a>>::new(n)}
  }

  pub fn setEnd(&mut self) {
    for a in &self.ap.actors {
      if a.exists {
        a.setEnd();
      }
    }
  }

  pub fn drawCenter(&mut self) {
    let sas = &self.actors.sort();
    for a in sas {
      if a.exists && !a.state.isOutside {
        a.draw();
      }
    }
  }

  pub fn drawOutside(&mut self) {
    for a in &self.actors {
      if a.exists && a.state.isOutside {
        a.draw();
      }
    }
  }
}

pub struct Pillar<'a> {
  //tok : Token!(PillarState, PillarSpec),
  _exists : bool, //from Actor
  pub state : PillarState<'a>,
  pub spec : &'a mut PillarSpec<'a>,
}

impl<'a> Actor for Pillar<'a> {
  fn new() -> Pillar<'a> {
    Pillar {
      state : PillarState::new(),
      spec : PillarSpec::new(),  //use generic spec or Option type?
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
    self.state = PillarState::new();
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

impl<'a> Token<PillarState<'a>, PillarSpec<'a>> for Pillar<'a> {
/*
  fn getExists(&self) -> bool {
    self._exists
  }

  fn setExists(&mut self, v : bool) -> bool {
    self._exists = v;
    v
  }
*/
  fn set5Vec(&self, spec : &PillarSpec, pos : Vector, deg : f32, speed : f32) {
    self.spec = spec;
    self.set5(pos.x, pos.y, deg, speed);
  }

  fn set6(&self, spec : &PillarSpec, x : f32, y : f32, deg : f32, speed : f32) {
    self.spec = spec;
    self.set5(x, y, deg, speed);
  }

  fn set5(&self, x : f32, y : f32, deg : f32, speed : f32) {
    self.state.clear();
    self.state.ts.pos.x = x;
    self.state.ts.pos.y = y;
    self.state.ts.deg = deg;
    self.state.ts.speed = speed;
    self.spec.set(self.state);
    self._exists = true;
  }

  fn remove(&self) {
    self._exists = false;
    self.spec.removed(self.state);
  }

  fn pos(&self) -> Vector {
    self.state.ts.pos
  }
}

impl<'a> Pillar<'a> {

  fn set(&mut self, ps : PillarSpec, y : f32, maxY : f32, pp : &Pillar, s : &PillarShape, vdeg : f32, outside : bool /*= false*/) {
    self.set(ps, 0.0, y, 0.0, 0.0);
    self.state.maxY = maxY;
    self.state.previousPillar = pp;
    self.state.pshape = s;
    self.state.vdeg = vdeg;
    self.state.isOutside = outside;
  }

  fn setEnd(&mut self) {
    self.state.isEnded = true;
  }

/*
  //TODO: use for sort..
  fn opCmp(&mut self, o : Object) -> i32 {
    let p = o as &Pillar;
    if !p {
      return 0;
    }
    (p.pos().y.abs() - self.pos().y.abs()) as i32
  }
*/
}

pub struct PillarState<'a> {
  ts : TokenState,
  previousPillar : Option<&'a Pillar<'a>>,
  vy : f32,
  vdeg: f32,
  maxY : f32,
  pshape : Option<&'a PillarShape>,
  isEnded : bool,
  isOutside : bool,
}

impl<'a> PillarState<'a> {
  fn new() -> PillarState<'a> {
    //taken from clear()
    PillarState {
      previousPillar : None,
      vy : 0.0,
      vdeg : 0.0,
      maxY : 0.0,
      pshape : None,
      isEnded : false,
      isOutside : false,
      ts : TokenState::new(),
    }
  }

  fn clear(&mut self) {
    self.previousPillar = None;
    self.vy = 0.0;
    self.vdeg = 0.0;
    self.maxY = 0.0;
    self.isEnded = false;
    self.isOutside = false;
    self.ts.clear();
  }
}

static VELOCITY_Y : f32 = 0.025;

pub struct PillarSpec<'a> {
  //ts : TokenSpec<PillarState>, //inline
  field : &'a mut Field<'a>,
  shape : &'a mut Shape,
}

impl<'a> TokenSpec<PillarState<'a>> for PillarSpec<'a> {
  fn set(&self, state : &PillarState) {}
  fn removed(&self, state : &PillarState) {}

  fn move2(&self, state : &PillarState) -> bool {
    true
  }

  fn draw(&self, state : &PillarState) {
    //with (state) {
      let p = self.field.calcCircularPos(state.ts.pos);
      let cd = Field::calcCircularDeg(state.ts.pos.x);
      self.shape.draw(p, cd, state.ts.deg);
    //}
  }
}

impl<'a> PillarSpec<'a> {
  pub fn new(field : &mut Field<'a>) -> PillarSpec<'a> {
    PillarSpec { shape : PillarState::new(), field : field }
  }

  fn move2(&mut self, ps : &PillarState) -> bool {
    //with (ps) {
      if !ps.isOutside {
        ps.vy += VELOCITY_Y;
        ps.vy *= 0.98;
        ps.ts.pos.y += ps.vy;
        if ps.vy > 0.0 {
          let mut ty = ps.maxY;
          if let Some(previousPillar) = ps.previousPillar {
            if previousPillar.exists() {
              ty = previousPillar.pos.y - TICKNESS
            }
          }
          ty -= TICKNESS;
          if !ps.isEnded && ps.ts.pos.y > ty {
            ps.vy *= -0.5;
            ps.ts.pos.y += (ty - ps.ts.pos.y) * 0.5;
            if let Some(p) = ps.previousPillar {
              p.state.vy -= ps.vy * 0.5;
            }
          }
          if ps.ts.pos.y > 100.0 {
            return false;
          }
        }
      } else {
        ps.ts.pos.y -= 0.2;
        if ps.ts.pos.y < -50.0 {
          return false;
        }
      }
      ps.ts.deg += ps.vdeg;
      true
    //}
  }

  fn draw(&self, ps : &PillarState) {
    ps.pshape.draw(ps.ts.pos.y, ps.ts.deg);
  }
}

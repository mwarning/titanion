/*
 * $Id: pillar.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.pillar;


private import src.util.actor;
private import src.ttn.field;
private import src.ttn.token;
private import src.ttn.shape;
private import src.ttn.enemy;
*/

use ttn::token::*;
use ttn::field::*;
use ttn::shape::*;
use ttn::dummy::*;
use ttn::bullet::*;
use ttn::enemy::*;
use util::actor::*;

/**
 * Pillars at the center and on the background.
 */

pub struct PillarPool<'a> {
  ap : ActorPoolData<Pillar<'a>>,
}

impl<'a> ActorPool<Pillar<'a>> for PillarPool<'a> {
  fn getActorPoolData(&mut self) -> &mut ActorPoolData<Pillar<'a>> {
    &self.ap
  }
}

impl<'a> PillarPool<'a> {
  fn new(n : i32) -> PillarPool<'a> {
    PillarPool{ap : PillarPool::<Pillar<'a>>::new(n)}
  }

  fn setEnd(&mut self) {
    for a in &self.ap.actors {
      if a.exists {
        a.setEnd();
      }
    }
  }

  fn drawCenter(&mut self) {
    let sas = &self.actors.sort();
    for a in sas {
      if a.exists && !a.state.isOutside {
        a.draw();
      }
    }
  }

  fn drawOutside(&mut self) {
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


  fn opCmp(&mut self, o : Object) -> i32 {
    let p = o as &Pillar;
    if !p {
      return 0;
    }
    (p.pos.y.abs() - self.pos().y.abs()) as i32
  }
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
}

impl<'a> PillarSpec<'a> {

  fn new(field : &mut Field<'a>) -> PillarSpec<'a> {
    PillarSpec { shape : PillarState::new(), field : field }
  }

  fn move2(&mut self, ps : &PillarState) -> bool {
    //with (ps) {
      if !ps.isOutside {
        ps.vy += VELOCITY_Y;
        ps.vy *= 0.98;
        ps.ts.pos.y += ps.vy;
        if ps.vy > 0.0 {
          let mut ty : f32 = if self.previousPillar && self.previousPillar.exists {
            self.previousPillar.pos.y - PillarShape::TICKNESS
          } else {
            ps.maxY
          };
          ps.ty -= PillarShape::TICKNESS;
          if !ps.isEnded && ps.ts.pos.y > ty {
            ps.vy *= -0.5;
            ps.ts.pos.y += (ps.ty - ps.ts.pos.y) * 0.5;
            if let Some(p) = ps.previousPillar {
              p.state.vy -= ps.vy * 0.5;
            }
          }
          if ps.ts.pos.y > 100 {
            return false;
          }
        }
      } else {
        ps.ts.pos.y -= 0.2;
        if ps.ts.pos.y < -50 {
          return false;
        }
      }
      ps.ts.deg += ps.vdeg;
      true
    //}
  }

  fn draw(ps : &PillarState) {
    ps.pshape.draw(ps.pos.y, ps.deg);
  }
}

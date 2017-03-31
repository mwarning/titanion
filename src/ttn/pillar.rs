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

/**
 * Pillars at the center and on the background.
 */

struct PillarPool {
  ap : ActorPool<Pillar>,
}

impl PillarPool {

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

struct Pillar<'a> {
  //tok : Token!(PillarState, PillarSpec),
  _exists : bool, //from Actor
  pub state : &'a mut PillarState,
  pub spec : &'a mut PillarSpec,
}

impl<'a> Actor for Pillar<'a> {
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

impl<'a> Token<PillarState, PillarSpec> for Pillar<'a> {
}

impl<'a> Pillar<'a> {

  fn set(&mut self, ps : PillarSpec, y : f32, maxY : f32, pp : &Pillar, s : &PillarShape, vdeg : f32, outside : bool /*= false*/) {
    self.tok.set(ps, 0.0, y, 0.0, 0.0);
    self.tok.state.maxY = maxY;
    self.tok.state.previousPillar = pp;
    self.tok.state.pshape = s;
    self.state.vdeg = vdeg;
    self.state.isOutside = outside;
  }

  fn setEnd(&mut self) {
    self.tok.state.isEnded = true;
  }


  fn opCmp(&mut self, o : Object) {
    let p = o as &Pillar;
    if !p {
      return 0;
    }
    (p.pos.y.abs() - pos.y.abs()) as i32
  }
}

struct PillarState {
  ts : TokenState,
  previousPillar : Option(&Pillar),
  vy : f32,
  vdeg: f32,
  maxY : f32,
  pshape : PillarShape,
  isEnded : bool,
  isOutside : bool,
}

impl PillarState {
  fn clear(&mut self) {
    self.previousPillar = None;
    self.vy = 0;
    self.vdeg = 0;
    self.maxY = 0;
    self.isEnded = false;
    self.isOutside = false;
    self.ts.clear();
  }
}

static VELOCITY_Y : f32 = 0.025;

struct PillarSpec {
  ts : TokenSpec<PillarState>,
}

impl PillarSpec {

  fn new(field : Field) -> PillarSpec {
    PillarSpec { ts : TokenSpec::<PillarState>::new(), field : field }
  }

  fn move2(&mut self, ps : PillarState) -> bool {
    //with (ps) {
      if !ps.isOutside {
      vy += VELOCITY_Y;
        vy *= 0.98;
        pos.y += vy;
        if vy > 0 {
          let mut ty : f32;
          if self.previousPillar && self.previousPillar.exists {
            ty = self.previousPillar.pos.y - PillarShape::TICKNESS;
          }
          else {
            ty = maxY;
          }
          ty -= PillarShape.TICKNESS;
          if !self.isEnded && pos.y > ty {
            vy *= -0.5;
            pos.y += (ty - pos.y) * 0.5;
            if self.previousPillar {
              self.previousPillar.state.vy -= vy * 0.5;
            }
          }
          if pos.y > 100 {
            return false;
          }
        }
      } else {
        pos.y -= 0.2;
        if pos.y < -50 {
          return false;
        }
      }
      deg += vdeg;
      true;
    //}
  }

  fn draw(ps : &PillarState) {
    ps.pshape.draw(ps.pos.y, ps.deg);
  }
}

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

/**
 * Pillars at the center and on the background.
 */

struct PillarPool {
  ap : ActorPool!(Pillar).
}

impl PillarPool {

  fn setEnd(&mut self) {
    for a in self.ap.actors {
      if a.exists {
        a.setEnd();
      }
    }
  }

  fn drawCenter(&mut self) {
    let sas = self.actors.sort();
    for a in sas {
      if a.exists && !a.state.isOutside {
        a.draw();
      }
    }
  }

  fn drawOutside(&mut self) {
    for a in self.actors {
      if a.exists && a.state.isOutside {
        a.draw();
      }
    }
  }
}

struct Pillar {
  tok : Token!(PillarState, PillarSpec),
}

impl Pillar {

  fn set(&mut self, ps : PillarSpec, y : f32, maxY : f32, pp : Pillar, s : PillarShape, vdeg : f32, outside : bool /*= false*/) {
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
    let p = o as Pillar;
    if !p {
      return 0;
    }
    (p.pos.y.abs() - pos.y.abs()) as i32
  }
}

struct PillarState {
  ts : TokenState,
  previousPillar : Pillar,
  vy : f32,
  vdeg: f32,
  maxY : f32,
  pshape : PillarShape,
  isEnded : bool,
  isOutside : bool,
}

impl PillarState {
  fn clear(&mut self) {
    self.previousPillar = null;
    self.vy = 0;
    self.vdeg = 0;
    self.maxY = 0;
    self.isEnded = false;
    self.isOutside = false;
    self.ts.clear();
  }
}

static const VELOCITY_Y : f32 = 0.025;

struct PillarSpec {
  ts : TokenSpec!(PillarState),
}

impl PillarSpec {

  fn this(field : Field) {
    self.ts.field = field;
  }

  fn move(&mut self, ps : PillarState) -> bool {
    //with (ps) {
      if !self.isOutside {
      vy += VELOCITY_Y;
        vy *= 0.98;
        pos.y += vy;
        if vy > 0 {
          let mut ty : f32;
          if self.previousPillar && self.previousPillar.exists {
            ty = self.previousPillar.pos.y - PillarShape.TICKNESS;
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

  fn draw(ps : PillarState) {
    ps.pshape.draw(ps.pos.y, ps.deg);
  }
}

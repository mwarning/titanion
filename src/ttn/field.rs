/*
 * $Id: field.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.field;


private import std.math;

private import derelict.opengl.gl;
private import derelict.opengl.glu;

private import src.util.vector;
private import src.util.math;
private import src.ttn.screen;
private import src.ttn.frame;
*/

use std::f32::consts::PI;

/**
 * Game field.
 */


static PIT_SIZE_Y_RATIO :f32 = 12.0;
static CIRCLE_RADIUS : f32 = 64.0;
static EYE_POS_DIST_RATIO : f32 = 1.25;
static X_EXPANSION_RATIO : f32 = 1.0;
static SIDEWALL_WIDTH : f32 = 145.0;
static TORUS_Y : f32 = -24.0;

struct Field {
  frame : &Frame,
  screen : &Screen,
  _size : Vector,
  _outerSize : Vector,
  _eyePos : Vector3,
  eyeDeg : f32,
  circlePos Vector3,
  cnt : i32,
}

impl Default for Field {
  fn default(frame : &Frame, screen : &Screen) -> Field {
    Field{
      frame : frame,
      screen : screen,
      _size : Vector::default(12.0, 12.0),
      _outerSize : Vector::default(13.0, 13.0),
      _eyePos : Vector3::default(0.0, 0.0, 0.0),
      eyeDeg : 0.0,
      circlePos : Vector3::default(0.0, 0.0, 0.0),
      cnt : 0.0,
    }
  }
}

impl Field {
  fn set(&mut self) {
    self._eyePos.x = 0.0;
    self._eyePos.y = 0.0;
    self._eyePos.z = 0.0;
    self.eyeDeg = 0.0;
    self.cnt = 0.0;
  }

  fn contains(&self, Vector p) -> bool {
    self.contains(p.x, p.y)
  }

  fn contains(&self, x : f32, y : f32) -> bool {
    self._size.contains(x, y)
  }

  fn containsOuter(&self, p : Vector) -> bool {
    self.containsOuter(p.x, p.y)
  }

  fn containsOuter(&self, x : f32, y : f32) -> bool {
    self._outerSize.contains(x, y)
  }

  fn containsOuterY(y : f32) -> bool {
    (y >= -_outerSize.y && y <= _outerSize.y)
  }

  fn containsIncludingPit(p : Vector) -> bool {
    (p.y >= -_outerSize.y && p.y <= (_size.y * PIT_SIZE_Y_RATIO * 1.1))
  }

  fn normalizeX(&self, x : f32) -> f32 {
    let rx : f32 = x;
    let hd : f32 = CIRCLE_RADIUS * PI / X_EXPANSION_RATIO;
    if rx < -hd {
      rx = hd * 2.0 - (-rx % (hd * 2.0));
    }
    (rx + hd) % (hd * 2.0) - hd
  }

  fn calcCircularDist(p1 : Vector, p2 : Vector) -> f32 {
    let ax : f32 = (normalizeX(p1.x - p2.x)).abs();
    let ay : f32 = (p1.y - p2.y).abs();
    if ax > ay {
      ax + ay / 2.0
    } else {
      ay + ax / 2.0
    }
  }

  fn circularDistance() -> f32 {
    CIRCLE_RADIUS * PI * 2.0 / X_EXPANSION_RATIO
  }

  fn calcCircularPos(Vector p) -> Vector3 {
    calcCircularPos(p.x, p.y)
  }

  fn calcCircularPos(x : f32, y : f32) -> Vector3 {
    let d : f32 = calcCircularDeg(x);
    if y < _size.y {
      circlePos.x = d.sin() * CIRCLE_RADIUS;
      circlePos.z = d.cos() * CIRCLE_RADIUS;
      circlePos.y = y;
    } else if y < (_size.y * 3.0) {
      float cd = (y - _size.y) * PI / 2.0 / (_size.y * 2.0);
      float cr = CIRCLE_RADIUS * (0.8 + 0.2 * cd.cos()));
      circlePos.x = d.sin() * cr;
      circlePos.z = d.cos() * cr;
      circlePos.y = _size.y + cd.sin() * CIRCLE_RADIUS * 0.2;
    } else if y < (_size.y * 7) {
      let cd : f32 = (y - _size.y * 3.0) * PI / 2.0 / (_size.y * 4.0);
      float cr = CIRCLE_RADIUS * (0.8 - 0.4 * sin(cd));
      circlePos.x = d.sin() * cr;
      circlePos.z = d.cos() * cr;
      circlePos.y = _size.y - CIRCLE_RADIUS * 0.2 + cos(cd) * CIRCLE_RADIUS * 0.4f;
    } else {
      float cr = CIRCLE_RADIUS * 0.4;
      circlePos.x = d.sin() * cr;
      circlePos.z = d.cos() * cr;
      circlePos.y = _size.y - CIRCLE_RADIUS * 0.2 - (y - _size.y * 7.0);
    }
    circlePos
  }

  fn calcCircularDeg(x : f32) -> f32 {
    x * X_EXPANSION_RATIO / CIRCLE_RADIUS
  }

  fn calcCircularDist(d : f32) -> f32 {
    d * CIRCLE_RADIUS / X_EXPANSION_RATIO
  }

  fn checkHitDist(pos : Vector, p : Vector, pp : Vector, dist : f32) -> bool {
    let mut bmvx : f32;
    let mut bmvy : f32;
    let mut inaa : f32;
    bmvx = pp.x;
    bmvy = pp.y;
    bmvx -= p.x;
    bmvy -= p.y;
    bmvx = normalizeX(bmvx);
    inaa = bmvx * bmvx + bmvy * bmvy;
    if inaa > 0.00001 {
      let mut sofsx : f32;
      let mut sofsy : f32;
      let mut inab : f32;
      let mut hd : f32;
      sofsx = pos.x;
      sofsy = pos.y;
      sofsx -= p.x;
      sofsy -= p.y;
      sofsx = normalizeX(sofsx);
      inab = bmvx * sofsx + bmvy * sofsy;
      if (inab >= 0) && (inab <= inaa) {
        hd = sofsx * sofsx + sofsy * sofsy - inab * inab / inaa;
        if (hd >= 0) && (hd <= dist) {
          return true;
        }
      }
    }
    false
  }

  fn addSlowdownRatio(&mut self, sr : f32) {
    self.frame.addSlowdownRatio(sr);
  }

  fn setEyePos(&mut self, p : Vector) {
    self.eyeDeg = calcCircularDeg(p.x) * 0.25;
    self._eyePos.x = eyeDeg.sin() * CIRCLE_RADIUS * EYE_POS_DIST_RATIO;
    self._eyePos.z = eyeDeg.cos() * CIRCLE_RADIUS * EYE_POS_DIST_RATIO;
  }

  fn setLookAt(&mut self) {
    glMatrixMode(GL_PROJECTION);
    self.screen.setPerspective();
    gluLookAt(self._eyePos.x, self._eyePos.y, self._eyePos.z, 0.0, self._eyePos.y, 0.0, 0.0, 1.0, 0.0);
    glMatrixMode(GL_MODELVIEW);
  }

  fn resetLookAt(&mut self) {
    glMatrixMode(GL_PROJECTION);
    self.screen.setPerspective();
    gluLookAt(0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    glMatrixMode(GL_MODELVIEW);
  }

  fn beginDrawingFront() {
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glOrtho(0.0, 640.0, 480.0, 0.0, -1.0, 1.0);
    glMatrixMode(GL_MODELVIEW);
    drawSidewall();
  }

  fn drawSidewall() {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    Screen.setColor(0.25, 0.25, 0.25, 0.5);
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0.0, 0.0, 0.0);
    glVertex3f(SIDEWALL_WIDTH, 0.0, 0.0);
    glVertex3f(SIDEWALL_WIDTH, 480.0, 0.0);
    glVertex3f(0.0, 480.0, 0.0);
    glEnd();
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(640.0, 0.0, 0.0);
    glVertex3f(640.0 - SIDEWALL_WIDTH, 0.0, 0.0);
    glVertex3f(640.0 - SIDEWALL_WIDTH, 480.0, 0.0);
    glVertex3f(640.0, 480.0, 0.0);
    glEnd();
    Screen.setColor(1.0, 1.0, 1.0, 0.8);
    glBegin(GL_LINES);
    glVertex3f(SIDEWALL_WIDTH, 0.0, 0.0);
    glVertex3f(SIDEWALL_WIDTH, 480.0, 0.0);
    glVertex3f(640.0 - SIDEWALL_WIDTH, 0.0, 0.0);
    glVertex3f(640.0 - SIDEWALL_WIDTH, 480.0, 0.0);
    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }

  fn move(&mut self) {
    self.cnt += 1;
  }

  fn drawBack(&self) {
    glPushMatrix();
    glTranslatef(0.0, TORUS_Y, 0.0);
    self.drawTorusShape(PI / 2.0);
    glPopMatrix();
  }

  fn drawFront(&self) {
    glPushMatrix();
    glTranslatef(0.0, TORUS_Y, 0.0);
    self.drawTorusShape(-PI / 2.0);
    glPopMatrix();
  }

  fn drawTorusShape(&self, d1s : f32) {
    let cp : Vector3;
    cp.y = 0;
    let ringOfs : Vector3;
    let torusRad : f32 = CIRCLE_RADIUS * 0.9;
    let mut ringRad : f32;
    let mut d1 : f32;
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glBegin(GL_QUADS);
    ringRad = CIRCLE_RADIUS * 0.3;
    d1 = d1s;
    for _ in 0..16 {
      let mut d2 : f32 = cnt * 0.003;
      for _ in 0..16 {
        cp.x = d1.sin() * torusRad;
        cp.z = d1.cos() * torusRad;
        createRingOffset(ringOfs, cp, ringRad, d1, d2);
        Screen.setColor(0.3, 0.3, 0.3, 0.8);
        Screen.glVertex(ringOfs);
        createRingOffset(ringOfs, cp, ringRad, d1, d2 + PI * 2.0 / 16.0);
        Screen.glVertex(ringOfs);
        cp.x = (d1 + PI * 2.0 / 32.0).sin() * torusRad;
        cp.z = (d1 + PI * 2.0 / 32.0).cos() * torusRad;
        createRingOffset(ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0, d2 + PI * 2.0 / 16.0);
        Screen.glVertex(ringOfs);
        createRingOffset(ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0, d2);
        Screen.setColor(0.3, 0.3, 0.3, 0.2);
        Screen.glVertex(ringOfs);
        d2 += PI * 2.0 / 16.0
      }
      d1 += PI * 2.0 / 32.0;
    }

    glEnd();
    glBegin(GL_LINE_STRIP);
    ringRad = CIRCLE_RADIUS * 0.3;
    Screen.setColor(0.1, 0.1, 0.1);
    d1 = d1s;
    for _ in 0..16 {
        float d2 = cnt * 0.003;
        for _ in 0..16 {
          cp.x = sin(d1 + PI * 2.0 / 32.0 * 0.1) * torusRad;
          cp.z = cos(d1 + PI * 2.0 / 32.0 * 0.1) * torusRad;
          createRingOffset(ringOfs, cp, ringRad, d1 + PI * 2 / 32 * 0.1, d2 + PI * 2.0 / 16.0 * 0.1);
          Screen.glVertex(ringOfs);
          createRingOffset(ringOfs, cp, ringRad, d1 + PI * 2 / 32 * 0.1, d2 + PI * 2.0 / 16.0 * 0.9);
          Screen.glVertex(ringOfs);
          cp.x = sin(d1 + PI * 2.0 / 32.0 * 0.9) * torusRad;
          cp.z = cos(d1 + PI * 2.0 / 32.0 * 0.9) * torusRad;
          createRingOffset(ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0 * 0.9, d2 + PI * 2.0 / 32.0 * 0.1);
          Screen.glVertex(ringOfs);
          createRingOffset(ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0 * 0.9, d2 + PI * 2.0 / 16.0 * 0.9);
          Screen.glVertex(ringOfs);
          d2 += PI * 2.0 / 16.0
        }
      d1 += PI * 2.0 / 32.0
    }

    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }

  fn createRingOffset(&mut self, ringOfs : Vector3, centerPos : Vector3,
                              rad : f32, d1 : f32, d2 : f32) {
    ringOfs.x = 0.0;
    ringOfs.y = 0.0;
    ringOfs.z = rad;
    ringOfs.rollX(d2);
    ringOfs.rollY(-d1);
    ringOfs += centerPos;
  }

  fn eyePos(&self) -> Vector3 {
    self._eyePos;
  }

  fn size(&self) -> Vector {
    self._size;
  }
}

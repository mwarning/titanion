/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::f32::consts::PI;

use util::vector::*;
use ttn::frame::*;
use ttn::screen::*;
use ttn::dummy::*;

/**
 * Game field.
 */

pub const PIT_SIZE_Y_RATIO :f32 = 12.0;
pub const CIRCLE_RADIUS : f32 = 64.0;
const EYE_POS_DIST_RATIO : f32 = 1.25;
const X_EXPANSION_RATIO : f32 = 1.0;
const SIDEWALL_WIDTH : f32 = 145.0;
const TORUS_Y : f32 = -24.0;

pub struct Field<'a> {
  frame : &'a mut Frame<'a>,
  screen : &'a mut Screen,
  _size : Vector,
  _outerSize : Vector,
  _eyePos : Vector3,
  eyeDeg : f32,
  circlePos : Vector3,
  cnt : i32,
}

impl<'a> Field<'a> {
  pub fn new(frame : &mut Frame<'a>, screen : &mut Screen) -> Field<'a> {
    Field{
      frame : frame,
      screen : screen,
      _size : Vector::new(12.0, 12.0),
      _outerSize : Vector::new(13.0, 13.0),
      _eyePos : Vector3::new(0.0, 0.0, 0.0),
      eyeDeg : 0.0,
      circlePos : Vector3::new(0.0, 0.0, 0.0),
      cnt : 0,
    }
  }

  pub fn set(&mut self) {
    self._eyePos.x = 0.0;
    self._eyePos.y = 0.0;
    self._eyePos.z = 0.0;
    self.eyeDeg = 0.0;
    self.cnt = 0;
  }

  pub fn contains1(&self, p : Vector) -> bool {
    self.contains(p.x, p.y)
  }

  pub fn contains2(&self, x : f32, y : f32) -> bool {
    self._size.contains(x, y)
  }

  pub fn containsOuter1(&self, p : Vector) -> bool {
    self.containsOuter(p.x, p.y)
  }

  pub fn containsOuter2(&self, x : f32, y : f32) -> bool {
    self._outerSize.contains(x, y)
  }

  pub fn containsOuterY(&self, y : f32) -> bool {
    (y >= -self._outerSize.y) && (y <= self._outerSize.y)
  }

  pub fn containsIncludingPit(&self, p : Vector) -> bool {
    (p.y >= -self._outerSize.y) && (p.y <= (self._size.y * PIT_SIZE_Y_RATIO * 1.1))
  }

  pub fn normalizeX(x : f32) -> f32 {
    let rx = x;
    let hd = CIRCLE_RADIUS * PI / X_EXPANSION_RATIO;
    if rx < -hd {
      rx = hd * 2.0 - (-rx % (hd * 2.0));
    }
    (rx + hd) % (hd * 2.0) - hd
  }

  pub fn calcCircularDist2(p1 : Vector, p2 : Vector) -> f32 {
    let ax  = (Field::normalizeX(p1.x - p2.x)).abs();
    let ay = (p1.y - p2.y).abs();
    if ax > ay {
      ax + ay / 2.0
    } else {
      ay + ax / 2.0
    }
  }

  pub fn circularDistance() -> f32 {
    CIRCLE_RADIUS * PI * 2.0 / X_EXPANSION_RATIO
  }

  pub fn calcCircularPos1(&mut self, p : Vector) -> Vector3 {
    self.calcCircularPos(p.x, p.y)
  }

  pub fn calcCircularPos(&mut self, x : f32, y : f32) -> Vector3 {
    let d = Field::calcCircularDeg(x);
    if y < self._size.y {
      self.circlePos.x = d.sin() * CIRCLE_RADIUS;
      self.circlePos.z = d.cos() * CIRCLE_RADIUS;
      self.circlePos.y = y;
    } else if y < (self._size.y * 3.0) {
      let cd = (y - self._size.y) * PI / 2.0 / (self._size.y * 2.0);
      let cr = CIRCLE_RADIUS * (0.8 + 0.2 * cd.cos());
      self.circlePos.x = d.sin() * cr;
      self.circlePos.z = d.cos() * cr;
      self.circlePos.y = self._size.y + cd.sin() * CIRCLE_RADIUS * 0.2;
    } else if y < (self._size.y * 7) {
      let cd = (y - self._size.y * 3.0) * PI / 2.0 / (self._size.y * 4.0);
      let cr = CIRCLE_RADIUS * (0.8 - 0.4 * cd.sin());
      self.circlePos.x = d.sin() * cr;
      self.circlePos.z = d.cos() * cr;
      self.circlePos.y = self._size.y - CIRCLE_RADIUS * 0.2 + cd.cos() * CIRCLE_RADIUS * 0.4;
    } else {
      let cr = CIRCLE_RADIUS * 0.4;
      self.circlePos.x = d.sin() * cr;
      self.circlePos.z = d.cos() * cr;
      self.circlePos.y = self._size.y - CIRCLE_RADIUS * 0.2 - (y - self._size.y * 7.0);
    }
    self.circlePos
  }

  pub fn calcCircularDeg(x : f32) -> f32 {
    x * X_EXPANSION_RATIO / CIRCLE_RADIUS
  }

  pub fn calcCircularDist1(d : f32) -> f32 {
    d * CIRCLE_RADIUS / X_EXPANSION_RATIO
  }

  pub fn checkHitDist(pos : Vector, p : Vector, pp : Vector, dist : f32) -> bool {
    let mut bmvx = pp.x - p.x;
    let mut bmvy = pp.y - p.y;
    bmvx = Field::normalizeX(bmvx);
    let inaa = bmvx * bmvx + bmvy * bmvy;
    if inaa > 0.00001 {
      let mut sofsx = pos.x - p.x;
      let mut sofsy = pos.y - p.y;
      sofsx = Field::normalizeX(sofsx);
      let inab = bmvx * sofsx + bmvy * sofsy;
      if (inab >= 0) && (inab <= inaa) {
        let hd = sofsx * sofsx + sofsy * sofsy - inab * inab / inaa;
        if (hd >= 0) && (hd <= dist) {
          return true;
        }
      }
    }
    false
  }

  pub fn addSlowdownRatio(&mut self, sr : f32) {
    self.frame.addSlowdownRatio(sr);
  }

  pub fn setEyePos(&mut self, p : Vector) {
    self.eyeDeg = Field::calcCircularDeg(p.x) * 0.25;
    self._eyePos.x = self.eyeDeg.sin() * CIRCLE_RADIUS * EYE_POS_DIST_RATIO;
    self._eyePos.z = self.eyeDeg.cos() * CIRCLE_RADIUS * EYE_POS_DIST_RATIO;
  }

  pub fn setLookAt(&mut self) {
    glMatrixMode(GL_PROJECTION);
    self.screen.setPerspective();
    gluLookAt(self._eyePos.x, self._eyePos.y, self._eyePos.z, 0.0, self._eyePos.y, 0.0, 0.0, 1.0, 0.0);
    glMatrixMode(GL_MODELVIEW);
  }

  pub fn resetLookAt(&mut self) {
    glMatrixMode(GL_PROJECTION);
    self.screen.setPerspective();
    gluLookAt(0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    glMatrixMode(GL_MODELVIEW);
  }

  pub fn beginDrawingFront(&self) {
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glOrtho(0.0, 640.0, 480.0, 0.0, -1.0, 1.0);
    glMatrixMode(GL_MODELVIEW);
    Field::drawSidewall();
  }

  pub fn drawSidewall() {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    Screen::setColor(0.25, 0.25, 0.25, 0.5);
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
    Screen::setColor(1.0, 1.0, 1.0, 0.8);
    glBegin(GL_LINES);
    glVertex3f(SIDEWALL_WIDTH, 0.0, 0.0);
    glVertex3f(SIDEWALL_WIDTH, 480.0, 0.0);
    glVertex3f(640.0 - SIDEWALL_WIDTH, 0.0, 0.0);
    glVertex3f(640.0 - SIDEWALL_WIDTH, 480.0, 0.0);
    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }

  pub fn move1(&mut self) {
    self.cnt += 1;
  }

  pub fn drawBack(&self) {
    glPushMatrix();
    glTranslatef(0.0, TORUS_Y, 0.0);
    self.drawTorusShape(PI / 2.0);
    glPopMatrix();
  }

  pub fn drawFront(&self) {
    glPushMatrix();
    glTranslatef(0.0, TORUS_Y, 0.0);
    self.drawTorusShape(-PI / 2.0);
    glPopMatrix();
  }

  pub fn drawTorusShape(&self, d1s : f32) {
    let mut cp = Vector3::new();
    cp.y = 0.0;
    let mut ringOfs = Vector3::new(0.0, 0.0, 0.0);
    let torusRad = CIRCLE_RADIUS * 0.9;
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glBegin(GL_QUADS);
    let mut ringRad = CIRCLE_RADIUS * 0.3;
    let mut d1 = d1s;
    for _ in 0..16 {
      let mut d2 = (self.cnt as f32) * 0.003;
      for _ in 0..16 {
        cp.x = d1.sin() * torusRad;
        cp.z = d1.cos() * torusRad;
        Field::createRingOffset(&mut ringOfs, cp, ringRad, d1, d2);
        Screen::setColor(0.3, 0.3, 0.3, 0.8);
        Screen::glVertex3(ringOfs);
        Field::createRingOffset(&mut ringOfs, cp, ringRad, d1, d2 + PI * 2.0 / 16.0);
        Screen::glVertex3(ringOfs);
        cp.x = (d1 + PI * 2.0 / 32.0).sin() * torusRad;
        cp.z = (d1 + PI * 2.0 / 32.0).cos() * torusRad;
        Field::createRingOffset(&mut ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0, d2 + PI * 2.0 / 16.0);
        Screen::glVertex3(ringOfs);
        Field::createRingOffset(&mut ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0, d2);
        Screen::setColor(0.3, 0.3, 0.3, 0.2);
        Screen::glVertex3(ringOfs);
        d2 += PI * 2.0 / 16.0
      }
      d1 += PI * 2.0 / 32.0;
    }

    glEnd();
    glBegin(GL_LINE_STRIP);
    ringRad = CIRCLE_RADIUS * 0.3;
    Screen::setColor(0.1, 0.1, 0.1, 1.0);
    d1 = d1s;
    for _ in 0..16 {
        let mut d2 = self.cnt as f32 * 0.003;
        for _ in 0..16 {
          cp.x = (d1 + PI * 2.0 / 32.0 * 0.1).sin() * torusRad;
          cp.z = (d1 + PI * 2.0 / 32.0 * 0.1).cos() * torusRad;
          Field::createRingOffset(&mut ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0 * 0.1, d2 + PI * 2.0 / 16.0 * 0.1);
          Screen::glVertex3(ringOfs);
          Field::createRingOffset(&mut ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0 * 0.1, d2 + PI * 2.0 / 16.0 * 0.9);
          Screen::glVertex3(ringOfs);
          cp.x = (d1 + PI * 2.0 / 32.0 * 0.9).sin() * torusRad;
          cp.z = (d1 + PI * 2.0 / 32.0 * 0.9).cos() * torusRad;
          Field::createRingOffset(&mut ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0 * 0.9, d2 + PI * 2.0 / 32.0 * 0.1);
          Screen::glVertex3(ringOfs);
          Field::createRingOffset(&mut ringOfs, cp, ringRad, d1 + PI * 2.0 / 32.0 * 0.9, d2 + PI * 2.0 / 16.0 * 0.9);
          Screen::glVertex3(ringOfs);
          d2 += PI * 2.0 / 16.0
        }
      d1 += PI * 2.0 / 32.0
    }

    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }

  pub fn createRingOffset(ringOfs : &mut Vector3, centerPos : Vector3, rad : f32, d1 : f32, d2 : f32) {
    ringOfs.x = 0.0;
    ringOfs.y = 0.0;
    ringOfs.z = rad;
    ringOfs.rollX(d2);
    ringOfs.rollY(-d1);
    ringOfs += centerPos;
  }

  pub fn eyePos(&self) -> Vector3 {
    self._eyePos
  }

  pub fn size(&self) -> Vector {
    self._size
  }
}

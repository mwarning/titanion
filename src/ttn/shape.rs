/*
 * $Id: shape.d,v 1.5 2006/12/04 16:04:27 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.shape;


private import std.math;

private import derelict.opengl.gl;

private import src.util.vector;
private import src.util.sdl.displaylist;
private import src.ttn.screen;
private import src.ttn.field;
*/

/**
 * 3D shapes of a player, enemies, particles, etc.
 */

trait Shape {
  fn draw(pos : Vector3, cd : f32, deg : f32);
}

struct DisplayListShape {
  DisplayList displayList;
}

impl Shape for DisplayListShape {
    fn draw(pos : Vector3, cd : f32, deg : f32) {
    glPushMatrix();
    Screen.glTranslate(pos);
    glRotatef(cd * 180.0 / PI, 0, 1, 0);
    Screen.glRotate(deg);
    displayList.call();
    glPopMatrix();
  }
}

impl DisplayListShape {
  fn this(&mut self) {
    self.displayList = new DisplayList(1);
    self.displayList.beginNewList();
    self.drawList();
    self.displayList.endNewList();
  }

  fn draw(&mut self) {
   self.drawList();
  }

  fn close(&mut self) {
    self.displayList.close();
  }
}

struct PyramidShape {

}

impl PyramidShape {
  fn draw() {
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0, 0, 0);
    glVertex3f(1, 1, 1);
    glVertex3f(1, 1, -1);
    glVertex3f(-1, 1, -1);
    glVertex3f(-1, 1, 1);
    glVertex3f(1, 1, 1);
    glEnd();
    Screen.setColor(0.1, 0.1, 0.1, 0.5);
    glBegin(GL_LINE_STRIP);
    glVertex3f(0, 0, 0);
    glVertex3f(1, 1, 1);
    glVertex3f(1, 1, -1);
    glVertex3f(0, 0, 0);
    glVertex3f(-1, 1, -1);
    glVertex3f(-1, 1, 1);
    glVertex3f(0, 0, 0);
    glEnd();
    glBegin(GL_LINES);
    glVertex3f(1, 1, 1);
    glVertex3f(-1, 1, 1);
    glVertex3f(1, 1, -1);
    glVertex3f(-1, 1, -1);
    glEnd();
  }

  fn drawShadow(r : f32, g : f32, b : f32, noAlpha : bool /*= false*/) {
    glBegin(GL_TRIANGLE_FAN);
    Screen.setColor(r, g, b);
    glVertex3f(0, 0, 0);
    if !noAlpha {
      Screen.setColor(r * 0.75, g * 0.75, b * 0.75, 0.33);
    } else {
      Screen.setColor(r * 0.75, g * 0.75, b * 0.75, 0.75);
    }
    glVertex3f(1, 1, 1);
    glVertex3f(1, 1, -1);
    glVertex3f(-1, 1, -1);
    glVertex3f(-1, 1, 1);
    glVertex3f(1, 1, 1);
    glEnd();
  }

  fn drawPolygonShape() {
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0, 0, 0);
    glVertex3f(1, 1, 1);
    glVertex3f(1, 1, -1);
    glVertex3f(-1, 1, -1);
    glVertex3f(-1, 1, 1);
    glVertex3f(1, 1, 1);
    glEnd();
  }

  fn drawLineShape() {
    glBegin(GL_LINE_STRIP);
    glVertex3f(0, 0, 0);
    glVertex3f(1, 1, 1);
    glVertex3f(1, 1, -1);
    glVertex3f(0, 0, 0);
    glVertex3f(-1, 1, -1);
    glVertex3f(-1, 1, 1);
    glVertex3f(0, 0, 0);
    glEnd();
    glBegin(GL_LINES);
    glVertex3f(1, 1, 1);
    glVertex3f(-1, 1, 1);
    glVertex3f(1, 1, -1);
    glVertex3f(-1, 1, -1);
    glEnd();
  }
}

struct PlayerShape {

}

impl DisplayListShape for PlayerShape {

  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(0, -0.6, 0);
    glScalef(0.4, 1.3, 0.4);
    PyramidShape.drawShadow(1, 0.5, 0.5, true);
    glPopMatrix();
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(0.5, -0.2, 0);
    glScalef(0.3, 0.9, 0.3);
    PyramidShape.drawShadow(1, 1, 1, true);
    glPopMatrix();
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(-0.5, -0.2, 0);
    glScalef(0.3, 0.9, 0.3);
    PyramidShape.drawShadow(1, 1, 1, true);
    glPopMatrix();
    Screen.setColor(1, 0.5, 0.5);
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(0, -0.6, 0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape.drawPolygonShape();
    glPopMatrix();
    Screen.setColor(1, 1, 1);
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(0.5, -0.2, 0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape.drawPolygonShape();
    glPopMatrix();
    Screen.setColor(1, 1, 1);
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(-0.5, -0.2, 0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape.drawPolygonShape();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct PlayerLineShape {
}

impl DisplayListShape for PlayerLineShape {
 fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(0, -0.6, 0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(0.5, -0.2, 0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(-0.5, -0.2, 0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct ShotShape {
}

impl DisplayListShape for ShotShape {

  fn drawList(&self) {
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(0.5, -0.5, 0);
    glScalef(0.1, 1.0, 0.1);
    Screen.setColor(0.4, 0.2, 0.8);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(180, 0, 0, 1);
    glTranslatef(-0.5, -0.5, 0);
    glScalef(0.1, 1.0, 0.1);
    Screen.setColor(0.4, 0.2, 0.8f);
    PyramidShape.drawLineShape();
    glPopMatrix();
  }
}

struct TractorBeamShape {
}

impl DisplayListShape for TractorBeamShape {

    fn drawTractorBeam(&self, r : f32, g : f32, b : f32) {
    Screen.setColor(r, g, b, 0.5);
    glBegin(GL_QUADS);
    glVertex3f(-1, 0, -1);
    glVertex3f(1, 0, -1);
    glVertex3f(1, 0, 1);
    glVertex3f(-1, 0, 1);
    glEnd();
    Screen.setColor(r, g, b);
    glBegin(GL_LINE_LOOP);
    glVertex3f(-1, 0, -1);
    glVertex3f(1, 0, -1);
    glVertex3f(1, 0, 1);
    glVertex3f(-1, 0, 1);
    glEnd();
  }

  fn drawTractorBeamLine(f&self, r : f32, g : f32, b : f32) {
    Screen.setColor(r, g, b);
    glBegin(GL_LINE_LOOP);
    glVertex3f(-1, 0, -1);
    glVertex3f(1, 0, -1);
    glVertex3f(1, 0, 1);
    glVertex3f(-1, 0, 1);
    glEnd();
  }
}

struct TractorBeamShapeRed {
}

impl TractorBeamShape for TractorBeamShapeRed {
  fn drawList(&self) {
    self.drawTractorBeam(0.5, 0.2, 0.2);
  }
}

struct TractorBeamShapeBlue {
}

impl TractorBeamShape for TractorBeamShapeBlue {
    fn drawList(&self) {
     self.drawTractorBeam(0.2, 0.2, 0.5);
  }
}

struct TractorBeamShapePurple {
}

impl TractorBeamShape for TractorBeamShapePurple {
  fn drawList(&self) {
    drawTractorBeam(0.5, 0.2, 0.5);
  }
}

struct TractorBeamShapeDarkRed {
}

impl TractorBeamShape for TractorBeamShapeDarkRed {
    fn drawList() {
        drawTractorBeamLine(0.4, 0.1, 0.1);
    }
}

struct TractorBeamShapeDarkBlue {
}

impl TractorBeamShape for TractorBeamShapeDarkBlue {
  fn drawList() {
    drawTractorBeamLine(0.1, 0.1, 0.4);
  }
}

struct TractorBeamShapeDarkPurple {
}


impl TractorBeamShape for TractorBeamShapeDarkPurple {

  fn drawList() {
    drawTractorBeamLine(0.4, 0.1, 0.4);
  }
}

struct BulletShapeBase {
}

impl DisplayListShape for BulletShapeBase {

  fn draw(&self, pos : Vector3, cd : f32, deg : f32, rd : f32) {
    glPushMatrix();
    Screen.glTranslate(pos);
    glRotatef(cd * 180.0 / PI, 0, 1, 0);
    Screen.glRotate(deg);
    glRotatef(rd, 0, 1, 0);
    displayList.call();
    glPopMatrix();
  }
}

struct BulletShape {
}

impl BulletShapeBase for BulletShape {
  fn drawList() {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    Screen.setColor(0, 0, 0);
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0, 0.5, 0);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glVertex3f(-0.34, -0.3, -0.2);
    glEnd();
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
    glScalef(1.2, 1.2, 1.2);
    Screen.setColor(0.1, 0.3, 0.3);
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0, 0.5, 0);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glVertex3f(-0.34, -0.3, -0.2);
    glEnd();
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glEnd();
  }
}

struct BulletLineShape {
}

impl BulletShapeBase for BulletLineShape {
  fn drawList() {
    glScalef(1.2, 1.2, 1.2);
    glBegin(GL_LINES);
    glVertex3f(0, 0.5, 0);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0, 0.5, 0);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, 0.5, 0);
    glVertex3f(0, -0.3, 0.4);
    glEnd();
    glBegin(GL_LINE_LOOP);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glEnd();
  }
}

struct MiddleBulletShape {
}

impl BulletShapeBase for MiddleBulletShape {
  fn drawList() {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glScalef(1.1, 1.0, 1.1);
    Screen.setColor(0, 0, 0);
    glBegin(GL_QUADS);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glVertex3f(0, 0.3, 0.2);
    glVertex3f(0, 0.3, 0.2);
    glVertex3f(0, -0.3, 0.4);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(-0.17, 0.3, -0.1);
    glEnd();
    glBegin(GL_TRIANGLES);
    glVertex3f(-0.17, -0.3, -0.1);
    glVertex3f(0.17, -0.3, -0.1);
    glVertex3f(0, -0.3, 0.2);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
    glScalef(1.4, 1.3, 1.4);
    Screen.setColor(0.1, 0.2, 0.3);
    glBegin(GL_QUADS);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glVertex3f(0, 0.3, 0.2);
    glVertex3f(0, 0.3, 0.2);
    glVertex3f(0, -0.3, 0.4);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(-0.17, 0.3, -0.1);
    glEnd();
    glBegin(GL_TRIANGLES);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0, 0.3, 0.2);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glEnd();
  }
}

struct MiddleBulletLineShape {
}

impl BulletShapeBase for MiddleBulletLineShape {
  fn drawList() {
    glScalef(1.4, 1.3, 1.4);
    glBegin(GL_LINES);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, 0.3, 0.2);
    glVertex3f(0, -0.3, 0.4);
    glEnd();
    glBegin(GL_LINE_LOOP);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0, 0.3, 0.2);
    glEnd();
    glBegin(GL_LINE_LOOP);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0, -0.3, 0.4);
    glEnd();
  }
}

struct RollBulletShapeBase {
}

impl BulletShapeBase for RollBulletShapeBase {

  fn draw(pos : Vector3, cd : f32, deg : f32, rd : f32) {
    glPushMatrix();
    Screen.glTranslate(pos);
    glRotatef(cd * 180.0 / PI, 0, 1, 0);
    glRotatef(rd, 0, 0, 1);
    displayList.call();
    glPopMatrix();
  }
}

struct CounterBulletShape {
}

impl RollBulletShapeBase for CounterBulletShape {
  fn drawList() {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    Screen.setColor(0, 0, 0);
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0, 0, 0.5);
    glVertex3f(0.5, 0, 0);
    glVertex3f(0, 0.5, 0);
    glVertex3f(-0.5, 0, 0);
    glVertex3f(0, -0.5, 0);
    glVertex3f(0.5, 0, 0);
    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
    glScalef(1.2, 1.2, 1.2);
    Screen.setColor(0.5, 0.5, 0.5);
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0, 0, 0.5);
    glVertex3f(0.5, 0, 0);
    glVertex3f(0, 0.5, 0);
    glVertex3f(-0.5, 0, 0);
    glVertex3f(0, -0.5, 0);
    glVertex3f(0.5, 0, 0);
    glEnd();
  }
}

struct CounterBulletLineShape {
}

impl RollBulletShapeBase for CounterBulletLineShape {
  fn drawList() {
    glScalef(1.2, 1.2, 1.2);
    glBegin(GL_LINE_LOOP);
    glVertex3f(0.5, 0, 0);
    glVertex3f(0, 0.5, 0);
    glVertex3f(-0.5, 0, 0);
    glVertex3f(0, -0.5, 0);
    glEnd();
    glBegin(GL_LINES);
    glVertex3f(0, 0, 0.5);
    glVertex3f(0.5, 0, 0);
    glVertex3f(0, 0, 0.5);
    glVertex3f(0, 0.5, 0);
    glVertex3f(0, 0, 0.5);
    glVertex3f(-0.5, 0, 0);
    glVertex3f(0, 0, 0.5);
    glVertex3f(0, -0.5, 0);
    glEnd();
  }
}

struct EnemyShape {
}

impl DisplayListShape for EnemyShape {}
  fn draw(pos : Vector3, cd : f32, deg : f32, cnt : f32, size : Vector) {
    draw(pos, cd, deg, cnt, size.x, size.y);
  }

  fn draw(pos : Vector3, cd : f32, deg : f32, cnt : f32, sx : f32, sy : f32) {
    glPushMatrix();
    Screen.glTranslate(pos);
    glRotatef(cd * 180 / PI, 0, 1, 0);
    Screen.glRotate(deg);
    glScalef(sx, sy, 1);
    glRotatef(cnt * 3.0f, 0, 1, 0);
    displayList.call();
    glPopMatrix();
  }
}

struct Enemy1Shape {
}

impl EnemyShape for Enemy1Shape {
  fn drawList() {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glTranslatef(0, -0.6, 0);
    glScalef(0.5, 1.4, 0.5);
    PyramidShape.drawShadow(0.5, 0.5, 0.3);
    glPopMatrix();
    glPushMatrix();
    glRotatef(120, 0, 0, 1);
    glTranslatef(0.5, -0.2, 0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape.drawShadow(0.2, 0.2, 0.5);
    glPopMatrix();
    Screen.setColor(0.2, 0.2, 0.5);
    glPushMatrix();
    glRotatef(240, 0, 0, 1);
    glTranslatef(-0.5, -0.2, 0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape.drawShadow(0.2, 0.2, 0.5);
    glPopMatrix();
    Screen.setColor(1, 1, 0.6);
    glPushMatrix();
    glTranslatef(0, -0.6, 0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape.draw();
    glPopMatrix();
    Screen.setColor(0.5, 0.5, 1);
    glPushMatrix();
    glRotatef(120, 0, 0, 1);
    glTranslatef(0.5, -0.2, 0);
    glScalef(0.2, 0.8f, 0.2);
    PyramidShape.draw();
    glPopMatrix();
    Screen.setColor(0.5, 0.5, 1);
    glPushMatrix();
    glRotatef(240, 0, 0, 1);
    glTranslatef(-0.5, -0.2, 0);
    glScalef(0.2, 0.8f, 0.2);
    PyramidShape.draw();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct Enemy1TrailShape {
}

impl EnemyShape for Enemy1TrailShape {
  fn drawList() {
    glPushMatrix();
    glTranslatef(0, -0.6, 0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(120, 0, 0, 1);
    glTranslatef(0.5, -0.2, 0);
    glScalef(0.2, 0.8f, 0.2);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(240, 0, 0, 1);
    glTranslatef(-0.5, -0.2, 0);
    glScalef(0.2, 0.8f, 0.2);
    PyramidShape.drawLineShape();
    glPopMatrix();
  }
}

struct Enemy2Shape {
}

impl EnemyShape for Enemy2Shape {
  fn drawList() {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glTranslatef(0, -0.5, 0);
    glScalef(0.5, 1.2, 0.5);
    PyramidShape.drawShadow(0.5, 0.4, 0.5);
    glPopMatrix();
    glPushMatrix();
    glRotatef(60, 0, 0, 1);
    glTranslatef(0.6, -0.7, 0);
    glScalef(0.4, 1.4, 0.4);
    PyramidShape.drawShadow(0.9, 0.6, 0.5);
    glPopMatrix();
    glPushMatrix();
    glRotatef(300, 0, 0, 1);
    glTranslatef(-0.6, -0.7, 0);
    glScalef(0.4, 1.4, 0.4);
    PyramidShape.drawShadow(0.9, 0.6, 0.5);
    glPopMatrix();
    Screen.setColor(1, 0.9, 1.0);
    glPushMatrix();
    glTranslatef(0, -0.5, 0);
    glScalef(0.3, 1.0, 0.3);
    PyramidShape.draw();
    glPopMatrix();
    Screen.setColor(0.9, 0.6, 0.5);
    glPushMatrix();
    glRotatef(60, 0, 0, 1);
    glTranslatef(0.6, -0.7, 0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape.draw();
    glPopMatrix();
    Screen.setColor(0.9, 0.6, 0.5);
    glPushMatrix();
    glRotatef(300, 0, 0, 1);
    glTranslatef(-0.6, -0.7, 0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape.draw();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct Enemy2TrailShape {
}

impl EnemyShape for Enemy2TrailShape {
  fn drawList() {
    glPushMatrix();
    glTranslatef(0, -0.5, 0);
    glScalef(0.3, 1.0, 0.3);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(60, 0, 0, 1);
    glTranslatef(0.6, -0.7, 0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(300, 0, 0, 1);
    glTranslatef(-0.6, -0.7, 0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape.drawLineShape();
    glPopMatrix();
  }
}

struct Enemy3Shape {
}

impl EnemyShape for Enemy3Shape {
  fn drawList() {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glTranslatef(0, -0.4, 0);
    glScalef(0.5, 1.4, 0.5);
    PyramidShape.drawShadow(0.5, 0.5, 0.3);
    glPopMatrix();
    glPushMatrix();
    glRotatef(150, 0, 0, 1);
    glTranslatef(0.5, 0.2, 0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape.drawShadow(0.2, 0.2, 0.5);
    glPopMatrix();
    Screen.setColor(0.2, 0.2, 0.5);
    glPushMatrix();
    glRotatef(210, 0, 0, 1);
    glTranslatef(-0.5, 0.2, 0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape.drawShadow(0.2, 0.2, 0.5);
    glPopMatrix();
    Screen.setColor(1, 0.6, 0.9);
    glPushMatrix();
    glTranslatef(0, -0.4, 0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape.draw();
    glPopMatrix();
    Screen.setColor(0.3, 0.5, 1);
    glPushMatrix();
    glRotatef(150, 0, 0, 1);
    glTranslatef(0.5, 0.2, 0);
    glScalef(0.2, 0.8f, 0.2);
    PyramidShape.draw();
    glPopMatrix();
    Screen.setColor(0.3, 0.5, 1);
    glPushMatrix();
    glRotatef(210, 0, 0, 1);
    glTranslatef(-0.5, 0.2, 0);
    glScalef(0.2, 0.8f, 0.2);
    PyramidShape.draw();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct Enemy3TrailShape {
}

impl EnemyShape for Enemy3TrailShape {
  fn drawList() {
    glPushMatrix();
    glTranslatef(0, -0.4, 0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(150, 0, 0, 1);
    glTranslatef(0.5, 0.2, 0);
    glScalef(0.2, 0.8f, 0.2);
    PyramidShape.drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(210, 0, 0, 1);
    glTranslatef(-0.5, 0.2, 0);
    glScalef(0.2, 0.8f, 0.2);
    PyramidShape.drawLineShape();
    glPopMatrix();
  }
}

struct TriangleParticleShape {
}

impl DisplayListShape for TriangleParticleShape {
  fn drawList() {
    glBegin(GL_LINE_LOOP);
    glVertex3f(0, 0.5, 0);
    glVertex3f(0.4, -0.3, 0);
    glVertex3f(-0.4, -0.3, 0);
    glEnd();
  }
}

struct PillarShape {
}

const TICKNESS : f32 = 4.0;
const RADIUS_RATIO : f32 = 0.3;

impl DisplayListShape for PillarShape {
  fn drawPillar(r : f32, g : f32, b : f32, outside : bool /*= false*/)) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glBegin(GL_QUADS);
    Screen.setColor(r, g, b);
    for i in 0..8 {
      let mut d : f32 = PI * 2.0 * (i as f32) / 8.0;
      glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
      d += PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
      glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
      d -= PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
    }
    glEnd();
    if !self.outside {
      Screen.setColor(r, g, b);
      glBegin(GL_TRIANGLES);
      for i in 0..8 {
        let d : f32 = PI * 2.0 * i / 8.0;
        glVertex3f(0, TICKNESS, 0);
        glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                   TICKNESS,
                   d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
        d += PI * 2 / 8;
        glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                   TICKNESS,
                   d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
        d -= PI * 2 / 8;
        glVertex3f(0, -TICKNESS, 0);
        glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                   -TICKNESS,
                   d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
        d += PI * 2 / 8;
        glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                   -TICKNESS,
                   d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
      }
      glEnd();
    }
    Screen.setColor(0.1, 0.1, 0.1);
    for i in 0..8 {
      let mut d : f32 = PI * 2.0 * (i as f32) / 8.0; 
      glBegin(GL_LINE_STRIP);
      glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
      d += PI * 2 / 8;
      glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
      glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
      d -= PI * 2 / 8;
      glVertex3f(d.sin() * Field.CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field.CIRCLE_RADIUS * RADIUS_RATIO);
      glEnd();
    }
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }

  fn draw(y : f32, deg : f32) {
    glPushMatrix();
    glTranslatef(0, y, 0);
    glRotatef(deg * 180.0 / PI, 0, 1, 0);
    displayList.call();
    glPopMatrix();
  }
}

struct Pillar1Shape {
}

impl PillarShape for Pillar1Shape{
  fn drawList() {
    glScalef(0.6, 1.0, 0.6);
    drawPillar(0.5, 0.4, 0.4);
  }
}

struct Pillar2Shape {
}

impl PillarShape for Pillar2Shape {
  fn drawList() {
    glScalef(0.8f, 1.0, 0.8f);
    drawPillar(0.6, 0.3, 0.3);
  }
}

struct Pillar3Shape {
}

impl PillarShape for Pillar3Shape {
  fn drawList() {
    drawPillar(0.5, 0.5, 0.4);
  }
}

struct Pillar4Shape {
}

impl PillarShape for Pillar4Shape {
  fn drawList() {
    glScalef(1.1, 1.0, 1.1);
    drawPillar(0.5, 0.4, 0.5);
  }
}


struct OutsidePillarShape{    
}

impl PillarShape for OutsidePillarShape {
  fn drawList() {
    glScalef(7.0f, 3.0f, 7.0f);
    drawPillar(0.2, 0.2, 0.3, true);
  }
}

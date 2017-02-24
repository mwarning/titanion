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

use std::f32::consts::PI;
use std::marker::Sized;

use util::vector::Vector;
use util::vector::Vector3;
use util::sdl::displaylist::DisplayList;
use ttn::dummy::*;

/*
//Dummy
struct DisplayList {
    size : usize,
}

impl DisplayList {
    fn new(num : usize) -> DisplayList {
        DisplayList{size : num}
    }
}*/


//#############

/**
 * 3D shapes of a player, enemies, particles, etc.
 */

pub trait Shape /*: Default*/ {

    fn get_display_list(&mut self) -> &mut DisplayList;

    fn draw4(&mut self, pos : Vector3, cd : f32, deg : f32) {
        let dl = self.get_display_list();
        glPushMatrix();
        Screen_glTranslate(pos);
        glRotatef(cd * 180.0 / PI, 0.0, 1.0, 0.0);
        Screen_glRotate(deg);
        dl.call(0);
        glPopMatrix();
    }
}

trait DisplayListShape : Shape {
    fn drawList(&self);

    //was this()
    fn new() -> Self
        where Self : Default
    {
        let mut inst : Self = Default::default();
        inst.get_display_list().beginNewList();
        inst.drawList();
        inst.get_display_list().endNewList();
        inst
    }

    fn draw(&mut self) {
        self.drawList()
    }

    fn close(&mut self) {
        let dl = self.get_display_list();
        dl.close();
    }
}

fn PyramidShape_draw() {
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0.0, 0.0, 0.0);
    glVertex3f(1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, 1.0);
    glEnd();
    Screen_setColor(0.1, 0.1, 0.1, 0.5);
    glBegin(GL_LINE_STRIP);
    glVertex3f(0.0, 0.0, 0.0);
    glVertex3f(1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, -1.0);
    glVertex3f(0.0, 0.0, 0.0);
    glVertex3f(-1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, 1.0);
    glVertex3f(0.0, 0.0, 0.0);
    glEnd();
    glBegin(GL_LINES);
    glVertex3f(1.0, 1.0, 1.0);
    glVertex3f(-1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, -1.0);
    glEnd();
}

fn PyramidShape_drawShadow(r : f32, g : f32, b : f32, noAlpha : bool /*= false*/) {
    glBegin(GL_TRIANGLE_FAN);
    Screen_setColor(r, g, b, 1.0);
    glVertex3f(0.0, 0.0, 0.0);
    if !noAlpha {
      Screen_setColor(r * 0.75, g * 0.75, b * 0.75, 0.33);
    } else {
      Screen_setColor(r * 0.75, g * 0.75, b * 0.75, 0.75);
    }
    glVertex3f(1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, 1.0);
    glEnd();
}

fn PyramidShape_drawPolygonShape() {
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0.0, 0.0, 0.0);
    glVertex3f(1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, 1.0);
    glEnd();
}

fn PyramidShape_drawLineShape() {
    glBegin(GL_LINE_STRIP);
    glVertex3f(0.0, 0.0, 0.0);
    glVertex3f(1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, -1.0);
    glVertex3f(0.0, 0.0, 0.0);
    glVertex3f(-1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, 1.0);
    glVertex3f(0.0, 0.0, 0.0);
    glEnd();
    glBegin(GL_LINES);
    glVertex3f(1.0, 1.0, 1.0);
    glVertex3f(-1.0, 1.0, 1.0);
    glVertex3f(1.0, 1.0, -1.0);
    glVertex3f(-1.0, 1.0, -1.0);
    glEnd();
}

struct PlayerShape {
    displayList : DisplayList,
}

impl Default for PlayerShape {
    fn default() -> PlayerShape { PlayerShape{ displayList: DisplayList::new(1) } }
}

impl Shape for PlayerShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl DisplayListShape for PlayerShape {
  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(0.0, -0.6, 0.0);
    glScalef(0.4, 1.3, 0.4);
    PyramidShape_drawShadow(1.0, 0.5, 0.5, true);
    glPopMatrix();
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, -0.2, 0.0);
    glScalef(0.3, 0.9, 0.3);
    PyramidShape_drawShadow(1.0, 1.0, 1.0, true);
    glPopMatrix();
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.2, 0.0);
    glScalef(0.3, 0.9, 0.3);
    PyramidShape_drawShadow(1.0, 1.0, 1.0, true);
    glPopMatrix();
    Screen_setColor(1.0, 0.5, 0.5, 1.0);
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(0.0, -0.6, 0.0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape_drawPolygonShape();
    glPopMatrix();
    Screen_setColor(1.0, 1.0, 1.0, 1.0);
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawPolygonShape();
    glPopMatrix();
    Screen_setColor(1.0, 1.0, 1.0, 1.0);
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawPolygonShape();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct PlayerLineShape {
    displayList : DisplayList,
}

impl Default for PlayerLineShape {
    fn default() -> PlayerLineShape { PlayerLineShape{ displayList: DisplayList::new(1) } }
}

impl Shape for PlayerLineShape {
  fn get_display_list(&mut self) -> &mut DisplayList {
    &mut self.displayList
  }
}

impl DisplayListShape for PlayerLineShape {
 fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(0.0, -0.6, 0.0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct ShotShape {
    displayList : DisplayList,
}

impl Default for ShotShape {
    fn default() -> ShotShape { ShotShape{ displayList: DisplayList::new(1) } }
}

impl Shape for ShotShape {
  fn get_display_list(&mut self) -> &mut DisplayList {
    &mut self.displayList
  }
}

impl DisplayListShape for ShotShape {
  fn drawList(&self) {
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, -0.5, 0.0);
    glScalef(0.1, 1.0, 0.1);
    Screen_setColor(0.4, 0.2, 0.8, 1.0);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.5, 0.0);
    glScalef(0.1, 1.0, 0.1);
    Screen_setColor(0.4, 0.2, 0.8, 1.0);
    PyramidShape_drawLineShape();
    glPopMatrix();
  }
}

trait TractorBeamShape : DisplayListShape {
    fn drawTractorBeam(&self, r : f32, g : f32, b : f32) {
    Screen_setColor(r, g, b, 0.5);
    glBegin(GL_QUADS);
    glVertex3f(-1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, 1.0);
    glVertex3f(-1.0, 0.0, 1.0);
    glEnd();
    Screen_setColor(r, g, b, 1.0);
    glBegin(GL_LINE_LOOP);
    glVertex3f(-1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, 1.0);
    glVertex3f(-1.0, 0.0, 1.0);
    glEnd();
  }

  fn drawTractorBeamLine(&self, r : f32, g : f32, b : f32) {
    Screen_setColor(r, g, b, 1.0);
    glBegin(GL_LINE_LOOP);
    glVertex3f(-1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, 1.0);
    glVertex3f(-1.0, 0.0, 1.0);
    glEnd();
  }
}

struct TractorBeamShapeRed {
    displayList : DisplayList,
}

impl Default for TractorBeamShapeRed {
    fn default() -> TractorBeamShapeRed { TractorBeamShapeRed{ displayList: DisplayList::new(1) } }
}

impl Shape for TractorBeamShapeRed {
  fn get_display_list(&mut self) -> &mut DisplayList {
    &mut self.displayList
  }
}

impl TractorBeamShape for TractorBeamShapeRed {
}

impl DisplayListShape for TractorBeamShapeRed {
  fn drawList(&self) {
    self.drawTractorBeam(0.5, 0.2, 0.2);
  }
}

struct TractorBeamShapeBlue {
    displayList : DisplayList, 
}

impl Default for TractorBeamShapeBlue {
    fn default() -> TractorBeamShapeBlue { TractorBeamShapeBlue{ displayList: DisplayList::new(1) } }
}

impl Shape for TractorBeamShapeBlue {
  fn get_display_list(&mut self) -> &mut DisplayList {
    &mut self.displayList
  }
}

impl TractorBeamShape for TractorBeamShapeBlue {
}

impl DisplayListShape for TractorBeamShapeBlue {
    fn drawList(&self) {
     self.drawTractorBeam(0.2, 0.2, 0.5);
  }
}

struct TractorBeamShapePurple {
    displayList : DisplayList,
}

impl Default for TractorBeamShapePurple {
    fn default() -> TractorBeamShapePurple { TractorBeamShapePurple{ displayList: DisplayList::new(1) } }
}

impl Shape for TractorBeamShapePurple {
  fn get_display_list(&mut self) -> &mut DisplayList {
    &mut self.displayList
  }
}

impl TractorBeamShape for TractorBeamShapePurple {
}

impl DisplayListShape for TractorBeamShapePurple {
  fn drawList(&self) {
    self.drawTractorBeam(0.5, 0.2, 0.5);
  }
}

struct TractorBeamShapeDarkRed {
    displayList : DisplayList,
}

impl Default for TractorBeamShapeDarkRed {
    fn default() -> TractorBeamShapeDarkRed { TractorBeamShapeDarkRed{ displayList: DisplayList::new(1) } }
}

impl Shape for TractorBeamShapeDarkRed {
  fn get_display_list(&mut self) -> &mut DisplayList {
    &mut self.displayList
  }
}

impl TractorBeamShape for TractorBeamShapeDarkRed {
}

impl DisplayListShape for TractorBeamShapeDarkRed {
    fn drawList(&self) {
        self.drawTractorBeamLine(0.4, 0.1, 0.1);
    }
}

struct TractorBeamShapeDarkBlue {
    displayList : DisplayList,
}

impl Default for TractorBeamShapeDarkBlue {
    fn default() -> TractorBeamShapeDarkBlue { TractorBeamShapeDarkBlue{ displayList: DisplayList::new(1) } }
}

impl Shape for TractorBeamShapeDarkBlue {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl TractorBeamShape for TractorBeamShapeDarkBlue {
}

impl DisplayListShape for TractorBeamShapeDarkBlue {
  fn drawList(&self) {
    self.drawTractorBeamLine(0.1, 0.1, 0.4);
  }
}

struct TractorBeamShapeDarkPurple {
    displayList : DisplayList,
}

impl Default for TractorBeamShapeDarkPurple {
    fn default() -> TractorBeamShapeDarkPurple { TractorBeamShapeDarkPurple{ displayList: DisplayList::new(1) } }
}

impl Shape for TractorBeamShapeDarkPurple {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl TractorBeamShape for TractorBeamShapeDarkPurple {
}

impl DisplayListShape for TractorBeamShapeDarkPurple {
  fn drawList(&self) {
    self.drawTractorBeamLine(0.4, 0.1, 0.4);
  }
}

pub trait BulletShapeBase : DisplayListShape {
  fn draw5(&mut self, pos : Vector3, cd : f32, deg : f32, rd : f32) {
    glPushMatrix();
    Screen_glTranslate(pos);
    glRotatef(cd * 180.0 / PI, 0.0, 1.0, 0.0);
    Screen_glRotate(deg);
    glRotatef(rd, 0.0, 1.0, 0.0);
    self.get_display_list().call(0);
    glPopMatrix();
  }
}

pub struct BulletShape {
    displayList : DisplayList,
}

impl Default for BulletShape {
    fn default() -> BulletShape { BulletShape{ displayList: DisplayList::new(1) } }
}

impl Shape for BulletShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl DisplayListShape for BulletShape {
  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    Screen_setColor(0.0, 0.0, 0.0, 1.0);
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glVertex3f(-0.34, -0.3, -0.2);
    glEnd();
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
    glScalef(1.2, 1.2, 1.2);
    Screen_setColor(0.1, 0.3, 0.3, 1.0);
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glVertex3f(-0.34, -0.3, -0.2);
    glEnd();
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glEnd();
  }
}

struct BulletLineShape {
    displayList : DisplayList,
}

impl Default for BulletLineShape {
    fn default() -> BulletLineShape { BulletLineShape{ displayList: DisplayList::new(1) } }
}

impl Shape for BulletLineShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl DisplayListShape for BulletLineShape {
  fn drawList(&self) {
    glScalef(1.2, 1.2, 1.2);
    glBegin(GL_LINES);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(0.0, -0.3, 0.4);
    glEnd();
    glBegin(GL_LINE_LOOP);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glEnd();
  }
}

struct MiddleBulletShape {
    displayList : DisplayList,
}

impl Default for MiddleBulletShape {
    fn default() -> MiddleBulletShape { MiddleBulletShape{ displayList: DisplayList::new(1) } }
}

impl Shape for MiddleBulletShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl DisplayListShape for MiddleBulletShape {
  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glScalef(1.1, 1.0, 1.1);
    Screen_setColor(0.0, 0.0, 0.0, 1.0);
    glBegin(GL_QUADS);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glVertex3f(0.0, 0.3, 0.2);
    glVertex3f(0.0, 0.3, 0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(-0.17, 0.3, -0.1);
    glEnd();
    glBegin(GL_TRIANGLES);
    glVertex3f(-0.17, -0.3, -0.1);
    glVertex3f(0.17, -0.3, -0.1);
    glVertex3f(0.0, -0.3, 0.2);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
    glScalef(1.4, 1.3, 1.4);
    Screen_setColor(0.1, 0.2, 0.3, 1.0);
    glBegin(GL_QUADS);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glVertex3f(0.0, 0.3, 0.2);
    glVertex3f(0.0, 0.3, 0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(-0.17, 0.3, -0.1);
    glEnd();
    glBegin(GL_TRIANGLES);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.0, 0.3, 0.2);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glEnd();
  }
}

struct MiddleBulletLineShape {
    displayList : DisplayList,
}

impl Default for MiddleBulletLineShape {
    fn default() -> MiddleBulletLineShape { MiddleBulletLineShape{ displayList: DisplayList::new(1) } }
}

impl Shape for MiddleBulletLineShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl DisplayListShape for MiddleBulletLineShape {
  fn drawList(&self) {
    glScalef(1.4, 1.3, 1.4);
    glBegin(GL_LINES);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, 0.3, 0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glEnd();
    glBegin(GL_LINE_LOOP);
    glVertex3f(-0.17, 0.3, -0.1);
    glVertex3f(0.17, 0.3, -0.1);
    glVertex3f(0.0, 0.3, 0.2);
    glEnd();
    glBegin(GL_LINE_LOOP);
    glVertex3f(-0.34, -0.3, -0.2);
    glVertex3f(0.34, -0.3, -0.2);
    glVertex3f(0.0, -0.3, 0.4);
    glEnd();
  }
}

trait RollBulletShapeBase : BulletShapeBase {
  fn draw5(&mut self, pos : Vector3, cd : f32, deg : f32, rd : f32) {
    glPushMatrix();
    Screen_glTranslate(pos);
    glRotatef(cd * 180.0 / PI, 0.0, 1.0, 0.0);
    glRotatef(rd, 0.0, 0.0, 1.0);
    self.get_display_list().call(0);
    glPopMatrix();
  }
}

struct CounterBulletShape {
    displayList : DisplayList,
}

impl Default for CounterBulletShape {
    fn default() -> CounterBulletShape { CounterBulletShape{ displayList: DisplayList::new(1) } }
}

impl Shape for CounterBulletShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl DisplayListShape for CounterBulletShape {
  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    Screen_setColor(0.0, 0.0, 0.0, 1.0);
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0.0, 0.0, 0.5);
    glVertex3f(0.5, 0.0, 0.0);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(-0.5, 0.0, 0.0);
    glVertex3f(0.0, -0.5, 0.0);
    glVertex3f(0.5, 0.0, 0.0);
    glEnd();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
    glScalef(1.2, 1.2, 1.2);
    Screen_setColor(0.5, 0.5, 0.5, 1.0);
    glBegin(GL_TRIANGLE_FAN);
    glVertex3f(0.0, 0.0, 0.5);
    glVertex3f(0.5, 0.0, 0.0);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(-0.5, 0.0, 0.0);
    glVertex3f(0.0, -0.5, 0.0);
    glVertex3f(0.5, 0.0, 0.0);
    glEnd();
  }
}

struct CounterBulletLineShape {
    displayList : DisplayList,
}

impl Default for CounterBulletLineShape {
    fn default() -> CounterBulletLineShape { CounterBulletLineShape{ displayList: DisplayList::new(1) } }
}

impl Shape for CounterBulletLineShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl DisplayListShape for CounterBulletLineShape {
  fn drawList(&self) {
    glScalef(1.2, 1.2, 1.2);
    glBegin(GL_LINE_LOOP);
    glVertex3f(0.5, 0.0, 0.0);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(-0.5, 0.0, 0.0);
    glVertex3f(0.0, -0.5, 0.0);
    glEnd();
    glBegin(GL_LINES);
    glVertex3f(0.0, 0.0, 0.5);
    glVertex3f(0.5, 0.0, 0.0);
    glVertex3f(0.0, 0.0, 0.5);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(0.0, 0.0, 0.5);
    glVertex3f(-0.5, 0.0, 0.0);
    glVertex3f(0.0, 0.0, 0.5);
    glVertex3f(0.0, -0.5, 0.0);
    glEnd();
  }
}

pub trait EnemyShape : DisplayListShape {

  fn draw6(&mut self, pos : Vector3, cd : f32, deg : f32, cnt : f32, size : Vector) {
    self.draw7(pos, cd, deg, cnt, size.x, size.y);
  }

  fn draw7(&mut self, pos : Vector3, cd : f32, deg : f32, cnt : f32, sx : f32, sy : f32) {
    glPushMatrix();
    Screen_glTranslate(pos);
    glRotatef(cd * 180.0 / PI, 0.0, 1.0, 0.0);
    Screen_glRotate(deg);
    glScalef(sx, sy, 1.0);
    glRotatef(cnt * 3.0, 0.0, 1.0, 0.0);
    self.get_display_list().call(0);
    glPopMatrix();
  }
}

struct Enemy1Shape {
    displayList : DisplayList,
}

impl Default for Enemy1Shape {
    fn default() -> Enemy1Shape { Enemy1Shape{ displayList: DisplayList::new(1) } }
}

impl Shape for Enemy1Shape {
   fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl EnemyShape for Enemy1Shape {
}

impl DisplayListShape for Enemy1Shape {
  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glTranslatef(0.0, -0.6, 0.0);
    glScalef(0.5, 1.4, 0.5);
    PyramidShape_drawShadow(0.5, 0.5, 0.3, false);
    glPopMatrix();
    glPushMatrix();
    glRotatef(120.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, -0.2, 0.0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape_drawShadow(0.2, 0.2, 0.5, false);
    glPopMatrix();
    Screen_setColor(0.2, 0.2, 0.5, 1.0);
    glPushMatrix();
    glRotatef(240.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.2, 0.0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape_drawShadow(0.2, 0.2, 0.5, false);
    glPopMatrix();
    Screen_setColor(1.0, 1.0, 0.6, 1.0);
    glPushMatrix();
    glTranslatef(0.0, -0.6, 0.0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape_draw();
    glPopMatrix();
    Screen_setColor(0.5, 0.5, 1.0, 1.0);
    glPushMatrix();
    glRotatef(120.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    Screen_setColor(0.5, 0.5, 1.0, 1.0);
    glPushMatrix();
    glRotatef(240.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct Enemy1TrailShape {
   displayList : DisplayList,
}

impl Default for Enemy1TrailShape {
    fn default() -> Enemy1TrailShape { Enemy1TrailShape{ displayList: DisplayList::new(1) } }
}

impl Shape for Enemy1TrailShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl EnemyShape for Enemy1TrailShape {
}

impl DisplayListShape for Enemy1TrailShape {
  fn drawList(&self) {
    glPushMatrix();
    glTranslatef(0.0, -0.6, 0.0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(120.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(240.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawLineShape();
    glPopMatrix();
  }
}

struct Enemy2Shape {
    displayList : DisplayList,
}

impl Default for Enemy2Shape {
    fn default() -> Enemy2Shape { Enemy2Shape{ displayList: DisplayList::new(1) } }
}

impl Shape for Enemy2Shape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl EnemyShape for Enemy2Shape {
}

impl DisplayListShape for Enemy2Shape {
  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glTranslatef(0.0, -0.5, 0.0);
    glScalef(0.5, 1.2, 0.5);
    PyramidShape_drawShadow(0.5, 0.4, 0.5, false);
    glPopMatrix();
    glPushMatrix();
    glRotatef(60.0, 0.0, 0.0, 1.0);
    glTranslatef(0.6, -0.7, 0.0);
    glScalef(0.4, 1.4, 0.4);
    PyramidShape_drawShadow(0.9, 0.6, 0.5, false);
    glPopMatrix();
    glPushMatrix();
    glRotatef(300.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.6, -0.7, 0.0);
    glScalef(0.4, 1.4, 0.4);
    PyramidShape_drawShadow(0.9, 0.6, 0.5, false);
    glPopMatrix();
    Screen_setColor(1.0, 0.9, 1.0, 1.0);
    glPushMatrix();
    glTranslatef(0.0, -0.5, 0.0);
    glScalef(0.3, 1.0, 0.3);
    PyramidShape_draw();
    glPopMatrix();
    Screen_setColor(0.9, 0.6, 0.5, 1.0);
    glPushMatrix();
    glRotatef(60.0, 0.0, 0.0, 1.0);
    glTranslatef(0.6, -0.7, 0.0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    Screen_setColor(0.9, 0.6, 0.5, 1.0);
    glPushMatrix();
    glRotatef(300.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.6, -0.7, 0.0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct Enemy2TrailShape {
    displayList : DisplayList,
}

impl Default for Enemy2TrailShape {
    fn default() -> Enemy2TrailShape { Enemy2TrailShape{ displayList: DisplayList::new(1) } }
}

impl Shape for Enemy2TrailShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl EnemyShape for Enemy2TrailShape {
}

impl DisplayListShape for Enemy2TrailShape {
  fn drawList(&self) {
    glPushMatrix();
    glTranslatef(0.0, -0.5, 0.0);
    glScalef(0.3, 1.0, 0.3);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(60.0, 0.0, 0.0, 1.0);
    glTranslatef(0.6, -0.7, 0.0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(300.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.6, -0.7, 0.0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape_drawLineShape();
    glPopMatrix();
  }
}

struct Enemy3Shape {
    displayList : DisplayList,
}

impl Default for Enemy3Shape {
    fn default() -> Enemy3Shape { Enemy3Shape{ displayList: DisplayList::new(1) } }
}

impl Shape for Enemy3Shape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl EnemyShape for Enemy3Shape {
}

impl DisplayListShape for Enemy3Shape {
  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glPushMatrix();
    glTranslatef(0.0, -0.4, 0.0);
    glScalef(0.5, 1.4, 0.5);
    PyramidShape_drawShadow(0.5, 0.5, 0.3, false);
    glPopMatrix();
    glPushMatrix();
    glRotatef(150.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, 0.2, 0.0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape_drawShadow(0.2, 0.2, 0.5, false);
    glPopMatrix();
    Screen_setColor(0.2, 0.2, 0.5, 1.0);
    glPushMatrix();
    glRotatef(210.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, 0.2, 0.0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape_drawShadow(0.2, 0.2, 0.5, false);
    glPopMatrix();
    Screen_setColor(1.0, 0.6, 0.9, 1.0);
    glPushMatrix();
    glTranslatef(0.0, -0.4, 0.0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape_draw();
    glPopMatrix();
    Screen_setColor(0.3, 0.5, 1.0, 1.0);
    glPushMatrix();
    glRotatef(150.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, 0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    Screen_setColor(0.3, 0.5, 1.0, 1.0);
    glPushMatrix();
    glRotatef(210.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, 0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

struct Enemy3TrailShape {
    displayList : DisplayList,
}

impl Default for Enemy3TrailShape {
    fn default() -> Enemy3TrailShape { Enemy3TrailShape{ displayList: DisplayList::new(1) } }
}

impl Shape for Enemy3TrailShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl EnemyShape for Enemy3TrailShape {
}

impl DisplayListShape for Enemy3TrailShape {
  fn drawList(&self) {
    glPushMatrix();
    glTranslatef(0.0, -0.4, 0.0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(150.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, 0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(210.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, 0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawLineShape();
    glPopMatrix();
  }
}

struct TriangleParticleShape {
    displayList : DisplayList,
}

impl Default for TriangleParticleShape {
    fn default() -> TriangleParticleShape { TriangleParticleShape{ displayList: DisplayList::new(1) } }
}

impl Shape for TriangleParticleShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl EnemyShape for TriangleParticleShape {
}

impl DisplayListShape for TriangleParticleShape {
  fn drawList(&self) {
    glBegin(GL_LINE_LOOP);
    glVertex3f(0.0, 0.5, 0.0);
    glVertex3f(0.4, -0.3, 0.0);
    glVertex3f(-0.4, -0.3, 0.0);
    glEnd();
  }
}

const TICKNESS : f32 = 4.0;
const RADIUS_RATIO : f32 = 0.3;

trait PillarShape : DisplayListShape {
  fn drawPillar(&self, r : f32, g : f32, b : f32, outside : bool /*= false*/) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glBegin(GL_QUADS);
    Screen_setColor(r, g, b, 1.0);
    for i in 0..8 {
      let mut d : f32 = PI * 2.0 * (i as f32) / 8.0;
      glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
      d += PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
      glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
      d -= PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
    }
    glEnd();
    if !outside {
      Screen_setColor(r, g, b, 1.0);
      glBegin(GL_TRIANGLES);
      for i in 0..8 {
        let mut d : f32 = PI * 2.0 * (i as f32) / 8.0;
        glVertex3f(0.0, TICKNESS, 0.0);
        glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                   TICKNESS,
                   d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
        d += PI * 2.0 / 8.0;
        glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                   TICKNESS,
                   d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
        d -= PI * 2.0 / 8.0;
        glVertex3f(0.0, -TICKNESS, 0.0);
        glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                   -TICKNESS,
                   d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
        d += PI * 2.0 / 8.0;
        glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                   -TICKNESS,
                   d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
      }
      glEnd();
    }
    Screen_setColor(0.1, 0.1, 0.1, 1.0);
    for i in 0..8 {
      let mut d : f32 = PI * 2.0 * (i as f32) / 8.0; 
      glBegin(GL_LINE_STRIP);
      glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
      d += PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
      glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
      d -= PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field_CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field_CIRCLE_RADIUS * RADIUS_RATIO);
      glEnd();
    }
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }

  fn draw3(&mut self, y : f32, deg : f32) {
    glPushMatrix();
    glTranslatef(0.0, y, 0.0);
    glRotatef(deg * 180.0 / PI, 0.0, 1.0, 0.0);
    self.get_display_list().call(0);
    glPopMatrix();
  }
}

struct Pillar1Shape {
    displayList : DisplayList,
}

impl Default for Pillar1Shape {
    fn default() -> Pillar1Shape { Pillar1Shape{ displayList: DisplayList::new(1) } }
}

impl Shape  for Pillar1Shape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl PillarShape for Pillar1Shape {
}

impl DisplayListShape for Pillar1Shape {
  fn drawList(&self) {
    glScalef(0.6, 1.0, 0.6);
    self.drawPillar(0.5, 0.4, 0.4, false);
  }
}

struct Pillar2Shape {
    displayList : DisplayList,
}

impl Default for Pillar2Shape
 {
    fn default() -> Pillar2Shape { Pillar2Shape{ displayList: DisplayList::new(1) } }
}

impl Shape for Pillar2Shape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl PillarShape for Pillar2Shape {
}

impl DisplayListShape for Pillar2Shape {
  fn drawList(&self) {
    glScalef(0.8, 1.0, 0.8);
    self.drawPillar(0.6, 0.3, 0.3, false);
  }
}

struct Pillar3Shape {
    displayList : DisplayList,
}

impl Default for Pillar3Shape {
    fn default() -> Pillar3Shape { Pillar3Shape{ displayList: DisplayList::new(1) } }
}

impl Shape for Pillar3Shape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl PillarShape for Pillar3Shape {
}

impl DisplayListShape for Pillar3Shape {
  fn drawList(&self) {
    self.drawPillar(0.5, 0.5, 0.4, false);
  }
}

struct Pillar4Shape {
    displayList : DisplayList,
}

impl Default for Pillar4Shape {
    fn default() -> Pillar4Shape { Pillar4Shape{ displayList: DisplayList::new(1) } }
}

impl Shape for Pillar4Shape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl PillarShape for Pillar4Shape {
}

impl DisplayListShape for Pillar4Shape {
  fn drawList(&self) {
    glScalef(1.1, 1.0, 1.1);
    self.drawPillar(0.5, 0.4, 0.5, false);
  }
}

struct OutsidePillarShape {
    displayList : DisplayList,  
}

impl Default for OutsidePillarShape {
    fn default() -> OutsidePillarShape { OutsidePillarShape{ displayList: DisplayList::new(1) } }
}

impl Shape for OutsidePillarShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl PillarShape for OutsidePillarShape {
}

impl DisplayListShape for OutsidePillarShape {
  fn drawList(&self) {
    glScalef(7.0, 3.0, 7.0);
    self.drawPillar(0.2, 0.2, 0.3, true);
  }
}

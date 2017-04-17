/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::f32::consts::PI;
use std::marker::Sized;

use util::vector::Vector;
use util::vector::Vector3;
use util::sdl::displaylist::DisplayList;
use ttn::field::*;
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
        Screen::glTranslate(pos);
        glRotatef(cd * 180.0 / PI, 0.0, 1.0, 0.0);
        Screen::glRotate(deg);
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
    Screen::setColor(0.1, 0.1, 0.1, 0.5);
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
    Screen::setColor(r, g, b, 1.0);
    glVertex3f(0.0, 0.0, 0.0);
    if !noAlpha {
      Screen::setColor(r * 0.75, g * 0.75, b * 0.75, 0.33);
    } else {
      Screen::setColor(r * 0.75, g * 0.75, b * 0.75, 0.75);
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

pub struct PlayerShape {
    displayList : DisplayList,
}

impl PlayerShape {
    fn new() -> PlayerShape { PlayerShape{ displayList: DisplayList::new(1) } }
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
    Screen::setColor(1.0, 0.5, 0.5, 1.0);
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(0.0, -0.6, 0.0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape_drawPolygonShape();
    glPopMatrix();
    Screen::setColor(1.0, 1.0, 1.0, 1.0);
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawPolygonShape();
    glPopMatrix();
    Screen::setColor(1.0, 1.0, 1.0, 1.0);
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_drawPolygonShape();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

pub struct PlayerLineShape {
    displayList : DisplayList,
}

impl PlayerLineShape {
    fn new() -> PlayerLineShape { PlayerLineShape{ displayList: DisplayList::new(1) } }
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

impl ShotShape {
    fn new() -> ShotShape { ShotShape{ displayList: DisplayList::new(1) } }
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
    Screen::setColor(0.4, 0.2, 0.8, 1.0);
    PyramidShape_drawLineShape();
    glPopMatrix();
    glPushMatrix();
    glRotatef(180.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.5, 0.0);
    glScalef(0.1, 1.0, 0.1);
    Screen::setColor(0.4, 0.2, 0.8, 1.0);
    PyramidShape_drawLineShape();
    glPopMatrix();
  }
}

pub trait TractorBeamShape : DisplayListShape {
    fn drawTractorBeam(&self, r : f32, g : f32, b : f32) {
    Screen::setColor(r, g, b, 0.5);
    glBegin(GL_QUADS);
    glVertex3f(-1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, 1.0);
    glVertex3f(-1.0, 0.0, 1.0);
    glEnd();
    Screen::setColor(r, g, b, 1.0);
    glBegin(GL_LINE_LOOP);
    glVertex3f(-1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, 1.0);
    glVertex3f(-1.0, 0.0, 1.0);
    glEnd();
  }

  fn drawTractorBeamLine(&self, r : f32, g : f32, b : f32) {
    Screen::setColor(r, g, b, 1.0);
    glBegin(GL_LINE_LOOP);
    glVertex3f(-1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, -1.0);
    glVertex3f(1.0, 0.0, 1.0);
    glVertex3f(-1.0, 0.0, 1.0);
    glEnd();
  }
}

pub struct TractorBeamShapeRed {
    displayList : DisplayList,
}

impl TractorBeamShapeRed {
    fn new() -> TractorBeamShapeRed { TractorBeamShapeRed{ displayList: DisplayList::new(1) } }
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

pub struct TractorBeamShapeBlue {
    displayList : DisplayList, 
}

impl TractorBeamShapeBlue {
    fn new() -> TractorBeamShapeBlue { TractorBeamShapeBlue{ displayList: DisplayList::new(1) } }
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

pub struct TractorBeamShapePurple {
    displayList : DisplayList,
}

impl TractorBeamShapePurple {
    fn new() -> TractorBeamShapePurple { TractorBeamShapePurple{ displayList: DisplayList::new(1) } }
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

pub struct TractorBeamShapeDarkRed {
    displayList : DisplayList,
}

impl TractorBeamShapeDarkRed {
    fn new() -> TractorBeamShapeDarkRed { TractorBeamShapeDarkRed{ displayList: DisplayList::new(1) } }
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

pub struct TractorBeamShapeDarkBlue {
    displayList : DisplayList,
}

impl TractorBeamShapeDarkBlue {
    fn new() -> TractorBeamShapeDarkBlue { TractorBeamShapeDarkBlue{ displayList: DisplayList::new(1) } }
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

pub struct TractorBeamShapeDarkPurple {
    displayList : DisplayList,
}

impl TractorBeamShapeDarkPurple {
    fn new() -> TractorBeamShapeDarkPurple { TractorBeamShapeDarkPurple{ displayList: DisplayList::new(1) } }
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
    Screen::glTranslate(pos);
    glRotatef(cd * 180.0 / PI, 0.0, 1.0, 0.0);
    Screen::glRotate(deg);
    glRotatef(rd, 0.0, 1.0, 0.0);
    self.get_display_list().call(0);
    glPopMatrix();
  }
}

pub struct BulletShape {
    displayList : DisplayList,
}

impl BulletShape {
    fn new() -> BulletShape { BulletShape{ displayList: DisplayList::new(1) } }
}

impl Shape for BulletShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl DisplayListShape for BulletShape {
  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    Screen::setColor(0.0, 0.0, 0.0, 1.0);
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
    Screen::setColor(0.1, 0.3, 0.3, 1.0);
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

pub struct BulletLineShape {
    displayList : DisplayList,
}

impl BulletLineShape {
    fn new() -> BulletLineShape { BulletLineShape{ displayList: DisplayList::new(1) } }
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

pub struct MiddleBulletShape {
    displayList : DisplayList,
}

impl MiddleBulletShape {
    fn new() -> MiddleBulletShape { MiddleBulletShape{ displayList: DisplayList::new(1) } }
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
    Screen::setColor(0.0, 0.0, 0.0, 1.0);
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
    Screen::setColor(0.1, 0.2, 0.3, 1.0);
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

pub struct MiddleBulletLineShape {
    displayList : DisplayList,
}

impl MiddleBulletLineShape {
    fn new() -> MiddleBulletLineShape { MiddleBulletLineShape{ displayList: DisplayList::new(1) } }
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

pub trait RollBulletShapeBase : BulletShapeBase {
  fn draw5(&mut self, pos : Vector3, cd : f32, deg : f32, rd : f32) {
    glPushMatrix();
    Screen::glTranslate(pos);
    glRotatef(cd * 180.0 / PI, 0.0, 1.0, 0.0);
    glRotatef(rd, 0.0, 0.0, 1.0);
    self.get_display_list().call(0);
    glPopMatrix();
  }
}

pub struct CounterBulletShape {
    displayList : DisplayList,
}

impl CounterBulletShape {
    fn new() -> CounterBulletShape { CounterBulletShape{ displayList: DisplayList::new(1) } }
}

impl Shape for CounterBulletShape {
    fn get_display_list(&mut self) -> &mut DisplayList {
        &mut self.displayList
    }
}

impl DisplayListShape for CounterBulletShape {
  fn drawList(&self) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    Screen::setColor(0.0, 0.0, 0.0, 1.0);
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
    Screen::setColor(0.5, 0.5, 0.5, 1.0);
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

pub struct CounterBulletLineShape {
    displayList : DisplayList,
}

impl CounterBulletLineShape {
    fn new() -> CounterBulletLineShape { CounterBulletLineShape{ displayList: DisplayList::new(1) } }
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
    Screen::glTranslate(pos);
    glRotatef(cd * 180.0 / PI, 0.0, 1.0, 0.0);
    Screen::glRotate(deg);
    glScalef(sx, sy, 1.0);
    glRotatef(cnt * 3.0, 0.0, 1.0, 0.0);
    self.get_display_list().call(0);
    glPopMatrix();
  }
}

pub struct Enemy1Shape {
    displayList : DisplayList,
}

impl Enemy1Shape {
    fn new() -> Enemy1Shape { Enemy1Shape{ displayList: DisplayList::new(1) } }
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
    Screen::setColor(0.2, 0.2, 0.5, 1.0);
    glPushMatrix();
    glRotatef(240.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.2, 0.0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape_drawShadow(0.2, 0.2, 0.5, false);
    glPopMatrix();
    Screen::setColor(1.0, 1.0, 0.6, 1.0);
    glPushMatrix();
    glTranslatef(0.0, -0.6, 0.0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape_draw();
    glPopMatrix();
    Screen::setColor(0.5, 0.5, 1.0, 1.0);
    glPushMatrix();
    glRotatef(120.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    Screen::setColor(0.5, 0.5, 1.0, 1.0);
    glPushMatrix();
    glRotatef(240.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, -0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

pub struct Enemy1TrailShape {
   displayList : DisplayList,
}

impl Enemy1TrailShape {
    fn new() -> Enemy1TrailShape { Enemy1TrailShape{ displayList: DisplayList::new(1) } }
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

pub struct Enemy2Shape {
    displayList : DisplayList,
}

impl Enemy2Shape {
    fn new() -> Enemy2Shape { Enemy2Shape{ displayList: DisplayList::new(1) } }
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
    Screen::setColor(1.0, 0.9, 1.0, 1.0);
    glPushMatrix();
    glTranslatef(0.0, -0.5, 0.0);
    glScalef(0.3, 1.0, 0.3);
    PyramidShape_draw();
    glPopMatrix();
    Screen::setColor(0.9, 0.6, 0.5, 1.0);
    glPushMatrix();
    glRotatef(60.0, 0.0, 0.0, 1.0);
    glTranslatef(0.6, -0.7, 0.0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    Screen::setColor(0.9, 0.6, 0.5, 1.0);
    glPushMatrix();
    glRotatef(300.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.6, -0.7, 0.0);
    glScalef(0.2, 1.2, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

pub struct Enemy2TrailShape {
    displayList : DisplayList,
}

impl Enemy2TrailShape {
    fn new() -> Enemy2TrailShape { Enemy2TrailShape{ displayList: DisplayList::new(1) } }
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

pub struct Enemy3Shape {
    displayList : DisplayList,
}

impl Enemy3Shape {
    fn new() -> Enemy3Shape { Enemy3Shape{ displayList: DisplayList::new(1) } }
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
    Screen::setColor(0.2, 0.2, 0.5, 1.0);
    glPushMatrix();
    glRotatef(210.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, 0.2, 0.0);
    glScalef(0.4, 1.0, 0.4);
    PyramidShape_drawShadow(0.2, 0.2, 0.5, false);
    glPopMatrix();
    Screen::setColor(1.0, 0.6, 0.9, 1.0);
    glPushMatrix();
    glTranslatef(0.0, -0.4, 0.0);
    glScalef(0.3, 1.2, 0.3);
    PyramidShape_draw();
    glPopMatrix();
    Screen::setColor(0.3, 0.5, 1.0, 1.0);
    glPushMatrix();
    glRotatef(150.0, 0.0, 0.0, 1.0);
    glTranslatef(0.5, 0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    Screen::setColor(0.3, 0.5, 1.0, 1.0);
    glPushMatrix();
    glRotatef(210.0, 0.0, 0.0, 1.0);
    glTranslatef(-0.5, 0.2, 0.0);
    glScalef(0.2, 0.8, 0.2);
    PyramidShape_draw();
    glPopMatrix();
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
  }
}

pub struct Enemy3TrailShape {
    displayList : DisplayList,
}

impl Enemy3TrailShape {
    fn new() -> Enemy3TrailShape { Enemy3TrailShape{ displayList: DisplayList::new(1) } }
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

pub struct TriangleParticleShape {
    displayList : DisplayList,
}

impl TriangleParticleShape {
    fn new() -> TriangleParticleShape { TriangleParticleShape{ displayList: DisplayList::new(1) } }
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

pub trait PillarShape : DisplayListShape {
  fn drawPillar(&self, r : f32, g : f32, b : f32, outside : bool /*= false*/) {
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glBegin(GL_QUADS);
    Screen::setColor(r, g, b, 1.0);
    for i in 0..8 {
      let mut d : f32 = PI * 2.0 * (i as f32) / 8.0;
      glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
      d += PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
      glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
      d -= PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
    }
    glEnd();
    if !outside {
      Screen::setColor(r, g, b, 1.0);
      glBegin(GL_TRIANGLES);
      for i in 0..8 {
        let mut d : f32 = PI * 2.0 * (i as f32) / 8.0;
        glVertex3f(0.0, TICKNESS, 0.0);
        glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                   TICKNESS,
                   d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
        d += PI * 2.0 / 8.0;
        glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                   TICKNESS,
                   d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
        d -= PI * 2.0 / 8.0;
        glVertex3f(0.0, -TICKNESS, 0.0);
        glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                   -TICKNESS,
                   d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
        d += PI * 2.0 / 8.0;
        glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                   -TICKNESS,
                   d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
      }
      glEnd();
    }
    Screen::setColor(0.1, 0.1, 0.1, 1.0);
    for i in 0..8 {
      let mut d : f32 = PI * 2.0 * (i as f32) / 8.0; 
      glBegin(GL_LINE_STRIP);
      glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
      d += PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                 TICKNESS,
                 d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
      glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
      d -= PI * 2.0 / 8.0;
      glVertex3f(d.sin() * Field::CIRCLE_RADIUS * RADIUS_RATIO,
                 -TICKNESS,
                 d.cos() * Field::CIRCLE_RADIUS * RADIUS_RATIO);
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

pub struct Pillar1Shape {
    displayList : DisplayList,
}

impl Pillar1Shape {
    fn new() -> Pillar1Shape { Pillar1Shape{ displayList: DisplayList::new(1) } }
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

pub struct Pillar2Shape {
    displayList : DisplayList,
}

impl Pillar2Shape
{
    fn new() -> Pillar2Shape { Pillar2Shape{ displayList: DisplayList::new(1) } }
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

pub struct Pillar3Shape {
    displayList : DisplayList,
}

impl Pillar3Shape {
    fn new() -> Pillar3Shape { Pillar3Shape{ displayList: DisplayList::new(1) } }
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

pub struct Pillar4Shape {
    displayList : DisplayList,
}

impl Pillar4Shape {
    fn new() -> Pillar4Shape { Pillar4Shape{ displayList: DisplayList::new(1) } }
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

pub struct OutsidePillarShape {
    displayList : DisplayList,  
}

impl OutsidePillarShape {
    fn new() -> OutsidePillarShape { OutsidePillarShape{ displayList: DisplayList::new(1) } }
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

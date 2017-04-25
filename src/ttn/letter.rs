/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */


use std::f32::consts::PI;

use util::sdl::displaylist::*;
use ttn::shape::*;
use ttn::screen::*;
use ttn::dummy::*;


const LETTER_WIDTH : f32 = 2.1;
const LETTER_HEIGHT : f32 = 3.0;

pub enum Direction {
  TO_RIGHT, TO_DOWN, TO_LEFT, TO_UP,
}

const LETTER_NUM : i32  = 44;
const DISPLAY_LIST_NUM : i32 = LETTER_NUM * 3;

pub enum LetterShape {
  NORMAL, POLYGON, LINE,
}

static LETTERSHAPES : [LetterShape; 3] = [LetterShape::NORMAL, LetterShape::POLYGON, LetterShape::LINE];

/**
 * Letters.
 */
pub struct Letter<'a> {
  displayList : &'a DisplayList,
}

impl<'a> Letter<'a> {
  //was init()
  fn new() -> Letter<'a> {
    let mut letter = Letter {
      displayList : DisplayList::new(DISPLAY_LIST_NUM),
    };

    letter.displayList.resetList();
    for j in LETTERSHAPES.into_iter() {
      for i in 0..LETTER_NUM {
        letter.displayList.newList();
        letter.setLetter(i, j);
        letter.displayList.endList();
      }
    }

    letter
  }

  fn close(&mut self) {
    self.displayList.close();
  }

  fn getWidth(n : i32, s : f32) -> f32 {
    (n as f32) * s * LETTER_WIDTH
  }

  fn getWidthNum(num : i32, s : f32) -> f32 {
    let n = num;
    let c = 1.0;
    loop {
      if n < 10 {
        break;
      }
      n /= 10;
      c += 1.0;
    }
    c * s * LETTER_WIDTH
  }

  fn getHeight(s : f32) -> f32 {
    s * LETTER_HEIGHT
  }

  fn drawLetter(&mut self, n : i32) {
    self.displayList.call(n);
  }

  fn drawLetter6(&mut self, n : i32, x : f32, y : f32, s : f32, d : f32) {
    glPushMatrix();
    glTranslatef(x, y, 0.0);
    glScalef(s, s, s);
    glRotatef(d, 0.0, 0.0, 1.0);
    self.displayList.call(n);
    glPopMatrix();
  }

  fn drawLetterRev(&mut self, n : i32, x : f32, y : f32, s : f32, d : f32) {
    glPushMatrix();
    glTranslatef(x, y, 0.0);
    glScalef(s, -s, s);
    glRotatef(d, 0.0, 0.0, 1.0);
    self.displayList.call(n);
    glPopMatrix();
  }

  fn convertCharToInt(c : char) -> i32 {
    match c {
      '0'...'9' => (c as i32) - ('0' as i32),
      'A'...'Z' => (c as i32) - ('A' as i32) + 10,
      '.' => 36,
      '-' => 38,
      '+' => 39,
      '_' => 37,
      '!' => 42,
      '/' => 43,
      _ => 0,
    }
  }


  pub fn drawString(string : &'static str, lx : f32, y : f32, s : f32) {
    Letter::drawString11(string, lx, y, s, Direction::TO_RIGHT, false, 0.0, 1.0, 1.0, 1.0);
  }

  pub fn drawString11(string : &'static str, lx : f32, y : f32, s : f32,
                                d : Direction /*= Direction::TO_RIGHT*/,
                                rev : bool /*= false*/, od : f32 /*= 0*/,
                                r : f32 /*= 1*/, g : f32 /*= 1*/,  b : f32 /*= 1*/) {
    lx += LETTER_WIDTH * s / 2.0;
    y += LETTER_HEIGHT * s / 2.0;
    let mut x = lx;
    let ld = match d {
      Direction::TO_RIGHT => 0.0,
      Direction::TO_DOWN => 90.0,
      Direction::TO_LEFT => 180.0,
      Direction::TO_UP => 270.0,
    };
    ld += od;
    for c in string.iter() {
      if c != ' ' {
        let idx = Letter::convertCharToInt(c);
        if (r == 1.0) && (g == 1.0) && (b == 1.0) {
          if rev {
            Letter::drawLetterRev(idx, x, y, s, ld);
          } else {
            Letter::drawLetter6(idx, x, y, s, ld);
          }
        } else {
          Screen::setColor(r, g, b, 0.5);
          if rev {
            Letter::drawLetterRev(idx + LETTER_NUM, x, y, s, ld);
          } else {
            Letter::drawLetter6(idx + LETTER_NUM, x, y, s, ld);
          }
          Screen::setColor(r, g, b, 1.0);
          if rev {
            Letter::drawLetterRev(idx + LETTER_NUM * 2, x, y, s, ld);
          } else {
            Letter::drawLetter6(idx + LETTER_NUM * 2, x, y, s, ld);
          }
        }
      }
      if od == 0.0 {
        match d {
          Direction::TO_RIGHT => { x += s * LETTER_WIDTH; },
          Direction::TO_DOWN => { y += s * LETTER_WIDTH; },
          Direction::TO_LEFT => { x -= s * LETTER_WIDTH; },
          Direction::TO_UP => { y -= s * LETTER_WIDTH; },
       }
      } else {
        x += (ld * PI / 180.0).cos() * s * LETTER_WIDTH;
        y += (ld * PI / 180.0).sin() * s * LETTER_WIDTH;
      }
    }
  }

  pub fn drawNum(num : i32, lx : f32, y : f32, s : f32) {
    Letter::drawNum7(num, lx, y, s, 0, -1.0, -1);
  }

  pub fn drawNum7(num : i32, lx : f32, y : f32, s : f32, dg : i32 /*= 0*/, headChar : f32 /*= -1*/, floatDigit : i32 /*= -1*/) {
    lx += LETTER_WIDTH * s / 2.0;
    y += LETTER_HEIGHT * s / 2.0;
    let mut n = num;
    let mut x = lx;
    let ld = 0.0;
    let mut digit = dg;
    let mut fd = floatDigit;
    loop {
      if fd <= 0 {
        Letter::drawLetter(n % 10, x, y, s, ld);
        x -= s * LETTER_WIDTH;
      } else {
        Letter::drawLetter(n % 10, x, y + s * LETTER_WIDTH * 0.25, s * 0.5, ld);
        x -= s * LETTER_WIDTH * 0.5;
      }
      n /= 10;
      digit -= 1;
      fd -= 1;
      if (n <= 0) && (digit <= 0) && (fd < 0) {
        break;
      }
      if fd == 0 {
        Letter::drawLetter6(36, x, y + s * LETTER_WIDTH * 0.25, s * 0.5, ld);
        x -= s * LETTER_WIDTH * 0.5;
      }
    }
    if headChar >= 0.0 {
      Letter::drawLetter(headChar, x + s * LETTER_WIDTH * 0.2, y + s * LETTER_WIDTH * 0.2, s * 0.6, ld);
    }
  }

  pub fn drawNumSign(num : i32, lx : f32, ly : f32, s : f32, headChar : i32 /*= -1*/, floatDigit : i32 /*= -1*/, type_ : i32 /* = 0*/) {
    let mut x  = lx;
    let mut y = ly;
    let mut n  = num;
    let mut fd = floatDigit;
    loop {
      if fd <= 0 {
        Letter::drawLetterRev(n % 10 + type_ * LETTER_NUM, x, y, s, 0.0);
        x -= s * LETTER_WIDTH;
      } else {
        Letter::drawLetterRev(n % 10 + type_ * LETTER_NUM, x, y - s * LETTER_WIDTH * 0.25, s * 0.5, 0.0);
        x -= s * LETTER_WIDTH * 0.5;
      }
      n /= 10;
      if n <= 0 {
        break;
      }
      fd -= 1;
      if fd == 0 {
        Letter::drawLetterRev(36 + type_ * LETTER_NUM, x, y - s * LETTER_WIDTH * 0.25, s * 0.5, 0.0);
        x -= s * LETTER_WIDTH * 0.5;
      }
    }
    if headChar >= 0 {
      Letter::drawLetterRev(headChar + type_ * LETTER_NUM, x + s * LETTER_WIDTH * 0.2, y - s * LETTER_WIDTH * 0.2, s * 0.6, 0.0);
    }
  }

  pub fn drawTime(&mut self, time : i32, lx : f32, y : f32, s : f32) {
    let mut n = time;
    if n < 0 {
      n = 0;
    }
    let mut x = lx;
    for i in 0..7 {
      if i != 4 {
        Letter::drawLetter(n % 10, x, y, s, (Direction::TO_RIGHT as i32) as f32);
        n /= 10;
      } else {
        Letter::drawLetter(n % 6, x, y, s, (Direction::TO_RIGHT as i32) as f32);
        n /= 6;
      }
      if ((i & 1) == 1) || (i == 0) {
        match i {
          3 => { Letter::drawLetter6(41, x + s * 1.16, y, s, (Direction::TO_RIGHT as i32) as f32); },
          5 => { Letter::drawLetter6(40, x + s * 1.16, y, s, (Direction::TO_RIGHT as i32) as f32); },
          _ => break,
        };
        x -= s * LETTER_WIDTH;
      } else {
        x -= s * LETTER_WIDTH * 1.3;
      }
      if n <= 0 {
        break;
      }
    }
  }

  pub fn setLetter(&mut self, idx : usize, type_ : LetterShape /* = Shape::NORMAL*/) {
    let mut i : usize = 0;
    loop {
      let deg = SP_DATA[idx][i][4] as i32;
      if deg > 99990 {
        break;
      };
      let mut x = -SP_DATA[idx][i][0];
      let mut y = -SP_DATA[idx][i][1];
      let mut size = SP_DATA[idx][i][2];
      let mut length = SP_DATA[idx][i][3];
      y *= 0.9;
      size *= 1.4;
      length *= 1.05;
      x = -x;
      y = y;
      deg %= 180;
      match type_ {
        LetterShape::NORMAL => Letter::drawSegment(x, y, size, length, deg as f32),
        LetterShape::POLYGON => Letter::drawSegmentPolygon(x, y, size, length, deg as f32),
        LetterShape::LINE => Letter::drawSegmentLine(x, y, size, length, deg as f32),
      }
      i += 1;
    }
  }

  fn drawSegment(x : f32, y : f32, width : f32, height : f32, deg : f32) {
    glPushMatrix();
    glTranslatef(x - width / 2.0, y, 0.0);
    glRotatef(deg, 0.0, 0.0, 1.0);
    Screen::setColor(1.0, 1.0, 1.0, 0.5);
    glBegin(GL_TRIANGLE_FAN);
    Letter::drawSegmentPart(width, height);
    glEnd();
    Screen::setColor(1.0, 1.0, 1.0, 1.0);
    glBegin(GL_LINE_LOOP);
    Letter::drawSegmentPart(width, height);
    glEnd();
    glPopMatrix();
  }

   fn drawSegmentPolygon(x : f32, y : f32, width : f32, height : f32, deg : f32) {
    glPushMatrix();
    glTranslatef(x - width / 2.0, y, 0.0);
    glRotatef(deg, 0.0, 0.0, 1.0);
    glBegin(GL_TRIANGLE_FAN);
    Letter::drawSegmentPart(width, height);
    glEnd();
    glPopMatrix();
  }

  fn drawSegmentLine(x : f32, y : f32, width : f32, height : f32, deg : f32) {
    glPushMatrix();
    glTranslatef(x - width / 2.0, y, 0.0);
    glRotatef(deg, 0.0, 0.0, 1.0);
    glBegin(GL_LINE_LOOP);
    Letter::drawSegmentPart(width, height);
    glEnd();
    glPopMatrix();
  }

  fn drawSegmentPart(width : f32, height : f32) {
    glVertex3f(-width / 2.0, 0.0, 0.0);
    glVertex3f(-width / 3.0, -height / 2.0, 0.0);
    glVertex3f( width / 3.0, -height / 2.0, 0.0);
    glVertex3f( width / 2.0, 0.0, 0.0);
    glVertex3f( width / 3.0,  height / 2.0, 0.0);
    glVertex3f(-width / 3.0,  height / 2.0, 0.0);
  }
}

static SP_DATA : [[[f32; 5]; 16]; 44] =
    [[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.6, 0.55, 0.65, 0.3, 90.0], [0.6, 0.55, 0.65, 0.3, 90.0],
     [-0.6, -0.55, 0.65, 0.3, 90.0], [0.6, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.5, 0.55, 0.65, 0.3, 90.0],
     [0.5, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
   [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
       [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
            [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
            [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
       [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//A
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.18, 1.15, 0.45, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.45, 0.55, 0.65, 0.3, 90.0],
     [-0.18, 0.0, 0.45, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
       [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.15, 1.15, 0.45, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.45, 0.45, 0.65, 0.3, 90.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
     [0.0, 0.0, 0.0, 0.0, 0.0],
[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
     [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
     [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0],
     [0.05, 0.0, 0.3, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 0.55, 0.65, 0.3, 90.0],
     [0.0, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0],
     [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.65, -0.55, 0.65, 0.3, 90.0], [-0.7, -0.7, 0.3, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
      
    ],[//K
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.4, 0.55, 0.65, 0.3, 100.0],
     [-0.25, 0.0, 0.45, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.6, -0.55, 0.65, 0.3, 80.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.5, 1.15, 0.3, 0.3, 0.0], [0.1, 1.15, 0.3, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.55, 0.65, 0.3, 90.0],
     [0.0, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//P
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.05, -0.55, 0.45, 0.3, 60.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
          [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.2, 0.0, 0.45, 0.3, 0.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.45, -0.55, 0.65, 0.3, 80.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
     [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [-0.65, 0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.65, 0.3, 0.0],
     [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
              [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.5, 1.15, 0.55, 0.3, 0.0], [0.5, 1.15, 0.55, 0.3, 0.0],
     [0.1, 0.55, 0.65, 0.3, 90.0],
     [0.1, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
              [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//U
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.5, -0.55, 0.65, 0.3, 90.0], [0.5, -0.55, 0.65, 0.3, 90.0],
     [-0.1, -1.15, 0.45, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.65, 0.55, 0.65, 0.3, 90.0], [0.65, 0.55, 0.65, 0.3, 90.0],
     [-0.65, -0.55, 0.65, 0.3, 90.0], [0.65, -0.55, 0.65, 0.3, 90.0],
     [-0.5, -1.15, 0.3, 0.3, 0.0], [0.1, -1.15, 0.3, 0.3, 0.0],
     [0.0, 0.55, 0.65, 0.3, 90.0],
     [0.0, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.4, 0.6, 0.85, 0.3, 360.0-120.0],
     [0.4, 0.6, 0.85, 0.3, 360.0-60.0],
     [-0.4, -0.6, 0.85, 0.3, 360.0-240.0],
     [0.4, -0.6, 0.85, 0.3, 360.0-300.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [-0.4, 0.6, 0.85, 0.3, 360.0-120.0],
     [0.4, 0.6, 0.85, 0.3, 360.0-60.0],
     [-0.1, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[
     [0.0, 1.15, 0.65, 0.3, 0.0],
     [0.3, 0.4, 0.65, 0.3, 120.0],
     [-0.3, -0.4, 0.65, 0.3, 120.0],
     [0.0, -1.15, 0.65, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//.
     [0.0, -1.15, 0.3, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//_
     [0.0, -1.15, 0.8, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//-
     [0.0, 0.0, 0.9, 0.3, 0.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//+
     [-0.5, 0.0, 0.45, 0.3, 0.0], [0.45, 0.0, 0.45, 0.3, 0.0],
     [0.1, 0.55, 0.65, 0.3, 90.0],
     [0.1, -0.55, 0.65, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//'
     [0.0, 1.0, 0.4, 0.2, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//''
     [-0.19, 1.0, 0.4, 0.2, 90.0],
     [0.2, 1.0, 0.4, 0.2, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[//!
     [0.56, 0.25, 1.1, 0.3, 90.0],
     [0.0, -1.0, 0.3, 0.3, 90.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ],[// /
     [0.8, 0.0, 1.75, 0.3, 120.0],
     [0.0, 0.0, 0.0, 0.0, 99999.0],
  [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0]
    ]];

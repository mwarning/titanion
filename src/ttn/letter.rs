/*
 * $Id: letter.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.letter;


private import std.math;

private import derelict.opengl.gl;

private import src.util.sdl.displaylist;
private import src.ttn.screen;
*/

let LETTER_WIDTH : f32 = 2.1;
let LETTER_HEIGHT : f32 = 3.0;
enum Direction {
  TO_RIGHT, TO_DOWN, TO_LEFT, TO_UP,
};

let LETTER_NUM : i32  = 44;
let  DISPLAY_LIST_NUM : i32 = LETTER_NUM * 3;
enum Shape {
  NORMAL, POLYGON, LINE,
};

/**
 * Letters.
 */
struct Letter {
  displaylist : &DisplayList,
}

impl Letter {

  fn init(&mit self) {
    self.displayList = DisplayList(DISPLAY_LIST_NUM);
    self.displayList.resetList();
    for j in  0..3 {
      for i in 0..LETTER_NUM {
        self.displayList.newList();
        self.setLetter(i, j);
        self.displayList.endList();
      }
    }
  }

  fn close(&mut self) {
    self.displayList.close();
  }

  fn getWidth(n : i32, s : f32) -> f32 {
    n * s * LETTER_WIDTH
  }

  fn getWidthNum(num : i32, s : f32) -> f32 {
    let dg : i32 = 1;
    let n : i32 = num;
    let c :i32 = 1;
    while true {
      if (n < 10) {
        break;
      }
      n /= 10;
      c += 1;
    }
    c * s * LETTER_WIDTH
  }

  fn getHeight(s : f32) -> f32 {
    return s * LETTER_HEIGHT;
  }

  fn drawLetter(&mut self, n : i32) -> f32 {
    self.displayList.call(n);
  }

  fn drawLetter(&mut self, n : i32, x : f32, y : f32, s : f32, d : f32) {
    glPushMatrix();
    glTranslatef(x, y, 0);
    glScalef(s, s, s);
    glRotatef(d, 0, 0, 1);
    self.displayList.call(n);
    glPopMatrix();
  }

  fn drawLetterRev(&mut self, n : i32, x : f32, y : f32, s : f32, d : f32) {
    glPushMatrix();
    glTranslatef(x, y, 0);
    glScalef(s, -s, s);
    glRotatef(d, 0, 0, 1);
    self.displayList.call(n);
    glPopMatrix();
  }

  fn convertCharToInt(c : u8) -> i32 {
    let mut idx : i32;
    if (c >= '0' && c <='9') {
      idx = c - '0';
    } else if (c >= 'A' && c <= 'Z') {
      idx = c - 'A' + 10;
    } else if (c >= 'a' && c <= 'z') {
      idx = c - 'a' + 10;
    } else if (c == '.') {
      idx = 36;
    } else if (c == '-') {
      idx = 38;
    } else if (c == '+') {
      idx = 39;
    } else if (c == '_') {
      idx = 37;
    } else if (c == '!') {
      idx = 42;
    } else if (c == '/') {
      idx = 43;
    }
    idx
  }

  fn drawString(&mut self, str : &String, lx : f32, y : f32, s : f32,
                                d : i32 /*= Direction.TO_RIGHT*/,
                                rev : bool /*= false*/, od : f32 /*= 0*/,
                                r : f32 /*= 1*/, g : f32 /*= 1*/,  b : f32 /*= 1*/) {
    lx += LETTER_WIDTH * s / 2;
    y += LETTER_HEIGHT * s / 2;
    let mut x : f32 = lx;
    let idx : i32;
    let ld : f32 = match d {
      Direction.TO_RIGHT => { 0 },
      Direction.TO_DOWN => { 90 },
      Direction.TO_LEFT => { 180 },
      Direction.TO_UP => { 270 },
    }
    ld += od;
    for c in str {
      if c != ' ' {
        idx = convertCharToInt(c);
        if (r == 1) && (g == 1) && (b == 1) {
          if (rev) {
            drawLetterRev(idx, x, y, s, ld);
          } else {
            drawLetter(idx, x, y, s, ld);
          }
        } else {
          Screen.setColor(r, g, b, 0.5f);
          if rev {
            drawLetterRev(idx + LETTER_NUM, x, y, s, ld);
          } else {
            drawLetter(idx + LETTER_NUM, x, y, s, ld);
          }
          Screen.setColor(r, g, b);
          if rev {
            drawLetterRev(idx + LETTER_NUM * 2, x, y, s, ld);
          } else {
            drawLetter(idx + LETTER_NUM * 2, x, y, s, ld);
          }
        }
      }
      if od == 0 {
        match d {
          Direction.TO_RIGHT => { x += s * LETTER_WIDTH; },
          Direction.TO_DOWN => { y += s * LETTER_WIDTH; },
          Direction.TO_LEFT => { x -= s * LETTER_WIDTH; }
          Direction.TO_UP => { y -= s * LETTER_WIDTH; },
       }
      } else {
        x += (ld * PI / 180.0).cos() * s * LETTER_WIDTH;
        y += (ld * PI / 180.0).sin() * s * LETTER_WIDTH;
      }
    }
  }

  fn drawNum(&mut self, num : i32, lx : f32, y : f32, s : f32,
                            dg : i32 /*= 0*/,
                            headChar : f32 /*= -1*/, floatDigit : i32 /*= -1*/) {
    lx += LETTER_WIDTH * s / 2;
    y += LETTER_HEIGHT * s / 2;
    let n : i32 = num;
    let x : f32 = lx;
    let ld : f32 = 0;
    let digit : i32 = dg;
    let fd : i32 = floatDigit;
    loop {
      if fd <= 0 {
        drawLetter(n % 10, x, y, s, ld);
        x -= s * LETTER_WIDTH;
      } else {
        drawLetter(n % 10, x, y + s * LETTER_WIDTH * 0.25f, s * 0.5, ld);
        x -= s * LETTER_WIDTH * 0.5;
      }
      n /= 10;
      digit -= 1;
      fd -= 1;
      if (n <= 0) && (digit <= 0) && (fd < 0) {
        break;
      }
      if fd == 0 {
        drawLetter(36, x, y + s * LETTER_WIDTH * 0.25, s * 0.5, ld);
        x -= s * LETTER_WIDTH * 0.5;
      }
    }
    if headChar >= 0 {
      drawLetter(headChar, x + s * LETTER_WIDTH * 0.2, y + s * LETTER_WIDTH * 0.2, s * 0.6, ld);
    }
  }

  fn drawNumSign(&mut self, num  i32, lx : f32, ly : f32, s : f32,
                                headChar : i32 /*= -1*/, floatDigit : i32 /*= -1*/, type : i32 /* = 0*/) {
    let x : f32 = lx;
    let y : f32 = ly;
    let n  i32 = num;
    let fd : i32 = floatDigit;
    loop {
      if fd <= 0 {
        drawLetterRev(n % 10 + type * LETTER_NUM, x, y, s, 0);
        x -= s * LETTER_WIDTH;
      } else {
        drawLetterRev(n % 10 + type * LETTER_NUM, x, y - s * LETTER_WIDTH * 0.25, s * 0.5, 0);
        x -= s * LETTER_WIDTH * 0.5;
      }
      n /= 10;
      if n <= 0 {
        break;
      }
      fd -= 1;
      if fd == 0 {
        drawLetterRev(36 + type * LETTER_NUM, x, y - s * LETTER_WIDTH * 0.25, s * 0.5, 0);
        x -= s * LETTER_WIDTH * 0.5;
      }
    }
    if headChar >= 0 {
      drawLetterRev(headChar + type * LETTER_NUM, x + s * LETTER_WIDTH * 0.2, y - s * LETTER_WIDTH * 0.2, s * 0.6, 0);
    }
  }

  fn drawTime(&mut self, time : i32, lx : f32, y : f32, s : f32) {
    let n : i32 = time;
    if n < 0 {
      n = 0;
    }
    let x : f32 = lx;
    for i in 0..7 {
      if i != 4 {
        drawLetter(n % 10, x, y, s, Direction.TO_RIGHT);
        n /= 10;
      } else {
        drawLetter(n % 6, x, y, s, Direction.TO_RIGHT);
        n /= 6;
      }
      if (i & 1) == 1) || (i == 0) {
        match i {
          3 => { drawLetter(41, x + s * 1.16f, y, s, Direction.TO_RIGHT); },
          5 => { drawLetter(40, x + s * 1.16f, y, s, Direction.TO_RIGHT); },
        //default:
         // break;
        }
        x -= s * LETTER_WIDTH;
      } else {
        x -= s * LETTER_WIDTH * 1.3f;
      }
      if n <= 0 {
        break;
      }
    }
  }

  fn setLetter(&mut self, idx : i32, type : i32 /* = Shape.NORMAL*/) {
    let mut x : f32;
    let mut y : f32;
    let mut length : f32;
    let size : f32;
    let t : f32;
    let deg : f32;
    loop {
      deg = spData[idx][i][4] as i32;
      if deg > 99990 {
        break;
      }
      x = -spData[idx][i][0];
      y = -spData[idx][i][1];
      size = spData[idx][i][2];
      length = spData[idx][i][3];
      y *= 0.9;
      size *= 1.4;
      length *= 1.05;
      x = -x;
      y = y;
      deg %= 180;
      match type {
        Shape.NORMAL => drawSegment(x, y, size, length, deg),
        Shape.POLYGON => drawSegmentPolygon(x, y, size, length, deg),
        Shape.LINE => drawSegmentLine(x, y, size, length, deg),
      }
      i += 1;
    }
  }

  fn drawSegment(&mut self, x : f32, y : f32, width : f32, height : f32, deg : f32) {
    glPushMatrix();
    glTranslatef(x - width / 2.0, y, 0.0);
    glRotatef(deg, 0.0, 0.0, 1.0);
    Screen.setColor(1.0, 1.0, 1.0, 0.5);
    glBegin(GL_TRIANGLE_FAN);
    self.drawSegmentPart(width, height);
    glEnd();
    Screen.setColor(1.0, 1.0, 1.0);
    glBegin(GL_LINE_LOOP);
    self.drawSegmentPart(width, height);
    glEnd();
    glPopMatrix();
  }

   fn drawSegmentPolygon(&mut self, x : f32, y : f32, width : f32, height : f32, deg : f32) {
    glPushMatrix();
    glTranslatef(x - width / 2.0, y, 0.0);
    glRotatef(deg, 0.0, 0.0, 1.0);
    glBegin(GL_TRIANGLE_FAN);
    self.drawSegmentPart(width, height);
    glEnd();
    glPopMatrix();
  }

  fn drawSegmentLine(&mut self, x : f32, y : f32, width : f32, height : f32, deg : f32) {
    glPushMatrix();
    glTranslatef(x - width / 2, y, 0);
    glRotatef(deg, 0, 0, 1);
    glBegin(GL_LINE_LOOP);
    self.drawSegmentPart(width, height);
    glEnd();
    glPopMatrix();
  }

  fn drawSegmentPart(width : f32, height : f32) {
    glVertex3f(-width / 2, 0, 0);
    glVertex3f(-width / 3 * 1, -height / 2, 0);
    glVertex3f( width / 3 * 1, -height / 2, 0);
    glVertex3f( width / 2, 0, 0);
    glVertex3f( width / 3 * 1,  height / 2, 0);
    glVertex3f(-width / 3 * 1,  height / 2, 0);
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
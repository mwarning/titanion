/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use util::vector::*;
use util::actor::*;
use util::rand::*;
use ttn::token::*;
use ttn::shape::*;
use ttn::bullet::*;
use ttn::field::*;
use ttn::player::*;
use ttn::enemy::*;
use ttn::pillar::*;
use ttn::frame::*;
use ttn::letter::*;
use ttn::preference::*;
use ttn::dummy::*;

/**
 * Title screen.
 */

pub struct Title<'a> {
  preference : &'a Preference,
  pad : &'a RecordablePad,
  frame : &'a Frame<'a>,
  cnt : i32,
  aPressed : bool,
  udPressed : bool,
  titleTexture : Texture,
  titlePos : Vector,
  titleSize : f32,
  cursorIdx : i32,
}

impl<'a> Title<'a> {
  fn new(preference : &'a Preference, pad : &'a RecordablePad, frame : &'a Frame<'a>) -> Title<'a> {
    Title{
      preference : preference,
      pad : pad, //(pad as &RecordablePad),
      frame : frame,
      cnt : 0,
      aPressed : false,
      udPressed : false,
      titleTexture : Texture::new("title.bmp"),
      titlePos : Vector::new(0.0, 0.0, 0.0),
      titleSize : 0.0,
      cursorIdx : 0,
    }
  }

  fn init(&mut self) {
    //moved to ctor
    //self.titleTexture = Texture::new("title.bmp");
  }

  fn close(&mut self) {
    self.titleTexture.close();
  }

  fn setMode(&mut self, mode : i32) {
    self.cursorIdx = mode;
  }

  fn start(&mut self) {
    self.cnt = 0;
    self.aPressed = true;
    self.udPressed = true;
    self.titlePos.x = 150;
    self.titlePos.y = 150;
    self.titleSize = 1.0;
  }

  fn move1(&mut self) {
    let input = self.pad.getState(false);
    if self.input.button & PadState.Button.A {
      if !self.aPressed {
        self.aPressed = true;
        self.frame.startInGame(self.cursorIdx);
      }
    } else {
      self.aPressed = false;
    }
    if self.input.dir & (PadStateDir::UP | PadStateDir::DOWN) {
      if !self.udPressed {
        self.udPressed = true;
        if self.input.dir & PadStateDir::UP {
          self.cursorIdx -= 1;
        } else if input.dir & PadStateDir::DOWN {
          self.cursorIdx += 1;
        }
        if self.cursorIdx < 0 {
          self.cursorIdx = GameState::MODE_NUM - 1;
        } else if self.cursorIdx > (GameState::MODE_NUM - 1) {
          self.cursorIdx = 0;
        }
      }
    } else {
      self.udPressed = false;
    }
    if (self.cnt > 180) && (self.cnt < 235) {
      self.titlePos.y -= 2;
    }
    if (self.cnt > 600) && (self.cnt < 675) {
      self.titlePos.x -= 2;
      self.titlePos.y += 1;
      self.titleSize -= 0.007;
    }
    self.cnt += 1;
  }

  fn draw(&mut self) {
    Screen.setColor(1, 1, 1);
    glEnable(GL_TEXTURE_2D);
    self.titleTexture.bind();
    self.drawBoard(self.titlePos.x, self.titlePos.y, 280 * self.titleSize, 64 * self.titleSize);
    glDisable(GL_TEXTURE_2D);
    if (self.cnt % 120) < 60 {
      let x = 175.0;
      let sz = 6.0;
      if self.cnt >= 600 {
        let c : i32 = self.cnt - 600;
        if c > 75 {
          c = 75;
        }
        x += c * 4.33;
        sz -= c * 0.045;
      }
      Letter::drawString("PUSH SHOT BUTTON TO START", x, 440, sz);
    }
    if self.cnt >= 240 {
      self.drawRanking();
    }
    if (self.cnt % 60) < 30 {
      self.drawTriangle(575, 398, 180);
      self.drawTriangle(575, 417, 0);
    }
    Letter::drawString(GameState::MODE_NAME[self.cursorIdx], 540, 400, 5);
  }

  fn drawBoard(x : f32, y : f32, w : f32, h : f32) {
    glBegin(GL_TRIANGLE_FAN);
    glTexCoord2f(0.0, 0.0);
    glVertex3f(x, y, 0.0);
    glTexCoord2f(1.0, 0.0);
    glVertex3f(x + w, y, 0.0);
    glTexCoord2f(1.0, 1.0);
    glVertex3f(x + w, y + h, 0.0);
    glTexCoord2f(0.0, 1.0);
    glVertex3f(x, y + h, 0.0);
    glEnd();
  }

  fn drawTriangle(x : f32, y : f32, d : f32) {
    glPushMatrix();
    glTranslatef(x, y, 0.0);
    glRotatef(d, 0.0, 0.0, 1.0);
    glScalef(5.0, 5.0, 1.0);
    glBegin(GL_TRIANGLE_FAN);
    Screen.setColor(1, 1, 1, 0.5);
    glVertex3f(0.0, 1.7, 0.0);
    glVertex3f(1.0, 0.0, 0.0);
    glVertex3f(-1.0, 0.0, 0.0);
    glEnd();
    glBegin(GL_LINE_LOOP);
    Screen.setColor(1, 1, 1, 1);
    glVertex3f(0.0, 1.7, 0.0);
    glVertex3f(1.0, 0.0, 0.0);
    glVertex3f(-1.0, 0.0, 0.0);
    glEnd();
    glPopMatrix();
  }

  fn drawRanking(&self) {
    let rn : i32 = (self.cnt - 240) / 30;
    if rn > Preference::RANKING_NUM {
      rn = Preference::RANKING_NUM;
    }
    let y = 140.0;
    for i in 0..rn {
      if self.cnt < 600 {
        let rstr = match i {
          0 => "1ST",
          1 => "2ND",
          2 => "3RD",
          _ => ((i + 1).to_string() + "TH"),
        };

        if i < 9 {
          Letter::drawString(rstr, 180, y, 7);
        } else {
          Letter::drawString(rstr, 166, y, 7);
        }
      }
      let mut sx = 450.0;
      let mut sy = y;
      let mut sz = 6.0;
      if self.cnt >= 600 {
        let c : i32 = self.cnt - 600;
        if c > 75 {
          c = 75;
        }
        sx += (c * 2.35) as i32;
        sz -= c * 0.03;
      }
      Letter::drawNum(self.preference.highScore[self.cursorIdx][i], sx, sy, sz);
      y += 24;
    }
  }
}

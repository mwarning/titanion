/*
 * $Id: title.d,v 1.4 2006/12/04 16:04:27 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.title;


private import derelict.opengl.gl;

private import src.util.vector;
private import src.util.sdl.pad;
private import src.util.sdl.texture;
private import src.ttn.screen;
private import src.ttn.letter;
private import src.ttn.preference;
private import src.ttn.frame;
*/

/**
 * Title screen.
 */

struct Title {
  preference : Preference,
  pad : RecordablePad,
  frame : Frame,
  cnt : i32,
  aPressed : bool,
  udPressed : bool,
  titleTexture : Texture,
  titlePos : Vector,
  titleSize : f32,
  cursorIdx : i32,
}

impl Default for Title {
  fn default() -> Title {
    Title{
      preference : preference,
      pad : (pad as RecordablePad),
      frame : frame,
      cnt : 0,
      aPressed : false,
      udPressed : false,
      titleTexture : texture,
      titlePos : Vector(0, 0, 0),
      titleSize : 0.0,
      cursorIdx : 0,
    }
  }
}

impl Title for Title {
  fn init(&mut self) {
    self.titleTexture = Texture("title.bmp");
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

  fn move(&mut self) {
    let input : PadState;
    self.input = pad.getState(false);
    if self.input.button & PadState.Button.A {
      if !self.aPressed {
        self.aPressed = true;
        self.frame.startInGame(self.cursorIdx);
      }
    } else {
      self.aPressed = false;
    }
    if self.input.dir & (PadState.Dir.UP | PadState.Dir.DOWN) {
      if !udPressed {
        self.udPressed = true;
        if self.input.dir & PadState.Dir.UP) {
          self.cursorIdx -= 1;
        } else if (input.dir & PadState.Dir.DOWN) {
          self.cursorIdx += 1;
        }
        if self.cursorIdx < 0 {
          self.cursorIdx = GameState.MODE_NUM - 1;
        } else if self.cursorIdx > GameState.MODE_NUM - 1) {
          self.cursorIdx = 0;
        }
      }
    } else {
      self.udPressed = false;
    }
    if (self.cnt > 180) && (self.cnt < 235) {
      self.titlePos.y -= 2;
    }
    if (self.cnt > 600) && (cnt < 675) {
      self.titlePos.x -= 2;
      self.titlePos.y += 1;
      self.titleSize -= 0.007f32;
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
      let x : 32 = 175.0;
      let sz : f32 = 6.0;
      if (self.cnt >= 600) {
        let c : i32 = self.cnt - 600;
        if (c > 75) {
          c = 75;
        }
        x += c * 4.33;
        sz -= c * 0.045;
      }
      Letter.drawString("PUSH SHOT BUTTON TO START", x, 440, sz);
    }
    if self.cnt >= 240 {
      self.drawRanking();
    }
    if (self.cnt % 60) < 30 {
      self.drawTriangle(575, 398, 180);
      self.drawTriangle(575, 417, 0);
    }
    Letter.drawString(GameState.MODE_NAME[self.cursorIdx], 540, 400, 5);
  }

  fn drawBoard(x : f32, y : f32, w : f32, h : f32) {
    glBegin(GL_TRIANGLE_FAN);
    glTexCoord2f(0, 0);
    glVertex3f(x, y, 0);
    glTexCoord2f(1, 0);
    glVertex3f(x + w, y, 0);
    glTexCoord2f(1, 1);
    glVertex3f(x + w, y + h, 0);
    glTexCoord2f(0, 1);
    glVertex3f(x, y + h, 0);
    glEnd();
  }

  fn drawTriangle(x : f32, y : f32, d : f32) {
    glPushMatrix();
    glTranslatef(x, y, 0);
    glRotatef(d, 0, 0, 1);
    glScalef(5, 5, 1);
    glBegin(GL_TRIANGLE_FAN);
    Screen.setColor(1, 1, 1, 0.5f);
    glVertex3f(0, 1.7f, 0);
    glVertex3f(1, 0, 0);
    glVertex3f(-1, 0, 0);
    glEnd();
    glBegin(GL_LINE_LOOP);
    Screen.setColor(1, 1, 1, 1);
    glVertex3f(0, 1.7f, 0);
    glVertex3f(1, 0, 0);
    glVertex3f(-1, 0, 0);
    glEnd();
    glPopMatrix();
  }

  fn drawRanking(&self) {
    let rn : i32 = (self.cnt - 240) / 30;
    if rn > Preference.RANKING_NUM {
      rn = Preference.RANKING_NUM;
    }
    let y : f32 = 140.0;
    for i in 0..rn {
      if self.cnt < 600 {
        let rstr = case i {
          0 => "1ST",
          1 => "2ND",
          2 => "3RD",
          _ => ((i + 1).to_string() + "TH"),
        }

        if i < 9 {
          Letter.drawString(rstr, 180, y, 7);
        } else {
          Letter.drawString(rstr, 166, y, 7);
        }
      }
      let mut sx : f32 = 450;
      let mut sy : f32 = y;
      let mut sz : f32 = 6;
      if self.cnt >= 600 {
        let c : i32 = cnt - 600;
        if c > 75 {
          c = 75;
        }
        sx += (c * 2.35) as i32
        sz -= c * 0.03;
      }
      Letter.drawNum(preference.highScore[self.cursorIdx][i], sx, sy, sz);
      y += 24;
    }
  }
}

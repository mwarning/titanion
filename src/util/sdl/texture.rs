/*
 * $Id: texture.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2005 Kenta Cho. Some rights reserved.
 */
 /*
module src.util.sdl.texture;


private import std.string;

private import derelict.opengl.gl;
private import derelict.opengl.glu;
private import derelict.sdl.sdl;

private import src.util.sdl.sdlexception;
*/
use std::collections::HashMap;

/**
 * OpenGL textures.
 */
let imagesDir = "images/";

struct Texture {
  /*static*/ surface : HashMap<String, &SDL_Surface>,
  num : GLuint,
  maskNum : GLuint,
  textureNum : i32,
  maskTextureNum : i32,
  pixels : u32[128 * 128];
  maskPixels : u32[128 * 128];
}

impl Default for Texture {
    publ this(string name) {
    SDL_Surface *s = loadBmp(name);
    glGenTextures(1, &num);
    glBindTexture(GL_TEXTURE_2D, num);
    gluBuild2DMipmaps(GL_TEXTURE_2D, 4, s.w, s.h, 
                      GL_RGBA, GL_UNSIGNED_BYTE, s.pixels);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  }
}

impl Texture {
  let loadBmp(&mut self, name : &String) -> &SDL_Surface {
    if name in surface {
      return self.surface[name];
    } else {
      let fileName = imagesDir + name;
      let s &SDL_Surface = SDL_LoadBMP(fileName);
      if !s {
        panic!("Unable to load: {}", fileName);
      }
      let mut format : SDL_PixelFormat;
      format.palette = null;
      format.BitsPerPixel = 32;
      format.BytesPerPixel = 4;
      format.Rmask = 0x000000ff;
      format.Gmask = 0x0000ff00;
      format.Bmask = 0x00ff0000;
      format.Amask = 0xff000000;
      format.Rshift = 0;
      format.Gshift = 8;
      format.Bshift = 16;
      format.Ashift = 24;
      format.Rloss = 0;
      format.Gloss = 0;
      format.Bloss = 0;
      format.Aloss = 0;
      format.alpha = 0;
      let &mut cs = SDL_ConvertSurface(s, &format, SDL_SWSURFACE);
      surface[name] = cs;
      cs
    }
  }

  fn this1(&mut self, name : &String) {
    let s : SDL_Surface = self.loadBmp(name);
    glGenTextures(1, &self.num);
    glBindTexture(GL_TEXTURE_2D, self.num);
    gluBuild2DMipmaps(GL_TEXTURE_2D, 4, s.w, s.h, 
                      GL_RGBA, GL_UNSIGNED_BYTE, s.pixels);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  }

  fn this2(&mut self, name : &String, sx : i32, sy : i32, xn : i32, yn : i32, panelWidth : i32, panelHeight : i32,
          maskColor : u32 /*= 0xffffffffu*/) {
    let &mut s : SDL_Surface = loadBmp(name);
    let surfacePixels : &u32= /*cast(Uint32*)*/ s.pixels;
    this3(surfacePixels, s.w, sx, sy, xn, yn, panelWidth, panelHeight, maskColor);
  }

  fn this3(&mut self, surfacePixels : &u32, surfaceWidth : i32,
            sx : i32, sy : i32, xn : i32, yn : i32, panelWidth : i32, panelHeight : i32,
            maskColor : u32 /*= 0xffffffffu*/) {
    self.textureNum = xn * yn;
    glGenTextures(self.textureNum, &num);
    if maskColor != 0xffffffffu {
      maskTextureNum = textureNum;
      glGenTextures(maskTextureNum, &maskNum);
    }
    let mut ti : i32 = 0;
    for oy in 0..yn {
      for ox in 0..xn {
        mut pi : i32 = 0;
        for y in 0..panelHeight {
          for x in 0..panelWidth {
            let p : u32 = surfacePixels[ox * panelWidth + x + sx + (oy * panelHeight + y + sy) * surfaceWidth];
            let m : u32;
            if p == maskColor {
              p = 0xff000000u32;
              m = 0x00ffffffu32;
            } else {
              m = 0x00000000u32;
            }
            self.pixels[pi] = p;
            if maskColor != 0xffffffffu32 {
              maskPixels[pi] = m;
            }
            pi += 1;
          }
        }
        glBindTexture(GL_TEXTURE_2D, num + ti);
        gluBuild2DMipmaps(GL_TEXTURE_2D, 4, panelWidth, panelHeight,
                          GL_RGBA, GL_UNSIGNED_BYTE, pixels.ptr);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER,GL_LINEAR_MIPMAP_NEAREST);
        glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER,GL_LINEAR);
        if maskColor != 0xffffffffu {
          glBindTexture(GL_TEXTURE_2D, maskNum + ti);
          gluBuild2DMipmaps(GL_TEXTURE_2D, 4, panelWidth, panelHeight,
                            GL_RGBA, GL_UNSIGNED_BYTE, self.maskPixels.ptr);
          glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER,GL_LINEAR_MIPMAP_NEAREST);
          glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER,GL_LINEAR);
        }
        ti += 1;
      }
    }
  }

  fn close(&self) {
    glDeleteTextures(self.textureNum, &self.num);
    if self.maskTextureNum > 0 {
      glDeleteTextures(self.maskTextureNum, &self.maskNum);
    }
  }

  fn bind(idx : i32 /*= 0*/) {
    glBindTexture(GL_TEXTURE_2D, self.num + idx);
  }

  public void bindMask(&self, idx : i32 /*= 0*/) {
    glBindTexture(GL_TEXTURE_2D, self.maskNum + idx);
  }
}

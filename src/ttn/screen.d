/*
 * $Id: screen.d,v 1.2 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.ttn.screen;


private import derelict.sdl.sdl;
private import derelict.opengl.gl;
private import derelict.opengl.glu;

private import src.util.sdl.screen3d;
private import src.ttn.field;


/**
 * OpenGL screen.
 */
public class Screen: Screen3D {
 private:
  static const char[] CAPTION = "Titanion";
  static const char[] ICON_FILE_NAME = "images/ttn_icon32.bmp";
  Field field;

  protected void setIcon() {
    SDL_WM_SetIcon(SDL_LoadBMP(ICON_FILE_NAME.ptr), null);
  }

  protected void init() {
    setCaption(CAPTION);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE);
    glEnable(GL_BLEND);
    glEnable(GL_LINE_SMOOTH);
    glDisable(GL_TEXTURE_2D);
    glDisable(GL_COLOR_MATERIAL);
    glDisable(GL_LIGHTING);
    glDisable(GL_DEPTH_TEST);
    glDisable(GL_CULL_FACE);
    setClearColor(0, 0, 0, 1);
  }

  public void setField(Field field) {
    this.field = field;
    screenResized();
  }

  protected void close() {}

  public override void screenResized() {
    super.screenResized();
    float lw = (cast(float) width / 640 + cast(float) height / 480) / 2;
    if (lw < 1)
      lw = 1;
    else if (lw > 4)
      lw = 4;
    glLineWidth(lw);
    glViewport(0, 0, width, height);
    if (field)
      field.setLookAt();
  }
}

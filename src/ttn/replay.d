/*
 * $Id: replay.d,v 1.4 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.ttn.replay;


private import tango.io.device.File;

private import src.util.sdl.pad;
private import src.util.sdl.recordableinput;


void read(T)(File fd, T* dst)
{
    auto count = fd.read ((cast(void*) &dst)[0..int.sizeof]);
    assert (count is int.sizeof);
}

void write(T)(File fd, T* dst)
{
    auto count = fd.write ((cast(void*) &dst)[0..int.sizeof]);
    assert (count is int.sizeof);
}

/**
 * Save/Load a replay data.
 */
public class ReplayData {
 public:
  static const char[] DIR = "replay";
  static /*const*/ int VERSION_NUM = 30;
  InputRecord!(PadState) inputRecord;
  long seed;
  int score = 0;
  int mode;
  bool stageRandomized;
 private:

  public void save(char[] fileName) {
    File fd = new File(DIR ~ "/" ~ fileName, File.WriteCreate);
    .write(fd, &VERSION_NUM);
    .write(fd, &seed);
    .write(fd, &score);
    .write(fd, &mode);
    .write(fd, cast(byte*) &stageRandomized);
    inputRecord.save(fd);
    fd.close();
  }

  public void load(char[] fileName) {
    File fd = new File(DIR ~ "/" ~ fileName, File.ReadExisting);
    int ver;
    .read(fd, &ver);
    if (ver != VERSION_NUM)
      throw new Exception("Wrong version num");
    .read(fd, &seed);
    .read(fd, &score);
    .read(fd, &mode);
    byte sr;
    .read(fd, &sr);
    stageRandomized = cast(bool) sr;
    inputRecord = new InputRecord!(PadState);
    inputRecord.load(fd);
    fd.close();
  }
}

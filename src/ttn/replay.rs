/*
 * $Id: replay.d,v 1.4 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.ttn.replay;


import std.stream;

private import src.util.sdl.pad;
private import src.util.sdl.recordableinput;
*/

fn read(T)(fd : &File, dst : &T)
{
    let count = fd.read ((cast(void*) &dst)[0..int.sizeof]);
    //assert (count is int.sizeof);
}

fn write(T)(fd : &File, dst : &T)
{
    let count = fd.write ((cast(void*) &dst)[0..int.sizeof]);
    //assert (count is int.sizeof);
}

let DIR = "replay";
let VERSION_NUM = 30;

/**
 * Save/Load a replay data.
 */
struct ReplayData {
  inputRecord : InputRecord<PadState>;
  seed : i64;
  score : i32; //= 0;
  int mode : i32;
  bool stageRandomized : bool;
}

impl ReplayData {
  fn save(&mut self, fileName : &String) {
    let fd : &File = new File(DIR + "/" + fileName, File.WriteCreate);
    .write(fd, &VERSION_NUM);
    .write(fd, &self.seed);
    .write(fd, &self.score);
    .write(fd, &self.mode);
    .write(fd, cast(byte*) &self.stageRandomized);
    self.inputRecord.save(fd);
    fd.close();
  }

  fm load(&mut self, fileName : &String) {
    let fd : &File  = File(DIR + "/" + fileName, File.ReadExisting);
    let mut ver : i32;
    read(fd, &ver);
    if ver != VERSION_NUM {
      pnaic!("Wrong version num");
    }
    .read(fd, &seed);
    .read(fd, &score);
    .read(fd, &mode);
    let sr : bool;
    .read(fd, &sr);
    self.stageRandomized = sr as bool;
    self.inputRecord = InputRecord<PadState>;
    self.inputRecord.load(fd);
    fd.close();
  }
}

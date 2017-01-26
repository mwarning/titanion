/*
 * $Id: recordableinput.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2005 Kenta Cho. Some rights reserved.
 */
 /*
module src.util.sdl.recordableinput;


private import std.stream;

private import src.util.iterator;
*/

/**
 * Record an input for a replay.
 * T represents a data structure of a specific device input.
 */

struct RecordableInput<T> {
  inputRecord : InputRecord!(T),
}

impl RecordableInput<T> {
  fn startRecord() {
    inputRecord = new InputRecord!(T);
    inputRecord.clear();
  }

  fn record(T d : T) {
    inputRecord.add(d);
  }

  fn startReplay(pr : InputRecord<T>) {
    inputRecord = pr;
    inputRecord.reset();
  }

  fn replay() -> T {
    if !inputRecord.hasNext() {
      panic!("No record data.");
    }
    return inputRecord.next();
  }
}

/*
public class NoRecordDataException: Exception {
  public this(string msg) {
    super(msg);
  }
}*/

struct Record<T> {
  series : i32,
  data : T
}

struct InputRecord<T> {
  record : Vec< Record<T> >;
  idx : i32,
  series : i32,
  replayData : T;
}

/*
  public this() {
    replayData = T.newInstance();
  }
*/
impl InputRecord<T> {}
  fn clear(&mut self) {
    self.record = null;
  }

  fn add(&mut self, d : T) {
    if self.record && self.record[self.record.len - 1].data.equals(d)) {
      self.record[record.length - 1].series += 1;
    } else {
      self.record.push(
         Record{series : 1, data : T(d)}
      );
    }
  }

  fn reset(&mut self) {
    self.idx = 0;
    self.series = 0;
  }

  fn hasNext(&self) -> bool {
    (self.idx < self.record)
  }

  fn next(&mut self) -> T {
    if idx >= record.length {
      panic!("No more items");
    }
    if self.series <= 0 {
      self.series = self.record[idx].series;
    }
    self.replayData.set(record[idx].data);
    series -= 1;
    if self.series <= 0 {
      idx += 1;
    }
    self.replayData;
  }

  fn save(&mut self, fd : &File) {
    fd.write(record.len);
    for let r in self.record {
      fd.write(r.series);
      r.data.write(fd);
    }
  }

  fn load(&mut self, fd : &File) {
    self.clear();
    let l : i32;
    let s : i32;
    let d : T;
    fd.read(l);
    for i in 0..l {
      fd.read(s);
      d = T.newInstance();
      d.read(fd);
      record ~= Record(series : s, data : d);
    }
  }
}

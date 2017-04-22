/*
 * Copyright 2005 Kenta Cho. Some rights reserved.
 */

/**
 * Record an input for a replay.
 * T represents a data structure of a specific device input.
 */

use util::sdl::pad::*;
use ttn::dummy::*;

/*
//inlined into pad.rs

struct RecordableInput<T> {
  inputRecord : InputRecord!(T),
}

impl RecordableInput<T> {
  fn startRecord(&mut self) {
    self.inputRecord = InputRecord::<T>::new();
    self.inputRecord.clear();
  }

  fn record(&mut self, d : T) {
    self.inputRecord.add(d);
  }

  fn startReplay(&mut self, pr : InputRecord<T>) {
    self.inputRecord = pr;
    self.inputRecord.reset();
  }

  fn replay(&mut self) -> T {
    if !self.inputRecord.hasNext() {
      panic!("No record data.");
    }
    self.inputRecord.next()
  }
}
*/

/*
public class NoRecordDataException: Exception {
  public this(string msg) {
    super(msg);
  }
}*/

struct Record<T> {
  series : i32,
  data : T,
}

pub struct InputRecord {
  record : Vec<Record<PadState>>,
  idx : usize, //i32,
  series : i32,
  replayData : PadState,
}

/*
  public this() {
    replayData = T.newInstance();
  }
*/

// was generic, but that makes not much sense
impl InputRecord { //<T> where T : PartialEq<T> + Default {
  pub fn new() -> InputRecord {
    InputRecord {
      record : Vec::<Record<PadState>>::new(),
      idx : 0,
      series : 0,
      replayData : PadState::new(),
    }
  }

  pub fn clear(&mut self) {
    self.record.clear();
  }

  pub fn add(&mut self, d : PadState) {
    //if Some(e) = self.record.last() {
    if (self.record.len() > 0) && (self.record[self.record.len() - 1].data == d) {
      self.record[self.record.len() - 1].series += 1;
    } else {
      self.record.push(
        Record{series : 1, data : d} //T::default(d)}
      );
    }
  }

  pub fn reset(&mut self) {
    self.idx = 0;
    self.series = 0;
  }

  pub fn hasNext(&self) -> bool {
    (self.idx < self.record.len())
  }

  pub fn next(&mut self) -> PadState {
    if self.idx >= self.record.len() {
      panic!("No more items");
    }
    if self.series <= 0 {
      self.series = self.record[self.idx].series;
    }
    self.replayData.set(self.record[self.idx].data);
    self.series -= 1;
    if self.series <= 0 {
      self.idx += 1;
    }
    self.replayData
  }

  pub fn save(&mut self, fd : &File) {
    fd.write2(self.record.len());
    for r in self.record {
      fd.write1(r.series);
      r.data.write(fd);
    }
  }

  pub fn load(&mut self, fd : &File) {
    self.clear();
    let l : i32;
    let s : i32;
    let d : PadState = PadState::new();
    fd.read2(&l);
    for _ in 0..l {
      fd.read2(&s);
      fd.read1(&d);
      self.record.push(Record{series : s, data : d});
    }
  }
}

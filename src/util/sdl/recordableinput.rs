/*
 * Copyright 2005 Kenta Cho. Some rights reserved.
 */

/**
 * Record an input for a replay.
 * T represents a data structure of a specific device input.
 */

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

pub struct InputRecord<T> {
  record : Vec<Record<T>>,
  idx : i32,
  series : i32,
  replayData : T,
}

/*
  public this() {
    replayData = T.newInstance();
  }
*/

impl<T> InputRecord<T> {
  pub fn new() -> InputRecord<T> {
    InputRecord::<T> {
      record : Vec::<Record<T>>::new(),
      idx : 0,
      series : 0,
      replayData : T::newInstance(),
    }
  }

  pub fn clear(&mut self) {
    self.record.clear();
  }

  pub fn add(&mut self, d : T) {
    //if Some(e) = self.record.last() {
    if (self.record.len() > 0) && (self.record[self.record.len() - 1].data == d) {
      self.record[self.record.len() - 1].series += 1;
    } else {
      self.record.push(
         Record{series : 1, data : T::new(d)}
      );
    }
  }

  pub fn reset(&mut self) {
    self.idx = 0;
    self.series = 0;
  }

  pub fn hasNext(&self) -> bool {
    (self.idx < self.record)
  }

  pub fn next(&mut self) -> T {
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
    self.replayData;
  }

  pub fn save(&mut self, fd : &File) {
    fd.write(self.record.len());
    for r in self.record {
      fd.write(r.series);
      r.data.write(fd);
    }
  }

  pub fn load(&mut self, fd : &File) {
    self.clear();
    let l : i32;
    let s : i32;
    let d : T;
    fd.read(l);
    for i in 0..l {
      fd.read(s);
      d = T::newInstance();
      d.read(fd);
      self.record.push_back(Record{series : s, data : d});
    }
  }
}

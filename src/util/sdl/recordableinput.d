/*
 * $Id: recordableinput.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2005 Kenta Cho. Some rights reserved.
 */
module src.util.sdl.recordableinput;


private import tango.io.device.File;

private import src.util.iterator;


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
 * Record an input for a replay.
 * T represents a data structure of a specific device input.
 */
public template RecordableInput(T) {
 public:
  InputRecord!(T) inputRecord;
 private:

  public void startRecord() {
    inputRecord = new InputRecord!(T);
    inputRecord.clear();
  }

  public void record(T d) {
    inputRecord.add(d);
  }

  public void startReplay(InputRecord!(T) pr) {
    inputRecord = pr;
    inputRecord.reset();
  }

  public T replay() {
    if (!inputRecord.hasNext())
      throw new NoRecordDataException("No record data.");
    return inputRecord.next();
  }
}

public class NoRecordDataException: Exception {
  public this(char[] msg) {
    super(msg);
  }
}

public class InputRecord(T) {
 private:
  struct Record {
    int series;
    T data;
  };
  Record[] record;
  int idx, series;
  T replayData;

  public this() {
    replayData = T.newInstance();
  }

  public void clear() {
    record = null;
  }

  public void add(T d) {
    if (record && record[record.length - 1].data.equals(d)) {
      record[record.length - 1].series++;
    } else {
      Record r;
      r.series = 1;
      r.data = T.newInstance(d);
      record ~= r;
    }
  }

  public void reset() {
    idx = 0;
    series = 0;
  }

  public bool hasNext() {
    if (idx >= record.length)
      return false;
    else
      return true;
  }

  public T next() {
    if (idx >= record.length)
      throw new NoRecordDataException("No more items");
    if (series <= 0)
      series = record[idx].series;
    replayData.set(record[idx].data);
    series--;
    if (series <= 0)
      idx++;
    return replayData;
  }

  public void save(File fd) {
	auto len = record.length;
  .write(fd, &len);
    foreach (Record r; record) {
      .write(fd ,&r.series);
      r.data.write(fd);
    }
  }

  public void load(File fd) {
    clear();
    int l, s;
    T d;
	.read(fd, &l);
    for (int i = 0; i < l; i++) {
      .read(fd, &s);
      d = T.newInstance();
      d.read(fd);
      Record r;
      r.series = s;
      r.data = d;
      record ~= r;
    }
  }
}

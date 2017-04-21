/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

/**
 * Logger for error and info messages.
 */

fn info(msg : string, nline : bool/*= true*/) {
  if nline {
    println(msg);
  } else {
    print(msg);
  }
}

fn info(n : f64, nline : bool /* = true*/) {
  if nline {
    println("{}", n);
  } else {
    print("{}", n);
  }
}

fn error(msg : string) {
  println("Error: {}", msg);
}


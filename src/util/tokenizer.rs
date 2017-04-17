/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::io::prelude::*;
use std::fs::File;

fn readFile(fileName : &string, separator : &string) -> string {
  let mut file = File::open(fileName.unwrap());
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut result : Vec[String];

  // Returns amount of bytes read and append the result to the buffer
  let result = file.read_to_end(&mut contents).unwrap();
  for line in result.lines() {
   for s in line.split(separator) {
     result.push(s.trim());
   }
  }
  result
}

fn main() {


}

/*

/*
 * Tokenizer.
 */
public class Tokenizer {
 private:

  public static string[] readFile(string fileName, string separator) {
     string[] result;
     auto file = new File(fileName, FileMode.In);
     char[][] lines = splitLines(file);
     foreach(line; lines) {
      char[][] spl = split(line, separator);
      foreach (char[] s; spl) {
        char[] r = trim(s);
        if (r.length > 0)
          result ~= r.idup;
      }
    }
    return result;
  }
}

/*
 * CSV format tokenizer.
 */
public class CSVTokenizer {
 private:

  public static char[][] readFile(string fileName) {
    return Tokenizer.readFile(fileName, ",");
  }
}

*/

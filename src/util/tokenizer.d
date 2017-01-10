/*
 * $Id: tokenizer.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.util.tokenizer;


private import std.stream;
private import std.string;


/**
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

/**
 * CSV format tokenizer.
 */
public class CSVTokenizer {
 private:

  public static char[][] readFile(string fileName) {
    return Tokenizer.readFile(fileName, ",");
  }
}

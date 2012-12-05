/*
 * $Id: tokenizer.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.util.tokenizer;


private import tango.io.device.File;
private import tango.text.Util;


/**
 * Tokenizer.
 */
public class Tokenizer {
 private:

  public static char[][] readFile(char[] fileName, char[] separator) {
     char[][] result;
     auto file = cast(char[]) File.get(fileName);
     char[][] lines = splitLines(file);
     foreach(line; lines) {
      char[][] spl = split(line, separator);
      foreach (char[] s; spl) {
        char[] r = trim(s);
        if (r.length > 0)
          result ~= r;
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

  public static char[][] readFile(char[] fileName) {
    return Tokenizer.readFile(fileName, ",");
  }
}

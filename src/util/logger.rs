/*
 * $Id: logger.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
//module src.util.logger;


//private import std.stdio;
//private import std.string;
//private import std.conv;

/**
 * Logger for error and info messages.
 */
 /*
version(Win32_release) {

private import std.string;
private import std.c.windows.windows;

public class Logger {

  public static void info(char[] msg, bool nline = true) {
  }

  public static void info(double n, bool nline = true) {
  }

  private static void putMessage(char[] msg) {
    MessageBoxA(null, toStringz(msg), "Error", MB_OK | MB_ICONEXCLAMATION);
  }

  public static void error(char[] msg) {
    putMessage("Error: " ~ msg);
  }

  public static void error(Exception e) {
    putMessage("Error: " ~ e.toString());
  }
}

} else {
*/

//public class Logger {

  fn info(string msg, nline : bool/*= true*/) {
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

  fn error(string msg) {
    println("Error: {}", msg);
  }

  fn error(Exception e) {
    println("Error: {}", e.toString());
    //if (e.next)
    //  error(e.next);
  }
//}

//}

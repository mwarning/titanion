/*
 * $Id: logger.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.util.logger;


private import tango.stdc.stdio;
private import tango.io.Stdout;


/**
 * Logger for error and info messages.
 */
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

public class Logger {

  public static void info(char[] msg, bool nline = true) {
    if (nline)
      Stdout(msg).newline;
    else
      Stdout(msg).flush;   
  }

  public static void info(double n, bool nline = true) {
    if (nline)
      Stdout(n).newline;
    else
      Stdout(n).flush;
  }

  public static void error(char[] msg) {
    Stdout("Error: ")(msg).newline;
  }

  public static void error(Exception e) {
    Stdout("Error: ")(e.toString()).newline;
  }
}

}

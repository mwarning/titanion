
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::env;

struct Screen {
  brightness : f32,
  window_mode : bool,
  width : i32,
  height : i32,
}

impl Default for Screen {
  fn default () -> Screen {
    Screen {window_mode: false, width: 800, height: 600, brightness: 0.0}
  }
}

struct Frame {
}

struct RecordablePad {
}

struct Preference {
}

struct Sound {
  noSound : bool,
  bgmVol : i32,
  seVol : i32,
}

impl Default for Sound {
  fn default () -> Sound {
    Sound {noSound: false, bgmVol: 100, setVol: 600}
  }
}

struct MainLoop {
  screen : Screen,
  input : RecordablePad,
  frame : Frame,
  preference : Preference,
}

impl MainLoop {

  fn run_loop(&self) {

  }
}

fn usage(progName : &str ) {
  println!("Usage: {} [-fullscreen] [-res x y] [-brightness [0-100]] [-nosound] [-bgmvol [0-128]] [-sevol [0-128]] [-exchange] [-trail] [-noslowdown] [-randomized]", progName);
}

fn parse_args(args : &mut env::Args, main_loop : &mut MainLoop) {
  let progName = args.next().unwrap();

  while let Some(arg) = args.next() {
    match arg.as_ref() {
      "-fullscreen" => {
        main_loop.screen.window_mode = false;
      },
      "-window" => {
        main_loop.screen.window_mode = true;
      },
      "-res" => {
        let mut width = 0;
        let mut height = 0;
        if let (Some(w), Some(h)) = (args.next(), args.next()) {
           width = w.parse::<i32>().unwrap_or(0);
           height = h.parse::<i32>().unwrap_or(0);
        }

        if width == 0 || height == 0 {
          main_loop.screen.width = width;
          main_loop.screen.height = height;
        } else {
           usage(progName.as_ref());
           panic!("Invalid options for {}", arg);
        }
      },
     "-brightness" => {
        let mut brightness : f32 = -1.0;
        if let Some(b) = args.next() {
         brightness = b.parse::<f32>().unwrap_or(-1.0) / 100.0;
        }

        if brightness < 0.0 || brightness > 1.0 {
          usage(progName.as_ref());
          panic!("Invalid option for {}", arg);
        }

        main_loop.screen.brightness = brightness;
      },
      "-nosound" => {
        main_loop.sound.noSound = true;
      },
      "-bgmvol" => {
        let v : i32 = -1;
        if let Some(b) = args.next() {
          v = b.parse::<i32>().unwrap_or(-1);
        }

        if v < 0 || v > 128 {
          usage(progName);
          panic!("Invalid options for {}", arg);
        } else {
          main_loop.sound.bgmVol = v;
        }
      },
      "-sevol" => {
        let v : i32 = -1;
        if let Some(b) = args.next() {
          v = b.parse::<i32>().unwrap_or(-1);
        }

        if v < 0 || v > 128 {
          usage(progName);
          panic!("Invalid options for {}", arg);
        } else {
          main_loop.sound.setVol = v;
        }
      },
      "-exchange" => {
        pad.buttonsExchanged = true;
      },
      "-trail" => {
        EnemyPool.trailEffect = true;
      },
      "-noslowdown" => {
        mainLoop.noSlowdown = true;
      },
      "-randomized" => {
        GameState.stageRandomized = true;
      },
      _ => {
        usage(progName);
        panic!("Invalid option {}", arg);
      }
    }
  }
}

enum EXIT {
 SUCCESS = 0,
 FAILURE = 1,
}

fn boot() -> i32 {
	let mut screen = Screen::default();
	let mut input = RecordablePad{};
	let mut frame = Frame{};
	let mut preference = Preference{};
	let mut main_loop = MainLoop{screen : screen, input : input, frame : frame, preference : preference};

  let mut args = env::args();
	parse_args(&mut args, &mut main_loop);

	main_loop.run_loop();

  std::process::exit(EXIT::SUCCESS as i32)
}

// Boot as the general executable.
fn main() {
	//set working directory to binary location
	//char[] path = dirname(args[0]);
	//Environment.cwd(path);

	let exit_code = boot();
  std::process::exit(exit_code);
}


/*

/*
 * $Id: boot.d,v 1.3 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.ttn.boot;


private import std.conv;

private import src.util.logger;
private import src.util.tokenizer;
private import src.util.sdl.mainloop;
private import src.util.sdl.input;
private import src.util.sdl.pad;
private import src.util.sdl.recordableinput;
private import src.util.sdl.sound;
private import src.ttn.screen;
private import src.ttn.frame;
private import src.ttn.preference;
private import src.ttn.enemy;
private import src.ttn.particle;





/**
 * Boot the game.
 */
private:
Screen screen;
RecordablePad input;
Frame frame;
Preference preference;
MainLoop mainLoop;

version (Win32_release) {
  // Boot as the Windows executable.
  private import std.c.windows.windows;
  private import std.string;

  extern (C) void gc_init();
  extern (C) void gc_term();
  extern (C) void _minit();
  extern (C) void _moduleCtor();

  extern (Windows)
    public int WinMain(HINSTANCE hInstance,
                     HINSTANCE hPrevInstance,
                     LPSTR lpCmdLine,
                     int nCmdShow) {
    int result;
    gc_init();
    _minit();
    try {
      _moduleCtor();
      char[4096] exe;
      GetModuleFileNameA(null, exe, 4096);
      char[][1] prog;
      prog[0] = std.string.toString(exe);
      result = boot(prog ~ std.string.split(std.string.toString(lpCmdLine)));
    } catch (Object o) {
      Logger.error("Exception: " ~ o.toString());
      result = EXIT_FAILURE;
    }
    gc_term();
    return result;
  }
} else {
    private import core.stdc.stdlib;
    
    char[] dirname(char[] path) {
      auto i = path.length;
      if(i == 0)
        return path;
      while(--i) {
          if(path[i] == '/')
            return path[0..i+1];
      }
      return null;
    }

  // Boot as the general executable.
  public int main(string[] args) {
    //set working directory to binary location
    //char[] path = dirname(args[0]);
    //Environment.cwd(path);
      
    return boot(args);
  }
}

public int boot(string[] args) {
  screen = new Screen;
  input = new RecordablePad;
  frame = new Frame;
  preference = new Preference;
  mainLoop = new MainLoop(screen, input, frame, preference);
  try {
    parseArgs(args, screen, input, mainLoop);
  } catch (Exception e) {
    return EXIT_FAILURE;
  }
  mainLoop.loop();
  return EXIT_SUCCESS;
}

private void parseArgs(string[] commandArgs, Screen screen, RecordablePad pad, MainLoop mainLoop) {
  string[] args = readOptionsIniFile();
  for (int i = 1; i < commandArgs.length; i++)
    args ~= commandArgs[i];
  string progName = commandArgs[0];
  for (int i = 0; i < args.length; i++) {
    switch (args[i]) {
    case "-fullscreen":
      screen.windowMode = false;
      break;
    case "-window":
      screen.windowMode = true;
      break;
    case "-res":
      if (i >= args.length - 2) {
        usage(progName);
        throw new Exception("Invalid options");
      }
      i++;
      int w = to!int(args[i]);
      i++;
      int h = to!int(args[i]);
      screen.width = w;
      screen.height = h;
      break;
    case "-brightness":
      if (i >= args.length - 1) {
        usage(progName);
        throw new Exception("Invalid options");
      }
      i++;
      float b = cast(float) to!int(args[i]) / 100;
      if (b < 0 || b > 1) {
        usage(args[0]);
        throw new Exception("Invalid options");
      }
      Screen.brightness = b;
      break;
    case "-nosound":
      Sound.noSound = true;
      break;
    case "-bgmvol":
      if (i >= args.length - 1) {
        usage(progName);
        throw new Exception("Invalid options");
      }
      i++;
      int v = to!int(args[i]);
      if (v < 0 || v > 128) {
        usage(args[0]);
        throw new Exception("Invalid options");
      }
      Sound.bgmVol = v;
      break;
    case "-sevol":
      if (i >= args.length - 1) {
        usage(progName);
        throw new Exception("Invalid options");
      }
      i++;
      int v = to!int(args[i]);
      if (v < 0 || v > 128) {
        usage(args[0]);
        throw new Exception("Invalid options");
      }
      Sound.seVol = v;
      break;
    case "-exchange":
      pad.buttonsExchanged = true;
      break;
    case "-trail":
      EnemyPool.trailEffect = true;
      break;
    case "-noslowdown":
      mainLoop.noSlowdown = true;
      break;
    case "-randomized":
      GameState.stageRandomized = true;
      break;
    default:
      usage(progName);
      throw new Exception("Invalid options");
    }
  }
}

private const string OPTIONS_INI_FILE = "options.ini";

private string[] readOptionsIniFile() {
  try {
    return Tokenizer.readFile(OPTIONS_INI_FILE, " ");
  } catch (Throwable e) {
    return null;
  }
}

private void usage(string progName) {
  Logger.error
    ("Usage: " ~ progName ~ " [-fullscreen] [-res x y] [-brightness [0-100]] [-nosound] [-bgmvol [0-128]] [-sevol [0-128]] [-exchange] [-trail] [-noslowdown] [-randomized]");
}

*/

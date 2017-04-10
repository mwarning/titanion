
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]


use std::env;
use std::process::exit;

use util::sdl::pad::*;
use util::sdl::mainloop::*;
use util::sdl::pad::*;
use ttn::screen::*;
use ttn::frame::*;
use ttn::preference::*;


fn usage(progName : &str ) {
  println!("Usage: {} [-fullscreen] [-res x y] [-brightness [0-100]] [-nosound] [-bgmvol [0-128]] [-sevol [0-128]] [-exchange] [-trail] [-noslowdown] [-randomized]", progName);
}

fn parse_args(args : &mut env::Args, main_loop : &mut MainLoop) -> bool {
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
        main_loop.frame.pad.buttonsExchanged = true;
      },
      "-trail" => {
        main_loop.enemies.trailEffect = true;
      },
      "-noslowdown" => {
        main_loop.noSlowdown = true;
      },
      "-randomized" => {
        main_loop.frame.gameState.stageRandomized = true;
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
  let screen = Screen::new();
  let input = RecordablePad::new();
  let preference = Preference::new();
  let frame = Frame::new(screen, input, preference);
  let mut main_loop = MainLoop::new(frame);

  let mut args = env::args();
  if !parse_args(&mut args, &mut main_loop) {
    main_loop.loop1();
    EXIT::SUCCESS as i32
  } else {
    EXIT::FAILURE as i32
  }
}

// Boot as the general executable.
fn main() {
	//set working directory to binary location
	//char[] path = dirname(args[0]);
	//Environment.cwd(path);

	let exit_code = boot();
  exit(exit_code);
}

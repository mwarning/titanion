
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]


use std::env;
use std::process::exit;

use util::sdl::pad::*;
use util::sdl::mainloop::*;
use util::sdl::sound::*;
use util::sdl::sound;
use util::sdl::pad::*;
use ttn::frame;
use ttn::screen::*;
use ttn::frame::*;
use ttn::enemy::*;
use ttn::enemy;
use ttn::preference::*;


fn usage(progName : &str ) {
  println!("Usage: {} [-fullscreen] [-res x y] [-brightness [0-100]] [-nosound] [-bgmvol [0-128]] [-sevol [0-128]] [-exchange] [-trail] [-noslowdown] [-randomized]", progName);
}

fn parse_args(args : &env::Args, main_loop : &MainLoop) -> bool {
  let progName = args.next().unwrap().as_ref();
  let frame = &main_loop.frame;
  let screen = frame.screen.borrow_mut();
  let sound = frame.sound.borrow_mut();
  let pad = frame.pad.borrow_mut();

  while let Some(arg) = args.next() {
    match arg.as_ref() {
      "-fullscreen" => {
        screen._windowMode = false;
      },
      "-window" => {
        screen._windowMode = true;
      },
      "-res" => {
        let mut width = 0;
        let mut height = 0;
        if let (Some(w), Some(h)) = (args.next(), args.next()) {
          width = w.parse::<i32>().unwrap_or(0);
          height = h.parse::<i32>().unwrap_or(0);
        }

        if (width == 0) || (height == 0) {
          screen.width2(width);
          screen.height2(height);
        } else {
          usage(progName);
          panic!("Invalid options for {}", arg);
        }
      },
     "-brightness" => {
        let mut brightness : f32 = -1.0;
        if let Some(b) = args.next() {
          brightness = b.parse::<f32>().unwrap_or(-1.0) / 100.0;
        }

        if brightness < 0.0 || brightness > 1.0 {
          usage(progName);
          panic!("Invalid option for {}", arg);
        }

        screen.brightness(brightness);
      },
      "-nosound" => {
        sound::noSound = true;
      },
      "-bgmvol" => {
        let v : i32 = -1;
        if let Some(b) = args.next() {
          v = b.parse::<i32>().unwrap_or(-1);
        }

        if (v < 0) || (v > 128) {
          usage(progName);
          panic!("Invalid options for {}", arg);
        } else {
          sound::bgmVol = v;
        }
      },
      "-sevol" => {
        let v : i32 = -1;
        if let Some(b) = args.next() {
          v = b.parse::<i32>().unwrap_or(-1);
        }

        if (v < 0) || (v > 128) {
          usage(progName);
          panic!("Invalid options for {}", arg);
        } else {
          sound::seVol = v;
        }
      },
      "-exchange" => {
        pad.pad.buttonsExchanged = true;
      },
      "-trail" => {
        enemy::trailEffect = true;
      },
      "-noslowdown" => {
        main_loop.noSlowdown = true;
      },
      "-randomized" => {
        frame::stageRandomized = true;
      },
      _ => {
        usage(progName);
        panic!("Invalid option {}", arg);
      }
    };
  }
  true
}

enum EXIT {
  SUCCESS = 0,
  FAILURE = 1,
}

pub fn boot() -> i32 {
  let screen = Screen::new();
  let input = RecordablePad::new();
  let preference = Preference::new();
  let frame = Frame::new(screen, input, preference);
  let mut main_loop = MainLoop::new(frame);

  let mut args = env::args();
  if !parse_args(&args, &main_loop) {
    main_loop.loop1();
    EXIT::SUCCESS as i32
  } else {
    EXIT::FAILURE as i32
  }
}

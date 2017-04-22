/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::collections::HashMap;
use std::fs;

use util::sdl::sound::*;
use util::rand::*;
use ttn::dummy::*;


/**
 * Manage BGMs and SEs.
 */

const seFileName : &'static [ &'static str ] = &[
  "shot.wav", "explosion1.wav", "explosion2.wav",
  "explosion3.wav","tractor.wav", "flying_down.wav",
  "player_explosion.wav", "flick.wav", "extend.wav"
];

const seChannel : [i32; 9] = [0, 1, 2, 3, 4, 5, 6, 6, 7];


pub struct Sound {
  bgm : HashMap<&'static str, Music>,
  se : HashMap<&'static str, Chunk>,
  seMark : HashMap<&'static str, bool>,
  bgmFileName : Vec<&'static str>,
  currentBgm : String,
  prevBgmIdx : i32,
  nextIdxMv : i32,
  _bgmEnabled : bool,
  _seEnabled : bool,
  rand : Rand,
}

impl Sound {
  pub fn new() -> Sound {
    Sound {
      bgm : HashMap::new(),
      se : HashMap::new(),
      seMark : HashMap::new(),
      bgmFileName : Vec::new(),
      currentBgm : "",
      prevBgmIdx : 0,
      nextIdxMv : 0,
      _bgmEnabled : false,
      _seEnabled : false,
      rand : Rand::new(),

      // inline from util/sdl/sound.rs
      //noSound : false,
      //bgmVol : 100,
      //seVol : 100,
    }
  }

  // inline from util/sdl/sound.rs
  pub fn init(&mut self) {
    if Sound::noSound {
      return;
    }

    //derelict specific
    DerelictSDLMixer::load(); 

    if SDL_InitSubSystem(SDL_INIT_AUDIO) < 0 {
      self.noSound = true;
      panic!("Unable to initialize SDL_AUDIO: {}", SDL_GetError());
    }
    let audio_rate : i32 = 44100;
    let audio_format : u16 = AUDIO_S16;
    let audio_channels : i32 = 1;
    let audio_buffers : i32 = 4096;
    if Mix_OpenAudio(audio_rate, audio_format, audio_channels, audio_buffers) < 0 {
      self.noSound = true;
      panic!("Couldn't open audio: {}", SDL_GetError());
    }
    Mix_QuerySpec(&audio_rate, &audio_format, &audio_channels);
    Mix_VolumeMusic(self.bgmVol);
    Mix_Volume(-1, self.seVol);
  }

  // inline from util/sdl/sound.rs
  pub fn close(&self) {
    if self.noSound {
      return;
    }
    if Mix_PlayingMusic() {
      Mix_HaltMusic();
    }
    Mix_CloseAudio();
  }

  pub fn load(&mut self) {
    self._bgmEnabled = true;
    self._seEnabled = true;
    self.loadMusics();
    self.loadChunks();
  }
/*
  fn load(&mut self) {
    self.loadMusics();
    self.loadChunks();
  }
*/
  fn loadMusics(&mut self) {
    let mut musics : HashMap<&str, Music> = HashMap::new();

    if let Ok(entries) = fs::read_dir(Music::dir) {
      for e in entries.filter_map(Result::ok) {
        let path = e.path();
        let fileName = path.to_str().unwrap_or("");
        if path.is_file() && (fileName.ends_with(".txt") || fileName.ends_with(".ogg")) {
            let music = Music::new();
            music.load(fileName);
            self.bgm[fileName] = music;
            self.bgmFileName.append(fileName);
            println!("Load bgm: {}", fileName);
        }
      }
    }
  }

  fn loadChunks(&mut self) {
    let i : i32 = 0;
    for fileName in self.seFileName {
      let chunk = Chunk::new();
      chunk.load(fileName, self.seChannel[i]);
      self.se[fileName] = chunk;
      self.seMark[fileName] = false;
      println!("Load SE: {}", fileName);
      i += 1;
    }
  }

  fn playBgm2(&mut self, name : &'static str) {
    self.currentBgm = name;
    if !self._bgmEnabled {
      return;
    }
    Music::halt();
    if self.bgm.contains(name) {
      self.bgm[name].play();
    } else {
      println!("Invalid bgm: {}", name);
    }
  }

  pub fn playBgm(&self) {
    let bgmIdx = self.rand.nextInt(self.bgm.length);
    let nextIdxMv = self.rand.nextInt(2) * 2 - 1;
    self.prevBgmIdx = self.bgmIdx;
    self.playBgm2(self.bgmFileName[bgmIdx as usize]);
  }

  pub fn nextBgm(&mut self) {
    let mut bgmIdx = self.prevBgmIdx + self.nextIdxMv;
    if bgmIdx < 0 {
      bgmIdx = self.bgm.length - 1;
    } else if bgmIdx >= self.bgm.length {
      bgmIdx = 0;
    }
    self.prevBgmIdx = bgmIdx;
    self.playBgm2(self.bgmFileName[bgmIdx as usize]);
  }

  pub fn playCurrentBgm(&mut self) {
    self.playBgm2(self.currentBgm);
  }

  pub fn fadeBgm() {
    Music::fade();
  }

  pub fn haltBgm() {
    Music::halt();
  }

  pub fn playSe(&mut self, name : &str) {
    if !self._seEnabled {
      return;
    }
    self.seMark[name] = true;
  }

  pub fn playMarkedSes(&self) {
    for (key, _) in self.seMark {
      if self.seMark.contains(key) {
        self.se[key].play();
        self.seMark[key] = false;
      }
    }
  }

  pub fn clearMarkedSes(&self) {
    for (key, _) in self.seMark {
      self.seMark[key] = false;
    }
  }

  fn bgmEnabled(&mut self, v : bool) {
    self._bgmEnabled = v;
  }

  fn seEnabled(&mut self, v : bool) {
    self._seEnabled = v;
  }
}

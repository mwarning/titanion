/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */


use std::ptr;

use ttn::dummy::*;

/**
 * Initialize and close SDL_mixer.
 */


// inline from util/sdl/sound.rs
pub static mut noSound : bool = false;
pub static mut bgmVol : i32 = 100;
pub static mut seVol : i32 = 100;


// inlined into ttn/sound.rs
/*
pub struct Sound {
  /*static*/ noSound : bool,
  /*static*/ bgmVol : i32,
  /*static*/ seVol : i32,
}

impl Sound {
  fn new() -> Sound {
    Sound{noSound : false, bgmVol : 100, seVol : 100}
  }

  fn init(&mut self) {
    if self.noSound {
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

  fn close(&self) {
    if self.noSound {
      return;
    }
    if Mix_PlayingMusic() {
      Mix_HaltMusic();
    }
    Mix_CloseAudio();
  }
}
*/

/**
 * Music.
 */
static mut fadeOutSpeed : i32 = 1280;

pub struct Music {
  music : *const Mix_Music,
}

impl Music {
  pub fn new() -> Music {
    Music{music : ptr::null()}
  }

  pub fn load(&mut self, name : &'static str) {
    if noSound {
      return;
    }

    let fileName = format!("sounds/music/{}", name);
    self.music = Mix_LoadMUS(fileName.as_ref());
    if self.music == ptr::null() {
      noSound = true;
      panic!("Couldn't load: {} ({})", fileName, Mix_GetError());
    }
  }

  pub fn free(&self) {
    self.halt();
    Mix_FreeMusic(self.music);
  }

  pub fn play(&self) {
    if !noSound {
      Mix_PlayMusic(self.music, -1);
    }
  }

  pub fn playOnce(&self) {
    if !noSound {
      Mix_PlayMusic(self.music, 1);
   }
  }

  pub fn fade() {
    if noSound {
      Mix_FadeOutMusic(fadeOutSpeed);
    }
  }

  pub fn halt() {
    if !noSound && (Mix_PlayingMusic() != 0) {
      Mix_HaltMusic();
    }
  }
}

/**
 * Sound chunk.
 */
pub struct Chunk {
  chunk : *const Mix_Chunk,
  chunkChannel : i32,
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      chunk : ptr::null(),
      chunkChannel : 0,
    }
  }

  pub fn load(&mut self, name : &'static str, ch : i32) {
    if noSound {
      return;
    }
    let fileName = format!("sounds/chunks/{}", name);
    self.chunk = Mix_LoadWAV(fileName.as_ref());
    if self.chunk.is_null() {
      noSound = true;
      panic!("Couldn't load: {} ({}", fileName, Mix_GetError());
    }
    self.chunkChannel = ch;
  }

  pub fn free(&self) {
    if !self.chunk.is_null() {
      self.halt();
      Mix_FreeChunk(self.chunk);
    }
  }

  pub fn play(&self) {
    if noSound {
      return;
    }
    Mix_PlayChannel(self.chunkChannel, self.chunk, 0);
  }

  pub fn halt(&self) {
    if noSound {
      return;
    }
    Mix_HaltChannel(self.chunkChannel);
  }
}

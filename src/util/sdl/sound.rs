/*
 * $Id: sound.d,v 1.2 2006/12/04 16:04:27 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
 /*
module src.util.sdl.sound;


private import std.string;
private import core.stdc.string;

private import derelict.sdl.sdl;
private import derelict.sdl.mixer;

private import src.util.sdl.sdlexception;
*/

/**
 * Initialize and close SDL_mixer.
 */
struct Sound {
  /*static*/ noSound : bool;
  /*static*/ bgmVol : i32;
  /*static*/ seVol : i32;
}

impl DSound {
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
    Mix_VolumeMusic(bgmVol);
    Mix_Volume(-1, seVol);
  }

  fn close(&self) {
    if (self.noSound) {
      return;
    }
    if Mix_PlayingMusic() {
      Mix_HaltMusic();
    }
    Mix_CloseAudio();
  }
}

/**
 * Music.
 */
struct Music {
  /*static*/ fadeOutSpeed : i32,
  /*static*/ dir : String,
  Mix_Music : &music,
}

impl Music {
  fn new() -> Music {
    Music{fadeOutSpeed : 1280, dir : "sounds/music"}
  }

  fn load(&mut self, name : &String) {
    if Sound::noSound {
      return;
    }
    let fileName : String = dir + "/" + name;
    self.music = Mix_LoadMUS(fileName);
    if !self.music {
      Sound::noSound = true;
      panic!("Couldn't load: {} ({})", fileName, Mix_GetError);
    }
  }

  fn free(&self) {
    if self.music {
      self.halt();
      Mix_FreeMusic(music);
    }
  }

  fn play(&self) {
    if !Sound::noSound {
     Mix_PlayMusic(music, -1);
    }
  }

  fn playOnce(&self) {
    if !Sound::noSound {
     Mix_PlayMusic(music, 1);
   }
  }

  fn fade(&self) {
    if Sound::noSound {
      Mix_FadeOutMusic(fadeOutSpeed);
    }
  }

  fn halt(&self) {
    if !Sound::noSound && Mix_PlayingMusic() {
      Mix_HaltMusic();
    }
  }
}

/**
 * Sound chunk.
 */
struct Chunk {
  chunk : &Mix_Chunk;
  chunkChannel : i32
}

let dir = "sounds/chunks";

impl Chunk {
  fn load(&mut self, name : &String, ch : i32) {
    if Sound::noSound {
      return;
    }
    let fileName : String = dir ~ "/" ~ name;
    self.chunk = Mix_LoadWAV(fileName);
    if !self.chunk) {
      Sound::noSound = true;
      panic!("Couldn't load: {} ({}", fileName, Mix_GetError());
    }
    self.chunkChannel = ch;
  }

  fn free(&self) {
    if self.chunk {
      self.halt();
      Mix_FreeChunk(self.chunk);
    }
  }

  fn play(&self) {
    if Sound::noSound {
      return;
    }
    Mix_PlayChannel(self.chunkChannel, self.chunk, 0);
  }

  fn halt(&self) {
    if Sound::noSound {
      return;
    }
    Mix_HaltChannel(self.chunkChannel);
  }
}

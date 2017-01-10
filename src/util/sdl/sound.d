/*
 * $Id: sound.d,v 1.2 2006/12/04 16:04:27 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.util.sdl.sound;


private import std.string;
private import core.stdc.string;

private import derelict.sdl.sdl;
private import derelict.sdl.mixer;

private import src.util.sdl.sdlexception;


string fromStringz(char* str) {
    return (str[0..strlen(str)]).idup;
}

/**
 * Initialize and close SDL_mixer.
 */
public class Sound {
 public:
  static bool noSound = false;
  static int bgmVol = 100;
  static int seVol = 100;
 private:

  public static void init() {
    if (noSound)
      return;

    //derelict specific
    DerelictSDLMixer.load(); 
    
    int audio_rate;
    Uint16 audio_format;
    int audio_channels;
    int audio_buffers;
    if (SDL_InitSubSystem(SDL_INIT_AUDIO) < 0) {
      noSound = true;
      throw new SDLInitFailedException
        ("Unable to initialize SDL_AUDIO: " ~ fromStringz(SDL_GetError()));
    }
    audio_rate = 44100;
    audio_format = AUDIO_S16;
    audio_channels = 1;
    audio_buffers = 4096;
    if (Mix_OpenAudio(audio_rate, audio_format, audio_channels, audio_buffers) < 0) {
      noSound = true;
      throw new SDLInitFailedException
        ("Couldn't open audio: " ~ fromStringz(SDL_GetError()));
    }
    Mix_QuerySpec(&audio_rate, &audio_format, &audio_channels);
    Mix_VolumeMusic(bgmVol);
    Mix_Volume(-1, seVol);
  }

  public static void close() {
    if (noSound)
      return;
    if (Mix_PlayingMusic())
      Mix_HaltMusic();
    Mix_CloseAudio();
  }
}

/**
 * Music.
 */
public class Music {
 public:
  static int fadeOutSpeed = 1280;
  static string dir = "sounds/musics";
 private:
  Mix_Music* music;

  public void load(string name) {
    if (Sound.noSound)
      return;
    string fileName = dir ~ "/" ~ name;
    music = Mix_LoadMUS(toStringz(fileName));
    if (!music) {
      Sound.noSound = true;
      throw new SDLException("Couldn't load: " ~ fileName ~ 
                             " (" ~ fromStringz(Mix_GetError()) ~ ")");
    }
  }

  public void free() {
    if (music) {
      halt();
      Mix_FreeMusic(music);
    }
  }

  public void play() {
    if (Sound.noSound)
      return;
    Mix_PlayMusic(music, -1);
  }

  public void playOnce() {
    if (Sound.noSound)
      return;
    Mix_PlayMusic(music, 1);
  }

  public static void fade() {
    if (Sound.noSound)
      return;
    Mix_FadeOutMusic(fadeOutSpeed);
  }

  public static void halt() {
    if (Sound.noSound)
      return;
    if (Mix_PlayingMusic())
      Mix_HaltMusic();
  }
}

/**
 * Sound chunk.
 */
public class Chunk {
 public:
  static string dir = "sounds/chunks";
 private:
  Mix_Chunk* chunk;
  int chunkChannel;

  public void load(string name, int ch) {
    if (Sound.noSound)
      return;
    string fileName = dir ~ "/" ~ name;
    chunk = Mix_LoadWAV(fileName);
    if (!chunk) {
      Sound.noSound = true;
      throw new SDLException("Couldn't load: " ~ fileName ~ 
                             " (" ~ fromStringz(Mix_GetError()) ~ ")");
    }
    chunkChannel = ch;
  }

  public void free() {
    if (chunk) {
      halt();
      Mix_FreeChunk(chunk);
    }
  }

  public void play() {
    if (Sound.noSound)
      return;
    Mix_PlayChannel(chunkChannel, chunk, 0);
  }

  public void halt() {
    if (Sound.noSound)
      return;
    Mix_HaltChannel(chunkChannel);
  }
}

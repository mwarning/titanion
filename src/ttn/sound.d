/*
 * $Id: sound.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.ttn.sound;


private import src.util.rand;
private import src.util.logger;
private import src.util.sdl.sound;


/**
 * Manage BGMs and SEs.
 */
public class Sound: src.util.sdl.sound.Sound {
  mixin StaticRandImpl;
 private static:
  string[] seFileName =
    ["shot.wav", "explosion1.wav", "explosion2.wav", "explosion3.wav",
     "tractor.wav", "flying_down.wav", "player_explosion.wav", "flick.wav", "extend.wav"];
  int[] seChannel =
    [0, 1, 2, 3, 4, 5, 6, 6, 7];
  Music[string] bgm;
  Chunk[string] se;
  bool[string] seMark;
  string[] bgmFileName;
  char[] currentBgm;
  int prevBgmIdx;
  int nextIdxMv;
  bool _bgmEnabled = true;
  bool _seEnabled = true;

  public static void load() {
    loadMusics();
    loadChunks();
  }

  private static void loadMusics() {
    Music[char[]] musics;
	
	foreach(child; Path.children(Music.dir)) {
		if(child.folder)
			continue;
		string fileName = child.name.dup;
		string ext = fileName.length > 3 ? fileName[$-3..$] : null;
		if (ext != "ogg" && ext != "wav")
			continue;
		Music music = new Music();
		music.load(fileName);
		bgm[fileName] = music;
		bgmFileName ~= fileName;
		Logger.info("Load bgm: " ~ fileName);
	}
  }

  private static void loadChunks() {
    int i = 0;
    foreach (string fileName; seFileName) {
      Chunk chunk = new Chunk();
      chunk.load(fileName, seChannel[i]);
      se[fileName] = chunk;
      seMark[fileName] = false;
      Logger.info("Load SE: " ~ fileName);
      i++;
    }
  }

  public static void playBgm(string name) {
    currentBgm = name;
    if (!_bgmEnabled)
      return;
    Music.halt();
    if(name in bgm)
        bgm[name].play();
    else
        Logger.info("Invalid bgm: " ~ name);
  }

  public static void playBgm() {
    int bgmIdx = rand.nextInt(bgm.length);
    nextIdxMv = rand.nextInt(2) * 2 - 1;
    prevBgmIdx = bgmIdx;
    playBgm(bgmFileName[bgmIdx]);
  }

  public static void nextBgm() {
    int bgmIdx = prevBgmIdx + nextIdxMv;
    if (bgmIdx < 0)
      bgmIdx = bgm.length - 1;
    else if (bgmIdx >= bgm.length)
      bgmIdx = 0;
    prevBgmIdx = bgmIdx;
    playBgm(bgmFileName[bgmIdx]);
  }

  public static void playCurrentBgm() {
    playBgm(currentBgm);
  }

  public static void fadeBgm() {
    Music.fade();
  }

  public static void haltBgm() {
    Music.halt();
  }

  public static void playSe(string name) {
    if (!_seEnabled)
      return;
    seMark[name] = true;
  }

  public static void playMarkedSes() {
    char[][] keys = seMark.keys;
    foreach (char[] key; keys) {
      if (seMark[key]) {
        se[key].play();
        seMark[key] = false;
      }
    }
  }

  public static void clearMarkedSes() {
    char[][] keys = seMark.keys;
    foreach (char[] key; keys)
      seMark[key] = false;
  }

  public static void bgmEnabled(bool v) {
    return _bgmEnabled = v;
  }

  public static void seEnabled(bool v) {
    return _seEnabled = v;
  }
}

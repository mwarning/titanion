/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use std::collections::HashMap;

/**
 * Manage BGMs and SEs.
 */

const seFileName : &'static [ &'static str ] = &["shot.wav", "explosion1.wav", "explosion2.wav", "explosion3.wav",
     "tractor.wav", "flying_down.wav", "player_explosion.wav", "flick.wav", "extend.wav"];

const seChannel : [i32] = &[0, 1, 2, 3, 4, 5, 6, 6, 7];

struct Sound {
  //src.util.sdl.sound.Sound
  //mixin StaticRandImpl;
 //private static:

  bgm : HashMap<&str, Music>,
  se : HashMap<&str, Chunk>,
  seMark : HashMap<&str, bool>,
  bgmFileName : Vec!<&str>,
  currentBgm : String,
  prevBgmIdx : i32;
  nextIdxMv : i32;
  _bgmEnabled : bool;
  _seEnabled : bool;
}

impl Sound {
  fn load(&mut self) {
    self._bgmEnabled = true;
    self._seEnabled = true;
    self.loadMusics();
    self.loadChunks();
  }

  fn load(&mut self) {
    self.loadMusics();
    self.loadChunks();
  }

  fn loadMusics(&mut self) {
    let mut musics : HashMap<&char, Music> = HashMap::new();

    if let Ok(entries) = fs::read_dir(Music.dir) {
      for e in entries.filter_map(Result::ok) {
        let path = e.path();
        let fileName = path.to_str().unwrap_or("");
        if path.is_file() && (fileName.ends_with(".txt") || fileName.ends_with(".ogg")) {
            let music = Music();
            music.load(fileName);
            bgm[fileName] = music;
            bgmFileName ~= fileName;
            Logger.info("Load bgm: {}", fileName);
        }
      }
    }
  }

  fn loadChunks(&mut self) {
    let i : i32 = 0;
    for fileName in self.seFileName {
      let chunk = Chunk();
      chunk.load(fileName, self.seChannel[i]);
      self.se[fileName] = chunk;
      self.seMark[fileName] = false;
      Logger.info("Load SE: {}", fileName);
      i += 1;
    }
  }

  fn playBgm(&mut self, name : &string) {
    self.currentBgm = name;
    if !self._bgmEnabled {
      return;
    }
    Music::halt();
    if self.bgm.contains(name) {
        bgm[name].play();
    } else {
        Logger.info("Invalid bgm: {}", name);
    }
  }

  fn playBgm(&self) {
    let bgmIdx : i32 = rand.nextInt(bgm.length);
    let nextIdxMv = rand.nextInt(2) * 2 - 1;
    self.prevBgmIdx = self.bgmIdx;
    self.playBgm(self.bgmFileName[bgmIdx]);
  }

  fn nextBgm(&self) {
    let mut bgmIdx : i32 = prevBgmIdx + nextIdxMv;
    if bgmIdx < 0 {
      bgmIdx = bgm.length - 1;
    } else if bgmIdx >= bgm.length) {
      bgmIdx = 0;
    }
    self.prevBgmIdx = bgmIdx;
    self.playBgm(self.bgmFileName[bgmIdx]);
  }

  fn playCurrentBgm(&self) {
    self.playBgm(self.currentBgm);
  }

  fn fadeBgm(&self) {
    Music::fade();
  }

  fn void haltBgm(&self) {
    Music::halt();
  }

  fn playSe(&mut self, name : String) {
    if !self._seEnabled {
      return;
    }
    self.seMark[name] = true;
  }

  fn playMarkedSes(&self) {
    for (key, _) in self.seMark {
      if self.seMark.contains(key) {
        self.se[key].play();
        self.seMark[key] = false;
      }
    }
  }

  fn clearMarkedSes(&self) {
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

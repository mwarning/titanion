/*
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */

use ttn::dummy::*;

/*
fn read(T)(fd : File, dst &T)
{
  let count = fd.read ((cast(void*) &dst)[0..int.sizeof]);
  //assert (count is int.sizeof);
}

fn write(T)(File fd, T* dst)
{
  let count = fd.write ((cast(void*) &dst)[0..int.sizeof]);
  //assert (count is int.sizeof);
}*/

fn read<T>(fd : &File, dst : &mut T) {
  //TODO
}

fn write<T>(fd : &File, dst : &T) {
  //TODO
}


pub const RANKING_NUM : usize = 10;
const MODE_NUM : usize = 3;
const VERSION_NUM : i32 = 30;
const PREF_FILE_NAME : &'static str = "ttn.prf";

/*
 * Load/Save/Record a high score table.
 */
pub struct Preference {
  //preference : src.util.preference.Preference:
  _highScore : [[i32; RANKING_NUM]; MODE_NUM],
  _lastMode : i32,
}

impl Preference {
  pub fn new() -> Self {
    Preference {
      _lastMode : 2,
      _highScore : [
        [100000, 90000, 80000, 70000, 60000, 50000, 40000, 30000, 20000, 10000],
        [100000, 90000, 80000, 70000, 60000, 50000, 40000, 30000, 20000, 10000],
        [100000, 90000, 80000, 70000, 60000, 50000, 40000, 30000, 20000, 10000]
      ],
    }
  }

  pub fn load(&mut self) {
    //let fd : File;
    //try {
    let fd = File::new(PREF_FILE_NAME, File::ReadExisting);
    let mut ver : i32;
    read::<i32>(fd, &ver);
    if ver != VERSION_NUM {
      panic!("Wrong version num");
    }
    read::<i32>(fd, &self._lastMode);
    for j in 0..MODE_NUM {
      for i in 0..RANKING_NUM {
        read::<i32>(fd, &self._highScore[j][i]);
      }
    }
    //} catch (Object e) {
    //  init();
    //} finally {
      if fd {
        fd.close();
      }
    //}
  }
/*
  fn init(&mut self) {
    self._lastMode = 2;
    for j in 0..MODE_NUM {
      for i in 0..RANKING_NUM {
        self._highScore[j][i] = (10 - i) * 10000;
      }
    }
  }
*/
  pub fn save(&mut self) {
    let fd = File::new(PREF_FILE_NAME, File::WriteCreate);
    write::<i32>(fd, &VERSION_NUM);
    write::<i32>(fd, &self._lastMode);
    for j in 0..MODE_NUM {
      for i in 0..RANKING_NUM {
        write::<i32>(fd, &self._highScore[j][i]);
      }
    }
    fd.close();
  }

  pub fn setMode(&mut self, mode : i32) {
    self._lastMode = mode;
  }

  pub fn recordResult(&mut self, score : i32, mode : i32) {
    self.setMode(mode);
    for i in 0..RANKING_NUM {
      if score > self._highScore[mode][i] {
        for j in ((i+1)..RANKING_NUM).rev() {
          self._highScore[mode][j] = self._highScore[mode][j - 1];
        }
        self._highScore[mode][i] = score;
        return;
      }
    }
  }

  pub fn highScore(&self) -> &[[i32; RANKING_NUM]; MODE_NUM] {
    self._highScore
  }

  pub fn lastMode(&self) -> i32 {
    self._lastMode
  }
}

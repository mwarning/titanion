/*
 * $Id: preference.d,v 1.4 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
/*
module src.ttn.preference;


private import src.util.preference;
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

fn write<T>(fd : &File, dst : &mut T) {
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
  fn load(&mut self) {
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

  fn init(&mut self) {
    self._lastMode = 2;
    for j in 0..MODE_NUM {
      for i in 0..RANKING_NUM {
        self._highScore[j][i] = (10 - i) * 10000;
      }
    }
  }

  fn save(&mut self) {
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

  fn setMode(&mut self, mode : i32) {
    self._lastMode = mode;
  }

  fn recordResult(&mut self, score : i32, mode : i32) {
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

  fn highScore(&self) -> &[[i32; RANKING_NUM]; MODE_NUM] {
    self._highScore
  }

  fn lastMode(&self) -> i32 {
    self._lastMode
  }
}

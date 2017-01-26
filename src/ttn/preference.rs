/*
 * $Id: preference.d,v 1.4 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
/*
module src.ttn.preference;


private import src.util.preference;
*/

fn read(T)(fd : File, dst &T)
{
    let count = fd.read ((cast(void*) &dst)[0..int.sizeof]);
    //assert (count is int.sizeof);
}

fn write(T)(File fd, T* dst)
{
    let count = fd.write ((cast(void*) &dst)[0..int.sizeof]);
    //assert (count is int.sizeof);
}


static RANKING_NUM : i32 = 10;
static MODE_NUM : i32 = 3;
static VERSION_NUM : i32 = 30;
static PREF_FILE_NAME = "ttn.prf";

/**
 * Load/Save/Record a high score table.
 */
 struct Preference {
	preference : src.util.preference.Preference:
  	_highScore : i32[RANKING_NUM][MODE_NUM];
  	_lastMode : i32;
}

impl Preference {
  fn load() {
    let fd : File;
    //try {
      fd = new File(PREF_FILE_NAME, File.ReadExisting);
      let mut ver : i32;
      .read(fd, &ver);
      if ver != VERSION_NUM {
        panic!("Wrong version num");
      }
    .read(fd, &_lastMode);
    for j in 0..MODE_NUM {
        for i in 0..RANKING_NUM {
           .read(fd, &_highScore[j][i]);
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
			_highScore[j][i] = (10 - i) * 10000;
		}
	}
  }

  fn save(&mut self) {
    let fd : File = new File(PREF_FILE_NAME, File.WriteCreate);
    .write(fd, &VERSION_NUM);
    .write(fd, &_lastMode);
    for j in 0..MODE_NUM {
     	for i in 0..RANKING_NUM {
        	.write(fd, &_highScore[j][i]);
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
      if score > self._highScore[mode][i]) {
		for j in ((i+1)..RANKING_NUM).rev() {
          self._highScore[mode][j] = self._highScore[mode][j - 1];
        }
        self._highScore[mode][i] = score;
        return;
      }
    }
  }

  fn highScore(&self) -> &i32[RANKING_NUM][] {
    self._highScore
  }

  fn lastMode(&self) -> i32 {
    self._lastMode
  }
}

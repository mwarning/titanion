/*
 * $Id: preference.d,v 1.4 2006/12/04 16:04:26 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
module src.ttn.preference;


private import src.util.preference;


void read(T)(File fd, T* dst)
{
    auto count = fd.read ((cast(void*) &dst)[0..int.sizeof]);
    assert (count is int.sizeof);
}

void write(T)(File fd, T* dst)
{
    auto count = fd.write ((cast(void*) &dst)[0..int.sizeof]);
    assert (count is int.sizeof);
}

/**
 * Load/Save/Record a high score table.
 */
public class Preference: src.util.preference.Preference {
 public:
  static const int RANKING_NUM = 10;
  static const int MODE_NUM = 3;
 private:
  static /*const*/ int VERSION_NUM = 30;
  static const char[] PREF_FILE_NAME = "ttn.prf";
  int[RANKING_NUM][MODE_NUM] _highScore;
  int _lastMode;

  public void load() {
    File fd;
    try {
      fd = new File(PREF_FILE_NAME, File.ReadExisting);
      int ver;
      .read(fd, &ver);
      if (ver != VERSION_NUM)
        throw new Exception("Wrong version num");
       .read(fd, &_lastMode);
      for(int j = 0; j < MODE_NUM; j++)
        for(int i = 0; i < RANKING_NUM; i++)
           .read(fd, &_highScore[j][i]);
    } catch (Object e) {
      init();
    } finally {
      if (fd)
          fd.close();
    }
  }

  private void init() {
    _lastMode = 2;
    for(int j = 0; j < MODE_NUM; j++)
      for(int i = 0; i < RANKING_NUM; i++)
        _highScore[j][i] = (10 - i) * 10000;
  }

  public void save() {
    File fd = new File(PREF_FILE_NAME, File.WriteCreate);
    .write(fd, &VERSION_NUM);
    .write(fd, &_lastMode);
    for(int j = 0; j < MODE_NUM; j++)
      for(int i = 0; i < RANKING_NUM; i++)
        .write(fd, &_highScore[j][i]);
    fd.close();
  }

  public void setMode(int mode) {
    _lastMode = mode;
  }

  public void recordResult(int score, int mode) {
    setMode(mode);
    for (int i = 0; i < RANKING_NUM; i++) {
      if (score > _highScore[mode][i]) {
        for (int j = RANKING_NUM - 1; j >= i + 1; j--) {
          _highScore[mode][j] = _highScore[mode][j - 1];
        }
        _highScore[mode][i] = score;
        return;
      }
    }
  }

  public int[RANKING_NUM][] highScore() {
    return _highScore;
  }

  public int lastMode() {
    return _lastMode;
  }
}

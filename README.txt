Titanion is a 2.5D shooter game for Windows, *nix and MacOSX.
You strike down super high-velocity swooping insects.

Titanion works on Windows98/2000/XP (OpenGL required), Unix, and MacOS.

Repository maintainer: Moritz Warning <moritzwarning@web.de>
Original author: (C) Kenta Cho


Strike down super high-velocity swooping insects.
Fixed shooter in the good old days, 'Titanion'.


* How to start

Unpack ttn0_4.zip, and run 'ttn.exe' or 'ttn'.
If you want to run in full-screen mode, run 'ttn_fullscreen.bat' or 'ttn_fullscreen-sh' instead.

At the title screen, you can select a game mode (CLASSIC, BASIC, MODERN)
with the up/down key. Push a shot button to start a game.

 + CLASSIC mode
  - Sporadic firing.
  - Use a capture beam to make a friend of an enemy.
  - When your ship collides against an enemy, your ship is destroyed.

 + BASIC mode
  - Middle course.
  - You can use the capture beam anytime.
  - When your ship collides against an enemy, your ship is knocked away.

 + MODERN mode
  - Bullet hell.
  - Use a provocation beam to force an enemy to fire more bullets and
    earn a bullet multiplier bonus.
  - When your ship collides against an enemy, nothing happens.
    Your ship and an enemy go through each other.


* How to play

Avoid incoming bullets and strike down all enemies.

- Controls

o Move
 Arrow / Num / [WASD] / [IJKL]  / Stick

o Shot
 [Z][L-Ctrl][R-Ctrl][.]         / Button 1, 3, 5, 7, 9, 11

 Hold down a button to fire automatically.

 + MODERN mode
  - Hold a provocation beam button at the same time
    to slow down speed of your ship and fire intensively.

o Capture beam / Provocation beam
 [X][L-Alt][R-Alt][L-Shift][R-Shift][/][Return] / Button 2, 4, 6, 8, 10, 12

 + CLASSIC mode - Capture beam
  You can capture enemies with the capture beam.
  To fire the capture beam, a capture beam energy (displayed in the left panel)
  should be full. The capture beam energy increased when you destroy an enemy.
  Captured enemies are lined up beside the player and
  counterattack to other enemies.
  Your ship is invincible when you are firing the capture beam.

 + BASIC mode - Capture beam
  You can use the capture beam anytime, but your ship isn't invincible
  while firing.

 + MODERN mode - Provocation beam
  The provocation beam forces an enemy to fire more bullets and
  you can earn more bullet multiplier bonus.
  You have to release the shot button to fire the provocation beam.

o Pause
 [P]

o Exit / Return to the title
 [ESC]

- Multiplier

A score multiplier is displayed in the upper right.
It increases when you destroy a enemy and decreases slowly
when the enemy is on the screen.

- Proximity multiplier

 + BASIC mode
  When you destroy an enemy at close range, you can get
  a proximity multiplier (Max x16).

- Bullet multiplier

 + MODERN mode
  When you destroy a pink midsize enemy, bullets flying side-by-side are
  changed into a bullet multiplier in a chain reaction.
  Use the provocation beam to the midsize enemy to get higher bonus.

- Extra ship

 + CLASSIC mode
  You earn an extra ship every 100,000 points.

 + BASIC mode
  You earn an extra ship every 1,000,000 points.

 + MODERN mode
  A score to earn the next extra ship is displayed in the upper left.


* Options

These command-line options are available:

 -brightness n    Set the brightness of the screen. (n = 0 - 100, default = 100)
 -res x y         Set the screen resolution to (x, y). (default = 640, 480)
 -nosound         Stop the sound.
 -bgmvol n        Set the volume of BGMs. (n = 0 - 128, default = 100)
 -sevol n         Set the volume of SEs. (n = 0 - 128, default = 100)
 -fullscreen      Run in full-screen mode.
 -exchange        Exchange the shot button and the beam button.
 -trail           Add a trail effect to enemies.
 -noslowdown      Stop an intentional slowdown.
 -randomized      Attack patterns of enemies randomly change with each play.


* Original Titanion webpage:
http://www.asahi-net.or.jp/~cs8k-cyu/windows/ttn_e.html

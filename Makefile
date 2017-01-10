
#DMD compiler version tested was 1.043

#SDL, SDL_mixer, GL, and GLU libraries are loaded on startup
LIBS_POSIX = -L-ldl
LIBS_WINDOWS = 

default:
	@echo "Use: make dmd-posix|ldc-posix|dmd-windows"


D_FILES = \
	derelict/sdl/mixer.di \
	derelict/sdl/sdltypes.di \
	derelict/sdl/sdlfuncs.di \
	derelict/sdl/sdl.di \
	derelict/opengl/glfuncs.di \
	derelict/util/exception.di \
	derelict/util/loader.di \
	derelict/util/wintypes.di \
	derelict/opengl/cgl.di \
	derelict/opengl/gl.di \
	derelict/opengl/glu.di \
	derelict/opengl/glx.di \
	derelict/opengl/wgl.di \
	src/util/rand.d \
	src/util/preference.d \
	src/util/sdl/mainloop.d \
	src/util/sdl/screen.d \
	src/util/sdl/input.d \
	src/util/sdl/frame.d \
	src/util/sdl/sdlexception.d \
	src/util/sdl/sound.d \
	src/util/iterator.d \
	src/util/sdl/recordableinput.d \
	src/util/sdl/pad.d \
	src/util/vector.d \
	src/util/sdl/screen3d.d \
	src/util/math.d \
	src/util/actor.d \
	src/util/sdl/displaylist.d \
	src/util/sdl/texture.d \
	src/util/logger.d \
	src/util/tokenizer.d \
	src/ttn/boot.d \
	src/ttn/screen.d \
	src/ttn/field.d \
	src/ttn/token.d \
	src/ttn/frame.d \
	src/ttn/shape.d \
	src/ttn/player.d \
	src/ttn/letter.d \
	src/ttn/particle.d \
	src/ttn/enemy.d \
	src/ttn/bullet.d \
	src/ttn/pillar.d \
	src/ttn/sound.d \
	src/ttn/stage.d \
	src/ttn/replay.d \
	src/ttn/preference.d \
	src/ttn/title.d

dmd-posix:
	dmd $(D_FILES) $(LIBS_POSIX) -oftitanion

ldc-posix:
	ldc  -singleobj $(D_FILES) $(LIBS_POSIX) -oftitanion

dmd-windows:
	dmd $(D_FILES) $(LIBS_WINDOWS) -oftitanion

clean:
	rm -f *.o *.map *.obj


#DMD compiler version tested was 1.043

#SDL, SDL_mixer, GL, and GLU libraries are loaded on startup
LIBS_POSIX = -L-ldl
LIBS_WINDOWS = 

default:
	@echo "Use: make dmd-posix|ldc-posix|dmd-windows"


D_FILES = \
	derelict/sdl/mixer.d \
	derelict/sdl/sdltypes.d \
	derelict/sdl/sdlfuncs.d \
	derelict/sdl/sdl.d \
	derelict/sdl/macinit/SDLMain.d \
	derelict/sdl/macinit/CoreFoundation.d \
	derelict/sdl/macinit/NSAutoreleasePool.d \
	derelict/sdl/macinit/NSMenu.d \
	derelict/sdl/macinit/NSString.d \
	derelict/sdl/macinit/string.d \
	derelict/sdl/macinit/ID.d \
	derelict/sdl/macinit/NSDictionary.d \
	derelict/sdl/macinit/NSMenuItem.d \
	derelict/sdl/macinit/NSZone.d \
	derelict/sdl/macinit/MacTypes.d \
	derelict/sdl/macinit/NSEnumerator.d \
	derelict/sdl/macinit/NSNotification.d \
	derelict/sdl/macinit/runtime.d \
	derelict/sdl/macinit/NSApplication.d \
	derelict/sdl/macinit/NSEvent.d \
	derelict/sdl/macinit/NSObject.d \
	derelict/sdl/macinit/NSArray.d \
	derelict/sdl/macinit/NSGeometry.d \
	derelict/sdl/macinit/NSProcessInfo.d \
	derelict/sdl/macinit/selectors.d \
	derelict/util/wrapper.d \
	derelict/opengl/glfuncs.d \
	derelict/util/exception.d \
	derelict/util/loader.d \
	derelict/util/wintypes.d \
	derelict/opengl/cgl.d \
	derelict/opengl/gl.d \
	derelict/opengl/glu.d \
	derelict/opengl/glx.d \
	derelict/opengl/wgl.d \
	derelict/opengl/gl12.d \
	derelict/opengl/gl13.d \
	derelict/opengl/gl14.d \
	derelict/opengl/gl15.d \
	derelict/opengl/gl20.d \
	derelict/opengl/gl21.d \
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

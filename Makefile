
.PHONY: all build run

all: build/src/picom

build/src/picom: src/*.c src/*.h
	meson setup --buildtype=release build
	ninja -C build

run: build/src/picom
	build/src/picom

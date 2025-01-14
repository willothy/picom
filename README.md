picom (luacomp?)
=====

> NOTE (willothy) <br>
> This is maintained for personal use only, and I make no guarantees about its stability.
> However, it is pretty cool and has functioning animations.
> Docs have not been updated at all by me.

While I have maintained the git history of Picom (Compton, XCompMgr, etc.), I will be
making tons of breaking changes and have no intent of upstreaming any of this.

I have already changed the formatting to a point that any merge would be nearly impossible,
as the old formatting was inconsistent and difficult for me to work with.

Changes I (willothy) have made:

- Ported some fixes over from [compfy](https://github.com/allusive-dev/compfy)
- Hacked around to fix some animation issues
- Embed LuaJIT for configuration

Changes that I am working on:

- Expand Lua configuration to include blacklists
- Expand Lua configuration to include wintype rules
- Remove dependency on `libconfig`

Changes I am considering / planning:

- Allowing animations to be implemented as Lua functions
- FFI interface for rules or event handling via Lua
- Better error reporting (use notify-send or similar)
- Graceful exits on hot-reload errors
  - This is trivial with Lua implemented, since the Lua state
    outlives the "session" (meaning the `session_t`, not the X session.
  - Move configuration entirely into Lua runtime
    - LuaJIT should make perf cost for this tolerable
- Porting event loop code to use Libuv instead of Libev,
  or maybe Tokio if I end up using Rust.
- Moving to either Cmake or the Zig build system (unlikely, I hate messing with C build systems)
- Porting some of this to Zig, Go or Rust (if I have time, this could be fun - probably would use Rust)

__picom__ is a compositor for X, which is a [fork of Compton](History.md).

**This is a development branch, bugs to be expected**

## Build

### Dependencies

Assuming you already have all the usual building tools installed (e.g. gcc, python, meson, ninja, etc.), you still need:

- luajit
- libx11
- libx11-xcb
- libXext
- xproto
- xcb
- xcb-util
- xcb-damage
- xcb-dpms
- xcb-xfixes
- xcb-shape
- xcb-renderutil
- xcb-render
- xcb-randr
- xcb-composite
- xcb-image
- xcb-present
- xcb-glx
- pixman
- libdbus (optional, disable with the `-Ddbus=false` meson configure flag)
- libconfig (optional, disable with the `-Dconfig_file=false` meson configure flag)
- libGL, libEGL (optional, disable with the `-Dopengl=false` meson configure flag)
- libpcre2 (optional, disable with the `-Dregex=false` meson configure flag)
- libev
- uthash

On Debian based distributions (e.g. Ubuntu), the needed packages are

```
libconfig-dev libdbus-1-dev libegl-dev libev-dev libgl-dev libpcre2-dev libpixman-1-dev libx11-xcb-dev libxcb1-dev libxcb-composite0-dev libxcb-damage0-dev libxcb-dpms0-dev libxcb-glx0-dev libxcb-image0-dev libxcb-present-dev libxcb-randr0-dev libxcb-render0-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-util-dev libxcb-xfixes0-dev libxext-dev meson ninja-build uthash-dev
```

On Fedora, the needed packages are

```
dbus-devel gcc git libconfig-devel libdrm-devel libev-devel libX11-devel libX11-xcb libXext-devel libxcb-devel libGL-devel libEGL-devel meson pcre2-devel pixman-devel uthash-devel xcb-util-image-devel xcb-util-renderutil-devel xorg-x11-proto-devel xcb-util-devel
```

To build the documents, you need `asciidoc`

### To build

```bash
meson setup --buildtype=release build
ninja -C build
```

Built binary can be found in `build/src`

If you have libraries and/or headers installed at non-default location (e.g. under `/usr/local/`), you might need to tell meson about them, since meson doesn't look for dependencies there by default.

You can do that by setting the `CPPFLAGS` and `LDFLAGS` environment variables when running `meson`. Like this:

```bash
LDFLAGS="-L/path/to/libraries" CPPFLAGS="-I/path/to/headers" meson setup --buildtype=release build
```

As an example, on FreeBSD, you might have to run meson with:

```bash
LDFLAGS="-L/usr/local/lib" CPPFLAGS="-I/usr/local/include" meson setup --buildtype=release build
ninja -C build
```

### To install

``` bash
ninja -C build install
```

Default install prefix is `/usr/local`, you can change it with `meson configure -Dprefix=<path> build`

## Running

To launch with all animations as a background process you can use:
`picom -b`

To only have specific animations, enable them with cli flags (see `picom --help`) or add them to your picom config.

## Configuration

Your `picom` (`luacomp`?) config lives in `~/.config/picom.lua` or `~/.config/picom/picom.lua`.

Config docs todo, but the keys are pretty much the same as they were in the `.conf` but with
underscores instead of dashes. Rules, wintypes and blacklists are not yet supported.

## How to Contribute

All contributions are welcome!

New features you think should be included in picom, a fix for a bug you found - please open a PR!

You can take a look at the [Issues](https://github.com/willothy/picom/issues).

Contributions to the documents and wiki are also appreciated.

Even if you don't want to add anything to picom, you are still helping by compiling and running this branch, and report any issue you can find.

As I mention at the top of the readme, I am not making any guarantees of stability or long-term maintenance of this fork. This means that I may not
get around to fixing issues quickly if I'm not actively experiencing them. However, PRs fixing issues are welcome!

## Contributors

See [CONTRIBUTORS](CONTRIBUTORS)

The README for the [original Compton project](https://github.com/chjj/compton/) can be found [here](History.md#Compton).

## Licensing

picom is free software, made available under the [MIT](LICENSES/MIT) and [MPL-2.0](LICENSES/MPL-2.0) software
licenses. See the individual source files for details.

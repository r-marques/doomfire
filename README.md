# DOOM Fire

Doom Fire rust implementation based on [Fabien Sanglard's blog
post](https://fabiensanglard.net/doom_fire_psx/).

![demo](demo.gif)

### Implementation

The implementation is independent of the graphics library used.

The [examples](https://github.com/r-marques/doomfire/tree/master/examples)
folder contains examples using different graphics libraries to render the DOOM
fire:

- [doomfire-minifb](https://github.com/r-marques/doomfire/tree/master/examples/doomfire-minifb)
  uses [minifb](https://github.com/emoon/rust_minifb)
- [doomfire-pixels](https://github.com/r-marques/doomfire/tree/master/examples/doomfire-pixels)
  uses [pixels](https://github.com/parasyte/pixels)
- [doomfire-sdl2](https://github.com/r-marques/doomfire/tree/master/examples/doomfire-sdl2)
  uses the rust [sdl2 bindings](https://github.com/Rust-SDL2/rust-sdl2)
- [doomfire-wasm](https://github.com/r-marques/doomfire/tree/master/examples/doomfire-wasm)
  please see the
  [README.md](https://github.com/r-marques/doomfire/blob/master/examples/doomfire-wasm/README.md)
  inside the example folder for instructions on how to build and run.

### How to run

To run the examples `cd` into one of the examples folder and run it using
cargo. For example:

```bash
$ cd examples/doomfire-sdl2
$ cargo run --release
```

Note that all of these graphics libraries have system dependencies so it the
build fails check the output and install the missing libraries using your
systems package manager.

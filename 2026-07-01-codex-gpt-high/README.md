# Dusk Runner

A compact pixel-art side-scrolling platformer written in Rust with [macroquad](https://macroquad.rs).

## Build and run

Prerequisites: a Rust toolchain (`rustup`). macroquad is pure Rust, so no separate graphics
library needs to be installed.

```sh
cargo run --release
```

## Controls

- Left/Right arrows or A/D: move
- Space, Up, or W: jump
- Stomp creatures from above
- Escape: quit
- R: restart after reaching the lantern

Collect the 18 amber shards and reach the lantern at the far end of the level. The game renders on a
320×180 logical canvas with integer nearest-neighbour scaling, procedural pixel art, parallax
scenery, particles, enemies, pits, and a follow camera.

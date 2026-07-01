# Pixel Slime

A pixel-art side-scrolling platformer written in Rust with [macroquad](https://macroquad.rs).

## Build and run

Prerequisites: a Rust toolchain (`rustup`). macroquad is pure Rust, so no separate graphics
library needs to be installed.

```sh
cargo run --release
```

## Controls

- Left/Right arrows or A/D: move
- Space or Up to jump (W also jumps)
- Escape: quit
- R: restart

Guide the slime across a 320×180 logical canvas: collect coins, avoid spikes and pits, and reach the
flag at the far end. Features parallax hills, drifting clouds, a gradient sky, squash-and-stretch
animation, coyote time, and jump buffering.

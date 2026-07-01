# Ember Knight

A 2D side-scrolling pixel-art platformer built with [macroquad](https://macroquad.rs).
All art is generated procedurally at startup (no image/audio assets) — pixel sprites,
tiles, and parallax cave backdrops are drawn onto textures in code.

Guide the knight through a torch-lit cave: jump chasms, dodge patrolling bats and
slimes and spike traps, collect gems, and reach the glowing portal at the end.

## Controls

| Action | Keys |
| --- | --- |
| Move | Arrow keys or `A` / `D` |
| Jump | `Space`, `Up`, or `W` (hold for a higher jump, tap for a short hop) |
| Start / Restart | `Enter` |

## Mechanics

- Coyote time + jump buffering for forgiving platforming.
- Jumping on an enemy's head defeats it and bounces you upward; touching it from
  the side costs a heart.
- 3 hearts, brief invulnerability after each hit, falling into a pit costs a heart
  and respawns you at the start.
- 15 gems scattered through the level (optional — collecting them doesn't affect
  the win condition, just your score).

## Running

From this directory:

```sh
cargo run --release
```

Or via the repo-root helper: `../run.sh sonnet` (or any alias matching this
directory name).

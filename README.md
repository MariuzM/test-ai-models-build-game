# test-ai-models-build-game

A small benchmark: give different AI models the **same prompt** and compare what they build.

## The prompt

> Create a 2D side-scrolling pixel-art game in Rust.

(One entry was asked for the same game in **Zig** instead — see the table.)

## Entries

Each directory is one model's answer — a self-contained project you can run.

| Directory | Model | Game | Stack |
| --- | --- | --- | --- |
| [`2026-07-01-claude-opus-4.8-high/`](2026-07-01-claude-opus-4.8-high/) | Claude Opus 4.8 | Pixel Slime | Rust + macroquad |
| [`2026-07-01-codex-gpt-high/`](2026-07-01-codex-gpt-high/) | Codex (GPT, high) | Dusk Runner | Rust + macroquad |
| [`2026-07-01-claude-sonet-5-high/`](2026-07-01-claude-sonet-5-high/) | Claude Sonnet 5 | Ember Knight | Rust + macroquad |
| [`2026-07-02-claude-opus-4.8-high/`](2026-07-02-claude-opus-4.8-high/) | Claude Opus 4.8 | Fennec Dash | Zig + SDL3 |

## Running an entry

The Rust entries use [macroquad](https://macroquad.rs) — pure Rust, no system libraries
to install. The Zig entry (Fennec Dash) needs Zig 0.16 and SDL3 (`brew install sdl3`).

Use the helper script from the repo root (name matching is fuzzy, with a few aliases):

```sh
./run.sh            # list the entries
./run.sh dusk       # Dusk Runner   (aliases: dusk, runner, codex, gpt)
./run.sh slime      # Pixel Slime   (aliases: slime, opus)
./run.sh knight     # Ember Knight  (aliases: knight, ember, sonnet, sonet)
./run.sh fennec     # Fennec Dash   (aliases: fennec, dash, zig)
```

It builds in release by default; pass your own flags to override, e.g.
`./run.sh dusk --debug`.

Or run an entry directly:

```sh
cd 2026-07-01-claude-opus-4.8-high   # a Rust entry
cargo run --release

cd 2026-07-02-claude-opus-4.8-high   # the Zig entry
zig build run -Doptimize=ReleaseFast
```

See each directory's own `README.md` for controls and details.

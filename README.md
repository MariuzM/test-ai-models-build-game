# test-ai-models-build-game

A small benchmark: give different AI models the **same prompt** and compare what they build.

## The prompt

> Create a 2D side-scrolling pixel-art game in Rust.

## Entries

Each directory is one model's answer — a self-contained Cargo project you can run.

| Directory | Model | Game |
| --- | --- | --- |
| [`2026-07-01-claude-opus-4.8-high/`](2026-07-01-claude-opus-4.8-high/) | Claude Opus 4.8 | Pixel Slime |
| [`2026-07-01-codex-gpt-high/`](2026-07-01-codex-gpt-high/) | Codex (GPT, high) | Dusk Runner |
| [`2026-07-01-claude-sonet-5-high/`](2026-07-01-claude-sonet-5-high/) | Claude Sonnet 5 | Ember Knight |

## Running an entry

Each entry uses [macroquad](https://macroquad.rs) — pure Rust, no system libraries to install.

Use the helper script from the repo root (name matching is fuzzy, with a few aliases):

```sh
./run.sh            # list the entries
./run.sh dusk       # Dusk Runner   (aliases: dusk, runner, codex, gpt)
./run.sh slime      # Pixel Slime   (aliases: slime, opus)
./run.sh knight     # Ember Knight  (aliases: knight, ember, sonnet, sonet)
```

It builds with `--release` by default; pass your own cargo flags to override, e.g.
`./run.sh dusk --debug`.

Or run an entry directly:

```sh
cd 2026-07-01-claude-opus-4.8   # or any entry
cargo run --release
```

See each directory's own `README.md` for controls and details.

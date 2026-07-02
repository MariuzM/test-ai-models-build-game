#!/bin/sh
# Build and run one of the model entries.
#
#   ./run.sh              list the entries
#   ./run.sh dusk         run the entry whose name matches "dusk"
#   ./run.sh slime        run the entry whose name matches "slime"
#   ./run.sh fennec       run the Zig entry (Fennec Dash)
#   ./run.sh codex        (any substring of the directory works)
#   ./run.sh dusk --debug build/run without --release
set -eu

ROOT="$(cd "$(dirname "$0")" && pwd)"

# Map a few friendly aliases to a substring found in the directory name.
alias_for() {
    case "$1" in
        slime|opus) echo "07-01-claude-opus" ;;
        dusk|runner|codex|gpt) echo "codex" ;;
        knight|ember|sonnet|sonet) echo "sonet-5" ;;
        fennec|dash|zig) echo "07-02-claude-opus" ;;
        fable) echo "fable" ;;
        *) echo "$1" ;;
    esac
}

entries() {
    for d in "$ROOT"/*/; do
        if [ -f "$d/Cargo.toml" ] || [ -f "$d/build.zig" ]; then
            basename "$d"
        fi
    done
}

if [ "$#" -eq 0 ]; then
    echo "Entries:"
    entries | sed 's/^/  /'
    echo
    echo "Usage: ./run.sh <name> [build args...]   e.g. ./run.sh fennec"
    exit 0
fi

query="$(alias_for "$1")"
shift

match=""
for name in $(entries); do
    case "$name" in
        *"$query"*)
            if [ -n "$match" ]; then
                echo "Ambiguous: '$query' matches multiple entries." >&2
                entries | sed 's/^/  /' >&2
                exit 1
            fi
            match="$name"
            ;;
    esac
done

if [ -z "$match" ]; then
    echo "No entry matches '$query'." >&2
    entries | sed 's/^/  /' >&2
    exit 1
fi

echo "Running $match ($*)"
cd "$ROOT/$match"

if [ -f build.zig ]; then
    # Zig entry (SDL3). Default to a release build.
    if [ "$#" -eq 0 ]; then
        set -- -Doptimize=ReleaseFast
    fi
    exec zig build run "$@"
else
    # Cargo entry (macroquad). Default to a release build.
    if [ "$#" -eq 0 ]; then
        set -- --release
    fi
    exec cargo run "$@"
fi

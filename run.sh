#!/bin/sh
# Build and run one of the model entries.
#
#   ./run.sh              list the entries
#   ./run.sh dusk         run the entry whose name matches "dusk"
#   ./run.sh slime        run the entry whose name matches "slime"
#   ./run.sh codex        (any substring of the directory works)
#   ./run.sh dusk --debug build/run without --release
set -eu

ROOT="$(cd "$(dirname "$0")" && pwd)"

# Map a few friendly aliases to a substring found in the directory name.
alias_for() {
    case "$1" in
        slime|opus) echo "opus" ;;
        dusk|runner|codex|gpt) echo "codex" ;;
        knight|ember|sonnet|sonet) echo "sonet-5" ;;
        *) echo "$1" ;;
    esac
}

entries() {
    for d in "$ROOT"/*/; do
        [ -f "$d/Cargo.toml" ] && basename "$d"
    done
}

if [ "$#" -eq 0 ]; then
    echo "Entries:"
    entries | sed 's/^/  /'
    echo
    echo "Usage: ./run.sh <name> [cargo args...]   e.g. ./run.sh dusk"
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

# Default to a release build unless the caller passes their own flags.
if [ "$#" -eq 0 ]; then
    set -- --release
fi

echo "Running $match ($*)"
cd "$ROOT/$match"
exec cargo run "$@"

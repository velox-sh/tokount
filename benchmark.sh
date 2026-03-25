#!/usr/bin/env bash
set -euo pipefail

# Compare tokount vs scc vs tokei (and optionally cloc) across real-world repos
#
# Usage:
#   ./benchmark.sh              run all cases without cloc
#   ./benchmark.sh --cloc       include cloc (slow)
#   ./benchmark.sh --no-build   skip cargo build
#
# Requires: hyperfine, scc, tokei, git, python3
# Optional: cloc (only when --cloc is set)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="/tmp/tokount-bench"
ASSETS="$SCRIPT_DIR/assets"
RESULTS="$ASSETS/bench_results.json"

NO_BUILD=0
USE_CLOC=0

for arg in "$@"; do
    [[ "$arg" == "--no-build" ]] && NO_BUILD=1
    [[ "$arg" == "--cloc" ]] && USE_CLOC=1
done

# ---------------------------------------------------------------------------
# build
# ---------------------------------------------------------------------------
if [[ $NO_BUILD -eq 0 ]]; then
    echo "Building tokount (release)..."
    cargo build --release -q --manifest-path "$SCRIPT_DIR/Cargo.toml"
    cp "$SCRIPT_DIR/target/release/tokount" "$BINARY"
fi

[[ -x "$BINARY" ]] || { echo "error: binary not found: $BINARY" >&2; exit 1; }

# ---------------------------------------------------------------------------
# tool checks
# ---------------------------------------------------------------------------
for tool in scc tokei hyperfine git python3; do
    command -v "$tool" &> /dev/null || { echo "error: $tool not found in PATH" >&2; exit 1; }
done

if [[ $USE_CLOC -eq 1 ]]; then
    command -v cloc &> /dev/null || { echo "error: cloc not found in PATH (required by --cloc)" >&2; exit 1; }
fi

TOOLS=("$BINARY" "scc" "tokei")
TOOL_NAMES=("tokount" "scc" "tokei")

if [[ $USE_CLOC -eq 1 ]]; then
    TOOLS+=("cloc")
    TOOL_NAMES+=("cloc")
fi

printf '\nTools: %s\n\n' "${TOOL_NAMES[*]}"

# ---------------------------------------------------------------------------
# cases
# ---------------------------------------------------------------------------
BENCH_TMPDIR="$(mktemp -d "/tmp/.bench_XXXXXX")"
trap 'echo "Cleaning up $BENCH_TMPDIR ..."; rm -rf "$BENCH_TMPDIR"' EXIT


CASE_NAMES=(
    "Tokount (25k lines)"
    "Redis (375k lines)"
    "Ruff (1M lines)"
    "CPython (2.2M lines)"
    "Rust (3.5M lines)"
    "Linux (31.3M lines)"
)
CASE_URLS=(
    "https://github.com/MihaiStreames/tokount"
    "https://github.com/redis/redis"
    "https://github.com/astral-sh/ruff"
    "https://github.com/python/cpython"
    "https://github.com/rust-lang/rust"
    "https://github.com/torvalds/linux"
)
CASE_DIRS=("tokount-src" "redis" "ruff" "cpython" "rust" "linux")

echo "Cloning cases into $BENCH_TMPDIR ..."
for i in "${!CASE_URLS[@]}"; do
    target="$BENCH_TMPDIR/${CASE_DIRS[$i]}"
    printf '  Cloning %s ...\n' "${CASE_NAMES[$i]}"
    git clone -q --depth=1 "${CASE_URLS[$i]}" "$target"
done

CASE_PATHS=(
    "$BENCH_TMPDIR/tokount-src"
    "$BENCH_TMPDIR/redis"
    "$BENCH_TMPDIR/ruff"
    "$BENCH_TMPDIR/cpython"
    "$BENCH_TMPDIR/rust"
    "$BENCH_TMPDIR/linux"
)

# ---------------------------------------------------------------------------
# run benchmarks
# ---------------------------------------------------------------------------
mkdir -p "$ASSETS"
echo "[" > "$RESULTS"
first=1

for i in "${!CASE_NAMES[@]}"; do
    name="${CASE_NAMES[$i]}"
    path="${CASE_PATHS[$i]}"

    echo "──────────────────────────────────────────────────────────────────────"
    printf 'Case: %s\nPath: %s\n\n' "$name" "$path"

    CMDS=()
    for t in "${!TOOLS[@]}"; do
        CMDS+=("${TOOLS[$t]} $path")
    done

    tmpfile=$(mktemp /tmp/bench_XXXXXX.json)
    hyperfine --warmup 10 --runs 10 --export-json "$tmpfile" "${CMDS[@]}"

    [[ $first -eq 0 ]] && echo "," >> "$RESULTS"
    first=0

    python3 "$SCRIPT_DIR/bench_parse.py" "$tmpfile" "$name" "${TOOL_NAMES[@]}" >> "$RESULTS"
    rm -f "$tmpfile"
    echo ""
done

echo "]" >> "$RESULTS"

echo "──────────────────────────────────────────────────────────────────────"

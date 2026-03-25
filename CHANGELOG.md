<a id="changelog-top"></a>

<div align="center">
  <h1>Changelog</h1>

  <h3>All notable changes to tokount</h3>

</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#v220--performance">v2.2.0</a></li>
    <li><a href="#v211--table-output-polish--release-alignment">v2.1.1</a></li>
    <li><a href="#v210--sections-abstractions-and-less-lint-pain">v2.1.0</a></li>
    <li><a href="#v200--simd-engine">v2.0.0</a></li>
    <li><a href="#v111--windows-symlink-guard">v1.1.1</a></li>
    <li><a href="#v110--human-readable-output--multi-path-support">v1.1.0</a></li>
    <li><a href="#v101--maintenance">v1.0.1</a></li>
    <li><a href="#v100--initial-release">v1.0.0</a></li>
  </ol>
</details>

## v2.2.0 — Performance

Flamegraph-driven optimization pass. ~2.5x faster than v2.1.1 on real workloads, with zero accuracy changes.

The big wins came from three places: upgrading the SIMD scanner from SSE2 to AVX2 (doubles the bytes per chunk), fixing a redundant `fstat64` on every mmap'd file, and adding a first-byte guard to token matching that short-circuits the common case. Running flamegraphs on a 44k-file Go codebase before and after confirmed the gains.

**New stuff:**

- `-x`/`--same-filesystem` — don't cross filesystem boundaries; fixes SIGKILL when scanning `/` due to OOM from virtual filesystems like `/proc` and `/sys`
- `-r`/`--rsort` — reverse sort by column (ascending); mutually exclusive with `--sort`
- `-C`/`--compact` — hide embedded child language rows from table and CSV output
- Bounded walker channel — backpressure so the walker can't get arbitrarily far ahead of the consumers and blow memory

**Improved:**

- SIMD scanner upgraded from SSE2 (16 bytes/chunk) to AVX2 (32 bytes/chunk) with runtime detection and SSE2 fallback; broadcast vectors hoisted out of the chunk loop
- `mmap` path no longer does a redundant `fstat64` — we already have the size from `metadata()`, so we pass it directly via `MmapOptions::new().len(size)`; also added `MADV_SEQUENTIAL` so the kernel prefetches ahead
- Token matching has a first-byte guard — if the first byte doesn't match any region opener, `starts_with` is never called
- Shebang detection now opens the file once, reads the first line, seeks back to 0, and hands the fd to `FileReader` — no second `open` syscall
- `--types` now validates names at startup and exits with a structured `UnknownLanguage` error on unknown input
- `main.rs` is pure orchestration now; validation, label formatting, and spinner logic live in `cli.rs` and `display.rs`
- Trimmed crate-level docs, added one-liners to all internal `pub` functions; `#[must_use]` on `count`, `supported_languages`, and `is_supported_language`

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v2.1.1 — Table output polish & release alignment

**Improved:**

- Reworked table rendering to use clearer borders and stable spacing rules
- Added explicit numeric column width constraints so `Files`, `Lines`, `Code`, `Comments`, and `Blanks` stay visually aligned
- Switched child-language labels to a stronger, consistent prefix (`>> <child>`)

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v2.1.0 — Sections, abstractions, and less lint pain

The key insight here is that every language, no matter how weird, is just a set of regions: each with an open/close delimiter and a type (comment, string, or child language).

Once you model it that way, the parser doesn't need to know anything about the specific language it's parsing. It just follows the rules defined in `languages.json`, and adding or fixing a language becomes a data change, not a code change.

**New stuff:**

- Region-based language modeling (`comment`, `string`, `child`) with explicit default modes and structured formats
- Structured/Jupyter format handling and richer embedded child-language detection (`fence`, `tag`, `fixed`) (Issue #3)
- `leading_doc_prefixes` support for language-specific doc-preface behavior (Issue #4)
- `tests/lang.expected/` sidecar files for accuracy fixtures with a unified harness

**Improved:**

- Parser split into focused modules: `mod.rs` (orchestration), `helpers.rs` (utilities), `state.rs` (FSM flow)
- Build-time generation reworked to emit the new language model from `languages.json`

**Fixed:**

- Removed parser-level complexity/size suppressions by restructuring instead of piling on `#[allow]`
- Removed legacy literate-only parsing path

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v2.0.0 — SIMD engine

Using some help and directions from @big-lip-bob, I managed to entirely rewrite the counting engine. `tokei` is now replaced by a custom byte-level FSM with SIMD-accelerated scanning, which makes `tokount` the fastest line counter available!

> Note: It is only the fastest when we take SSE2 into account. Other architectures will need a deeper look into in order to figure out how to implement SIMD as efficiently as SSE2 does on `x86_64`.

**New stuff:**

- Custom byte-level FSM handling line/block comments, string literals, raw strings, nested comments, shebangs
- SIMD-accelerated byte scanning via `memchr` (SSE2/AVX2 under the hood)
- Language definitions generated at compile time via `build.rs` + `phf` — 280 languages verified against tokei/scc
- Shebang detection for extensionless files (`#!/usr/bin/env ruby` → Ruby)
- `ignore`-crate parallel walker (same engine as ripgrep)
- Thread-local `FileReader` with buffer reuse — zero heap allocation in the hot path
- Hybrid I/O: mmap for files >64 KB (zero-copy), buffered read for small files
- `crossbeam_channel::unbounded` + `rayon::par_bridge` streaming pipeline
- `-o`/`--output` — output format: `table` (default), `json`, `csv`
- `-s`/`--sort` — sort by column: `files`, `lines`, `blank`, `comment`, `code`
- `-t`/`--types` — filter to specific language(s), comma-separated
- `--no-ignore` — disable `.gitignore` / `.prettierignore` entirely
- `-l`/`--languages` — print all supported languages and exit

**Changed:**

- `opt-level` set to `3` (was `"z"`), enables SIMD auto-vectorization
- `--json`/`-j` removed; use `--output json` / `-o json` instead
- Removed `tokei` and `walkdir` dependencies
- `src/git.rs` and `src/analyze.rs` removed; logic absorbed into the engine

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v1.1.1 — Windows symlink guard

**Changed:**

- `-L`/`--follow-symlinks` now exits with an error on Windows (unsupported platform)

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v1.1.0 — Human-readable output & multi-path support

First real usability pass: table output, multi-path support, and a proper test suite.

**New stuff:**

- Human-readable table by default with timing stats (files/s, lines/s), file count, and git repo count
- `--json` / `-j` flag for machine-readable JSON output (replaces old default behaviour)
- Multiple path arguments: `tokount file1 file2 dir/` and `tokount $(git ls-files)`
- Integration test suite with fixture directories and JSON snapshots

**Changed:**

- Excluded dirs are now a named flag `-e`/`--excluded` instead of a positional arg (breaking change)

**Fixed:**

- `homepage` and `repository` in `Cargo.toml` pointing to wrong repo

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v1.0.1 — Maintenance

**New stuff:**

- `.editorconfig`, `rustfmt.toml`, `clippy.toml`, `rust-toolchain.toml` for consistent formatting and lint configuration
- `.prettierrc` for YAML/JSON/TOML/Markdown formatting

**Changed:**

- `clap` bumped from 4.5.55 to 4.5.59
- `extract-changelog.sh` renamed to `extract_changelog.sh`

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v1.0.0 — Initial release

First version. A fast line counter for codebases, powered by `tokei`

**New stuff:**

- JSON output format with per-language stats (files, blank, comment, code)
- Structured JSON error output to stderr
- Support for excluding directories via comma-separated list
- Symlink following with `-L`/`--follow-symlinks` flag
- Git repository detection and gitignore pattern collection
- CLI argument parsing with `clap` derive macros

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

<div align="center">
  <p>Back to <a href="README.md">README</a>?</p>
</div>

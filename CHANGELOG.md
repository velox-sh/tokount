<a id="changelog-top"></a>

<div align="center">
  <h1>Changelog</h1>

  <h3>All notable changes to tokount</h3>

</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#v214--new-flags">v2.1.4</a></li>
    <li><a href="#v213--oom-fix--same-filesystem">v2.1.3</a></li>
    <li><a href="#v212--docs-polish">v2.1.2</a></li>
    <li><a href="#v211--table-output-polish--release-alignment">v2.1.1</a></li>
    <li><a href="#v210--sections-abstractions-and-less-lint-pain">v2.1.0</a></li>
    <li><a href="#v200--simd-engine">v2.0.0</a></li>
    <li><a href="#v111--windows-symlink-guard">v1.1.1</a></li>
    <li><a href="#v110--human-readable-output--multi-path-support">v1.1.0</a></li>
    <li><a href="#v101--maintenance">v1.0.1</a></li>
    <li><a href="#v100--initial-release">v1.0.0</a></li>
  </ol>
</details>

## v2.1.4 — New flags

**New:**

- `-r` / `--rsort <COLUMN>` — reverse-sort output by column (ascending); conflicts with `--sort`
- `-C` / `--compact` — suppress child language rows (embedded languages like Markdown-in-Rust)
- `-x` / `--same-filesystem` flag is now wired through the CLI (was added to the engine in v2.1.3)

**Improved:**

- Path validation moved into `Args::validate()` — cleaner separation from `main`
- Label logic moved into `Args::label()` — single-path shows path, multi-path shows count
- `display::render()` and `display::start_spinner()` replace the inline match in `main`

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v2.1.3 — OOM fix & same-filesystem

**Fixed:**

- Running `tokount /` (or any scan spanning virtual filesystems) no longer causes OOM SIGKILL. The walker channel is now bounded (256) so backpressure kicks in before memory fills up (fixes [#5](https://github.com/MihaiStreames/tokount/issues/5), reported by [@UnderNowhere](https://github.com/UnderNowhere))

**New:**

- `-x` / `--same-filesystem` flag — skip `/proc`, `/sys`, NFS mounts, and any path that crosses a filesystem boundary

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v2.1.2 — Docs polish

**New stuff:**

- `EngineConfig` now implements `Default`

**Improved:**

- `--types` now validates names at startup and exits with a structured `UnknownLanguage` error on unknown input
- Trimmed crate-level docs, added one-liners to all internal `pub` functions
- `#[must_use]` on `count`, `supported_languages`, and `is_supported_language`

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

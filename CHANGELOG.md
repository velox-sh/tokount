<a id="changelog-top"></a>

<div align="center">
  <h1>Changelog</h1>

  <h3>All notable changes to tokount</h3>

</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#v210--sections-abstractions-and-less-lint-pain">v2.1.0</a></li>
    <li><a href="#v200--simd-engine">v2.0.0</a></li>
    <li><a href="#v111--windows-symlink-guard">v1.1.1</a></li>
    <li><a href="#v110--human-readable-output--multi-path-support">v1.1.0</a></li>
    <li><a href="#v101--maintenance">v1.0.1</a></li>
    <li><a href="#v100--initial-release">v1.0.0</a></li>
  </ol>
</details>

## v2.1.0 — Sections, abstractions, and less lint pain

This release came from changing perspective on the engine shape.

Instead of letting language definitions and parser logic grow sideways forever, I split responsibilities into clearer sections and pushed more behavior into data-driven abstractions.

That made `languages.json` easier to scale, made parser code easier to reason about, and helped remove lint suppressions that were making day-to-day work more annoying than it needed to be.

**Why this release exists:**

- I wanted to stop fighting growing config/code size every time a language edge case appeared
- I wanted parsing flow to read like roles (entrypoint/helpers/state) instead of one giant function
- I wanted strict linting to be a guardrail, not a tax

**What changed:**

- Refactored language definitions to region-based modeling (`comment`, `string`, `child`) with explicit default modes and structured formats
- Reworked build-time generation to emit the new language model from `languages.json`
- Removed legacy literate-only parsing path and folded behavior into the unified FSM model
- Split parser implementation into focused modules:
  - `parser/mod.rs` for orchestration and entrypoint
  - `parser/helpers.rs` for reusable parsing helpers and format-specific utilities
  - `parser/state.rs` for state-machine flow
- Added support for structured/Jupyter handling and richer embedded child-language detection (`fence`, `tag`, `fixed`)
- Added `leading_doc_prefixes` handling to support language-specific doc-preface behavior
- Expanded stable library-facing exports in `lib.rs` (`count`, `EngineConfig`, `OutputStats`, `LangStats`) and improved crate docs
- Added `CONTRIBUTING.md` covering workflow, ordering, API policy, lint/test expectations
- Extended accuracy fixtures into `tests/lang.expected/` sidecar files and aligned accuracy harness around sidecar expectations

**Lint and quality direction:**

- Removed parser-level complexity/size suppressions by restructuring code
- Addressed strict clippy feedback through refactors instead of piling on `#[allow]`
- Fixed an ATS classification regression introduced during parser refactor and revalidated accuracy

**Result:**

1. engine stays fast
2. parsing rules scale without turning into giant special-case branches
3. stricter linting with less suppression noise
4. easier to iterate on language support without turning maintenance into hell

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v2.0.0 — SIMD engine

Using some help and directions from @big-lip-bob, I managed to entirely rewrite counting engine. That means `tokei` is now replaced by a custom byte-level FSM with SIMD-accelerated scanning which now makes `tokount` the fastest line counter available!

> Note: It is only the fastest when we take SSE2 into account. Other architectures will need a deeper look into in order to figure out how to implement SIMD as efficiently as SSE2 does on `x86_64`.

**Engine:**

- Custom byte-level finite state machine (FSM) which handles line/block comments, string literals, raw strings, nested comments, shebangs
- SIMD-accelerated byte scanning via `memchr` (SSE2/AVX2 under the hood)
- Language definitions generated at compile time via `build.rs` + `phf`
- Shebang detection for extensionless files (`#!/usr/bin/env ruby` -> Ruby)
- 280 languages verified against tokei/scc fixture files

**I/O pipeline:**

- `ignore`-crate parallel walker (same engine as ripgrep) replaces `walkdir`
- Thread-local `FileReader` with buffer reuse (zero heap allocation in the hot path)
- Hybrid I/O: mmap for files >64 KB (zero-copy), buffered read for small files
- `crossbeam_channel::unbounded` + `rayon::par_bridge` streaming pipeline
- Git-aware ignore logic: respects `.gitignore` in git repos and `.prettierignore` everywhere

**CLI:**

- `-o`/`--output` — output format: `table` (default), `json`, `csv`
- `-s`/`--sort` — sort by column: `files`, `lines`, `blank`, `comment`, `code` (default: `code`)
- `-t`/`--types` — filter to specific language(s), comma-separated (e.g. `-t Rust,Python`)
- `--no-ignore` — disable `.gitignore` / `.prettierignore` entirely
- `-l`/`--languages` — print all supported languages and exit

**Changed:**

- `opt-level` changed from `"z"` (size) to `3` (speed), that enables SIMD auto-vectorization
- Removed `tokei` and `walkdir` dependencies
- `src/git.rs` removed; git repo counting and ignore collection are now part of the engine walker
- `src/analyze.rs` removed; engine called directly from `main.rs`
- `--json`/`-j` removed; use `--output json` / `-o json` instead

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v1.1.1 — Windows symlink guard

**Changed:**

- `-L`/`--follow-symlinks` now exits with an error on Windows (unsupported platform)

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

## v1.1.0 — Human-readable output & multi-path support

**New stuff:**

- Human-readable table by default with timing stats (files/s, lines/s), file count, and git repo count
- `--json` / `-j` flag for machine-readable JSON output (replaces old default behaviour)
- Spinner progress indicator during scan; suppressed automatically when piping or using `--json`
- Multiple path arguments: `tokount file1 file2 dir/` and `tokount $(git ls-files)`
- Integration test suite with fixture directories and JSON snapshots

**Changed:**

- Excluded dirs are now a named flag `-e`/`--excluded` instead of a positional arg (breaking change)
- Fixed `homepage` and `repository` in `Cargo.toml` pointing to wrong repo

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
- Git repository detection
- Gitignore pattern collection
- CLI argument parsing with `clap` derive macros

<p align="right">(<a href="#changelog-top">back to top</a>)</p>

---

<div align="center">
  <p>Back to <a href="README.md">README</a>?</p>
</div>

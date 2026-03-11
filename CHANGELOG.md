<a id="changelog-top"></a>

<div align="center">
  <h1>Changelog</h1>

  <h3>All notable changes to tokount</h3>

</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#v101--maintenance">v1.0.1</a></li>
    <li><a href="#v100--initial-release">v1.0.0</a></li>
  </ol>
</details>

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

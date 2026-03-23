<a id="readme-top"></a>

<!-- PROJECT SHIELDS -->
<div align="center">

[![Stars](https://img.shields.io/github/stars/MihaiStreames/tokount?style=social)](https://github.com/MihaiStreames/tokount/stargazers)
[![AUR Version](https://img.shields.io/aur/version/tokount?label=AUR)](https://aur.archlinux.org/packages/tokount)
[![Rust Edition](https://img.shields.io/badge/Rust-2024-ed7a1f)](https://www.rust-lang.org/)
[![License](https://img.shields.io/github/license/MihaiStreames/tokount?label=License)](LICENSE)

</div>

<!-- PROJECT LOGO -->
<div align="center">
  <h1>tokount</h1>

  <h3 align="center">The fastest line counter for codebases</h3>

  <p align="center">
    Language-aware code/comment/blank classification at SIMD speed
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#output-formats">Output Formats</a></li>
    <li><a href="#benchmarks">Benchmarks</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

`tokount` counts lines of code across an entire codebase and breaks them down by language into code, comments, and blank lines.

Why use `tokount`?

- **Fastest available** — beats `tokei`, `scc`, and `cloc` at every repo size from 375K lines up (if SSE2 is available!)
- **Verified accuracy** — 280 languages tested fixture-by-fixture against `tokei` and `scc`
- **Flexible output** — table (default), JSON, or CSV for scripts/CI/dashboards
- **Respects ignore files** — `.gitignore` in git repos, `.prettierignore` everywhere

Why not use `tokount` (yet)?

- You need complexity metrics or COCOMO/ULOC (not implemented)
- You need follow-symlink mode on Windows (`-L` is intentionally unsupported there)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

- [Rust](https://www.rust-lang.org/)
- [clap](https://github.com/clap-rs/clap)
- [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore)
- [rayon](https://github.com/rayon-rs/rayon)
- [crossbeam-channel](https://github.com/crossbeam-rs/crossbeam)
- [memchr](https://github.com/BurntSushi/memchr)
- [memmap2](https://github.com/RazrFalcon/memmap2-rs)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

### Prerequisites

- Linux or macOS (Windows supported except `-L`)
- Rust toolchain if installing with `cargo`

### Installation

```bash
# with cargo
cargo install tokount

# or with yay (AUR)
yay -S tokount

# or with paru (AUR)
paru -S tokount
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE -->

## Usage

```bash
# analyze current directory
tokount .

# analyze a specific path
tokount /path/to/project

# analyze many paths (e.g. from git)
tokount $(git ls-files)

# exclude directories
tokount . -e node_modules,vendor

# machine output
tokount . -o json
tokount . -o csv

# sort by a different column
tokount . -s lines

# count only specific languages
tokount . -t Rust,Python

# disable ignore file behavior
tokount . --no-ignore

# list all supported languages
tokount -l
```

### All the Flags

| Flag                | Short | What it does                                          |
| ------------------- | ----- | ----------------------------------------------------- |
| `--excluded <DIRS>` | `-e`  | comma-separated directories to exclude                |
| `--follow-symlinks` | `-L`  | follow symbolic links when scanning                   |
| `--output <FORMAT>` | `-o`  | output format: `table` (default), `json`, `csv`       |
| `--sort <COLUMN>`   | `-s`  | sort by: `files`, `lines`, `blank`, `comment`, `code` |
| `--types <LANGS>`   | `-t`  | filter to specific language(s), comma-separated       |
| `--no-ignore`       |       | disable `.gitignore` / `.prettierignore` respect      |
| `--languages`       | `-l`  | print all supported languages and exit                |
| `--help`            | `-h`  | print help                                            |
| `--version`         | `-V`  | print version                                         |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- OUTPUT FORMATS -->

## Output Formats

**Table** (default):

```console
github.com/MihaiStreames/tokount v2.0.0  T=0.17s  (1513 files/s, 77285 lines/s)
251 files  •  1 git repos  •  tokount/

────────────────────────────────────────────────────────────────────────────────
 Language                               Files      Blank      Comment      Code
════════════════════════════════════════════════════════════════════════════════
 Rust                                      24        294          141      1767
────────────────────────────────────────────────────────────────────────────────
 YAML                                      16         66           10       436
────────────────────────────────────────────────────────────────────────────────
 SUM                                      251       1631         1595      9599
────────────────────────────────────────────────────────────────────────────────
```

**JSON** (`-o json`):

```json
{
  "Rust": { "nFiles": 24 , "blank": 294 , "comment": 141 , "code": 1767 },
  "SUM":  { "nFiles": 251, "blank": 1631, "comment": 1595, "code": 9599 },
  "gitRepos": 1,
  "gitignorePatterns": ["target/", "node_modules/", ... ]
}
```

**CSV** (`-o csv`): tabular export for spreadsheets and downstream tooling.

Errors go to stderr as structured JSON:

```json
{
  "error": {
    "kind": "NotFound",
    "message": "Path does not exist",
    "details": {"path": "/nonexistent"}
  }
}
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- BENCHMARKS -->

## Benchmarks

All benchmarks were run on an Intel Core i7-8650U @ 1.90 GHz / 16 GB RAM / Artix Linux using [hyperfine](https://github.com/sharkdp/hyperfine) (`--warmup 10 --runs 10`).

<table>
  <tr>
    <td><img src="assets/benchmarks/tokount-25k-lines.png" alt="tokount repo (~25k lines)"/></td>
    <td><img src="assets/benchmarks/redis-375k-lines.png" alt="Redis (~375k lines)"/></td>
    <td><img src="assets/benchmarks/ruff-1m-lines.png" alt="Ruff (~1M lines)"/></td>
  </tr>
  <tr>
    <td><img src="assets/benchmarks/cpython-2-2m-lines.png" alt="CPython (~2.2M lines)"/></td>
    <td><img src="assets/benchmarks/rust-3-5m-lines.png" alt="Rust compiler (~3.5M lines)"/></td>
    <td><img src="assets/benchmarks/linux-31-3m-lines.png" alt="Linux kernel (~31.3M lines)"/></td>
  </tr>
</table>

To reproduce:

```bash
./benchmark.sh
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->

## Acknowledgments

Many thanks to these projects for their work and inspiration, especially for publishing language definition files and pattern research that were useful for testing `tokount`:

- [cloc](https://github.com/AlDanial/cloc)
- [scc](https://github.com/boyter/scc)
- [tokei](https://github.com/XAMPPRocky/tokei)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->

## License

MIT. Do whatever you want with it. See [LICENSE](LICENSE) for details.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

<div align="center">

Made with ❤️

</div>

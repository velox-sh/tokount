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
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li><a href="#installation">Installation</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#output">Output</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

tokount is a fast CLI line counter for codebases. It outputs a human-readable table by default, with JSON and CSV options for piping into other tools like [ghlang](https://github.com/MihaiStreames/ghlang).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- INSTALLATION -->

## Installation

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

# analyze specific path
tokount /path/to/project

# multiple paths (e.g. from git ls-files)
tokount $(git ls-files)

# exclude directories
tokount . -e node_modules,vendor

# JSON output
tokount . -o json

# CSV output
tokount . -o csv

# sort by lines instead of code
tokount . -s lines

# filter to specific languages
tokount . -t Rust,Python

# disable .gitignore respect
tokount . --no-ignore

# list all 475+ supported languages
tokount -l
```

### Flags

| Flag                | Short | What it does                                         |
| ------------------- | ----- | ---------------------------------------------------- |
| `--excluded <DIRS>` | `-e`  | comma-separated directories to exclude               |
| `--follow-symlinks` | `-L`  | follow symbolic links when scanning                  |
| `--output <FORMAT>` | `-o`  | output format: `table` (default), `json`, `csv`      |
| `--sort <COLUMN>`   | `-s`  | sort by: `files`, `lines`, `blank`, `comment`, `code`|
| `--types <LANGS>`   | `-t`  | filter to specific language(s), comma-separated      |
| `--no-ignore`       |       | disable `.gitignore` / `.prettierignore` respect     |
| `--languages`       | `-l`  | print all supported languages and exit               |
| `--help`            | `-h`  | print help                                           |
| `--version`         | `-V`  | print version                                        |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- OUTPUT -->

## Output

By default, tokount prints a table with timing stats:

```console
github.com/MihaiStreames/tokount v2.1.0  T=0.22s  (250 files/s, 60690 lines/s)
54 files  •  1 git repos  •  .

────────────────────────────────────────────────────────────
 Language          Files      Blank      Comment       Code
════════════════════════════════════════════════════════════
 JSON                  6          0            0      10335
────────────────────────────────────────────────────────────
 Rust                 19        175           71       1295
────────────────────────────────────────────────────────────
 Markdown              4        137            0        304
────────────────────────────────────────────────────────────
 YAML                 11         36            7        249
────────────────────────────────────────────────────────────
 Python                3         48            1        178
────────────────────────────────────────────────────────────
 Shell                 3         27           25         86
────────────────────────────────────────────────────────────
 TOML                  5         11            3         86
────────────────────────────────────────────────────────────
 TypeScript            2          4            1         16
────────────────────────────────────────────────────────────
 TSX                   1          1            1         12
────────────────────────────────────────────────────────────
 SUM                  54        439          109      12561
────────────────────────────────────────────────────────────
```

With `-o json`:

```json
{
  "Rust": {"nFiles": 19, "blank": 175, "comment": 71,  "code": 1295},
  "TOML": {"nFiles": 5,  "blank": 11,  "comment": 3,   "code": 86},
  "SUM":  {"nFiles": 54, "blank": 439, "comment": 109, "code": 12561},
  "gitRepos": 1,
  "gitignorePatterns": ["target/", "node_modules/", "..."]
}
```

Errors are emitted as structured JSON to stderr:

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

<!-- ACKNOWLEDGMENTS -->

## Acknowledgments

Many thanks to these projects for their work and inspiration, especially for publishing language definition files and pattern research that were useful for testing tokount:

- [cloc](https://github.com/AlDanial/cloc)
- [scc](https://github.com/boyter/scc)
- [tokei](https://github.com/XAMPPRocky/tokei)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->

## License

MIT. Do whatever you want with it.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

<div align="center">

Made with ❤️

</div>

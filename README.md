<a id="readme-top"></a>

<!-- PROJECT SHIELDS -->
<div align="center">

[![AUR Version](https://img.shields.io/aur/version/tokount?label=AUR)](https://aur.archlinux.org/packages/tokount)
[![License](https://img.shields.io/github/license/MihaiStreames/tokount?label=License)](LICENSE)

</div>

<!-- PROJECT LOGO -->
<div align="center">
  <h1>tokount</h1>

  <h3 align="center">Fast line counter for codebases</h3>

  <p align="center">
    Powered by <a href="https://github.com/XAMPPRocky/tokei">tokei</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li><a href="#installation">Installation</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#output">Output</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

tokount is a fast CLI line counter built on [tokei](https://github.com/XAMPPRocky/tokei). It outputs a human-readable table by default, or raw JSON with `--json` for piping into other tools like [ghlang](https://github.com/MihaiStreames/ghlang).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- INSTALLATION -->

## Installation

```bash
# with yay (AUR)
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
tokount . --excluded node_modules,vendor

# machine-readable JSON output
tokount . --json
```

### Flags

| Flag                | Short | What it does                                      |
| ------------------- | ----- | ------------------------------------------------- |
| `--excluded <DIRS>` | `-e`  | comma-separated directories to exclude            |
| `--follow-symlinks` | `-L`  | follow symbolic links when scanning               |
| `--json`            | `-j`  | output raw JSON instead of a human-readable table |
| `--help`            | `-h`  | print help                                        |
| `--version`         | `-V`  | print version                                     |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- OUTPUT -->

## Output

By default, tokount prints a table with timing stats:

```text
github.com/MihaiStreames/tokount v1.1.0  T=0.02s  (1703 files/s, 51810 lines/s)
41 files  •  0 git repos  •  55 paths

─────────────────────────────────────────────────────────────
 Language               Files     Blank     Comment     Code
═════════════════════════════════════════════════════════════
 JSON                       6         0           0      137
─────────────────────────────────────────────────────────────
 Pacman's makepkg           1         4           1       32
─────────────────────────────────────────────────────────────
 Python                     1         2           1        4
─────────────────────────────────────────────────────────────
 Rust                      13       100          14      507
─────────────────────────────────────────────────────────────
 Shell                      2         6           5       21
─────────────────────────────────────────────────────────────
 TOML                       5        10           3       78
─────────────────────────────────────────────────────────────
 TSX                        1         1           1       12
─────────────────────────────────────────────────────────────
 TypeScript                 2         4           1       16
─────────────────────────────────────────────────────────────
 YAML                      10        33           2      252
─────────────────────────────────────────────────────────────
 SUM                       41       160          28     1059
─────────────────────────────────────────────────────────────
```

With `--json`, tokount outputs to stdout:

```json
{
  "Rust": {"nFiles": 12, "blank": 89, "comment": 11, "code": 416},
  "TOML": {"nFiles": 5,  "blank": 10, "comment": 3,  "code": 78},
  "SUM":  {"nFiles": 17, "blank": 99, "comment": 14, "code": 494},
  "gitRepos": 1,
  "gitignorePatterns": ["target/", "node_modules/"]
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

<!-- LICENSE -->

## License

MIT. See [LICENSE](LICENSE) for details.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

<div align="center">

Made with ❤️

</div>

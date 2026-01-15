<a id="readme-top"></a>

<!-- PROJECT SHIELDS -->
<div align="center">

[![Crates.io](https://img.shields.io/crates/v/tokount?label=crates.io)](https://crates.io/crates/tokount)
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

tokount is a simple CLI wrapper around tokei that outputs JSON stats. Built for use with [ghlang](https://github.com/MihaiStreames/ghlang), but works standalone too.

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

# exclude directories
tokount /path/to/project node_modules,vendor,.git
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- OUTPUT -->

## Output

tokount outputs JSON to stdout:

```json
{
  "Rust": {"nFiles": 5, "blank": 120, "comment": 45, "code": 890},
  "Python": {"nFiles": 3, "blank": 50, "comment": 20, "code": 340},
  "SUM": {"nFiles": 8, "blank": 170, "comment": 65, "code": 1230}
}
```

Errors are output as structured JSON to stderr:

```json
{
  "error": {
    "kind": "NotFound",
    "message": "Path does not exist"
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

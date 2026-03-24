<a id="contributing-top"></a>

<div align="center">
  <h1>Contributing</h1>

  <h3>How to contribute to tokount</h3>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#project-goals">Project Goals</a></li>
    <li><a href="#development-setup">Development Setup</a></li>
    <li><a href="#workflow">Workflow</a></li>
    <li><a href="#code-standards">Code Standards</a></li>
    <li><a href="#rust-ordering-rules">Rust Ordering Rules</a></li>
    <li><a href="#comments-and-docs">Comments And Docs</a></li>
    <li><a href="#linting-and-tests">Linting And Tests</a></li>
    <li><a href="#library-api-policy">Library API Policy</a></li>
    <li><a href="#commit-style">Commit Style</a></li>
  </ol>
</details>

## Project Goals

`tokount` has two non-negotiable goals:

1. Speed: remain near top benchmark speeds
2. Accuracy: line counts must match tokei/scc/cloc expectations

Every change should protect both goals.

## Development Setup

```bash
cargo check
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

Use the same commands before opening a PR.

## Workflow

1. Keep changes focused and small
2. Prefer existing utilities over adding new abstractions
3. Do not use panic!, unreachable!, or unwrap() in normal paths
4. Run checks after each meaningful change
5. Include evidence in PR description (what you ran and results)

For accuracy-sensitive work, compare outputs against reference tools:

```bash
tokount . -o json > /tmp/tokount.json
tokei   . -o json > /tmp/tokei.json
scc     . --format json > /tmp/scc.json
```

## Code Standards

- Imports at top of file
- Avoid large monolithic files/functions; split by responsibility
- Prefer explicit names over abbreviations
- If suppressing clippy is required, use #[expect(...)] and explain why
- Do not add dead code to satisfy future ideas; add code when needed

## Rust Ordering Rules

Use this order in Rust files unless there is a strong reason not to:

1. Use statements
2. Mod declarations
3. Constants and type aliases
4. Enums and structs
5. Impl blocks
6. Free functions (public before private)
7. #[cfg(test)] test module at end

Inside impl blocks:

1. Constructors/builders
2. Public methods
3. Private helpers

For parser/state-machine style modules, split by role:

- mod.rs: public entrypoint and module wiring
- helpers.rs: pure helpers and parsing utilities
- state.rs: state machine/data flow

## Comments And Docs

Comments:

- Explain why, invariants, or non-obvious trade-offs
- Do not narrate obvious operations
- Inline comments should be lowercase and concise

Rust docs:

- Public API should have rustdoc comments
- Crate-level docs belong in src/lib.rs
- Include at least one runnable or no_run example for public entrypoints

## Linting And Tests

Required before commit:

1. cargo fmt
2. cargo clippy --all-targets -- -D warnings
3. cargo test

If a change affects parsing/counting behavior, also run targeted accuracy tests.

## Library API Policy

tokount is CLI-first, with a supported library surface.

Supported library surface should be small and explicit:

- EngineConfig
- count
- OutputStats
- LangStats

Prefer re-exports from src/lib.rs for stable API access. Avoid exposing internals unless necessary.

When adding/changing public API:

1. Document it in rustdoc
2. Add usage example in src/lib.rs or README
3. Treat changes as semver-relevant

## Commit Style

Use conventional commits with scope:

```text
type(scope): short imperative summary

- bullet describing change
- bullet describing change
```

Types commonly used:

- feat(scope)
- fix(scope)
- refactor(scope)
- docs(scope)

<p align="right">(<a href="#contributing-top">back to top</a>)</p>

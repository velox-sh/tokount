//! # tokount
//!
//! Fast, accurate line counter for codebases. Primarily a
//! [CLI tool](https://github.com/velox-sh/tokount), this crate
//! exposes the counting engine as a library.
//!
//! # Quick start
//!
//! ```no_run
//! use std::path::Path;
//!
//! use tokount::EngineConfig;
//! use tokount::count;
//!
//! let stats = count(&[Path::new(".")], &EngineConfig::default());
//! println!("{} code lines", stats.languages["SUM"].code);
//! ```

#[doc(hidden)]
pub mod engine;
#[doc(hidden)]
pub mod types;

pub use engine::EngineConfig;
pub use engine::count;
pub use types::LangStats;
pub use types::OutputStats;

/// Returns all supported language display names.
///
/// The returned names are the same values accepted by `EngineConfig.types_filter`
/// and printed by the CLI's `--languages` output.
///
/// # Example
///
/// ```
/// let langs = tokount::supported_languages();
/// assert!(langs.contains(&"Rust"));
/// ```
#[must_use]
pub fn supported_languages() -> &'static [&'static str] {
    engine::language::LanguageDef::all_names()
}

/// Returns true when `name` matches a supported language (case-insensitive).
///
/// # Example
///
/// ```
/// assert!(tokount::is_supported_language("rust"));
/// assert!(tokount::is_supported_language("Rust"));
/// assert!(!tokount::is_supported_language("DefinitelyNotALanguage"));
/// ```
#[must_use]
pub fn is_supported_language(name: &str) -> bool {
    engine::language::LanguageDef::from_name(name).is_some()
}

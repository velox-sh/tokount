//! tokount: The fastest line counter for codebases
//!
//! tokount is CLI-first, but it also exposes a small and stable library API
//! for programmatic counting.
//!
//! # Recommended API
//!
//! Prefer the re-exported surface at crate root:
//! - [`EngineConfig`]
//! - [`count`]
//! - [`OutputStats`]
//! - [`LangStats`]
//!
//! `count` returns an [`OutputStats`] map keyed by language name and always
//! includes a `SUM` row with totals across all counted files.
//!
//! # Data shape
//!
//! `OutputStats.languages` is a map of language name -> [`LangStats`].
//! Embedded-language counts are available through [`LangStats::children`].
//!
//! # Quick start
//!
//! ```no_run
//! use std::path::Path;
//!
//! use tokount::EngineConfig;
//! use tokount::count;
//!
//! let config = EngineConfig {
//!     excluded: &[],
//!     follow_symlinks: false,
//!     no_ignore: false,
//!     types_filter: None,
//! };
//! let stats = count(&[Path::new(".")], &config);
//! println!("{} code lines", stats.languages["SUM"].code);
//! ```

#[doc(hidden)]
pub mod engine;
#[doc(hidden)]
pub mod types;

/// Engine configuration used by [`count`]
pub use engine::EngineConfig;
/// Count code/comment/blank lines across one or more paths
pub use engine::count;
/// Per-language statistics row
pub use types::LangStats;
/// Top-level output payload with language stats and metadata
pub use types::OutputStats;

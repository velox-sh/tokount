//! tokount: The fastest line counter for codebases
//!
//! tokount is CLI-first, but it also exposes a small library API for
//! programmatic counting.
//!
//! # Stable library surface
//!
//! - [`EngineConfig`]
//! - [`count`]
//! - [`OutputStats`]
//! - [`LangStats`]
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

pub mod engine;
pub mod types;

pub use engine::EngineConfig;
pub use engine::count;
pub use types::LangStats;
pub use types::OutputStats;

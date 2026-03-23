//! tokount: The fastest line counter for codebases
//!
//! # Quick start
//!
//! ```no_run
//! use std::path::Path;
//!
//! use tokount::engine::EngineConfig;
//! use tokount::engine::count;
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

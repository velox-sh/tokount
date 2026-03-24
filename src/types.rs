//! Public output types used by the crate-level counting API

use std::collections::HashMap;

use serde::Serialize;

/// Per-language line statistics
#[derive(Serialize)]
pub struct LangStats {
    /// Number of files matched to this language
    #[serde(rename = "nFiles")]
    pub n_files: usize,
    /// Total lines for this row (`blank + comment + code`)
    pub lines: usize,
    /// Blank lines
    pub blank: usize,
    /// Comment lines
    pub comment: usize,
    /// Code lines
    pub code: usize,
    /// Embedded child language blocks (e.g. Rust code inside Markdown fences)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub children: HashMap<String, LangStats>,
}

/// Complete output structure with language stats and metadata
#[derive(Serialize)]
pub struct OutputStats {
    /// Language rows keyed by language name, includes `SUM`
    #[serde(flatten)]
    pub languages: HashMap<String, LangStats>,
    /// Number of git repositories discovered while walking input roots
    #[serde(rename = "gitRepos")]
    pub git_repos: usize,
    /// Collected ignore patterns used during walk/filtering
    #[serde(rename = "gitignorePatterns")]
    pub gitignore_patterns: Vec<String>,
}

/// Structured error payload for stderr output
#[doc(hidden)]
#[derive(Serialize)]
pub struct ErrorPayload {
    pub error: ErrorBody,
}

/// Error details emitted by tokount
#[doc(hidden)]
#[derive(Serialize)]
pub struct ErrorBody {
    pub kind: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
}

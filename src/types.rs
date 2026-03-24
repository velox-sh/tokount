use std::collections::HashMap;

use serde::Serialize;

/// Per-language line statistics.
///
/// A row in the output table/json keyed by language name.
#[derive(Serialize)]
pub struct LangStats {
    /// Number of files matched to this language.
    ///
    /// Child rows use `0` because they are embedded blocks inside a parent file.
    #[serde(rename = "nFiles")]
    pub n_files: usize,
    pub lines: usize,
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
    /// Embedded child language blocks keyed by child language name.
    ///
    /// Example: Rust code fences inside Markdown appear under the Markdown row.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub children: HashMap<String, LangStats>,
}

/// Complete counting output with per-language stats and walk metadata.
#[derive(Serialize)]
pub struct OutputStats {
    /// Language rows keyed by language name.
    ///
    /// Always includes a `SUM` key with totals across all counted files.
    #[serde(flatten)]
    pub languages: HashMap<String, LangStats>,
    #[serde(rename = "gitRepos")]
    pub git_repos: usize,
    #[serde(rename = "gitignorePatterns")]
    pub gitignore_patterns: Vec<String>,
}

// structured error payload for stderr output
#[doc(hidden)]
#[derive(Serialize)]
pub struct ErrorPayload {
    pub error: ErrorBody,
}

// error details emitted to stderr as JSON
#[doc(hidden)]
#[derive(Serialize)]
pub struct ErrorBody {
    pub kind: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
}

use std::collections::HashMap;

use serde::Serialize;

/// Per-language line statistics
#[derive(Serialize)]
pub struct LangStats {
    #[serde(rename = "nFiles")]
    pub n_files: usize,
    pub lines: usize,
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
    /// Embedded child language blocks (e.g. Rust code inside Markdown fences)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub children: HashMap<String, LangStats>,
}

/// Complete output structure with language stats and metadata
#[derive(Serialize)]
pub struct OutputStats {
    #[serde(flatten)]
    pub languages: HashMap<String, LangStats>,
    #[serde(rename = "gitRepos")]
    pub git_repos: usize,
    #[serde(rename = "gitignorePatterns")]
    pub gitignore_patterns: Vec<String>,
}

/// Structured error payload for stderr output
#[derive(Serialize)]
pub struct ErrorPayload {
    pub error: ErrorBody,
}

/// Error details emitted by tokount
#[derive(Serialize)]
pub struct ErrorBody {
    pub kind: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
}

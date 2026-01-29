use crate::types::{ErrorBody, ErrorPayload};
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;

/// tokei-powered fast line counter for codebases
#[derive(Parser, Debug)]
#[command(name = "tokount", version, about)]
pub struct Args {
    /// Path to analyze
    pub path: PathBuf,

    /// Comma-separated directories to exclude
    #[arg(value_delimiter = ',')]
    pub excluded: Option<Vec<String>>,

    /// Follow symbolic links
    #[arg(short = 'L', long)]
    pub follow_symlinks: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get excluded directories as a Vec of &str
    pub fn excluded_dirs(&self) -> Vec<&str> {
        self.excluded
            .as_ref()
            .map(|v| v.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }
}

/// Emit a structured error payload to stderr and exit with a non-zero code
pub fn emit_error(kind: &str, message: &str, details: Option<HashMap<String, String>>) -> ! {
    let payload = ErrorPayload {
        error: ErrorBody {
            kind: kind.to_string(),
            message: message.to_string(),
            details,
        },
    };

    match serde_json::to_string(&payload) {
        Ok(json) => eprintln!("{json}"),
        Err(err) => {
            eprintln!("{{\"error\":{{\"kind\":\"SerializeError\",\"message\":\"{err}\"}}}}");
        }
    }

    process::exit(2);
}

use std::collections::HashMap;
use std::path::PathBuf;
use std::process;

use clap::Parser;
use clap::ValueEnum;
use tokount::types::ErrorBody;
use tokount::types::ErrorPayload;

/// Fast line counter for codebases (faster than tokei, scc, and cloc btw)
#[derive(Parser, Debug)]
#[command(name = "tokount", version, about)]
pub struct Args {
    /// Paths to analyze (files or directories)
    #[arg(num_args(1..))]
    pub paths: Vec<PathBuf>,

    /// Comma-separated directories to exclude
    #[arg(short = 'e', long, value_delimiter = ',')]
    pub exclude: Option<Vec<String>>,

    /// Follow symbolic links
    #[arg(short = 'L', long)]
    pub follow_symlinks: bool,

    /// Output format
    #[arg(short = 'o', long, value_name = "FORMAT")]
    pub output: Option<OutputFormat>,

    /// Sort output by column (default: code)
    #[arg(short = 's', long, value_name = "COLUMN")]
    pub sort: Option<SortColumn>,

    /// Filter output to specific language(s), comma-separated
    /// (e.g. Rust,Python)
    #[arg(short = 't', long, value_delimiter = ',')]
    pub types: Option<Vec<String>>,

    /// Disable .gitignore / .prettierignore respect
    #[arg(long)]
    pub no_ignore: bool,

    /// Disable ANSI colors in table output
    #[arg(long)]
    pub no_color: bool,

    /// Print all supported languages and exit
    #[arg(short = 'l', long)]
    pub languages: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum SortColumn {
    Files,
    Lines,
    Blank,
    Comment,
    #[default]
    Code,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Csv,
}

impl Args {
    pub fn parse_args() -> Self {
        let args = Self::parse();

        #[cfg(target_os = "windows")]
        if args.follow_symlinks {
            eprintln!("error: -L/--follow-symlinks is not supported on Windows");
            process::exit(2);
        }

        if let Some(types) = &args.types
            && let Some(invalid) = types
                .iter()
                .find(|name| !tokount::is_supported_language(name))
        {
            let mut details = HashMap::new();
            details.insert("language".to_string(), invalid.clone());
            emit_error(
                "UnknownLanguage",
                "Unsupported language name in --types",
                Some(details),
            );
        }

        args
    }

    pub fn format(&self) -> OutputFormat {
        self.output.unwrap_or_default()
    }

    pub fn sort_column(&self) -> SortColumn {
        self.sort.unwrap_or_default()
    }

    pub fn excluded_dirs(&self) -> Vec<&str> {
        self.exclude
            .as_ref()
            .map(|v| v.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    pub fn types_filter(&self) -> Option<Vec<&str>> {
        self.types
            .as_ref()
            .map(|v| v.iter().map(String::as_str).collect())
    }
}

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

mod cli;
mod display;

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use cli::Args;
use cli::OutputFormat;
use cli::emit_error;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use tokount::engine;
use tokount::engine::EngineConfig;
use tokount::types;

fn spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

fn main() {
    let args = Args::parse_args();

    // --languages: print all supported languages and exit
    if args.languages {
        for name in engine::language::LanguageDef::all_names() {
            println!("{name}");
        }
        return;
    }

    for path in &args.paths {
        if !path.exists() {
            let mut details = HashMap::new();
            details.insert("path".to_string(), path.display().to_string());
            emit_error("NotFound", "Path does not exist", Some(details));
        }

        if let Err(err) = path.metadata() {
            let mut details = HashMap::new();
            details.insert("path".to_string(), path.display().to_string());
            details.insert("error".to_string(), err.to_string());
            emit_error("IoError", "Failed to read path metadata", Some(details));
        }
    }

    let excluded = args.excluded_dirs();
    let types_filter = args.types_filter();
    let types_refs: Option<Vec<&str>> = types_filter
        .as_ref()
        .map(|ts| ts.iter().map(|s| s as &str).collect());
    let path_refs: Vec<&Path> = args.paths.iter().map(PathBuf::as_path).collect();
    let fmt = args.format();
    let sort = args.sort_column();

    let pb = match fmt {
        OutputFormat::Table => {
            let pb = spinner();
            pb.set_message("Counting lines...");
            Some(pb)
        }
        _ => None,
    };

    let start = Instant::now();

    let output = engine::count(
        &path_refs,
        &EngineConfig {
            excluded: &excluded,
            follow_symlinks: args.follow_symlinks,
            no_ignore: args.no_ignore,
            types_filter: types_refs.as_deref(),
        },
    );

    let elapsed = start.elapsed();

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    let label = if args.paths.len() == 1 {
        args.paths[0].display().to_string()
    } else {
        format!("{} paths", args.paths.len())
    };

    match fmt {
        OutputFormat::Table => display::print_table(&output, &label, elapsed, sort),
        OutputFormat::Json => println!("{}", serde_json::to_string(&output).unwrap()),
        OutputFormat::Csv => display::print_csv(&output, sort),
    }
}

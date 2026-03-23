mod cli;
mod display;

use std::collections::HashMap;
use std::io::IsTerminal;
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

fn single_detail(key: &str, value: String) -> HashMap<String, String> {
    let mut details = HashMap::new();
    details.insert(key.to_string(), value);
    details
}

fn io_details(path: &Path, err: &std::io::Error) -> HashMap<String, String> {
    let mut details = single_detail("path", path.display().to_string());
    details.insert("error".to_string(), err.to_string());
    details
}

fn validate_paths(paths: &[PathBuf]) {
    for path in paths {
        if !path.exists() {
            emit_error(
                "NotFound",
                "Path does not exist",
                Some(single_detail("path", path.display().to_string())),
            );
        }

        if let Err(err) = path.metadata() {
            emit_error(
                "IoError",
                "Failed to read path metadata",
                Some(io_details(path, &err)),
            );
        }
    }
}

fn spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .expect("spinner template is valid"),
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

    validate_paths(&args.paths);

    let excluded = args.excluded_dirs();
    let types_filter = args.types_filter();
    let types_refs: Option<Vec<&str>> = types_filter
        .as_ref()
        .map(|ts| ts.iter().map(|s| s as &str).collect());
    let path_refs: Vec<&Path> = args.paths.iter().map(PathBuf::as_path).collect();
    let fmt = args.format();
    let sort = args.sort_column();
    let table_color = fmt == OutputFormat::Table
        && !args.no_color
        && std::io::stdout().is_terminal()
        && std::env::var_os("NO_COLOR").is_none();

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
        OutputFormat::Table => display::print_table(&output, &label, elapsed, sort, table_color),
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string(&output).expect("output is serializable")
        ),
        OutputFormat::Csv => display::print_csv(&output, sort),
    }
}

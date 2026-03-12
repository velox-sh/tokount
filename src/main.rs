mod analyze;
mod cli;
mod display;
mod git;
mod types;

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use cli::Args;
use cli::emit_error;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

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
    let path_refs: Vec<&Path> = args.paths.iter().map(PathBuf::as_path).collect();

    // spinner is a no-op when stderr is not a TTY (CI, pipes), so --json piping is
    // clean
    let pb = if args.json {
        None
    } else {
        let pb = spinner();
        pb.set_message("Scanning git info...");
        Some(pb)
    };

    let start = Instant::now();

    let git_info = git::collect_git_info(&path_refs, args.follow_symlinks);

    if let Some(ref pb) = pb {
        pb.set_message("Counting lines...");
    }

    let output = analyze::count_lines(
        &path_refs,
        &excluded,
        args.follow_symlinks,
        git_info.repo_count,
        git_info.patterns,
    );

    let elapsed = start.elapsed();

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    if args.json {
        println!("{}", serde_json::to_string(&output).unwrap());
    } else {
        // single path -> show it; multiple -> show count
        let label = if args.paths.len() == 1 {
            args.paths[0].display().to_string()
        } else {
            format!("{} paths", args.paths.len())
        };
        display::print_table(&output, &label, elapsed);
    }
}

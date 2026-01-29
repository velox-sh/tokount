mod analyze;
mod cli;
mod git;
mod types;

use cli::{Args, emit_error};
use std::collections::HashMap;

fn main() {
    let args = Args::parse_args();

    if !args.path.exists() {
        emit_error("NotFound", "Path does not exist", None);
    }

    if let Err(err) = args.path.metadata() {
        let mut details = HashMap::new();
        details.insert("error".to_string(), err.to_string());
        emit_error("IoError", "Failed to read path metadata", Some(details));
    }

    let excluded = args.excluded_dirs();

    let git_info = git::collect_git_info(&args.path, args.follow_symlinks);

    let output = analyze::count_lines(
        &args.path,
        &excluded,
        args.follow_symlinks,
        git_info.repo_count,
        git_info.patterns,
    );

    println!("{}", serde_json::to_string(&output).unwrap());
}

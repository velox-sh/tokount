mod cli;
mod display;

use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use cli::Args;
use tokount::EngineConfig;
use tokount::count;
use tokount::supported_languages;

fn main() {
    let args = Args::parse_args();

    if args.languages {
        for name in supported_languages() {
            println!("{name}");
        }
        return;
    }

    args.validate();

    let excluded = args.excluded_dirs();
    let types_filter = args.types_filter();
    let path_refs: Vec<&Path> = args.paths.iter().map(PathBuf::as_path).collect();

    let pb = display::start_spinner(args.format());
    let start = Instant::now();

    let output = count(
        &path_refs,
        &EngineConfig {
            excluded: &excluded,
            follow_symlinks: args.follow_symlinks,
            no_ignore: args.no_ignore,
            types_filter: types_filter.as_deref(),
            same_filesystem: args.same_filesystem,
        },
    );

    let elapsed = start.elapsed();

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    display::render(&output, &args.label(), elapsed, &args);
}

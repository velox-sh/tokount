mod cli;
mod display;

use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use cli::Args;
use tokount::EngineConfig;
use tokount::count;
use tokount::supported_languages;

/// Returns `true` when the current process is running with root / administrator
/// privileges.  On Unix we check the effective UID by reading `/proc/self/status`
/// (Linux) or falling back to the `SUDO_USER` environment variable (set by
/// sudo on any Unix).  On non-Unix platforms we always return `false`.
#[cfg(unix)]
fn is_elevated() -> bool {
    // Fastest path: sudo always sets SUDO_USER.
    if std::env::var_os("SUDO_USER").is_some() {
        return true;
    }
    // Reliable fallback: parse the Uid line from /proc/self/status (Linux).
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("Uid:") {
                // Format: "Uid: real effective saved filesystem"
                // Check the effective UID (second field) for root privileges.
                return rest.split_whitespace().nth(1) == Some("0");
            }
        }
    }
    false
}

#[cfg(not(unix))]
fn is_elevated() -> bool {
    false
}

fn main() {
    let args = Args::parse_args();

    if args.languages {
        for name in supported_languages() {
            println!("{name}");
        }
        return;
    }

    args.validate();

    if is_elevated() && !args.same_filesystem {
        eprintln!(
            "warning: running as root without --same-filesystem (-x).\n\
             Scanning system paths (e.g. /proc, /sys) can cause the program to crash\n\
             due to out-of-memory errors. Re-run with -x to stay within a single filesystem."
        );
    }

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

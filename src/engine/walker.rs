use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crossbeam_channel::Sender;
use ignore::WalkBuilder;
use ignore::WalkState;

/// Configuration for recursive directory traversal
pub struct WalkConfig<'a> {
    pub roots: &'a [&'a Path],
    pub excluded: &'a [&'a str],
    pub follow_symlinks: bool,
    pub no_ignore: bool,
    pub same_filesystem: bool,
}

/// Results from a complete filesystem walk
pub struct WalkResult {
    pub git_repos: usize,
    pub gitignore_patterns: Vec<String>,
}

#[inline]
fn empty_walk_result() -> WalkResult {
    WalkResult {
        git_repos: 0,
        gitignore_patterns: Vec::new(),
    }
}

fn is_git_repo(path: &Path) -> bool {
    let mut current = Some(path);
    while let Some(dir) = current {
        if dir.join(".git").exists() {
            return true;
        }
        current = dir.parent();
    }
    false
}

fn parse_ignore_file(path: &Path) -> Vec<String> {
    let mut patterns = Vec::new();
    if let Ok(file) = fs::File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let trimmed = line.trim().to_string();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                patterns.push(trimmed);
            }
        }
    }
    patterns
}

#[inline]
fn extend_patterns(patterns: &Mutex<Vec<String>>, new_patterns: Vec<String>) {
    if !new_patterns.is_empty()
        && let Ok(mut guard) = patterns.lock()
    {
        guard.extend(new_patterns);
    }
}

/// Walk `config.roots` in parallel, sending discovered file paths to `tx`
pub fn walk_parallel(config: &WalkConfig<'_>, tx: &Sender<PathBuf>) -> WalkResult {
    if config.roots.is_empty() {
        return empty_walk_result();
    }

    let git_repos = Arc::new(AtomicUsize::new(0));
    let patterns = Arc::new(Mutex::new(Vec::<String>::new()));

    let git_repos_clone = git_repos.clone();
    let patterns_clone = patterns.clone();

    let use_git_ignore = !config.no_ignore && is_git_repo(config.roots[0]);

    let mut builder = WalkBuilder::new(config.roots[0]);
    for root in &config.roots[1..] {
        builder.add(*root);
    }

    let excluded: Vec<String> = config.excluded.iter().map(ToString::to_string).collect();

    builder
        .follow_links(config.follow_symlinks)
        .same_file_system(config.same_filesystem)
        .git_ignore(use_git_ignore)
        .git_global(use_git_ignore)
        .git_exclude(use_git_ignore)
        .require_git(false)
        .hidden(false) // count .env, .bashrc, etc.
        .filter_entry(move |entry| {
            let name = entry.file_name().to_str().unwrap_or("");

            if entry.file_type().is_some_and(|t| t.is_dir()) {
                return !excluded.iter().any(|e| e == name);
            }

            true
        });

    if !config.no_ignore {
        if !use_git_ignore {
            builder.add_custom_ignore_filename(".gitignore");
        }
        builder.add_custom_ignore_filename(".prettierignore");
    }

    builder.build_parallel().run(|| {
        let tx = tx.clone();
        let git_repos = git_repos_clone.clone();
        let patterns = patterns_clone.clone();

        Box::new(move |result| {
            let Ok(entry) = result else {
                return WalkState::Continue;
            };

            let Some(file_type) = entry.file_type() else {
                return WalkState::Continue;
            };

            if file_type.is_dir() {
                if entry.file_name() == ".git" {
                    git_repos.fetch_add(1, Ordering::Relaxed);
                }
                return WalkState::Continue;
            }

            if file_type.is_file() {
                let name = entry.file_name().to_str().unwrap_or("");
                if name == ".gitignore" || name == ".prettierignore" {
                    extend_patterns(&patterns, parse_ignore_file(entry.path()));
                    return WalkState::Continue;
                }

                let _ = tx.send(entry.into_path());
            }

            WalkState::Continue
        })
    });

    let git_count = git_repos.load(Ordering::Relaxed);
    drop(git_repos_clone);
    drop(patterns_clone);

    // Arc strong count is 1 here (clone was dropped above); Mutex is never poisoned
    // (no panics inside the parallel walk body), so both unwraps are infallible
    let mut pats = Arc::try_unwrap(patterns)
        .expect("Patterns Arc still has live clones")
        .into_inner()
        .expect("Patterns Mutex was poisoned");
    pats.sort();
    pats.dedup();

    WalkResult {
        git_repos: git_count,
        gitignore_patterns: pats,
    }
}

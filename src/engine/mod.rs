// engine internals backing the crate-level count API
// most users should call count() via crate root re-exports
// not through this module directly

#[doc(hidden)]
pub mod fsm;
#[doc(hidden)]
pub mod language;
#[doc(hidden)]
pub mod reader;
#[doc(hidden)]
pub mod scanner;
#[doc(hidden)]
pub mod stats;
#[doc(hidden)]
pub mod walker;

use std::cell::RefCell;
use std::fs;
use std::io::BufRead as _;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use crossbeam_channel::unbounded;
use rayon::prelude::*;

/// Configuration for a [`count`] run
#[derive(Default)]
pub struct EngineConfig<'a> {
    /// Glob patterns for paths to exclude (e.g. `["target", "vendor"]`)
    pub excluded: &'a [&'a str],
    /// Whether to follow symbolic links when walking directories
    pub follow_symlinks: bool,
    /// Disable ignore-file filtering (`.gitignore`, `.ignore`, `.prettierignore`)
    pub no_ignore: bool,
    /// If set, only count files whose language name matches one of these strings
    /// (case-insensitive)
    pub types_filter: Option<&'a [&'a str]>,
}

// thread-local reader reuses the same buffer across all files on a given
// thread, avoiding per-file heap allocation in the hot path
thread_local! {
    static READER: RefCell<reader::FileReader> = RefCell::new(reader::FileReader::new());
}

fn peek_shebang(path: &Path) -> Option<&'static language::LanguageDef> {
    let file = fs::File::open(path).ok()?;
    let mut line = String::new();
    std::io::BufReader::new(file).read_line(&mut line).ok()?;
    language::LanguageDef::from_shebang(line.trim_end())
}

/// Walk paths, detect languages, and return aggregate line statistics.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
///
/// use tokount::EngineConfig;
/// use tokount::count;
///
/// let config = EngineConfig {
///     excluded: &["target", "node_modules"],
///     ..Default::default()
/// };
///
/// let stats = count(&[Path::new(".")], &config);
/// let total = &stats.languages["SUM"];
/// println!(
///     "files={} lines={} code={}",
///     total.n_files, total.lines, total.code
/// );
/// ```
#[must_use]
pub fn count(paths: &[&Path], config: &EngineConfig<'_>) -> crate::types::OutputStats {
    // unbounded: walker never blocks waiting for consumers (mirrors tokei)
    let (tx, rx) = unbounded::<PathBuf>();

    let owned_paths: Vec<PathBuf> = paths.iter().map(|p| p.to_path_buf()).collect();
    let excluded_owned: Vec<String> = config.excluded.iter().map(ToString::to_string).collect();
    let follow_symlinks = config.follow_symlinks;
    let no_ignore = config.no_ignore;
    let types_filter: Arc<Option<Vec<String>>> = Arc::new(
        config
            .types_filter
            .map(|ts| ts.iter().map(ToString::to_string).collect()),
    );

    let walker_thread = std::thread::spawn(move || {
        let path_refs: Vec<&Path> = owned_paths.iter().map(PathBuf::as_path).collect();
        let excluded_refs: Vec<&str> = excluded_owned.iter().map(String::as_str).collect();

        let walk_config = walker::WalkConfig {
            roots: &path_refs,
            excluded: &excluded_refs,
            follow_symlinks,
            no_ignore,
        };

        walker::walk_parallel(&walk_config, &tx)
    });

    let merged = rx
        .into_iter()
        .par_bridge()
        .map(|path| {
            let mut thread_stats = stats::ThreadStats::new();

            let ext_raw = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let ext_lower;

            let ext = if ext_raw.bytes().any(|b| b.is_ascii_uppercase()) {
                ext_lower = ext_raw.to_ascii_lowercase();
                ext_lower.as_str()
            } else {
                ext_raw
            };

            let filename_raw = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let filename_lower;

            let filename = if filename_raw.bytes().any(|b| b.is_ascii_uppercase()) {
                filename_lower = filename_raw.to_ascii_lowercase();
                filename_lower.as_str()
            } else {
                filename_raw
            };

            let lang = language::LanguageDef::from_extension(ext)
                .or_else(|| language::LanguageDef::from_filename(filename))
                .or_else(|| {
                    if ext.is_empty() {
                        peek_shebang(&path)
                    } else {
                        None
                    }
                });

            if let Some(lang) = lang {
                if let Some(types) = types_filter.as_deref()
                    && !types.iter().any(|t| t.eq_ignore_ascii_case(lang.name))
                {
                    return thread_stats;
                }

                READER.with(|r| {
                    if let Some(content) = r.borrow_mut().read(&path) {
                        let result = fsm::count_file(content, lang);
                        thread_stats.add(lang.name, result);
                    }
                });
            }

            thread_stats
        })
        .reduce(stats::ThreadStats::new, |mut a, b| {
            a.merge(b);
            a
        });

    let walk_info = walker_thread.join().expect("walker thread panicked");

    let mut merged = merged;
    merged.git_repos = walk_info.git_repos;
    merged.gitignore_patterns = walk_info.gitignore_patterns;

    merged.into_output()
}

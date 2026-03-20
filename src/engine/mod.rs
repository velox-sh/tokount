pub mod fsm;
pub mod language;
pub mod reader;
pub mod scanner;
pub mod stats;
pub mod walker;

use std::cell::RefCell;
use std::fs;
use std::io::BufRead as _;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use crossbeam_channel::unbounded;
use rayon::prelude::*;

pub struct EngineConfig<'a> {
    pub excluded: &'a [&'a str],
    pub follow_symlinks: bool,
    pub no_ignore: bool,
    pub types_filter: Option<&'a [&'a str]>,
}

// thread-local reader reuses the same buffer across all files on a given
// thread, avoiding per-file heap allocation in the hot path
thread_local! {
    static READER: RefCell<reader::FileReader> = RefCell::new(reader::FileReader::new());
}

pub fn count(paths: &[&Path], config: &EngineConfig<'_>) -> crate::types::OutputStats {
    // unbounded: walker never blocks waiting for consumers (mirrors tokei)
    let (tx, rx) = unbounded::<PathBuf>();

    let owned_paths: Vec<PathBuf> = paths.iter().map(|p| p.to_path_buf()).collect();
    let excluded_owned: Vec<String> = config.excluded.iter().map(ToString::to_string).collect();
    let follow_symlinks = config.follow_symlinks;
    let no_ignore = config.no_ignore;
    // Arc so the types list is shared across rayon threads without cloning per-file
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
            // extensions in the map are lowercase; normalise before lookup
            let ext_lower;
            let ext = if ext_raw.bytes().any(|b| b.is_ascii_uppercase()) {
                ext_lower = ext_raw.to_ascii_lowercase();
                ext_lower.as_str()
            } else {
                ext_raw
            };
            let filename_raw = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            // filename map is keyed lowercase; normalise before lookup
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
                    // shebang detection only for files with no recognized extension
                    if ext.is_empty() {
                        peek_shebang(&path)
                    } else {
                        None
                    }
                });

            if let Some(lang) = lang {
                // skip files whose language isn't in the types filter
                if let Some(types) = types_filter.as_deref()
                    && !types.iter().any(|t| t.eq_ignore_ascii_case(lang.name))
                {
                    return thread_stats;
                }

                READER.with(|r| {
                    if let Some(content) = r.borrow_mut().read(&path) {
                        let counts = fsm::count_file(content, lang);
                        thread_stats.add(lang.name, counts);
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

/// Read the first line of a file and check for a shebang interpreter name
fn peek_shebang(path: &Path) -> Option<&'static language::LanguageDef> {
    let file = fs::File::open(path).ok()?;
    let mut line = String::new();
    std::io::BufReader::new(file).read_line(&mut line).ok()?;
    language::LanguageDef::from_shebang(line.trim_end())
}

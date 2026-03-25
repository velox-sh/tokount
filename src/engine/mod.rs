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
use std::io::Seek as _;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use crossbeam_channel::bounded;
use rayon::prelude::*;

/// Configuration for a [`count`] run
#[derive(Default)]
pub struct EngineConfig<'a> {
    pub excluded: &'a [&'a str],
    pub follow_symlinks: bool,
    /// Disable ignore-file filtering (`.gitignore`, `.ignore`, `.prettierignore`)
    pub no_ignore: bool,
    /// If set, only count files whose language name matches one of these strings
    pub types_filter: Option<&'a [&'a str]>,
    /// Do not cross filesystem boundaries (skips `/proc`, `/sys`, NFS mounts, etc.)
    pub same_filesystem: bool,
}

// thread-local reader reuses the same buffer across all files on a given
// thread, avoiding per-file heap allocation in the hot path
thread_local! {
    static READER: RefCell<reader::FileReader> = RefCell::new(reader::FileReader::new());
}

// opens the file once, reads the shebang, then seeks back to 0 so the caller
// can pass the same fd to FileReader::read_open without a second open syscall
fn open_with_shebang(path: &Path) -> Option<(&'static language::LanguageDef, fs::File, u64)> {
    let mut file = fs::File::open(path).ok()?;

    let metadata = file.metadata().ok()?;
    if !metadata.file_type().is_file() {
        return None;
    }

    let size = metadata.len();
    if size == 0 {
        return None;
    }

    let mut line = String::new();
    std::io::BufReader::new(&file).read_line(&mut line).ok()?;

    let lang = language::LanguageDef::from_shebang(line.trim_end())?;
    file.seek(std::io::SeekFrom::Start(0)).ok()?;
    Some((lang, file, size))
}

// named so it shows as a distinct frame in flamegraphs rather than count::{{closure}}
fn detect_language(path: &Path) -> Option<&'static language::LanguageDef> {
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

    language::LanguageDef::from_extension(ext)
        .or_else(|| language::LanguageDef::from_filename(filename))
}

// named so it shows as a distinct frame in flamegraphs rather than count::{{closure}}
fn process_file(path: &Path, types_filter: &Option<Vec<String>>) -> stats::ThreadStats {
    let mut thread_stats = stats::ThreadStats::new();

    // fast path: extension + filename, no I/O
    let (lang, open_file, file_size) = if let Some(lang) = detect_language(path) {
        (lang, None, 0u64)
    } else if path.extension().is_none() {
        // extensionless: open once, read shebang, seek back
        let Some((lang, file, size)) = open_with_shebang(path) else {
            return thread_stats;
        };
        (lang, Some(file), size)
    } else {
        return thread_stats;
    };

    if let Some(types) = types_filter.as_deref()
        && !types.iter().any(|t| t.eq_ignore_ascii_case(lang.name))
    {
        return thread_stats;
    }

    READER.with(|r| {
        let mut reader = r.borrow_mut();

        let content = if let Some(file) = open_file {
            reader.read_open(file, file_size)
        } else {
            reader.read(path)
        };

        if let Some(content) = content {
            thread_stats.add(lang.name, fsm::count_file(content, lang));
        }
    });

    thread_stats
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
    // bounded: backpressure prevents OOM when walker outruns consumers on huge scans
    let (tx, rx) = bounded::<PathBuf>(256);

    let owned_paths: Vec<PathBuf> = paths.iter().map(|p| p.to_path_buf()).collect();
    let excluded_owned: Vec<String> = config.excluded.iter().map(ToString::to_string).collect();
    let follow_symlinks = config.follow_symlinks;
    let no_ignore = config.no_ignore;
    let same_filesystem = config.same_filesystem;
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
            same_filesystem,
        };

        walker::walk_parallel(&walk_config, &tx)
    });

    let merged = rx
        .into_iter()
        .par_bridge()
        .map(|path| process_file(&path, &types_filter))
        // path: PathBuf is held by the closure; &path coerces to &Path via Deref
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

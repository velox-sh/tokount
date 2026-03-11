use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use tokei::Config;
use tokei::Languages;
use walkdir::WalkDir;

use crate::types::LangStats;
use crate::types::OutputStats;

/// Resolve symlinks in a directory and return all unique parent directories
fn resolve_symlinks_in_dir(root: &Path) -> Vec<String> {
    let mut resolved_paths: HashSet<String> = HashSet::new();

    let walker = WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok);

    for entry in walker {
        let path = entry.path();
        if path.is_file() {
            // canonical path to avoid duplicates
            if let Ok(canonical) = fs::canonicalize(path)
                && let Some(parent) = canonical.parent()
            {
                resolved_paths.insert(parent.to_string_lossy().to_string());
            }
        }
    }

    resolved_paths.into_iter().collect()
}

/// Run analysis on the given path
pub fn count_lines(
    path: &Path,
    excluded: &[&str],
    follow_symlinks: bool,
    git_repo_count: usize,
    gitignore_patterns: Vec<String>,
) -> OutputStats {
    let config = Config::default();
    let mut languages = Languages::new();

    let path_str = path.to_string_lossy();

    if follow_symlinks {
        let resolved_dirs = resolve_symlinks_in_dir(path);
        let mut all_paths: Vec<&str> = vec![&path_str];
        let resolved_refs: Vec<&str> = resolved_dirs.iter().map(String::as_str).collect();
        all_paths.extend(resolved_refs);

        languages.get_statistics(&all_paths, excluded, &config);
    } else {
        let path_arr: &[&str] = &[&path_str];
        languages.get_statistics(path_arr, excluded, &config);
    }

    let mut result: HashMap<String, LangStats> = HashMap::new();

    let mut total_files: usize = 0;
    let mut total_blank: usize = 0;
    let mut total_comment: usize = 0;
    let mut total_code: usize = 0;

    for (lang_type, lang) in languages.iter() {
        if lang.code > 0 {
            let file_count = lang.reports.len();

            result.insert(
                lang_type.name().to_string(),
                LangStats {
                    n_files: file_count,
                    blank: lang.blanks,
                    comment: lang.comments,
                    code: lang.code,
                },
            );

            total_files += file_count;
            total_blank += lang.blanks;
            total_comment += lang.comments;
            total_code += lang.code;
        }
    }

    result.insert(
        "SUM".to_string(),
        LangStats {
            n_files: total_files,
            blank: total_blank,
            comment: total_comment,
            code: total_code,
        },
    );

    OutputStats {
        languages: result,
        git_repos: git_repo_count,
        gitignore_patterns,
    }
}

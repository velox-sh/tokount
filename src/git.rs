use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use walkdir::WalkDir;

/// Parse a .gitignore file and extract patterns (non-empty, non-comment lines)
fn parse_gitignore(path: &Path) -> Vec<String> {
    let mut patterns = Vec::new();

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let trimmed = line.trim();

            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                patterns.push(trimmed.to_string());
            }
        }
    }

    patterns
}

/// Git repository information collected from a directory tree
pub struct GitInfo {
    /// Unique gitignore patterns found
    pub patterns: Vec<String>,
    /// Number of git repositories detected
    pub repo_count: usize,
}

/// Collect all unique gitignore patterns and count git repositories across
/// multiple roots
pub fn collect_git_info(roots: &[&Path], follow_symlinks: bool) -> GitInfo {
    let mut gitignore_patterns: HashSet<String> = HashSet::new();
    let mut git_repo_count: usize = 0;

    for root in roots {
        let walker = WalkDir::new(root)
            .follow_links(follow_symlinks)
            .into_iter()
            .filter_map(Result::ok);

        for entry in walker {
            let path = entry.path();

            // .git directory
            if path.is_dir() && path.file_name().is_some_and(|n| n == ".git") {
                git_repo_count += 1;
            }

            // .gitignore files
            if path.is_file() && path.file_name().is_some_and(|n| n == ".gitignore") {
                for pattern in parse_gitignore(path) {
                    gitignore_patterns.insert(pattern);
                }
            }
        }
    }

    let mut patterns_vec: Vec<String> = gitignore_patterns.into_iter().collect();
    patterns_vec.sort();

    GitInfo {
        patterns: patterns_vec,
        repo_count: git_repo_count,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    fn tmp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("tokount_test_{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn gitignore_returns_empty_for_missing_file() {
        let path = PathBuf::from("/nonexistent/path/.gitignore");
        assert!(parse_gitignore(&path).is_empty());
    }

    #[test]
    fn gitignore_returns_empty_for_all_comments() {
        let dir = tmp_dir("parse_comments_only");
        let file = dir.join(".gitignore");
        fs::write(&file, "# one\n# two\n\n").unwrap();
        assert!(parse_gitignore(&file).is_empty());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn collect_counts_git_repos() {
        let dir = tmp_dir("collect_repos");
        fs::create_dir_all(dir.join("proj_a/.git")).unwrap();
        fs::create_dir_all(dir.join("proj_b/.git")).unwrap();
        let info = collect_git_info(&[dir.as_path()], false);
        assert_eq!(info.repo_count, 2);
        fs::remove_dir_all(dir).unwrap();
    }
}

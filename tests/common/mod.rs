use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

// shared across test binaries; not every binary uses every helper
#[allow(dead_code)]
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[allow(dead_code)]
pub fn fixtures_dir() -> PathBuf {
    repo_root().join("tests/fixtures")
}

#[allow(dead_code)]
pub fn lang_dir() -> PathBuf {
    repo_root().join("tests/lang")
}

/// Run the tokount binary and return parsed JSON output
#[allow(dead_code)]
pub fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tokount"))
        .args(args)
        .output()
        .expect("failed to spawn tokount")
}

/// Run tokount on a path with extra args, return parsed JSON
#[allow(dead_code)]
pub fn run_json(path: &Path, extra_args: &[&str]) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_tokount"))
        .arg(path)
        .args(extra_args)
        .output()
        .expect("failed to spawn tokount");

    assert!(
        out.status.success(),
        "tokount exited with {}: {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    serde_json::from_slice(&out.stdout).expect("output is not valid JSON")
}

/// Expected line counts parsed from a tokei fixture file header
#[allow(dead_code)]
pub struct ExpectedCounts {
    pub lines: u32,
    pub code: u32,
    pub comment: u32,
    pub blank: u32,
}

/// Parse expected counts from the first 6 lines of a file's content
///
/// Handles all formats found in tokei fixtures:
///   `// 50 lines 33 code 8 comments 9 blanks`
///   `# 15 lines, 10 code, 2 comments, 3 blanks`
///   `# 16 lines, 9 code, 5 blanks, 2 comments`
///   `dnl 7 lines 3 code 1 blanks 3 comments`
///   `/* 50 lines 34 code 8 comments 8 blanks */`
///
/// Returns None if no count line is found (file should be skipped)
#[allow(dead_code)]
pub fn parse_expected_counts(content: &str) -> Option<ExpectedCounts> {
    content.lines().take(6).find_map(try_parse_counts)
}

fn try_parse_counts(line: &str) -> Option<ExpectedCounts> {
    // Each metric is a number immediately preceding its label word
    // Fields can appear in any order and may be comma-separated
    let n_before = |label: &str| -> Option<u32> {
        let words: Vec<&str> = line.split_whitespace().collect();

        for (i, &word) in words.iter().enumerate() {
            // strip trailing comma/punctuation from the label candidate
            let clean = word.trim_end_matches([',', '.', '/', '*', ')']);

            if (clean == label || clean == label.trim_end_matches('s')) && i > 0 {
                // strip trailing comma from the number candidate
                let num_str = words[i - 1].trim_end_matches(',');

                if let Ok(n) = num_str.parse::<u32>() {
                    return Some(n);
                }
            }
        }
        None
    };

    let lines = n_before("lines")?;
    let code = n_before("code")?;
    // "comments" or "comment"
    let comment = n_before("comments").or_else(|| n_before("comment"))?;
    // "blanks" or "blank"
    let blank = n_before("blanks").or_else(|| n_before("blank"))?;

    Some(ExpectedCounts {
        lines,
        code,
        comment,
        blank,
    })
}

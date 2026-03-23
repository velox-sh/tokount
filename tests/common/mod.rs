use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

#[allow(dead_code)]
pub struct ExpectedCounts {
    pub lines: u32,
    pub code: u32,
    pub comment: u32,
    pub blank: u32,
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
    let comment = n_before("comments").or_else(|| n_before("comment"))?;
    let blank = n_before("blanks").or_else(|| n_before("blank"))?;

    Some(ExpectedCounts {
        lines,
        code,
        comment,
        blank,
    })
}

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

#[allow(dead_code)]
pub fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tokount"))
        .args(args)
        .output()
        .expect("failed to spawn tokount")
}

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

// formats: `// N lines N code N comments N blanks`, `# N lines, N code, ...`,
// `dnl N lines ...`, `/* N lines ... */` (fields in any order, comma-separated ok)
#[allow(dead_code)]
pub fn parse_expected_counts(content: &str) -> Option<ExpectedCounts> {
    content.lines().take(6).find_map(try_parse_counts)
}

// sidecar format: single line `lines code comment blank` (for blank-only languages like JSON)
#[allow(dead_code)]
pub fn parse_expected_file(sidecar: &std::path::Path) -> Option<ExpectedCounts> {
    let content = std::fs::read_to_string(sidecar).ok()?;
    let line = content.lines().next()?;

    let nums: Vec<u32> = line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if nums.len() >= 4 {
        Some(ExpectedCounts {
            lines: nums[0],
            code: nums[1],
            comment: nums[2],
            blank: nums[3],
        })
    } else {
        None
    }
}

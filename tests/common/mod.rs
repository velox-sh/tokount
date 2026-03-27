use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

#[allow(dead_code)]
pub(crate) struct ExpectedCounts {
    pub(crate) lines: u32,
    pub(crate) code: u32,
    pub(crate) comment: u32,
    pub(crate) blank: u32,
}

// shared across test binaries; not every binary uses every helper
#[allow(dead_code)]
pub(crate) fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[allow(dead_code)]
pub(crate) fn fixtures_dir() -> PathBuf {
    repo_root().join("tests/fixtures")
}

#[allow(dead_code)]
pub(crate) fn lang_dir() -> PathBuf {
    repo_root().join("tests/lang")
}

#[allow(dead_code)]
pub(crate) fn lang_expected_dir() -> PathBuf {
    repo_root().join("tests/lang.expected")
}

#[allow(dead_code)]
pub(crate) fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tokount"))
        .args(args)
        .output()
        .expect("failed to spawn tokount")
}

#[allow(dead_code)]
pub(crate) fn run_json(path: &Path, extra_args: &[&str]) -> Value {
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

// sidecar format: first line is `lines code comment blank` (4 space-separated integers)
// optional attribution on subsequent lines is ignored
#[allow(dead_code)]
pub(crate) fn parse_expected_file(sidecar: &std::path::Path) -> Option<ExpectedCounts> {
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

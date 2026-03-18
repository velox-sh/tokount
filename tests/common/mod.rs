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

/// Load a JSON snapshot file relative to the given snapshots directory
#[allow(dead_code)]
pub fn load_snapshot(snapshots_dir: &Path, name: &str) -> Value {
    let path = snapshots_dir.join(format!("{name}.json"));
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("snapshot not found: {}", path.display()));
    serde_json::from_str(&raw).expect("snapshot is not valid JSON")
}

/// Assert actual JSON equals the named snapshot, with a diff-friendly message on failure
#[allow(dead_code)]
pub fn assert_snapshot(actual: &Value, snapshots_dir: &Path, name: &str) {
    let expected = load_snapshot(snapshots_dir, name);
    assert_eq!(
        actual, &expected,
        "snapshot mismatch for '{name}'\n  actual:   {actual}\n  expected: {expected}"
    );
}

use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn snapshots() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/snapshots")
}

fn load_snapshot(name: &str) -> Value {
    let path = snapshots().join(format!("{name}.json"));
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("snapshot not found: {}", path.display()));
    serde_json::from_str(&raw).expect("snapshot is not valid JSON")
}

/// Run the tokount binary against a fixture dir, return parsed JSON output
fn run(fixture: &str) -> Value {
    let bin = env!("CARGO_BIN_EXE_tokount");
    let root = fixtures().join(fixture);

    let out = Command::new(bin)
        .arg(&root)
        .arg("--json")
        .output()
        .expect("failed to run tokount");

    assert!(
        out.status.success(),
        "tokount exited with {}: {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    serde_json::from_slice(&out.stdout).expect("output is not valid JSON")
}

#[test]
fn snapshot_single_rust() {
    assert_eq!(run("single_rust"), load_snapshot("single_rust"));
}

#[test]
fn snapshot_multi_lang() {
    assert_eq!(run("multi_lang"), load_snapshot("multi_lang"));
}

#[test]
fn snapshot_nested_gitignore() {
    assert_eq!(run("nested_gitignore"), load_snapshot("nested_gitignore"));
}

#[test]
fn snapshot_monorepo() {
    assert_eq!(run("monorepo"), load_snapshot("monorepo"));
}

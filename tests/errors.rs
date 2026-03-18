#[path = "common/mod.rs"]
mod common;

#[test]
fn error_nonexistent_path_exits_nonzero_with_notfound() {
    let out = common::run(&["/nonexistent/path/that/does/not/exist"]);

    assert!(
        !out.status.success(),
        "expected non-zero exit for nonexistent path, got success"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("NotFound"),
        "stderr did not contain 'NotFound'; got: {stderr}"
    );
}

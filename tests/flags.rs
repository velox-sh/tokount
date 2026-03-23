#[path = "common/mod.rs"]
mod common;

#[test]
fn flag_output_csv_starts_with_header() {
    let root = common::fixtures_dir().join("monorepo");
    let out = common::run(&[root.to_str().unwrap(), "--output", "csv"]);

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.starts_with("language,files,lines,blank,comment,code"),
        "CSV output did not start with expected header; got: {text:?}"
    );
}

#[test]
fn flag_sort_files_sum_matches_total() {
    let root = common::fixtures_dir().join("monorepo");
    let json = common::run_json(&root, &["--output", "json", "--sort", "files"]);

    let sum_nfiles = json["SUM"]["nFiles"]
        .as_u64()
        .expect("SUM.nFiles not found in JSON output");

    let total: u64 = json
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| *k != "SUM" && k.chars().next().is_some_and(char::is_uppercase))
        .filter_map(|(_, v)| v.get("nFiles").and_then(serde_json::Value::as_u64))
        .sum();

    assert_eq!(
        sum_nfiles, total,
        "SUM.nFiles ({sum_nfiles}) does not match sum of per-language nFiles ({total})"
    );
}

#[test]
fn flag_types_rust_only_rust_and_sum() {
    let root = common::fixtures_dir().join("single_rust");
    let json = common::run_json(&root, &["--output", "json", "--types", "Rust"]);

    let obj = json.as_object().expect("output is not a JSON object");
    for key in obj.keys() {
        if key == "gitRepos" || key == "gitignorePatterns" {
            continue;
        }
        assert!(
            key == "Rust" || key == "SUM",
            "unexpected key in --types Rust output: {key}"
        );
    }

    assert!(obj.contains_key("Rust"), "Rust key missing from output");
    assert!(obj.contains_key("SUM"), "SUM key missing from output");
}

#[test]
fn flag_no_ignore_gte_with_ignore() {
    let root = common::fixtures_dir().join("single_rust");

    let with_ignore = common::run_json(&root, &["--output", "json"]);
    let without_ignore = common::run_json(&root, &["--output", "json", "--no-ignore"]);

    let files_with = with_ignore["SUM"]["nFiles"].as_u64().unwrap_or(0);
    let files_without = without_ignore["SUM"]["nFiles"].as_u64().unwrap_or(0);

    assert!(
        files_without >= files_with,
        "--no-ignore produced fewer files ({files_without}) than default ({files_with})"
    );
}

#[test]
fn flag_languages_nonempty_and_contains_rust() {
    let out = common::run(&["--languages"]);

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(!text.is_empty(), "--languages produced no output");
    assert!(
        text.contains("Rust"),
        "--languages output did not contain 'Rust'; got: {text}"
    );
}

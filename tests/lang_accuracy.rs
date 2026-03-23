#[path = "common/mod.rs"]
mod common;

use std::fs;

#[test]
fn all_languages() {
    let lang_dir = common::lang_dir();

    let mut entries: Vec<_> = fs::read_dir(&lang_dir)
        .expect("failed to read tests/lang")
        .filter_map(Result::ok)
        .filter(|e| {
            if !e.file_type().is_ok_and(|t| t.is_file()) {
                return false;
            }
            // skip sidecar .expected files (they're not language fixtures)
            let name = e.file_name();
            !name.to_string_lossy().ends_with(".expected")
        })
        .collect();

    entries.sort_by_key(std::fs::DirEntry::file_name);

    let mut mismatches: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();

    for entry in entries {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();

        // skip unreadable fixtures (usually binary)
        let Ok(content) = fs::read_to_string(&path) else {
            skipped.push(format!("{name} (unreadable)"));
            continue;
        };

        let expected = match common::parse_expected_counts(&content) {
            Some(c) => c,
            None => {
                // support fixtures that keep expected counts in a sidecar file
                let sidecar = path.with_file_name(format!("{name}.expected"));
                match common::parse_expected_file(&sidecar) {
                    Some(c) => c,
                    None => {
                        skipped.push(format!("{name} (no count header)"));
                        continue;
                    }
                }
            }
        };

        let json = common::run_json(&path, &["--output", "json", "--no-ignore"]);

        // fixture expectations represent full-file totals
        // SUM includes parent language plus embedded child blocks
        let sum = &json["SUM"];

        let actual_code = sum["code"].as_u64().unwrap_or(0) as u32;
        let actual_comment = sum["comment"].as_u64().unwrap_or(0) as u32;
        let actual_blank = sum["blank"].as_u64().unwrap_or(0) as u32;

        // unrecognized fixtures produce no language rows and a zero SUM
        if actual_code == 0
            && actual_comment == 0
            && actual_blank == 0
            && json["SUM"]["nFiles"].as_u64().unwrap_or(0) == 0
        {
            skipped.push(format!("{name} (language not recognized)"));
            continue;
        }

        if actual_code != expected.code
            || actual_comment != expected.comment
            || actual_blank != expected.blank
        {
            mismatches.push(format!(
				"  {name}\n    expected: code={} comment={} blank={}\n    actual:   code={} comment={} blank={}",
				expected.code, expected.comment, expected.blank,
				actual_code, actual_comment, actual_blank,
			));
        }
    }

    eprintln!(
        "\n{} skipped (no count header or unrecognized language):",
        skipped.len()
    );
    for s in &skipped {
        eprintln!("  {s}");
    }

    if !mismatches.is_empty() {
        panic!(
            "\n{} language(s) failed:\n{}\n",
            mismatches.len(),
            mismatches.join("\n")
        );
    }
}

// requires tokei and scc installed
// run with: `cargo test ... -- --ignored cross_tool_compare`
#[test]
#[ignore]
fn cross_tool_compare() {
    use std::process::Command;

    let lang_dir = common::lang_dir();

    let mut entries: Vec<_> = fs::read_dir(&lang_dir)
        .expect("failed to read tests/lang")
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_ok_and(|t| t.is_file())
                && !e.file_name().to_string_lossy().ends_with(".expected")
        })
        .collect();

    entries.sort_by_key(std::fs::DirEntry::file_name);

    let mut disagreements: Vec<String> = Vec::new();

    for entry in entries {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();

        // tokount baseline
        let tokount_json = common::run_json(&path, &["--output", "json", "--no-ignore"]);
        let sum = &tokount_json["SUM"];
        if sum["nFiles"].as_u64().unwrap_or(0) == 0 {
            continue; // skip unrecognized fixtures
        }
        let tc_code = sum["code"].as_u64().unwrap_or(0);
        let tc_comment = sum["comment"].as_u64().unwrap_or(0);
        let tc_blank = sum["blank"].as_u64().unwrap_or(0);

        // tokei comparison
        let tokei_out = Command::new("tokei")
            .arg("--output")
            .arg("json")
            .arg(&path)
            .output();

        // scc comparison
        let scc_out = Command::new("scc")
            .arg("--format")
            .arg("json")
            .arg(&path)
            .output();

        let mut row =
            format!("{name}: tokount code={tc_code} comment={tc_comment} blank={tc_blank}");
        let mut any_diff = false;

        if let Ok(o) = tokei_out
            && o.status.success()
            && let Ok(v) = serde_json::from_slice::<serde_json::Value>(&o.stdout)
        {
            // sum per-language rows for parity with tokount SUM totals
            let mut tok_code = 0u64;
            let mut tok_comment = 0u64;
            let mut tok_blank = 0u64;

            if let Some(obj) = v.as_object() {
                for (k, lang) in obj {
                    if k == "Total" {
                        continue;
                    }
                    tok_code += lang["code"].as_u64().unwrap_or(0);
                    tok_comment += lang["comments"].as_u64().unwrap_or(0);
                    tok_blank += lang["blanks"].as_u64().unwrap_or(0);
                }
            }

            row.push_str(&format!(
                " | tokei code={tok_code} comment={tok_comment} blank={tok_blank}"
            ));

            if tc_code != tok_code || tc_comment != tok_comment || tc_blank != tok_blank {
                any_diff = true;
            }
        }

        if let Ok(o) = scc_out
            && o.status.success()
            && let Ok(arr) = serde_json::from_slice::<serde_json::Value>(&o.stdout)
        {
            let mut scc_code = 0u64;
            let mut scc_comment = 0u64;
            let mut scc_blank = 0u64;

            if let Some(langs) = arr.as_array() {
                for lang in langs {
                    scc_code += lang["Code"].as_u64().unwrap_or(0);
                    scc_comment += lang["Comment"].as_u64().unwrap_or(0);
                    scc_blank += lang["Blank"].as_u64().unwrap_or(0);
                }
            }

            row.push_str(&format!(
                " | scc code={scc_code} comment={scc_comment} blank={scc_blank}"
            ));

            if tc_code != scc_code || tc_comment != scc_comment || tc_blank != scc_blank {
                any_diff = true;
            }
        }

        if any_diff {
            disagreements.push(row);
        }
    }

    if !disagreements.is_empty() {
        eprintln!("\n{} disagreements found:", disagreements.len());

        for d in &disagreements {
            eprintln!("  {d}");
        }
        // diagnostic only: cross-tool disagreements are expected for some formats
    }
}

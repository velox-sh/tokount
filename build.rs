use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
struct LangDef {
    extensions: Vec<String>,
    line_comment: Vec<String>,
    multi_line: Vec<Vec<String>>,
    quotes: Vec<Vec<String>>,
    #[serde(default)]
    nested: bool,
    #[serde(default)]
    filenames: Vec<String>,
}

fn escape_bytes(s: &str) -> String {
    let mut out = String::new();

    for b in s.bytes() {
        if b == b'\\' {
            out.push_str("\\\\");
        } else if b == b'"' {
            out.push_str("\\\"");
        } else if b.is_ascii_graphic() || b == b' ' {
            out.push(b as char);
        } else {
            out.push_str(&format!("\\x{b:02x}"));
        }
    }

    out
}

#[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let json_path = Path::new(&manifest_dir).join("languages.json");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest = Path::new(&out_dir).join("generated_languages.rs");

    println!("cargo:rerun-if-changed=languages.json");

    let raw = fs::read_to_string(&json_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", json_path.display()));
    let langs: BTreeMap<String, LangDef> =
        serde_json::from_str(&raw).expect("failed to parse languages.json");

    let mut out = fs::File::create(&dest).expect("failed to create output file");

    // generate static LanguageDef instances
    // (deduplicate const names with numeric suffix)
    let mut used_const_names: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    let mut lang_const_names: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for name in langs.keys() {
        let base = to_const_name(name);
        let count = used_const_names.entry(base.clone()).or_insert(0);
        let const_name = if *count == 0 {
            base.clone()
        } else {
            format!("{base}_{count}")
        };
        *count += 1;
        lang_const_names.insert(name.clone(), const_name);
    }

    for (name, lang) in &langs {
        let const_name = lang_const_names[name].clone();

        // line_comments: longest-first so longer markers shadow shorter prefixes
        let mut line_comments_sorted = lang.line_comment.clone();
        line_comments_sorted.sort_by_key(|s| std::cmp::Reverse(s.len()));

        let lc: Vec<String> = line_comments_sorted
            .iter()
            .map(|s| format!("b\"{}\"", escape_bytes(s)))
            .collect();
        let lc_str = format!("&[{}]", lc.join(", "));

        // block_comments: longest-first so longer openers shadow shorter prefixes
        let mut multi_line_sorted: Vec<&Vec<String>> = lang
            .multi_line
            .iter()
            .filter(|pair| pair.len() == 2)
            .collect();
        multi_line_sorted.sort_by_key(|p| std::cmp::Reverse(p[0].len()));

        let bc: Vec<String> = multi_line_sorted
            .iter()
            .map(|pair| {
                format!(
                    "(b\"{}\", b\"{}\")",
                    escape_bytes(&pair[0]),
                    escape_bytes(&pair[1])
                )
            })
            .collect();
        let bc_str = format!("&[{}]", bc.join(", "));

        // string_literals: longest-first so longer openers shadow shorter prefixes
        let mut quotes_sorted: Vec<&Vec<String>> =
            lang.quotes.iter().filter(|pair| pair.len() >= 2).collect();
        quotes_sorted.sort_by_key(|p| std::cmp::Reverse(p[0].len()));
        let sl: Vec<String> = quotes_sorted
            .iter()
            .map(|pair| {
                let raw = pair.get(2).is_some_and(|s| s == "raw");
                format!(
                    "(b\"{}\", b\"{}\", {})",
                    escape_bytes(&pair[0]),
                    escape_bytes(&pair[1]),
                    raw
                )
            })
            .collect();
        let sl_str = format!("&[{}]", sl.join(", "));

        // interest_mask: bytes that could start a token
        let mut mask = [false; 256];
        mask[b'\n' as usize] = true;

        for s in &lang.line_comment {
            if let Some(b) = s.bytes().next() {
                mask[b as usize] = true;
            }
        }

        for pair in &lang.multi_line {
            if pair.len() == 2 {
                if let Some(b) = pair[0].bytes().next() {
                    mask[b as usize] = true;
                }
                if let Some(b) = pair[1].bytes().next() {
                    mask[b as usize] = true;
                }
            }
        }

        for pair in &lang.quotes {
            if pair.len() >= 2 {
                if let Some(b) = pair[0].bytes().next() {
                    mask[b as usize] = true;
                }
                if let Some(b) = pair[1].bytes().next() {
                    mask[b as usize] = true;
                }
            }
        }

        let mask_str = {
            let vals: Vec<&str> = mask
                .iter()
                .map(|b| if *b { "true" } else { "false" })
                .collect();
            format!("[{}]", vals.join(", "))
        };

        writeln!(
            out,
            "static {const_name}: LanguageDef = LanguageDef {{\n\
			\tname: \"{name}\",\n\
			\tline_comments: {lc_str},\n\
			\tblock_comments: {bc_str},\n\
			\tstring_literals: {sl_str},\n\
			\tnested_comments: {},\n\
			\tinterest_mask: {mask_str},\n\
			}};\n",
            lang.nested
        )
        .unwrap();
    }

    // collect extension entries first so strings outlive phf_codegen borrows
    let mut seen_exts = std::collections::HashSet::new();
    let mut ext_entries: Vec<(String, String)> = Vec::new();

    for (name, lang) in &langs {
        let const_name = lang_const_names[name].clone();
        for ext in &lang.extensions {
            let ext_lower = ext.to_lowercase();

            if seen_exts.insert(ext_lower.clone()) {
                ext_entries.push((ext_lower, format!("&{const_name}")));
            }
        }
    }

    let mut ext_map = phf_codegen::Map::new();

    for (k, v) in &ext_entries {
        ext_map.entry(k.as_str(), v.as_str());
    }

    writeln!(
        out,
        "pub(super) static EXTENSION_MAP: phf::Map<&'static str, &'static LanguageDef> = {};",
        ext_map.build()
    )
    .unwrap();

    // collect filename entries first so strings outlive phf_codegen borrows
    let mut seen_fns = std::collections::HashSet::new();
    let mut fn_entries: Vec<(String, String)> = Vec::new();

    for (name, lang) in &langs {
        let const_name = lang_const_names[name].clone();
        for fname in &lang.filenames {
            if seen_fns.insert(fname.clone()) {
                fn_entries.push((fname.clone(), format!("&{const_name}")));
            }
        }
    }

    let mut fn_map = phf_codegen::Map::new();

    for (k, v) in &fn_entries {
        fn_map.entry(k.as_str(), v.as_str());
    }

    writeln!(
        out,
        "pub(super) static FILENAME_MAP: phf::Map<&'static str, &'static LanguageDef> = {};",
        fn_map.build()
    )
    .unwrap();

    // emit sorted unique language display names
    let names: Vec<String> = langs
        .keys()
        .map(|n| format!("\"{}\"", n.replace('"', "\\\"")))
        .collect();
    writeln!(
        out,
        "pub(super) static LANGUAGE_NAMES: &[&str] = &[{}];",
        names.join(", ")
    )
    .unwrap();
}

fn to_const_name(name: &str) -> String {
    let mut result = String::new();
    let mut prev_was_sep = true;

    for c in name.chars() {
        if c.is_alphanumeric() {
            if prev_was_sep {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c);
            }
            prev_was_sep = false;
        } else {
            if !result.is_empty() && !prev_was_sep {
                result.push('_');
            }
            prev_was_sep = true;
        }
    }
    // remove trailing underscore
    while result.ends_with('_') {
        result.pop();
    }

    format!("LANG_{result}")
}

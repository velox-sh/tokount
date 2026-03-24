use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
struct LangDef {
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(default)]
    filenames: Vec<String>,
    #[serde(default)]
    env: Vec<String>,
    #[serde(default)]
    shebangs: Vec<String>,
    default_mode: String,
    #[serde(default)]
    structured_format: Option<String>,
    #[serde(default)]
    leading_doc_prefixes: Vec<String>,
    #[serde(default)]
    regions: Vec<RegionDef>,
}

#[derive(Deserialize)]
struct RegionDef {
    open: String,
    close: String,
    kind: String,
    #[serde(default)]
    nested: bool,
    #[serde(default)]
    close_line_is_code: bool,
    #[serde(default = "default_true")]
    escape: bool,
    #[serde(default)]
    raw: bool,
    #[serde(default, alias = "default")]
    default_lang: Option<String>,
    #[serde(default)]
    detect: Option<String>,
}

#[derive(Clone)]
enum DetectionDef {
    Fence,
    Tag,
    Fixed,
}

#[derive(Clone)]
enum RegionKindDef {
    Comment {
        nested: bool,
        close_line_is_code: bool,
    },
    String {
        escape: bool,
    },
    Child {
        default_lang: Option<String>,
        detect: DetectionDef,
    },
}

#[derive(Clone)]
struct RegionEmit {
    open: String,
    close: String,
    kind: RegionKindDef,
}

fn default_true() -> bool {
    true
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

fn emit_phf_map(
    out: &mut fs::File,
    static_name: &str,
    entries: &[(String, String)],
) -> std::io::Result<()> {
    let mut map = phf_codegen::Map::new();
    for (k, v) in entries {
        map.entry(k.as_str(), v.as_str());
    }
    writeln!(
        out,
        "pub(super) static {static_name}: phf::Map<&'static str, &'static LanguageDef> = {};",
        map.build()
    )
}

#[inline]
fn insert_first_byte(set: &mut std::collections::BTreeSet<u8>, token: &str) {
    if let Some(b) = token.bytes().next() {
        set.insert(b);
    }
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

    while result.ends_with('_') {
        result.pop();
    }

    format!("LANG_{result}")
}

fn parse_default_mode(lang: &LangDef) -> &'static str {
    if lang.default_mode.eq_ignore_ascii_case("comment") {
        "DefaultMode::Comment"
    } else {
        "DefaultMode::Code"
    }
}

fn parse_structured_format(lang: &LangDef) -> &'static str {
    match lang.structured_format.as_deref() {
        Some(s) if s.eq_ignore_ascii_case("jupyter") => "StructuredFormat::Jupyter",
        _ => "StructuredFormat::None",
    }
}

fn emit_byte_str_list(list: &[String]) -> String {
    if list.is_empty() {
        return "&[]".to_string();
    }

    let entries = list
        .iter()
        .map(|s| format!("b\"{}\"", escape_bytes(s)))
        .collect::<Vec<_>>()
        .join(", ");

    format!("&[{entries}]")
}

fn parse_detect(v: Option<&str>) -> DetectionDef {
    match v {
        Some(s) if s.eq_ignore_ascii_case("fence") => DetectionDef::Fence,
        Some(s) if s.eq_ignore_ascii_case("tag") => DetectionDef::Tag,
        _ => DetectionDef::Fixed,
    }
}

fn regions_from_new(regions: &[RegionDef]) -> Vec<RegionEmit> {
    regions
        .iter()
        .map(|r| {
            let kind = if r.kind.eq_ignore_ascii_case("comment") {
                RegionKindDef::Comment {
                    nested: r.nested,
                    close_line_is_code: r.close_line_is_code,
                }
            } else if r.kind.eq_ignore_ascii_case("string") {
                let escape = if r.raw { false } else { r.escape };
                RegionKindDef::String { escape }
            } else {
                RegionKindDef::Child {
                    default_lang: r.default_lang.clone(),
                    detect: parse_detect(r.detect.as_deref()),
                }
            };

            RegionEmit {
                open: r.open.clone(),
                close: r.close.clone(),
                kind,
            }
        })
        .collect()
}

fn region_expr(region: &RegionEmit) -> String {
    let kind_expr = match &region.kind {
        RegionKindDef::Comment {
            nested,
            close_line_is_code,
        } => format!(
            "RegionKind::Comment {{ nested: {nested}, close_line_is_code: {close_line_is_code} }}"
        ),
        RegionKindDef::String { escape } => format!("RegionKind::String {{ escape: {escape} }}"),
        RegionKindDef::Child {
            default_lang,
            detect,
        } => {
            let detect_expr = match detect {
                DetectionDef::Fence => "Detection::Fence",
                DetectionDef::Tag => "Detection::Tag",
                DetectionDef::Fixed => "Detection::Fixed",
            };
            let default_expr = default_lang.as_deref().map_or_else(
                || "None".to_string(),
                |s| format!("Some(\"{}\")", s.replace('"', "\\\"")),
            );
            format!("RegionKind::Child {{ default_lang: {default_expr}, detect: {detect_expr} }}")
        }
    };

    format!(
        "Region {{ open: b\"{}\", close: b\"{}\", kind: {kind_expr} }}",
        escape_bytes(&region.open),
        escape_bytes(&region.close)
    )
}

fn build_lang_const_names(
    langs: &BTreeMap<String, LangDef>,
) -> std::collections::HashMap<String, String> {
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

    lang_const_names
}

fn write_language_defs(
    out: &mut fs::File,
    langs: &BTreeMap<String, LangDef>,
    lang_const_names: &std::collections::HashMap<String, String>,
) -> std::io::Result<()> {
    for (name, lang) in langs {
        let const_name = lang_const_names[name].clone();

        let mut regions = regions_from_new(&lang.regions);
        regions.sort_by_key(|r| std::cmp::Reverse(r.open.len()));

        let region_exprs: Vec<String> = regions.iter().map(region_expr).collect();
        let regions_str = format!("&[{}]", region_exprs.join(", "));

        let mut interesting: std::collections::BTreeSet<u8> = std::collections::BTreeSet::new();
        interesting.insert(b'\n');
        for region in &regions {
            insert_first_byte(&mut interesting, &region.open);
        }

        let mask_str = {
            let vals: Vec<String> = interesting
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            format!("&[{}u8]", vals.join(", "))
        };

        let default_mode = parse_default_mode(lang);
        let structured_format = parse_structured_format(lang);
        let leading_doc_prefixes = emit_byte_str_list(&lang.leading_doc_prefixes);

        writeln!(
            out,
            "static {const_name}: LanguageDef = LanguageDef {{\n\
            	name: \"{name}\",\n\
            	regions: {regions_str},\n\
            	interest_bytes: {mask_str},\n\
            	default_mode: {default_mode},\n\
             	structured_format: {structured_format},\n\
             	leading_doc_prefixes: {leading_doc_prefixes},\n\
            }};\n",
        )?;
    }

    Ok(())
}

fn build_extension_entries(
    langs: &BTreeMap<String, LangDef>,
    lang_const_names: &std::collections::HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut seen_exts = std::collections::HashSet::new();
    let mut ext_entries: Vec<(String, String)> = Vec::new();

    for (name, lang) in langs {
        let const_name = lang_const_names[name].clone();
        for ext in &lang.extensions {
            let ext_lower = ext.to_lowercase();

            if seen_exts.insert(ext_lower.clone()) {
                ext_entries.push((ext_lower, format!("&{const_name}")));
            }
        }
    }

    ext_entries
}

fn build_filename_entries(
    langs: &BTreeMap<String, LangDef>,
    lang_const_names: &std::collections::HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut seen_fns = std::collections::HashSet::new();
    let mut fn_entries: Vec<(String, String)> = Vec::new();

    for (name, lang) in langs {
        let const_name = lang_const_names[name].clone();
        for fname in &lang.filenames {
            let fname_lower = fname.to_lowercase();
            if seen_fns.insert(fname_lower.clone()) {
                fn_entries.push((fname_lower, format!("&{const_name}")));
            }
        }
    }

    fn_entries
}

fn build_shebang_entries(
    langs: &BTreeMap<String, LangDef>,
    lang_const_names: &std::collections::HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut seen_envs = std::collections::HashSet::new();
    let mut env_entries: Vec<(String, String)> = Vec::new();

    for (name, lang) in langs {
        let const_name = lang_const_names[name].clone();

        for interp in &lang.env {
            if seen_envs.insert(interp.clone()) {
                env_entries.push((interp.clone(), format!("&{const_name}")));
            }
        }

        for shebang in &lang.shebangs {
            let rest = shebang.strip_prefix("#!").unwrap_or(shebang);
            let first_word = rest.split_whitespace().next().unwrap_or("");
            let basename = first_word.rsplit('/').next().unwrap_or(first_word);

            if !basename.is_empty() && seen_envs.insert(basename.to_string()) {
                env_entries.push((basename.to_string(), format!("&{const_name}")));
            }
        }
    }

    env_entries
}

fn build_name_entries(
    langs: &BTreeMap<String, LangDef>,
    lang_const_names: &std::collections::HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut name_entries: Vec<(String, String)> = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    for name in langs.keys() {
        let const_name = lang_const_names[name].clone();
        let lower = name.to_lowercase();
        if seen_names.insert(lower.clone()) {
            name_entries.push((lower, format!("&{const_name}")));
        }
    }

    name_entries
}

fn load_languages(json_path: &Path) -> BTreeMap<String, LangDef> {
    let json_raw = fs::read_to_string(json_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", json_path.display()));
    let langs: BTreeMap<String, LangDef> = serde_json::from_str(&json_raw)
        .unwrap_or_else(|e| panic!("failed to parse {} as JSON: {e}", json_path.display()));

    langs
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let json_path = Path::new(&manifest_dir).join("languages.json");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest = Path::new(&out_dir).join("generated_languages.rs");

    println!("cargo:rerun-if-changed=languages.json");

    let langs = load_languages(&json_path);

    let mut out = fs::File::create(&dest).expect("failed to create output file");
    let lang_const_names = build_lang_const_names(&langs);

    write_language_defs(&mut out, &langs, &lang_const_names)
        .expect("failed to write generated language defs");

    let ext_entries = build_extension_entries(&langs, &lang_const_names);

    emit_phf_map(&mut out, "EXTENSION_MAP", &ext_entries).unwrap();
    let fn_entries = build_filename_entries(&langs, &lang_const_names);

    emit_phf_map(&mut out, "FILENAME_MAP", &fn_entries).unwrap();
    let env_entries = build_shebang_entries(&langs, &lang_const_names);

    emit_phf_map(&mut out, "SHEBANG_MAP", &env_entries).unwrap();
    let name_entries = build_name_entries(&langs, &lang_const_names);

    emit_phf_map(&mut out, "NAME_MAP", &name_entries).unwrap();

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

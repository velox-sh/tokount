/// Static definition of a language's syntax, generated at build time from `languages.json`
pub struct LanguageDef {
    /// Display name (e.g. `"Rust"`, `"Python"`)
    pub name: &'static str,
    /// Line comment prefixes, longest-first (e.g. `["//", "#"]`)
    pub line_comments: &'static [&'static [u8]],
    /// Block comment `(open, close)` pairs, longest-first (e.g. `[("/*", "*/")]`)
    pub block_comments: &'static [(&'static [u8], &'static [u8])],
    /// String literal `(open, close, raw)` triples; `raw = true`
    /// means backslash escapes are ignored
    pub string_literals: &'static [(&'static [u8], &'static [u8], bool)],
    /// Whether block comments can be nested (e.g. Kotlin `/* /* */ */`)
    pub nested_comments: bool,
    /// When a block comment closes mid-line on a continuation line, upgrade
    /// the line to Code if alphanumeric content follows the close delimiter
    /// (e.g. `=end DESCRIPTION` → Code).
    pub close_line_is_code: bool,
    /// Deduplicated first-bytes of all tokens used by the scanner to skip uninteresting bytes
    pub interest_bytes: &'static [u8],
    /// All non-blank lines outside code fences are comments (e.g. Plain Text, Markdown)
    pub literate: bool,
    /// Markers that toggle between literate-comment and code mode (e.g. `["```"]` for Markdown)
    pub important_syntax: &'static [&'static [u8]],
}

#[allow(dead_code, non_upper_case_globals, unused, non_snake_case)]
mod generated {
    use super::LanguageDef;
    include!(concat!(env!("OUT_DIR"), "/generated_languages.rs"));
}

impl LanguageDef {
    pub fn from_extension(ext: &str) -> Option<&'static LanguageDef> {
        generated::EXTENSION_MAP.get(ext).copied()
    }

    pub fn from_filename(name: &str) -> Option<&'static LanguageDef> {
        generated::FILENAME_MAP.get(name).copied()
    }

    /// Detect language from a shebang line (e.g. `#!/usr/bin/env ruby`)
    pub fn from_shebang(first_line: &str) -> Option<&'static LanguageDef> {
        let rest = first_line.strip_prefix("#!")?;
        let mut words = rest.split_whitespace();
        let first = words.next()?;
        let basename = first.rsplit('/').next()?;
        // #!/usr/bin/env <interpreter> (the real name is the next token)
        let name = if basename == "env" {
            words.next()?
        } else {
            basename
        };
        generated::SHEBANG_MAP.get(name).copied()
    }

    pub fn all_names() -> &'static [&'static str] {
        generated::LANGUAGE_NAMES
    }

    /// Look up a language by its display name (case-insensitive)
    pub fn from_name(name: &str) -> Option<&'static LanguageDef> {
        // avoid allocation for the common case where name is already lowercase
        if name.bytes().any(|b| b.is_ascii_uppercase()) {
            generated::NAME_MAP
                .get(name.to_ascii_lowercase().as_str())
                .copied()
        } else {
            generated::NAME_MAP.get(name).copied()
        }
    }
}

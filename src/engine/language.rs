pub struct LanguageDef {
    pub name: &'static str,
    pub line_comments: &'static [&'static [u8]],
    pub block_comments: &'static [(&'static [u8], &'static [u8])],
    pub string_literals: &'static [(&'static [u8], &'static [u8], bool)],
    pub nested_comments: bool,
    /// when a block comment closes mid-line on a continuation line, upgrade
    /// the line to Code if alphanumeric content follows the close delimiter
    /// (e.g. `=end DESCRIPTION` -> Code). Default false (tokei-like behavior)
    pub close_line_is_code: bool,
    /// bytes that could start any token (comment/string opener, newline)
    pub interest_bytes: &'static [u8],
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
}

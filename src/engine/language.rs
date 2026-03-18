#[allow(dead_code)]
pub struct LanguageDef {
    pub name: &'static str,
    pub line_comments: &'static [&'static [u8]],
    pub block_comments: &'static [(&'static [u8], &'static [u8])],
    pub string_literals: &'static [(&'static [u8], &'static [u8], bool)],
    pub nested_comments: bool,
    /// precomputed: true for bytes that could start any token
    /// (comment, string, newline, backslash)
    pub interest_mask: [bool; 256],
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

    pub fn all_names() -> &'static [&'static str] {
        generated::LANGUAGE_NAMES
    }
}

/// Static definition of a language's syntax, generated at build time from `languages.json`
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Detection {
    Fence,
    Tag,
    Fixed,
}

#[derive(Clone, Copy)]
pub enum RegionKind {
    /// Lines are comments, `close = b"\n"` for line comments
    Comment {
        nested: bool,
        close_line_is_code: bool,
    },
    /// Lines are code, delimiter content is opaque
    String { escape: bool },
    /// Switch to a child language until `close`
    Child {
        default_lang: Option<&'static str>,
        detect: Detection,
    },
}

#[derive(Clone, Copy)]
pub struct Region {
    pub open: &'static [u8],
    pub close: &'static [u8],
    pub kind: RegionKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DefaultMode {
    Code,
    Comment,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StructuredFormat {
    None,
    Jupyter,
}

pub struct LanguageDef {
    pub name: &'static str,
    pub regions: &'static [Region],
    pub interest_bytes: &'static [u8],
    pub default_mode: DefaultMode,
    pub structured_format: StructuredFormat,
    pub leading_doc_prefixes: &'static [&'static [u8]],
}

#[allow(dead_code, non_upper_case_globals, unused, non_snake_case)]
mod generated {
    use super::DefaultMode;
    use super::Detection;
    use super::LanguageDef;
    use super::Region;
    use super::RegionKind;
    use super::StructuredFormat;
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

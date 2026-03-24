use crate::engine::language::Detection;
use crate::engine::language::LanguageDef;

/// Result of counting a single file, including any embedded child language blocks
pub struct FileResult {
    pub counts: LineCounts,
    /// Embedded child language blocks: `(language_name, counts)`
    pub children: Vec<(&'static str, LineCounts)>,
}

/// Line counts for a single file, broken down by classification
#[derive(Copy, Clone)]
pub struct LineCounts {
    /// Lines containing code (possibly also containing a comment)
    pub code: u32,
    /// Lines containing only comments (no code)
    pub comment: u32,
    /// Empty or whitespace-only lines
    pub blank: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum LineType {
    Blank,
    Code,
    Comment,
}

// context is stored in the variant -> no separate `block_open`, `string_close` etc.
#[derive(Clone, Copy)]
pub(super) enum ParseState {
    Normal,
    LineComment,
    BlockComment {
        depth: u8,
        open: &'static [u8],
        close: &'static [u8],
        nested: bool,
        close_line_is_code: bool,
    },
    InString {
        close: &'static [u8],
        escape: bool,
    },
    InChild {
        close: &'static [u8],
        child_lang: Option<&'static LanguageDef>,
    },
}

pub(super) enum TokenMatch {
    LineComment,
    BlockComment {
        open: &'static [u8],
        close: &'static [u8],
        nested: bool,
        close_line_is_code: bool,
    },
    StringLiteral {
        close: &'static [u8],
        escape: bool,
    },
    Child {
        close: &'static [u8],
        default_lang: Option<&'static str>,
        detect: Detection,
    },
    Other,
}

use crate::engine::language::Detection;
use crate::engine::language::LanguageDef;

/// Result of counting a single file, including any embedded child language blocks
pub struct FileResult {
    pub counts: LineCounts,
    pub children: Vec<(&'static str, LineCounts)>,
}

/// Line counts for a single file, broken down by classification
#[derive(Copy, Clone)]
pub struct LineCounts {
    pub code: u32,
    pub comment: u32,
    pub blank: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum LineType {
    Blank,
    Code,
    Comment,
}

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

use super::types::LineCounts;
use super::types::LineType;
use super::types::ParseState;

#[inline(always)]
pub(super) fn emit(counts: &mut LineCounts, line_type: LineType, parse: ParseState) -> LineType {
    match line_type {
        LineType::Blank => counts.blank += 1,
        LineType::Code => counts.code += 1,
        LineType::Comment => counts.comment += 1,
    }

    // if we're still inside a block comment, the next line opens as Comment
    match parse {
        ParseState::BlockComment { .. } => LineType::Comment,
        _ => LineType::Blank,
    }
}

// Comment -> Code when block opened+closed on same line, or `close_line_is_code` (Raku `=end`),
// but only if alphanumeric content is present, orphaned `*/` must not trigger this
#[inline(always)]
pub(super) fn classify_prefix(
    prefix: &[u8],
    line_type: LineType,
    block_started_this_line: bool,
    close_line_is_code: bool,
    default_is_comment: bool,
) -> LineType {
    match line_type {
        LineType::Blank => {
            if prefix.iter().any(|&c| !matches!(c, b' ' | b'\t' | b'\r')) {
                if default_is_comment {
                    LineType::Comment
                } else {
                    LineType::Code
                }
            } else {
                LineType::Blank
            }
        }
        LineType::Comment if block_started_this_line || close_line_is_code => {
            if prefix
                .iter()
                .any(|&c| c.is_ascii_alphanumeric() || c == b'_')
            {
                LineType::Code
            } else {
                LineType::Comment
            }
        }
        _ => line_type,
    }
}

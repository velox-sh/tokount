use super::parser::count_file;
use super::types::FileResult;
use super::types::LineCounts;
use crate::engine::language::LanguageDef;
use crate::engine::scanner;

#[inline(always)]
fn has_non_ws(line: &[u8]) -> bool {
    line.iter().any(|&b| b != b' ' && b != b'\t' && b != b'\r')
}

#[inline(always)]
fn zero_counts() -> LineCounts {
    LineCounts {
        code: 0,
        comment: 0,
        blank: 0,
    }
}

#[inline(always)]
fn merge_counts(into: &mut LineCounts, from: LineCounts) {
    into.code += from.code;
    into.comment += from.comment;
    into.blank += from.blank;
}

#[inline(always)]
fn is_closing_fence(line: &[u8], fence_marker: &[u8]) -> bool {
    line.starts_with(fence_marker)
        && line[fence_marker.len()..]
            .iter()
            .all(|&b| b == b' ' || b == b'\t' || b == b'\r')
}

#[inline(always)]
fn fence_language(line: &[u8], fence_marker: &[u8]) -> Option<&'static LanguageDef> {
    let after = &line[fence_marker.len()..];
    let ident = after
        .iter()
        .position(|&b| b == b' ' || b == b'\t' || b == b'\r')
        .map_or(after, |i| &after[..i]);

    if ident.is_empty() {
        return None;
    }

    std::str::from_utf8(ident)
        .ok()
        .and_then(|s| LanguageDef::from_extension(s).or_else(|| LanguageDef::from_name(s)))
}

#[inline(always)]
fn push_fence_line(buf: &mut Vec<u8>, line: &[u8], has_newline: bool) {
    buf.extend_from_slice(line);
    if has_newline {
        buf.push(b'\n');
    }
}

/// Count lines in a literate file, treating all non-blank lines as comments
pub(super) fn count_pure_literate(content: &[u8]) -> LineCounts {
    let mut counts = zero_counts();
    let mut pos = 0;

    while pos < content.len() {
        match scanner::find_newline(&content[pos..]) {
            Some(i) => {
                let line = &content[pos..pos + i];
                if has_non_ws(line) {
                    counts.comment += 1;
                } else {
                    counts.blank += 1;
                }
                pos += i + 1;
            }

            None => {
                let line = &content[pos..];
                if !line.is_empty() {
                    if has_non_ws(line) {
                        counts.comment += 1;
                    } else {
                        counts.blank += 1;
                    }
                }
                break;
            }
        }
    }

    counts
}

/// Count lines in a literate file, treating fenced blocks as child languages if possible
pub(super) fn count_literate_file(content: &[u8], lang: &LanguageDef) -> FileResult {
    let mut counts = zero_counts();
    let mut children: Vec<(&'static str, LineCounts)> = Vec::new();

    let mut fence_buf: Vec<u8> = Vec::new();
    let mut in_fence = false;
    let mut fence_lang: Option<&'static LanguageDef> = None;
    let mut pos = 0;

    let fence_marker: &[u8] = lang.important_syntax.first().copied().unwrap_or(b"```");

    while pos < content.len() {
        let line_end = scanner::find_newline(&content[pos..]).map_or(content.len(), |i| pos + i);
        let line = &content[pos..line_end];
        let has_newline = line_end < content.len();

        if in_fence {
            if is_closing_fence(line, fence_marker) {
                counts.comment += 1;

                if let Some(child) = fence_lang {
                    let child_result = count_file(&fence_buf, child);
                    children.push((child.name, child_result.counts));
                    children.extend(child_result.children);
                } else {
                    // unknown child lang: count fenced lines as parent code
                    merge_counts(&mut counts, count_pure_code(&fence_buf));
                }

                fence_buf.clear();
                fence_lang = None;
                in_fence = false;
            } else {
                // include newline so line counts stay correct
                push_fence_line(&mut fence_buf, line, has_newline);
            }
        } else if line.starts_with(fence_marker) {
            counts.comment += 1;

            fence_lang = fence_language(line, fence_marker);
            in_fence = true;
        } else if has_non_ws(line) {
            counts.comment += 1;
        } else {
            counts.blank += 1;
        }

        pos = line_end + usize::from(has_newline);
    }

    // unclosed fence: flush remaining content as parent code
    if in_fence && !fence_buf.is_empty() {
        let fenced = count_pure_code(&fence_buf);
        counts.code += fenced.code;
        counts.blank += fenced.blank;
    }

    FileResult { counts, children }
}

/// Count non-blank lines as code (used for fenced blocks with unknown child language)
pub(super) fn count_pure_code(content: &[u8]) -> LineCounts {
    let mut counts = zero_counts();
    let mut pos = 0;

    while pos < content.len() {
        let end = scanner::find_newline(&content[pos..]).map_or(content.len(), |i| pos + i);
        let line = &content[pos..end];

        if has_non_ws(line) {
            counts.code += 1;
        } else if !line.is_empty() || end < content.len() {
            counts.blank += 1;
        }
        pos = end + if end < content.len() { 1 } else { 0 };
    }

    counts
}

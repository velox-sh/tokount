use crate::engine::language::LanguageDef;
use crate::engine::scanner;

/// Result of counting a single file, including any embedded child language blocks
pub struct FileResult {
    pub counts: LineCounts,
    /// Embedded child language blocks: `(language_name, counts)`
    pub children: Vec<(&'static str, LineCounts)>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LineType {
    Blank,
    Code,
    Comment,
}

// context is stored in the variant -> no separate `block_open`, `string_close` etc.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ParseState {
    Normal,
    LineComment,
    BlockComment {
        depth: u8,
        open: &'static [u8],
        close: &'static [u8],
    },
    InString {
        close: &'static [u8],
        raw: bool,
    },
}

enum TokenMatch {
    LineComment,
    BlockComment {
        open: &'static [u8],
        close: &'static [u8],
    },
    StringLiteral {
        close: &'static [u8],
        raw: bool,
    },
    Other,
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

// longest-match: longer opener beats shorter prefix even across token types
// (e.g. `////` block comment wins over `//` line comment in AsciiDoc)
#[inline(always)]
fn match_token(rest: &[u8], lang: &LanguageDef) -> (TokenMatch, usize) {
    let mut best: Option<TokenMatch> = None;
    let mut best_len = 0usize;

    for &lc in lang.line_comments {
        if lc.len() > best_len && rest.starts_with(lc) {
            best = Some(TokenMatch::LineComment);
            best_len = lc.len();
        }
    }

    for &(open, close) in lang.block_comments {
        if open.len() > best_len && rest.starts_with(open) {
            best = Some(TokenMatch::BlockComment { open, close });
            best_len = open.len();
        }
    }

    for &(open, close, raw) in lang.string_literals {
        if open.len() > best_len && rest.starts_with(open) {
            best = Some(TokenMatch::StringLiteral { close, raw });
            best_len = open.len();
        }
    }

    match best {
        Some(m) => (m, best_len),
        None => (TokenMatch::Other, 1),
    }
}

#[inline(always)]
fn emit(counts: &mut LineCounts, line_type: LineType, parse: ParseState) -> LineType {
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
// but only if alphanumeric content is present — orphaned `*/` must not trigger this
#[inline(always)]
fn classify_prefix(
    prefix: &[u8],
    line_type: LineType,
    block_started_this_line: bool,
    close_line_is_code: bool,
) -> LineType {
    match line_type {
        LineType::Blank => {
            if prefix.iter().any(|&c| !matches!(c, b' ' | b'\t' | b'\r')) {
                LineType::Code
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

/// Pure literate: every non-blank line is a comment (e.g. Plain Text)
/// No syntax tokens, no code possible
fn count_pure_literate(content: &[u8]) -> LineCounts {
    let mut counts = LineCounts {
        code: 0,
        comment: 0,
        blank: 0,
    };
    let mut line_has_content = false;
    let mut pos = 0;

    while pos < content.len() {
        match scanner::find_newline(&content[pos..]) {
            Some(i) => {
                let line = &content[pos..pos + i];
                if line_has_content || line.iter().any(|&b| b != b' ' && b != b'\t' && b != b'\r') {
                    counts.comment += 1;
                } else {
                    counts.blank += 1;
                }
                line_has_content = false;
                pos += i + 1;
            }

            None => {
                let line = &content[pos..];
                if !line.is_empty() {
                    if line.iter().any(|&b| b != b' ' && b != b'\t' && b != b'\r') {
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

/// Literate with code fences (e.g. Markdown, MDX, Djot)
/// Lines outside fences are comments (or blank); lines inside fences belong to the child language
/// Fence delimiter lines (``` or ```rust) count as parent comments
fn count_literate_file(content: &[u8], lang: &LanguageDef) -> FileResult {
    let mut counts = LineCounts {
        code: 0,
        comment: 0,
        blank: 0,
    };
    let mut children: Vec<(&'static str, LineCounts)> = Vec::new();

    // reusable buffer for fenced block content
    let mut fence_buf: Vec<u8> = Vec::new();
    let mut in_fence = false;
    let mut fence_lang: Option<&'static LanguageDef> = None;
    let mut pos = 0;

    // check if a line starts with an important_syntax marker (e.g. ```)
    let fence_marker: &[u8] = lang.important_syntax.first().copied().unwrap_or(b"```");

    while pos < content.len() {
        let line_end = scanner::find_newline(&content[pos..]).map_or(content.len(), |i| pos + i);

        let line = &content[pos..line_end];
        let has_newline = line_end < content.len();

        if in_fence {
            // check for closing fence
            if line.starts_with(fence_marker)
                && line[fence_marker.len()..]
                    .iter()
                    .all(|&b| b == b' ' || b == b'\t' || b == b'\r')
            {
                // closing fence line — parent comment
                counts.comment += 1;
                // count fenced content under child language
                if let Some(child) = fence_lang {
                    let child_result = count_file(&fence_buf, child);
                    children.push((child.name, child_result.counts));
                    // merge any deeper children (rare but possible)
                    children.extend(child_result.children);
                } else {
                    // unknown child lang: count fenced lines as parent code
                    let fenced = count_pure_code(&fence_buf);
                    counts.code += fenced.code;
                    counts.comment += fenced.comment;
                    counts.blank += fenced.blank;
                }
                fence_buf.clear();
                fence_lang = None;
                in_fence = false;
            } else {
                // accumulate fenced content (include the newline so line counts are right)
                fence_buf.extend_from_slice(line);
                if has_newline {
                    fence_buf.push(b'\n');
                }
            }
        } else {
            // check for opening fence
            if line.starts_with(fence_marker) {
                // opening fence line — parent comment
                counts.comment += 1;
                // extract language identifier after the fence marker
                let after = &line[fence_marker.len()..];
                let ident = after
                    .iter()
                    .position(|&b| b == b' ' || b == b'\t' || b == b'\r')
                    .map_or(after, |i| &after[..i]);
                // try to find the child language: first by extension, then by name
                fence_lang = if ident.is_empty() {
                    None
                } else if let Ok(s) = std::str::from_utf8(ident) {
                    LanguageDef::from_extension(s).or_else(|| LanguageDef::from_name(s))
                } else {
                    None
                };
                in_fence = true;
            } else if line.iter().any(|&b| b != b' ' && b != b'\t' && b != b'\r') {
                counts.comment += 1;
            } else {
                counts.blank += 1;
            }
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
fn count_pure_code(content: &[u8]) -> LineCounts {
    let mut counts = LineCounts {
        code: 0,
        comment: 0,
        blank: 0,
    };
    let mut pos = 0;

    while pos < content.len() {
        let end = scanner::find_newline(&content[pos..]).map_or(content.len(), |i| pos + i);
        let line = &content[pos..end];
        if line.iter().any(|&b| b != b' ' && b != b'\t' && b != b'\r') {
            counts.code += 1;
        } else if !line.is_empty() || end < content.len() {
            counts.blank += 1;
        }
        pos = end + if end < content.len() { 1 } else { 0 };
    }

    counts
}

#[expect(clippy::cognitive_complexity, clippy::too_many_lines)]
pub fn count_file(content: &[u8], lang: &LanguageDef) -> FileResult {
    // literate languages have their own fast paths
    if lang.literate {
        return if lang.important_syntax.is_empty() {
            FileResult {
                counts: count_pure_literate(content),
                children: Vec::new(),
            }
        } else {
            count_literate_file(content, lang)
        };
    }

    let mut counts = LineCounts {
        code: 0,
        comment: 0,
        blank: 0,
    };

    if content.is_empty() {
        return FileResult {
            counts,
            children: Vec::new(),
        };
    }

    let mut parse = ParseState::Normal;
    let mut line_type = LineType::Blank;
    let mut pos = 0;
    // true when a block comment opened in Normal state on the current line
    // (not a continuation from a previous line)
    let mut block_started_this_line = false;

    while pos < content.len() {
        let bytes = &content[pos..];

        match parse {
            ParseState::LineComment => match scanner::find_newline(bytes) {
                Some(nl) => {
                    line_type = emit(&mut counts, line_type, parse);
                    block_started_this_line = false;
                    parse = ParseState::Normal;
                    pos += nl + 1;
                }
                None => {
                    emit(&mut counts, line_type, parse);
                    return FileResult {
                        counts,
                        children: Vec::new(),
                    };
                }
            },

            ParseState::BlockComment { depth, open, close } => {
                if lang.nested_comments {
                    let open_first = open.first().copied().unwrap_or(b'/');
                    let close_first = close.first().copied().unwrap_or(b'*');

                    match scanner::find_nested_block(bytes, open_first, close_first) {
                        Some(i) => {
                            if bytes[i] == b'\n' {
                                line_type = emit(&mut counts, line_type, parse);
                                block_started_this_line = false;
                                pos += i + 1;
                            } else {
                                let rest = &content[pos + i..];
                                if rest.starts_with(close) {
                                    parse = if depth > 1 {
                                        ParseState::BlockComment {
                                            depth: depth - 1,
                                            open,
                                            close,
                                        }
                                    } else {
                                        ParseState::Normal
                                    };
                                    pos += i + close.len();
                                } else if rest.starts_with(open) {
                                    parse = ParseState::BlockComment {
                                        depth: depth.saturating_add(1),
                                        open,
                                        close,
                                    };
                                    pos += i + open.len();
                                } else {
                                    pos += i + 1;
                                }
                            }
                        }

                        None => {
                            emit(&mut counts, line_type, parse);
                            return FileResult {
                                counts,
                                children: Vec::new(),
                            };
                        }
                    }
                } else {
                    let close_first = close.first().copied().unwrap_or(b'*');

                    match scanner::find_newline_or(bytes, close_first) {
                        Some(i) => {
                            if bytes[i] == b'\n' {
                                line_type = emit(&mut counts, line_type, parse);
                                block_started_this_line = false;
                                pos += i + 1;
                            } else {
                                let rest = &content[pos + i..];
                                if rest.starts_with(close) {
                                    parse = ParseState::Normal;
                                    pos += i + close.len();
                                } else {
                                    // false positive: close_first matched but not the full
                                    // delimiter
                                    pos += i + 1;
                                }
                            }
                        }

                        None => {
                            emit(&mut counts, line_type, parse);
                            return FileResult {
                                counts,
                                children: Vec::new(),
                            };
                        }
                    }
                }
            }

            ParseState::InString { close, raw } => {
                let close_first = close.first().copied().unwrap_or(b'"');
                let found = if raw {
                    scanner::find_string_end_no_escape(bytes, close_first)
                } else {
                    scanner::find_string_end(bytes, close_first)
                };

                match found {
                    Some(i) => {
                        if bytes[i] == b'\n' {
                            // multi-line string: both this line and the next are code
                            emit(&mut counts, LineType::Code, parse);
                            block_started_this_line = false;
                            line_type = LineType::Code;
                            pos += i + 1;
                        } else if bytes[i] == b'\\' {
                            // \<LF>: continuation line is still inside the string → code
                            if content.get(pos + i + 1).copied() == Some(b'\n') {
                                emit(&mut counts, LineType::Code, parse);
                                block_started_this_line = false;
                                line_type = LineType::Code;
                            }
                            pos += i + 2;
                        } else {
                            let rest = &content[pos + i..];
                            if rest.starts_with(close) {
                                parse = ParseState::Normal;
                                pos += i + close.len();
                            } else {
                                pos += i + 1;
                            }
                        }
                    }

                    None => {
                        counts.code += 1;
                        return FileResult {
                            counts,
                            children: Vec::new(),
                        };
                    }
                }
            }

            ParseState::Normal => {
                match scanner::find_interesting(bytes, lang.interest_bytes) {
                    Some(i) => {
                        if i > 0 {
                            line_type = classify_prefix(
                                &bytes[..i],
                                line_type,
                                block_started_this_line,
                                lang.close_line_is_code,
                            );
                        }

                        pos += i;

                        if bytes[i] == b'\n' {
                            line_type = emit(&mut counts, line_type, parse);
                            block_started_this_line = false;
                            pos += 1;
                            continue;
                        }

                        let rest = &content[pos..];
                        let (token, advance) = match_token(rest, lang);

                        match token {
                            TokenMatch::LineComment => {
                                if line_type == LineType::Blank {
                                    line_type = LineType::Comment;
                                }

                                parse = ParseState::LineComment;
                            }

                            TokenMatch::BlockComment { open, close } => {
                                if line_type == LineType::Blank {
                                    line_type = LineType::Comment;
                                }

                                block_started_this_line = true;
                                parse = ParseState::BlockComment {
                                    depth: 1,
                                    open,
                                    close,
                                };
                            }

                            TokenMatch::StringLiteral { close, raw } => {
                                line_type = LineType::Code;
                                parse = ParseState::InString { close, raw };
                            }

                            TokenMatch::Other => {
                                // only upgrade blank -> code; a comment line stays comment
                                // even if orphaned delimiters (e.g. `*/`) follow a block close
                                if line_type == LineType::Blank
                                    && !matches!(bytes[i], b' ' | b'\t' | b'\r')
                                {
                                    line_type = LineType::Code;
                                }
                            }
                        }

                        pos += advance;
                    }
                    None => {
                        // no more interesting bytes: classify any trailing content
                        line_type = classify_prefix(
                            bytes,
                            line_type,
                            block_started_this_line,
                            lang.close_line_is_code,
                        );

                        if !bytes.is_empty() {
                            emit(&mut counts, line_type, parse);
                        }

                        return FileResult {
                            counts,
                            children: Vec::new(),
                        };
                    }
                }
            }
        }
    }

    // file has no trailing newline: the final line was typed but never emitted
    // skip if the file ends with '\n' since that newline already emitted the last
    // real line and set line_type for a "next" line that doesn't exist
    if line_type != LineType::Blank && content.last().copied() != Some(b'\n') {
        emit(&mut counts, line_type, parse);
    }

    FileResult {
        counts,
        children: Vec::new(),
    }
}

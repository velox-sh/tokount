use crate::engine::language::LanguageDef;
use crate::engine::scanner;

/// What kind of content we've seen on the current line so far
#[derive(Clone, Copy, PartialEq, Eq)]
enum LineType {
    Blank,
    Code,
    Comment,
}

/// Top-level parser state
#[derive(Clone, Copy, PartialEq, Eq)]
enum ParseState {
    Normal,
    LineComment,
    /// depth: nesting level (1 = outermost); only >1 when lang.nested_comments
    BlockComment {
        depth: u8,
    },
    String,
}

/// Result of matching a token at the current position in Normal state
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
    /// non-whitespace byte that didn't start any token
    Other,
}

#[derive(Copy, Clone)]
pub struct LineCounts {
    pub code: u32,
    pub comment: u32,
    pub blank: u32,
}

/// Identify which token (if any) starts at `rest` for the given language
/// Returns the token kind and the number of bytes to advance past the opener
#[inline(always)]
fn match_token(rest: &[u8], lang: &LanguageDef) -> (TokenMatch, usize) {
    for &lc in lang.line_comments {
        if rest.starts_with(lc) {
            return (TokenMatch::LineComment, lc.len());
        }
    }
    for &(open, close) in lang.block_comments {
        if rest.starts_with(open) {
            return (TokenMatch::BlockComment { open, close }, open.len());
        }
    }
    for &(open, close, raw) in lang.string_literals {
        if rest.starts_with(open) {
            return (TokenMatch::StringLiteral { close, raw }, open.len());
        }
    }
    (TokenMatch::Other, 1)
}

/// Emit the current line into `counts`
/// Returns the initial LineType for the next line
#[inline(always)]
fn emit(counts: &mut LineCounts, line_type: LineType, parse: ParseState) -> LineType {
    match line_type {
        LineType::Blank => counts.blank += 1,
        LineType::Code => counts.code += 1,
        LineType::Comment => counts.comment += 1,
    }
    // continuing inside a block comment: next line opens as comment
    match parse {
        ParseState::BlockComment { .. } => LineType::Comment,
        _ => LineType::Blank,
    }
}

/// Count lines of code, comments, and blank lines for the given language
#[expect(clippy::cognitive_complexity)]
pub fn count_file(content: &[u8], lang: &LanguageDef) -> LineCounts {
    let mut counts = LineCounts {
        code: 0,
        comment: 0,
        blank: 0,
    };
    if content.is_empty() {
        return counts;
    }

    let mut parse = ParseState::Normal;
    let mut line_type = LineType::Blank;
    // close delimiters are &'static [u8] from LanguageDef
    let mut string_close: &[u8] = b"";
    let mut block_close: &[u8] = b"*/";
    let mut block_open: &[u8] = b"/*";
    let mut string_raw: bool = false;
    let mut pos = 0;

    while pos < content.len() {
        let bytes = &content[pos..];

        match parse {
            ParseState::LineComment => {
                match scanner::find_newline(bytes) {
                    Some(nl) => {
                        line_type = emit(&mut counts, line_type, parse);
                        parse = ParseState::Normal;
                        pos += nl + 1;
                    }
                    None => {
                        // EOF: count final line
                        emit(&mut counts, line_type, parse);
                        return counts;
                    }
                }
            }

            ParseState::BlockComment { depth } => {
                if lang.nested_comments {
                    let open_first = block_open.first().copied().unwrap_or(b'/');
                    let close_first = block_close.first().copied().unwrap_or(b'*');

                    match scanner::find_nested_block(bytes, open_first, close_first) {
                        Some(i) => {
                            if bytes[i] == b'\n' {
                                line_type = emit(&mut counts, line_type, parse);
                                pos += i + 1;
                            } else {
                                let rest = &content[pos + i..];
                                if rest.starts_with(block_close) {
                                    if depth > 1 {
                                        parse = ParseState::BlockComment { depth: depth - 1 };
                                    } else {
                                        parse = ParseState::Normal;
                                    }
                                    pos += i + block_close.len();
                                } else if rest.starts_with(block_open) {
                                    parse = ParseState::BlockComment {
                                        depth: depth.saturating_add(1),
                                    };
                                    pos += i + block_open.len();
                                } else {
                                    pos += i + 1;
                                }
                            }
                        }
                        None => {
                            emit(&mut counts, line_type, parse);
                            return counts;
                        }
                    }
                } else {
                    let close_first = block_close.first().copied().unwrap_or(b'*');
                    match scanner::find_newline_or(bytes, close_first) {
                        Some(i) => {
                            if bytes[i] == b'\n' {
                                line_type = emit(&mut counts, line_type, parse);
                                pos += i + 1;
                            } else {
                                let rest = &content[pos + i..];
                                if rest.starts_with(block_close) {
                                    parse = ParseState::Normal;
                                    pos += i + block_close.len();
                                } else {
                                    // false positive: close_first matched but not the full
                                    // delimiter
                                    pos += i + 1;
                                }
                            }
                        }
                        None => {
                            // EOF inside block comment
                            emit(&mut counts, line_type, parse);
                            return counts;
                        }
                    }
                }
            }

            ParseState::String => {
                let close_first = string_close.first().copied().unwrap_or(b'"');
                let found = if string_raw {
                    scanner::find_string_end_no_escape(bytes, close_first)
                } else {
                    scanner::find_string_end(bytes, close_first)
                };
                match found {
                    Some(i) => {
                        let b = bytes[i];
                        if b == b'\n' {
                            // multi-line string: both this line and the next are code
                            emit(&mut counts, LineType::Code, parse);
                            line_type = LineType::Code;
                            pos += i + 1;
                        } else if b == b'\\' {
                            // escape sequence: skip the escaped byte (only when !string_raw)
                            pos += i + 2;
                        } else {
                            let rest = &content[pos + i..];
                            if rest.starts_with(string_close) {
                                parse = ParseState::Normal;
                                pos += i + string_close.len();
                            } else {
                                pos += i + 1;
                            }
                        }
                    }
                    None => {
                        // EOF inside string
                        counts.code += 1;
                        return counts;
                    }
                }
            }

            ParseState::Normal => {
                match scanner::find_interesting(bytes, &lang.interest_mask) {
                    Some(i) => {
                        // bytes we skipped to reach the interesting byte may be code content
                        if i > 0 && line_type == LineType::Blank {
                            let has_nonws = bytes[..i]
                                .iter()
                                .any(|&c| !matches!(c, b' ' | b'\t' | b'\r'));
                            if has_nonws {
                                line_type = LineType::Code;
                            }
                        }

                        pos += i;

                        if bytes[i] == b'\n' {
                            line_type = emit(&mut counts, line_type, parse);
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
                                block_open = open;
                                block_close = close;
                                if line_type == LineType::Blank {
                                    line_type = LineType::Comment;
                                }
                                parse = ParseState::BlockComment { depth: 1 };
                            }
                            TokenMatch::StringLiteral { close, raw } => {
                                string_close = close;
                                string_raw = raw;
                                line_type = LineType::Code;
                                parse = ParseState::String;
                            }
                            TokenMatch::Other => {
                                if bytes[i] != b' ' && bytes[i] != b'\t' && bytes[i] != b'\r' {
                                    line_type = LineType::Code;
                                }
                            }
                        }

                        pos += advance;
                    }
                    None => {
                        // no more interesting bytes -> check for a trailing non-newline line
                        let has_nonws = bytes.iter().any(|&b| !matches!(b, b' ' | b'\t' | b'\r'));
                        if has_nonws {
                            line_type = LineType::Code;
                        }
                        if !bytes.is_empty() {
                            emit(&mut counts, line_type, parse);
                        }
                        return counts;
                    }
                }
            }
        }
    }

    counts
}

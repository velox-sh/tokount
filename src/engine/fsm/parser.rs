use super::classify::classify_prefix;
use super::classify::emit;
use super::literate::count_literate_file;
use super::literate::count_pure_literate;
use super::token::match_token;
use super::types::FileResult;
use super::types::LineCounts;
use super::types::LineType;
use super::types::ParseState;
use super::types::TokenMatch;
use crate::engine::language::LanguageDef;
use crate::engine::scanner;

#[inline(always)]
fn finish(counts: LineCounts) -> FileResult {
    FileResult {
        counts,
        children: Vec::new(),
    }
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
fn emit_line_and_reset(
    counts: &mut LineCounts,
    line_type: &mut LineType,
    parse: ParseState,
    block_started_this_line: &mut bool,
) {
    *line_type = emit(counts, *line_type, parse);
    *block_started_this_line = false;
}

#[inline(always)]
fn emit_code_line_and_reset(
    counts: &mut LineCounts,
    line_type: &mut LineType,
    parse: ParseState,
    block_started_this_line: &mut bool,
) {
    emit(counts, LineType::Code, parse);
    *line_type = LineType::Code;
    *block_started_this_line = false;
}

#[inline(always)]
fn promote_blank_to_comment(line_type: &mut LineType) {
    if *line_type == LineType::Blank {
        *line_type = LineType::Comment;
    }
}

#[inline(always)]
fn emit_and_finish(counts: &mut LineCounts, line_type: LineType, parse: ParseState) -> FileResult {
    emit(counts, line_type, parse);
    finish(*counts)
}

#[expect(clippy::cognitive_complexity, clippy::too_many_lines)]
pub fn count_file(content: &[u8], lang: &LanguageDef) -> FileResult {
    // literate languages have their own fast paths
    if lang.literate {
        return if lang.important_syntax.is_empty() {
            finish(count_pure_literate(content))
        } else {
            count_literate_file(content, lang)
        };
    }

    let mut counts = zero_counts();

    if content.is_empty() {
        return finish(counts);
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
                    emit_line_and_reset(
                        &mut counts,
                        &mut line_type,
                        parse,
                        &mut block_started_this_line,
                    );
                    parse = ParseState::Normal;
                    pos += nl + 1;
                }
                None => {
                    return emit_and_finish(&mut counts, line_type, parse);
                }
            },

            ParseState::BlockComment { depth, open, close } => {
                if lang.nested_comments {
                    let open_first = open.first().copied().unwrap_or(b'/');
                    let close_first = close.first().copied().unwrap_or(b'*');

                    match scanner::find_nested_block(bytes, open_first, close_first) {
                        Some(i) => {
                            if bytes[i] == b'\n' {
                                emit_line_and_reset(
                                    &mut counts,
                                    &mut line_type,
                                    parse,
                                    &mut block_started_this_line,
                                );
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
                            return emit_and_finish(&mut counts, line_type, parse);
                        }
                    }
                } else {
                    let close_first = close.first().copied().unwrap_or(b'*');

                    match scanner::find_newline_or(bytes, close_first) {
                        Some(i) => {
                            if bytes[i] == b'\n' {
                                emit_line_and_reset(
                                    &mut counts,
                                    &mut line_type,
                                    parse,
                                    &mut block_started_this_line,
                                );
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
                            return emit_and_finish(&mut counts, line_type, parse);
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
                            emit_code_line_and_reset(
                                &mut counts,
                                &mut line_type,
                                parse,
                                &mut block_started_this_line,
                            );
                            pos += i + 1;
                        } else if bytes[i] == b'\\' {
                            // <LF> continuation line is still inside the string -> code
                            if content.get(pos + i + 1).copied() == Some(b'\n') {
                                emit_code_line_and_reset(
                                    &mut counts,
                                    &mut line_type,
                                    parse,
                                    &mut block_started_this_line,
                                );
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
                        return finish(counts);
                    }
                }
            }

            ParseState::Normal => match scanner::find_interesting(bytes, lang.interest_bytes) {
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
                        emit_line_and_reset(
                            &mut counts,
                            &mut line_type,
                            parse,
                            &mut block_started_this_line,
                        );
                        pos += 1;
                        continue;
                    }

                    let rest = &content[pos..];
                    let (token, advance) = match_token(rest, lang);

                    match token {
                        TokenMatch::LineComment => {
                            promote_blank_to_comment(&mut line_type);

                            parse = ParseState::LineComment;
                        }

                        TokenMatch::BlockComment { open, close } => {
                            promote_blank_to_comment(&mut line_type);

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

                    return if bytes.is_empty() {
                        finish(counts)
                    } else {
                        emit_and_finish(&mut counts, line_type, parse)
                    };
                }
            },
        }
    }

    // file has no trailing newline: the final line was typed but never emitted
    // skip if the file ends with '\n' since that newline already emitted the last
    // real line and set line_type for a "next" line that doesn't exist
    if line_type != LineType::Blank && content.last().copied() != Some(b'\n') {
        return emit_and_finish(&mut counts, line_type, parse);
    }

    finish(counts)
}

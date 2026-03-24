use super::super::classify::classify_prefix;
use super::super::classify::emit;
use super::super::token::match_token;
use super::super::types::FileResult;
use super::super::types::LineCounts;
use super::super::types::LineType;
use super::super::types::ParseState;
use super::super::types::TokenMatch;
use super::helpers::ChildOpenResult;
use super::helpers::count_pure_code;
use super::helpers::default_line_type;
use super::helpers::detect_child_open;
use super::helpers::push_child_before_close;
use super::helpers::push_child_result;
use crate::engine::language::DefaultMode;
use crate::engine::language::Detection;
use crate::engine::language::LanguageDef;
use crate::engine::scanner;

enum StepOutcome {
    Continue,
    Break,
    Return(FileResult),
}

pub(super) struct Parser<'a> {
    content: &'a [u8],
    lang: &'a LanguageDef,
    counts: LineCounts,
    children: Vec<(&'static str, LineCounts)>,
    parse: ParseState,
    line_type: LineType,
    pos: usize,
    block_started_this_line: bool,
    close_line_is_code: bool,
    child_delimiter_line_type: LineType,
    child_buf: Vec<u8>,
}

impl<'a> Parser<'a> {
    pub(super) fn new(
        content: &'a [u8],
        lang: &'a LanguageDef,
        counts: LineCounts,
        children: Vec<(&'static str, LineCounts)>,
    ) -> Self {
        Self {
            content,
            lang,
            counts,
            children,
            parse: ParseState::Normal,
            line_type: default_line_type(lang),
            pos: 0,
            block_started_this_line: false,
            close_line_is_code: false,
            child_delimiter_line_type: LineType::Code,
            child_buf: Vec::new(),
        }
    }

    pub(super) fn run(mut self) -> FileResult {
        while self.pos < self.content.len() {
            match self.step() {
                StepOutcome::Continue => {}
                StepOutcome::Break => break,
                StepOutcome::Return(result) => return result,
            }
        }

        self.finish_after_loop()
    }

    fn take_result(&mut self) -> FileResult {
        FileResult {
            counts: self.counts,
            children: std::mem::take(&mut self.children),
        }
    }

    fn reset_after_newline(&mut self) {
        self.parse = ParseState::Normal;
        self.line_type = default_line_type(self.lang);
        self.block_started_this_line = false;
        self.close_line_is_code = false;
    }

    fn default_is_comment(&self) -> bool {
        self.lang.default_mode == DefaultMode::Comment
    }

    fn step(&mut self) -> StepOutcome {
        match self.parse {
            ParseState::LineComment => self.step_line_comment(),
            ParseState::BlockComment {
                depth,
                open,
                close,
                nested,
                close_line_is_code,
            } => self.step_block_comment(depth, open, close, nested, close_line_is_code),
            ParseState::InString { close, escape } => self.step_in_string(close, escape),
            ParseState::InChild { close, child_lang } => self.step_in_child(close, child_lang),
            ParseState::Normal => self.step_normal(),
        }
    }

    fn step_line_comment(&mut self) -> StepOutcome {
        let bytes = &self.content[self.pos..];
        match scanner::find_newline(bytes) {
            Some(nl) => {
                emit(&mut self.counts, self.line_type, self.parse);
                self.reset_after_newline();
                self.pos += nl + 1;
                StepOutcome::Continue
            }
            None => {
                emit(&mut self.counts, self.line_type, self.parse);
                StepOutcome::Return(self.take_result())
            }
        }
    }

    fn step_block_comment(
        &mut self,
        depth: u8,
        open: &'static [u8],
        close: &'static [u8],
        nested: bool,
        block_close_is_code: bool,
    ) -> StepOutcome {
        if nested {
            self.step_nested_block_comment(depth, open, close, block_close_is_code)
        } else {
            self.step_flat_block_comment(close, block_close_is_code)
        }
    }

    fn step_nested_block_comment(
        &mut self,
        depth: u8,
        open: &'static [u8],
        close: &'static [u8],
        block_close_is_code: bool,
    ) -> StepOutcome {
        let bytes = &self.content[self.pos..];
        let open_first = open.first().copied().unwrap_or(b'/');
        let close_first = close.first().copied().unwrap_or(b'*');

        match scanner::find_nested_block(bytes, open_first, close_first) {
            Some(i) => {
                if bytes[i] == b'\n' {
                    emit(&mut self.counts, self.line_type, self.parse);
                    self.line_type = LineType::Comment;
                    self.close_line_is_code = block_close_is_code;
                    self.block_started_this_line = false;
                    self.pos += i + 1;
                    return StepOutcome::Continue;
                }

                let rest = &self.content[self.pos + i..];
                if rest.starts_with(close) {
                    self.parse = if depth > 1 {
                        ParseState::BlockComment {
                            depth: depth - 1,
                            open,
                            close,
                            nested: true,
                            close_line_is_code: block_close_is_code,
                        }
                    } else {
                        ParseState::Normal
                    };
                    self.pos += i + close.len();
                } else if rest.starts_with(open) {
                    self.parse = ParseState::BlockComment {
                        depth: depth.saturating_add(1),
                        open,
                        close,
                        nested: true,
                        close_line_is_code: block_close_is_code,
                    };
                    self.pos += i + open.len();
                } else {
                    self.pos += i + 1;
                }

                StepOutcome::Continue
            }
            None => {
                emit(&mut self.counts, self.line_type, self.parse);
                StepOutcome::Return(self.take_result())
            }
        }
    }

    fn step_flat_block_comment(
        &mut self,
        close: &'static [u8],
        block_close_is_code: bool,
    ) -> StepOutcome {
        let bytes = &self.content[self.pos..];
        let close_first = close.first().copied().unwrap_or(b'*');

        match scanner::find_newline_or(bytes, close_first) {
            Some(i) => {
                if bytes[i] == b'\n' {
                    emit(&mut self.counts, self.line_type, self.parse);
                    self.line_type = LineType::Comment;
                    self.close_line_is_code = block_close_is_code;
                    self.block_started_this_line = false;
                    self.pos += i + 1;
                    return StepOutcome::Continue;
                }

                let rest = &self.content[self.pos + i..];
                if rest.starts_with(close) {
                    self.parse = ParseState::Normal;
                    self.pos += i + close.len();
                } else {
                    // false positive: close_first matched but not the full delimiter
                    self.pos += i + 1;
                }

                StepOutcome::Continue
            }
            None => {
                emit(&mut self.counts, self.line_type, self.parse);
                StepOutcome::Return(self.take_result())
            }
        }
    }

    fn step_in_string(&mut self, close: &'static [u8], escape: bool) -> StepOutcome {
        let bytes = &self.content[self.pos..];
        let close_first = close.first().copied().unwrap_or(b'"');
        let found = if escape {
            scanner::find_string_end(bytes, close_first)
        } else {
            scanner::find_string_end_no_escape(bytes, close_first)
        };

        let Some(i) = found else {
            self.counts.code += 1;
            return StepOutcome::Return(self.take_result());
        };

        if bytes[i] == b'\n' {
            emit(&mut self.counts, LineType::Code, self.parse);
            self.line_type = LineType::Code;
            self.block_started_this_line = false;
            self.close_line_is_code = false;
            self.pos += i + 1;
            return StepOutcome::Continue;
        }

        if bytes[i] == b'\\' {
            if self.content.get(self.pos + i + 1).copied() == Some(b'\n') {
                emit(&mut self.counts, LineType::Code, self.parse);
                self.line_type = LineType::Code;
                self.block_started_this_line = false;
                self.close_line_is_code = false;
            }
            self.pos += i + 2;
            return StepOutcome::Continue;
        }

        let rest = &self.content[self.pos + i..];
        if rest.starts_with(close) {
            self.parse = ParseState::Normal;
            self.pos += i + close.len();
        } else {
            self.pos += i + 1;
        }

        StepOutcome::Continue
    }

    fn step_in_child(
        &mut self,
        close: &'static [u8],
        child_lang: Option<&'static LanguageDef>,
    ) -> StepOutcome {
        let bytes = &self.content[self.pos..];
        let close_first = close.first().copied().unwrap_or(b'<');

        match scanner::find_newline_or(bytes, close_first) {
            Some(i) => {
                if bytes[i] == b'\n' {
                    self.child_buf.extend_from_slice(&bytes[..=i]);
                    self.pos += i + 1;
                    return StepOutcome::Continue;
                }

                let rest = &self.content[self.pos + i..];
                if rest.starts_with(close) {
                    push_child_before_close(&mut self.child_buf, &bytes[..i]);
                    if let Some(child_lang) = child_lang {
                        push_child_result(&mut self.children, child_lang, &self.child_buf);
                    } else {
                        let child_counts = count_pure_code(&self.child_buf);
                        self.counts.code += child_counts.code;
                        self.counts.blank += child_counts.blank;
                    }
                    self.child_buf.clear();

                    self.parse = ParseState::Normal;
                    self.line_type = self.child_delimiter_line_type;
                    self.block_started_this_line = false;
                    self.close_line_is_code = false;
                    self.pos += i + close.len();
                } else {
                    self.child_buf.extend_from_slice(&bytes[..=i]);
                    self.pos += i + 1;
                }

                StepOutcome::Continue
            }
            None => {
                self.child_buf.extend_from_slice(bytes);
                StepOutcome::Break
            }
        }
    }

    fn step_normal(&mut self) -> StepOutcome {
        let bytes = &self.content[self.pos..];
        let Some(i) = scanner::find_interesting(bytes, self.lang.interest_bytes) else {
            self.line_type = classify_prefix(
                bytes,
                self.line_type,
                self.block_started_this_line,
                self.close_line_is_code,
                self.default_is_comment(),
            );

            if !bytes.is_empty() {
                emit(&mut self.counts, self.line_type, self.parse);
            }
            return StepOutcome::Return(self.take_result());
        };

        if i > 0 {
            self.line_type = classify_prefix(
                &bytes[..i],
                self.line_type,
                self.block_started_this_line,
                self.close_line_is_code,
                self.default_is_comment(),
            );
        }

        self.pos += i;
        if bytes[i] == b'\n' {
            emit(&mut self.counts, self.line_type, self.parse);
            self.reset_after_newline();
            self.pos += 1;
            return StepOutcome::Continue;
        }

        self.step_normal_token()
    }

    fn step_normal_token(&mut self) -> StepOutcome {
        let rest = &self.content[self.pos..];
        let (token, advance) = match_token(rest, self.lang);

        match token {
            TokenMatch::LineComment => {
                if self.line_type == LineType::Blank {
                    self.line_type = LineType::Comment;
                }
                self.parse = ParseState::LineComment;
            }
            TokenMatch::BlockComment {
                open,
                close,
                nested,
                close_line_is_code: block_close_is_code,
            } => {
                if self.line_type == LineType::Blank {
                    self.line_type = LineType::Comment;
                }

                self.block_started_this_line = true;
                self.close_line_is_code = block_close_is_code;
                self.parse = ParseState::BlockComment {
                    depth: 1,
                    open,
                    close,
                    nested,
                    close_line_is_code: block_close_is_code,
                };
            }
            TokenMatch::StringLiteral { close, escape } => {
                self.line_type = LineType::Code;
                self.parse = ParseState::InString { close, escape };
                self.close_line_is_code = false;
            }
            TokenMatch::Child {
                close,
                default_lang,
                detect,
            } => {
                self.open_child_region(close, default_lang, detect, advance);
                return StepOutcome::Continue;
            }
            TokenMatch::Other => {
                if self.line_type == LineType::Blank
                    && !matches!(self.content[self.pos], b' ' | b'\t' | b'\r')
                {
                    self.line_type = if self.default_is_comment() {
                        LineType::Comment
                    } else {
                        LineType::Code
                    };
                }
            }
        }

        self.pos += advance;
        StepOutcome::Continue
    }

    fn open_child_region(
        &mut self,
        close: &'static [u8],
        default_lang: Option<&'static str>,
        detect: Detection,
        open_len: usize,
    ) {
        let after_open = &self.content[self.pos + open_len..];
        let ChildOpenResult {
            detected_child,
            mut consume_after_open,
            delim_line_type,
        } = detect_child_open(self.content, self.pos, open_len, default_lang, detect);

        self.line_type = delim_line_type;
        self.child_delimiter_line_type = delim_line_type;
        self.child_buf.clear();

        if detect == Detection::Fence
            && consume_after_open > 0
            && after_open.get(consume_after_open - 1).copied() == Some(b'\n')
        {
            emit(&mut self.counts, self.line_type, ParseState::Normal);
            self.line_type = default_line_type(self.lang);
        }

        if detect == Detection::Tag && after_open.get(consume_after_open).copied() == Some(b'\n') {
            emit(&mut self.counts, self.line_type, ParseState::Normal);
            self.line_type = default_line_type(self.lang);
            consume_after_open += 1;
        }

        self.parse = ParseState::InChild {
            close,
            child_lang: detected_child,
        };
        self.pos += open_len + consume_after_open;
        self.block_started_this_line = false;
        self.close_line_is_code = false;
    }

    fn finish_after_loop(&mut self) -> FileResult {
        if let ParseState::InChild { child_lang, .. } = self.parse {
            if let Some(child_lang) = child_lang {
                push_child_result(&mut self.children, child_lang, &self.child_buf);
            } else {
                let child_counts = count_pure_code(&self.child_buf);
                self.counts.code += child_counts.code;
                self.counts.blank += child_counts.blank;
            }
        }

        if self.line_type != LineType::Blank && self.content.last().copied() != Some(b'\n') {
            emit(&mut self.counts, self.line_type, self.parse);
        }

        self.take_result()
    }
}

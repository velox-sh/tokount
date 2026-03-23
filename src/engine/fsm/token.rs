use super::types::TokenMatch;
use crate::engine::language::LanguageDef;

// longest-match: longer opener beats shorter prefix even across token types
// (e.g. `////` block comment wins over `//` line comment in AsciiDoc)
#[inline(always)]
pub(super) fn match_token(rest: &[u8], lang: &LanguageDef) -> (TokenMatch, usize) {
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

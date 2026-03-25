use super::types::TokenMatch;
use crate::engine::language::LanguageDef;
use crate::engine::language::RegionKind;

#[inline(always)]
pub(super) fn match_token(rest: &[u8], lang: &LanguageDef) -> (TokenMatch, usize) {
    let Some(&first) = rest.first() else {
        return (TokenMatch::Other, 1);
    };
    let mut best: Option<TokenMatch> = None;
    let mut best_len = 0usize;

    for region in lang.regions {
        // first-byte guard: skips starts_with for regions that can't match
        if region.open.first().copied() == Some(first)
            && region.open.len() > best_len
            && rest.starts_with(region.open)
        {
            best = Some(match region.kind {
                RegionKind::Comment {
                    nested,
                    close_line_is_code,
                } => {
                    if region.close == b"\n" {
                        TokenMatch::LineComment
                    } else {
                        TokenMatch::BlockComment {
                            open: region.open,
                            close: region.close,
                            nested,
                            close_line_is_code,
                        }
                    }
                }

                RegionKind::String { escape } => TokenMatch::StringLiteral {
                    close: region.close,
                    escape,
                },

                RegionKind::Child {
                    default_lang,
                    detect,
                } => TokenMatch::Child {
                    close: region.close,
                    default_lang,
                    detect,
                },
            });

            best_len = region.open.len();
        }
    }

    match best {
        Some(m) => (m, best_len),
        None => (TokenMatch::Other, 1),
    }
}

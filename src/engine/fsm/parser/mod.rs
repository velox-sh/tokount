use super::types::FileResult;
use super::types::LineCounts;
use crate::engine::language::LanguageDef;
use crate::engine::language::StructuredFormat;

mod helpers;
mod state;

use helpers::leading_doc_child;
use helpers::parse_jupyter_cells;
use helpers::zero_counts;
use state::Parser;

pub fn count_file(content: &[u8], lang: &LanguageDef) -> FileResult {
    if lang.structured_format == StructuredFormat::Jupyter {
        return parse_jupyter_cells(content);
    }

    let counts = zero_counts();
    let mut children: Vec<(&'static str, LineCounts)> = Vec::new();

    let mut content = content;
    if !lang.leading_doc_prefixes.is_empty() {
        let (doc_child, start_pos) = leading_doc_child(content, lang.leading_doc_prefixes);
        if let Some(c) = doc_child {
            children.push(("Markdown", c));
            content = &content[start_pos..];
        }
    }

    if content.is_empty() {
        return FileResult { counts, children };
    }

    Parser::new(content, lang, counts, children).run()
}

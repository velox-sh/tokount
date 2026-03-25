use super::super::types::FileResult;
use super::super::types::LineCounts;
use super::super::types::LineType;
use crate::engine::language::Detection;
use crate::engine::language::LanguageDef;
use crate::engine::scanner;

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum JupyterSource {
    Single(String),
    Many(Vec<String>),
}

#[derive(serde::Deserialize)]
struct JupyterCell {
    cell_type: String,
    #[serde(default)]
    source: Option<JupyterSource>,
}

#[derive(serde::Deserialize)]
struct JupyterNotebook {
    #[serde(default)]
    cells: Vec<JupyterCell>,
}

pub(super) struct ChildOpenResult {
    pub(super) detected_child: Option<&'static LanguageDef>,
    pub(super) consume_after_open: usize,
    pub(super) delim_line_type: LineType,
}

#[inline(always)]
pub(super) fn zero_counts() -> LineCounts {
    LineCounts {
        code: 0,
        comment: 0,
        blank: 0,
    }
}

#[inline(always)]
pub(super) fn default_line_type(_lang: &LanguageDef) -> LineType {
    LineType::Blank
}

#[inline(always)]
pub(super) fn push_child_result(
    children: &mut Vec<(&'static str, LineCounts)>,
    child_lang: &'static LanguageDef,
    child_buf: &[u8],
) {
    if child_buf.is_empty() {
        return;
    }

    let child_result = super::count_file(child_buf, child_lang);
    children.push((child_lang.name, child_result.counts));
    children.extend(child_result.children);
}

#[inline(always)]
pub(super) fn count_pure_code(content: &[u8]) -> LineCounts {
    let mut counts = zero_counts();
    let mut pos = 0;

    while pos < content.len() {
        let end = scanner::find_newline(&content[pos..]).map_or(content.len(), |i| pos + i);
        let line = &content[pos..end];

        if line.iter().any(|&b| !matches!(b, b' ' | b'\t' | b'\r')) {
            counts.code += 1;
        } else if !line.is_empty() || end < content.len() {
            counts.blank += 1;
        }

        pos = end + if end < content.len() { 1 } else { 0 };
    }

    counts
}

#[inline(always)]
fn aggregate_counts_with_children(result: &FileResult) -> LineCounts {
    let mut total = result.counts;
    for (_, c) in &result.children {
        total.code += c.code;
        total.comment += c.comment;
        total.blank += c.blank;
    }
    total
}

#[inline(always)]
fn line_starts_with_any_prefix<'a>(line: &'a [u8], prefixes: &[&[u8]]) -> Option<&'a [u8]> {
    prefixes
        .iter()
        .copied()
        .find_map(|prefix| line.strip_prefix(prefix))
}

#[inline(always)]
pub(super) fn push_child_before_close(buf: &mut Vec<u8>, chunk: &[u8]) {
    if chunk.is_empty() {
        return;
    }

    let start = chunk
        .iter()
        .rposition(|&b| b == b'\n')
        .map_or(0, |idx| idx + 1);

    // for close-delimiter lines like `    </script>`, keep the previous content
    // but drop indentation-only bytes on that same line
    if chunk[start..]
        .iter()
        .all(|&b| matches!(b, b' ' | b'\t' | b'\r'))
    {
        buf.extend_from_slice(&chunk[..start]);
    } else {
        buf.extend_from_slice(chunk);
    }
}

#[inline(always)]
fn resolve_lang_ident(ident: &str) -> Option<&'static LanguageDef> {
    if ident.is_empty() {
        return None;
    }

    LanguageDef::from_extension(ident)
        .or_else(|| LanguageDef::from_name(ident))
        .or_else(|| {
            ident
                .rsplit('/')
                .next()
                .and_then(|tail| tail.strip_prefix("x-"))
                .and_then(LanguageDef::from_extension)
        })
        .or_else(|| {
            ident
                .rsplit('/')
                .next()
                .and_then(LanguageDef::from_extension)
        })
}

#[inline(always)]
fn extract_attr_value<'a>(header: &'a str, attr: &str) -> Option<&'a str> {
    let mut idx = 0;
    let bytes = header.as_bytes();
    let needle = attr.as_bytes();

    while idx + needle.len() < bytes.len() {
        if bytes[idx..].starts_with(needle) {
            let mut j = idx + needle.len();
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if bytes.get(j).copied() != Some(b'=') {
                idx += 1;
                continue;
            }
            j += 1;
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if j >= bytes.len() {
                return None;
            }

            let quote = bytes[j];
            if quote == b'\'' || quote == b'"' {
                let start = j + 1;
                let end = header[start..].find(quote as char).map(|off| start + off)?;
                return Some(&header[start..end]);
            }

            let start = j;
            let mut end = start;
            while end < bytes.len()
                && !bytes[end].is_ascii_whitespace()
                && bytes[end] != b'>'
                && bytes[end] != b'/'
            {
                end += 1;
            }
            return Some(&header[start..end]);
        }
        idx += 1;
    }

    None
}

#[inline(always)]
fn detect_tag_lang(
    header: &[u8],
    default_lang: Option<&'static str>,
) -> Option<&'static LanguageDef> {
    let header = std::str::from_utf8(header).ok()?.to_ascii_lowercase();

    for attr in ["lang", "type"] {
        if let Some(v) = extract_attr_value(&header, attr)
            && let Some(lang) = resolve_lang_ident(v.trim())
        {
            return Some(lang);
        }
    }

    default_lang.and_then(resolve_lang_ident)
}

pub(super) fn parse_jupyter_cells(content: &[u8]) -> FileResult {
    let mut py_total = zero_counts();
    let mut md_total = zero_counts();

    let Ok(notebook) = serde_json::from_slice::<JupyterNotebook>(content) else {
        return FileResult {
            counts: zero_counts(),
            children: Vec::new(),
        };
    };

    let py = LanguageDef::from_name("Python");
    let md = LanguageDef::from_name("Markdown");

    for cell in notebook.cells {
        let src = match cell.source {
            Some(JupyterSource::Single(s)) => s,
            Some(JupyterSource::Many(lines)) => lines.join(""),
            None => String::new(),
        };

        let selected = if cell.cell_type.eq_ignore_ascii_case("markdown") {
            md
        } else {
            py
        };

        if let Some(cell_lang) = selected {
            let r = super::count_file(src.as_bytes(), cell_lang);
            let r_total = aggregate_counts_with_children(&r);
            if cell_lang.name == "Markdown" {
                md_total.code += r_total.code;
                md_total.comment += r_total.comment;
                md_total.blank += r_total.blank;
            } else {
                py_total.code += r_total.code;
                py_total.comment += r_total.comment;
                py_total.blank += r_total.blank;
            }
        } else {
            let r = count_pure_code(src.as_bytes());
            py_total.code += r.code;
            py_total.comment += r.comment;
            py_total.blank += r.blank;
        }
    }

    let counts = LineCounts {
        code: py_total.code + md_total.code,
        comment: py_total.comment + md_total.comment,
        blank: py_total.blank + md_total.blank,
    };

    let mut children = Vec::new();
    if md_total.code + md_total.comment + md_total.blank > 0 {
        children.push(("Markdown", md_total));
    }
    if py_total.code + py_total.comment + py_total.blank > 0 {
        children.push(("Python", py_total));
    }

    FileResult { counts, children }
}

pub(super) fn detect_child_open(
    content: &[u8],
    pos: usize,
    open_len: usize,
    default_lang: Option<&'static str>,
    detect: Detection,
) -> ChildOpenResult {
    let after_open = &content[pos + open_len..];

    let (detected_child, consume_after_open, delim_line_type) = match detect {
        Detection::Fixed => (default_lang.and_then(resolve_lang_ident), 0, LineType::Code),
        Detection::Fence => {
            let line_end = scanner::find_newline(after_open).map_or(after_open.len(), |n| n);
            let fence_info = &after_open[..line_end];
            let ident = fence_info
                .iter()
                .skip_while(|&&b| b.is_ascii_whitespace())
                .copied()
                .collect::<Vec<u8>>();
            let ident = ident
                .iter()
                .position(u8::is_ascii_whitespace)
                .map_or(ident.as_slice(), |n| &ident[..n]);
            let detected = std::str::from_utf8(ident)
                .ok()
                .and_then(resolve_lang_ident)
                .or_else(|| default_lang.and_then(resolve_lang_ident));

            let consume = if line_end < after_open.len() {
                line_end + 1
            } else {
                line_end
            };
            (detected, consume, LineType::Comment)
        }
        Detection::Tag => {
            let gt = after_open.iter().position(|&b| b == b'>');
            if let Some(gt) = gt {
                let header = &content[pos..pos + open_len + gt + 1];
                (
                    detect_tag_lang(header, default_lang),
                    gt + 1,
                    LineType::Code,
                )
            } else {
                (default_lang.and_then(resolve_lang_ident), 0, LineType::Code)
            }
        }
    };

    ChildOpenResult {
        detected_child,
        consume_after_open,
        delim_line_type,
    }
}

pub(super) fn leading_doc_child(content: &[u8], prefixes: &[&[u8]]) -> (Option<LineCounts>, usize) {
    if prefixes.is_empty() {
        return (None, 0);
    }

    let mut pos = 0usize;
    let mut md_buf = Vec::new();

    while pos < content.len() {
        let end = scanner::find_newline(&content[pos..]).map_or(content.len(), |i| pos + i);
        let line = &content[pos..end];

        let Some(mut body) = line_starts_with_any_prefix(line, prefixes) else {
            break;
        };

        if body.first().copied() == Some(b' ') {
            body = &body[1..];
        }
        md_buf.extend_from_slice(body);
        md_buf.push(b'\n');

        if end < content.len() {
            pos = end + 1;
        } else {
            pos = end;
        }
    }

    if md_buf.is_empty() {
        return (None, 0);
    }

    let counts = LanguageDef::from_name("Markdown").map_or_else(
        || count_pure_code(&md_buf),
        |md| {
            let r = super::count_file(&md_buf, md);
            aggregate_counts_with_children(&r)
        },
    );

    (Some(counts), pos)
}

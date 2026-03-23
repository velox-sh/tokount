use std::cmp::Ordering;
use std::time::Duration;

use comfy_table::Attribute;
use comfy_table::Cell;
use comfy_table::CellAlignment;
use comfy_table::Color;
use comfy_table::ContentArrangement;
use comfy_table::Table;
use comfy_table::presets::UTF8_HORIZONTAL_ONLY;

use crate::cli::SortColumn;
use crate::types::LangStats;
use crate::types::OutputStats;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO: &str = "github.com/MihaiStreames/tokount";

fn sort_langs(langs: &mut Vec<&str>, output: &OutputStats, sort: SortColumn) {
    langs.sort_unstable_by(|a, b| {
        let as_ = &output.languages[*a];
        let bs = &output.languages[*b];
        let cmp = cmp_stats(bs, as_, sort);
        cmp.then_with(|| a.cmp(b))
    });
}

fn sort_child_langs(langs: &mut Vec<&str>, parent: &crate::types::LangStats, sort: SortColumn) {
    langs.sort_unstable_by(|a, b| {
        let as_ = &parent.children[*a];
        let bs = &parent.children[*b];
        let cmp = cmp_stats(bs, as_, sort);
        cmp.then_with(|| a.cmp(b))
    });
}

fn sorted_lang_names(output: &OutputStats, sort: SortColumn) -> Vec<&str> {
    let mut langs: Vec<&str> = output
        .languages
        .keys()
        .filter(|k| k.as_str() != "SUM")
        .map(String::as_str)
        .collect();
    sort_langs(&mut langs, output, sort);
    langs
}

fn sorted_child_names(parent: &LangStats, sort: SortColumn) -> Vec<&str> {
    let mut children: Vec<&str> = parent.children.keys().map(String::as_str).collect();
    sort_child_langs(&mut children, parent, sort);
    children
}

fn print_summary(
    total_files: usize,
    git_repos: usize,
    label: &str,
    secs: f64,
    files_per_sec: f64,
    lines_per_sec: f64,
) {
    println!(
        "{REPO} v{VERSION}  T={secs:.2}s  ({files_per_sec:.0} files/s, {lines_per_sec:.0} lines/s)"
    );
    println!("{total_files} files  •  {git_repos} git repos  •  {label}");
    println!();
}

fn build_table(color: bool) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_HORIZONTAL_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            header_cell("Language", false, color),
            header_cell("Files", true, color),
            header_cell("Lines", true, color),
            header_cell("Blanks", true, color),
            header_cell("Comments", true, color),
            header_cell("Code", true, color),
        ]);

    if let Some(col) = table.column_mut(0) {
        col.set_padding((1, 3));
    }
    for col_idx in 1..=5 {
        if let Some(col) = table.column_mut(col_idx) {
            col.set_padding((4, 1));
        }
    }

    table
}

fn header_cell(name: &str, right_aligned: bool, color: bool) -> Cell {
    let mut cell = Cell::new(name).add_attribute(Attribute::Bold);
    if right_aligned {
        cell = cell.set_alignment(CellAlignment::Right);
    }
    if color {
        cell = cell.fg(Color::Cyan);
    }
    cell
}

#[inline]
fn cmp_stats(a: &LangStats, b: &LangStats, sort: SortColumn) -> Ordering {
    match sort {
        SortColumn::Files => a.n_files.cmp(&b.n_files),
        SortColumn::Lines => {
            total_lines(a.blank, a.comment, a.code).cmp(&total_lines(b.blank, b.comment, b.code))
        }
        SortColumn::Blank => a.blank.cmp(&b.blank),
        SortColumn::Comment => a.comment.cmp(&b.comment),
        SortColumn::Code => a.code.cmp(&b.code),
    }
}

#[inline]
fn total_lines(blank: usize, comment: usize, code: usize) -> usize {
    blank + comment + code
}

fn render_row(
    name: &str,
    files: usize,
    blank: usize,
    comment: usize,
    code: usize,
    embedded: bool,
    color: bool,
) -> Vec<Cell> {
    let lines = total_lines(blank, comment, code);
    let mut lang = Cell::new(name);
    let mut files_cell = Cell::new(files).set_alignment(CellAlignment::Right);
    let mut lines_cell = Cell::new(lines).set_alignment(CellAlignment::Right);
    let mut blank_cell = Cell::new(blank).set_alignment(CellAlignment::Right);
    let mut comment_cell = Cell::new(comment).set_alignment(CellAlignment::Right);
    let mut code_cell = Cell::new(code).set_alignment(CellAlignment::Right);

    if color {
        lang = if embedded {
            lang.fg(Color::Green).add_attribute(Attribute::Dim)
        } else {
            lang.fg(Color::Green)
        };
        files_cell = numeric_fg(files_cell, Color::White);
        lines_cell = numeric_fg(lines_cell, Color::White);
        blank_cell = numeric_fg(blank_cell, Color::DarkGrey);
        comment_cell = numeric_fg(comment_cell, Color::Yellow);
        code_cell = numeric_fg(code_cell, Color::Magenta);
    }

    vec![
        lang,
        files_cell,
        lines_cell,
        blank_cell,
        comment_cell,
        code_cell,
    ]
}

fn render_sum_row(
    files: usize,
    blank: usize,
    comment: usize,
    code: usize,
    color: bool,
) -> Vec<Cell> {
    let lines = total_lines(blank, comment, code);
    let mut lang = Cell::new("SUM").add_attribute(Attribute::Bold);
    let mut files_cell = Cell::new(files)
        .add_attribute(Attribute::Bold)
        .set_alignment(CellAlignment::Right);
    let mut lines_cell = Cell::new(lines)
        .add_attribute(Attribute::Bold)
        .set_alignment(CellAlignment::Right);
    let mut blank_cell = Cell::new(blank)
        .add_attribute(Attribute::Bold)
        .set_alignment(CellAlignment::Right);
    let mut comment_cell = Cell::new(comment)
        .add_attribute(Attribute::Bold)
        .set_alignment(CellAlignment::Right);
    let mut code_cell = Cell::new(code)
        .add_attribute(Attribute::Bold)
        .set_alignment(CellAlignment::Right);

    if color {
        lang = lang.fg(Color::Cyan);
        files_cell = files_cell.fg(Color::Cyan);
        lines_cell = lines_cell.fg(Color::Cyan);
        blank_cell = blank_cell.fg(Color::Cyan);
        comment_cell = comment_cell.fg(Color::Cyan);
        code_cell = code_cell.fg(Color::Cyan);
    }

    vec![
        lang,
        files_cell,
        lines_cell,
        blank_cell,
        comment_cell,
        code_cell,
    ]
}

#[inline]
fn numeric_fg(cell: Cell, color: Color) -> Cell {
    cell.fg(color)
}

pub fn print_table(
    output: &OutputStats,
    label: &str,
    elapsed: Duration,
    sort: SortColumn,
    color: bool,
) {
    let sum = output.languages.get("SUM");
    let total_files = sum.map_or(0, |s| s.n_files);
    let total_lines = sum.map_or(0, |s| s.blank + s.comment + s.code);
    let secs = elapsed.as_secs_f64();

    let files_per_sec = if secs > 0.0 {
        total_files as f64 / secs
    } else {
        0.0
    };
    let lines_per_sec = if secs > 0.0 {
        total_lines as f64 / secs
    } else {
        0.0
    };

    print_summary(
        total_files,
        output.git_repos,
        label,
        secs,
        files_per_sec,
        lines_per_sec,
    );

    let mut table = build_table(color);
    let langs = sorted_lang_names(output, sort);

    for lang in langs {
        let s = &output.languages[lang];

        table.add_row(render_row(
            lang, s.n_files, s.blank, s.comment, s.code, false, color,
        ));

        if !s.children.is_empty() {
            let children = sorted_child_names(s, sort);

            for child in children {
                let child_stats = &s.children[child];
                let child_label = format!("|- {child}");
                table.add_row(render_row(
                    &child_label,
                    child_stats.n_files,
                    child_stats.blank,
                    child_stats.comment,
                    child_stats.code,
                    true,
                    color,
                ));
            }
        }
    }

    if let Some(sum) = sum {
        table.add_row(render_sum_row(
            sum.n_files,
            sum.blank,
            sum.comment,
            sum.code,
            color,
        ));
    }

    println!("{table}");
}

pub fn print_csv(output: &OutputStats, sort: SortColumn) {
    println!("language,files,blank,comment,code");

    for lang in sorted_lang_names(output, sort) {
        let s = &output.languages[lang];
        println!(
            "{},{},{},{},{}",
            lang, s.n_files, s.blank, s.comment, s.code
        );
    }

    if let Some(sum) = output.languages.get("SUM") {
        println!(
            "SUM,{},{},{},{}",
            sum.n_files, sum.blank, sum.comment, sum.code
        );
    }
}

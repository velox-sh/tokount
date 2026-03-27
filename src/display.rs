use std::cmp::Ordering;
use std::io::IsTerminal;
use std::time::Duration;

use comfy_table::Attribute;
use comfy_table::Cell;
use comfy_table::CellAlignment;
use comfy_table::Color;
use comfy_table::ColumnConstraint;
use comfy_table::ContentArrangement;
use comfy_table::Table;
use comfy_table::Width;
use comfy_table::presets::UTF8_BORDERS_ONLY;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use tokount::types::LangStats;
use tokount::types::OutputStats;

use crate::cli::Args;
use crate::cli::OutputFormat;
use crate::cli::SortColumn;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO: &str = "github.com/velox-sh/tokount";
const CHILD_ROW_PREFIX: &str = ">> ";

fn sort_by_stats<'a, F>(names: &mut [&str], sort: SortColumn, reverse: bool, get: F)
where
    F: Fn(&str) -> &'a LangStats,
{
    names.sort_unstable_by(|a, b| {
        let cmp = if reverse {
            cmp_stats(get(a), get(b), sort)
        } else {
            cmp_stats(get(b), get(a), sort)
        };
        cmp.then_with(|| a.cmp(b))
    });
}

fn build_table(color: bool) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Disabled)
        .set_header(vec![
            header_cell("Language", false, color),
            header_cell("Files", true, color),
            header_cell("Lines", true, color),
            header_cell("Code", true, color),
            header_cell("Comments", true, color),
            header_cell("Blanks", true, color),
        ]);

    if let Some(col) = table.column_mut(0) {
        col.set_padding((1, 1));
    }

    for col_idx in 1..=5 {
        if let Some(col) = table.column_mut(col_idx) {
            col.set_padding((1, 1));
            col.set_constraint(ColumnConstraint::Absolute(Width::Fixed(11)));
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
    blank + comment + code // wow such math
}

#[derive(Clone, Copy)]
struct RowDisplay<'a> {
    name: &'a str,
    files: usize,
    blank: usize,
    comment: usize,
    code: usize,
    color: bool,
    bold: bool,
    uniform_color: Option<Color>,
}

impl<'a> RowDisplay<'a> {
    fn from_stats(name: &'a str, s: &LangStats, color: bool) -> Self {
        Self {
            name,
            files: s.n_files,
            blank: s.blank,
            comment: s.comment,
            code: s.code,
            color,
            bold: false,
            uniform_color: None,
        }
    }

    fn sum(s: &LangStats, color: bool) -> Self {
        Self {
            name: "SUM",
            files: s.n_files,
            blank: s.blank,
            comment: s.comment,
            code: s.code,
            color,
            bold: true,
            uniform_color: Some(Color::Cyan),
        }
    }
}

fn render_row(row: RowDisplay<'_>) -> Vec<Cell> {
    let RowDisplay {
        name,
        files,
        blank,
        comment,
        code,
        color,
        bold,
        uniform_color,
    } = row;

    let lines = total_lines(blank, comment, code);
    let mut lang = Cell::new(name);
    let mut files_cell = Cell::new(files).set_alignment(CellAlignment::Right);
    let mut lines_cell = Cell::new(lines).set_alignment(CellAlignment::Right);
    let mut code_cell = Cell::new(code).set_alignment(CellAlignment::Right);
    let mut comment_cell = Cell::new(comment).set_alignment(CellAlignment::Right);
    let mut blank_cell = Cell::new(blank).set_alignment(CellAlignment::Right);

    if bold {
        lang = lang.add_attribute(Attribute::Bold);
        files_cell = files_cell.add_attribute(Attribute::Bold);
        lines_cell = lines_cell.add_attribute(Attribute::Bold);
        blank_cell = blank_cell.add_attribute(Attribute::Bold);
        comment_cell = comment_cell.add_attribute(Attribute::Bold);
        code_cell = code_cell.add_attribute(Attribute::Bold);
    }

    if color {
        if let Some(sum_color) = uniform_color {
            lang = lang.fg(sum_color);
            files_cell = files_cell.fg(sum_color);
            lines_cell = lines_cell.fg(sum_color);
            code_cell = code_cell.fg(sum_color);
            comment_cell = comment_cell.fg(sum_color);
            blank_cell = blank_cell.fg(sum_color);
        } else {
            // keep color only on language names
            lang = lang.fg(Color::Green);
        }
    }

    vec![
        lang,
        files_cell,
        lines_cell,
        code_cell,
        comment_cell,
        blank_cell,
    ]
}

fn print_table(
    output: &OutputStats,
    label: &str,
    elapsed: Duration,
    sort: SortColumn,
    sort_reverse: bool,
    color: bool,
    compact: bool,
) {
    let sum = output.languages.get("SUM");
    let total_files = sum.map_or(0, |s| s.n_files);
    let total_lines_count = sum.map_or(0, |s| s.blank + s.comment + s.code);
    let secs = elapsed.as_secs_f64();

    let files_per_sec = if secs > 0.0 {
        total_files as f64 / secs
    } else {
        0.0
    };

    let lines_per_sec = if secs > 0.0 {
        total_lines_count as f64 / secs
    } else {
        0.0
    };

    println!(
        "{REPO} v{VERSION}  T={secs:.2}s  ({files_per_sec:.0} files/s, {lines_per_sec:.0} lines/s)"
    );
    println!(
        "{total_files} files  •  {git_repos} git repos  •  {label}",
        git_repos = output.git_repos
    );
    println!();

    let mut table = build_table(color);
    let mut langs: Vec<&str> = output
        .languages
        .keys()
        .filter(|k| k.as_str() != "SUM")
        .map(String::as_str)
        .collect();
    sort_by_stats(&mut langs, sort, sort_reverse, |name| {
        &output.languages[name]
    });

    for lang in langs {
        let s = &output.languages[lang];

        table.add_row(render_row(RowDisplay::from_stats(lang, s, color)));

        if !compact && !s.children.is_empty() {
            let mut children: Vec<&str> = s.children.keys().map(String::as_str).collect();
            sort_by_stats(&mut children, sort, sort_reverse, |name| &s.children[name]);

            for child in children {
                let child_stats = &s.children[child];
                let child_label = format!("{CHILD_ROW_PREFIX}{child}");
                table.add_row(render_row(RowDisplay::from_stats(
                    &child_label,
                    child_stats,
                    color,
                )));
            }
        }
    }

    if let Some(sum) = sum {
        table.add_row(render_row(RowDisplay::sum(sum, color)));
    }

    println!("{table}");
}

fn print_csv(output: &OutputStats, sort: SortColumn, sort_reverse: bool, compact: bool) {
    println!("language,files,lines,blank,comment,code");

    let mut langs: Vec<&str> = output
        .languages
        .keys()
        .filter(|k| k.as_str() != "SUM")
        .map(String::as_str)
        .collect();
    sort_by_stats(&mut langs, sort, sort_reverse, |name| {
        &output.languages[name]
    });

    for lang in langs {
        let s = &output.languages[lang];
        let lines = total_lines(s.blank, s.comment, s.code);
        println!(
            "{},{},{},{},{},{}",
            lang, s.n_files, lines, s.blank, s.comment, s.code
        );

        if !compact && !s.children.is_empty() {
            let mut children: Vec<&str> = s.children.keys().map(String::as_str).collect();
            sort_by_stats(&mut children, sort, sort_reverse, |name| &s.children[name]);

            for child in children {
                let child_stats = &s.children[child];
                let lines = total_lines(child_stats.blank, child_stats.comment, child_stats.code);
                println!(
                    "\"{}{}\",0,{},{},{},{}",
                    CHILD_ROW_PREFIX,
                    child,
                    lines,
                    child_stats.blank,
                    child_stats.comment,
                    child_stats.code
                );
            }
        }
    }

    if let Some(sum) = output.languages.get("SUM") {
        let lines = total_lines(sum.blank, sum.comment, sum.code);
        println!(
            "SUM,{},{},{},{},{}",
            sum.n_files, lines, sum.blank, sum.comment, sum.code
        );
    }
}

fn spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .expect("spinner template is valid"),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

pub(crate) fn start_spinner(fmt: OutputFormat) -> Option<ProgressBar> {
    if fmt == OutputFormat::Table {
        let pb = spinner();
        pb.set_message("Counting lines...");
        Some(pb)
    } else {
        None
    }
}

pub(crate) fn render(output: &OutputStats, label: &str, elapsed: Duration, args: &Args) {
    let fmt = args.format();
    let sort = args.sort_column();
    let sort_reverse = args.sort_reverse();

    match fmt {
        OutputFormat::Table => {
            let color = !args.no_color
                && std::io::stdout().is_terminal()
                && std::env::var_os("NO_COLOR").is_none();
            print_table(
                output,
                label,
                elapsed,
                sort,
                sort_reverse,
                color,
                args.compact,
            );
        }
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string(output).expect("output is serializable")
        ),
        OutputFormat::Csv => print_csv(output, sort, sort_reverse, args.compact),
    }
}

use std::time::Duration;

use comfy_table::Attribute;
use comfy_table::Cell;
use comfy_table::CellAlignment;
use comfy_table::ContentArrangement;
use comfy_table::Table;
use comfy_table::presets::UTF8_HORIZONTAL_ONLY;

use crate::cli::SortColumn;
use crate::types::OutputStats;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO: &str = "github.com/MihaiStreames/tokount";

pub fn print_table(output: &OutputStats, label: &str, elapsed: Duration, sort: SortColumn) {
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

    println!(
        "{REPO} v{VERSION}  T={secs:.2}s  ({files_per_sec:.0} files/s, {lines_per_sec:.0} lines/s)"
    );
    println!(
        "{total_files} files  •  {} git repos  •  {label}",
        output.git_repos
    );
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_HORIZONTAL_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Language").add_attribute(Attribute::Bold),
            Cell::new("Files")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
            Cell::new("Blank")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
            Cell::new("Comment")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
            Cell::new("Code")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
        ]);

    table.column_mut(0).unwrap().set_padding((1, 3));
    for col_idx in 1..=4 {
        table.column_mut(col_idx).unwrap().set_padding((4, 1));
    }

    let mut langs: Vec<&str> = output
        .languages
        .keys()
        .filter(|k| k.as_str() != "SUM")
        .map(String::as_str)
        .collect();

    sort_langs(&mut langs, output, sort);

    for lang in langs {
        let s = &output.languages[lang];

        table.add_row(vec![
            Cell::new(lang),
            Cell::new(s.n_files).set_alignment(CellAlignment::Right),
            Cell::new(s.blank).set_alignment(CellAlignment::Right),
            Cell::new(s.comment).set_alignment(CellAlignment::Right),
            Cell::new(s.code).set_alignment(CellAlignment::Right),
        ]);
    }

    if let Some(sum) = sum {
        table.add_row(vec![
            Cell::new("SUM").add_attribute(Attribute::Bold),
            Cell::new(sum.n_files)
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
            Cell::new(sum.blank)
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
            Cell::new(sum.comment)
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
            Cell::new(sum.code)
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
        ]);
    }

    println!("{table}");
}

pub fn print_csv(output: &OutputStats, sort: SortColumn) {
    println!("language,files,blank,comment,code");

    let mut langs: Vec<&str> = output
        .languages
        .keys()
        .filter(|k| k.as_str() != "SUM")
        .map(String::as_str)
        .collect();

    sort_langs(&mut langs, output, sort);

    for lang in langs {
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

fn sort_langs(langs: &mut Vec<&str>, output: &OutputStats, sort: SortColumn) {
    langs.sort_unstable_by(|a, b| {
        let as_ = &output.languages[*a];
        let bs = &output.languages[*b];
        let cmp = match sort {
            SortColumn::Files => bs.n_files.cmp(&as_.n_files),
            SortColumn::Lines => {
                let bl = bs.blank + bs.comment + bs.code;
                let al = as_.blank + as_.comment + as_.code;
                bl.cmp(&al)
            }
            SortColumn::Blank => bs.blank.cmp(&as_.blank),
            SortColumn::Comment => bs.comment.cmp(&as_.comment),
            SortColumn::Code => bs.code.cmp(&as_.code),
        };
        cmp.then_with(|| a.cmp(b))
    });
}

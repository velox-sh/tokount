use std::collections::HashMap;

use crate::engine::fsm::FileResult;
use crate::types::LangStats;
use crate::types::OutputStats;

#[derive(Default, Clone)]
pub struct LangEntry {
    pub files: usize,
    pub code: usize,
    pub comment: usize,
    pub blank: usize,
}

#[derive(Default)]
pub struct ThreadStats {
    pub langs: HashMap<&'static str, LangEntry>,
    pub children: HashMap<&'static str, HashMap<&'static str, LangEntry>>,
    pub git_repos: usize,
    pub gitignore_patterns: Vec<String>,
}

impl ThreadStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, lang_name: &'static str, result: FileResult) {
        let entry = self.langs.entry(lang_name).or_default();
        entry.files += 1;
        entry.code += result.counts.code as usize;
        entry.comment += result.counts.comment as usize;
        entry.blank += result.counts.blank as usize;

        for (child_name, child_counts) in result.children {
            let child = self
                .children
                .entry(lang_name)
                .or_default()
                .entry(child_name)
                .or_default();

            child.code += child_counts.code as usize;
            child.comment += child_counts.comment as usize;
            child.blank += child_counts.blank as usize;
        }
    }

    pub fn merge(&mut self, other: ThreadStats) {
        for (name, entry) in other.langs {
            let e = self.langs.entry(name).or_default();
            e.files += entry.files;
            e.code += entry.code;
            e.comment += entry.comment;
            e.blank += entry.blank;
        }

        for (parent, child_map) in other.children {
            let dest = self.children.entry(parent).or_default();
            for (child, entry) in child_map {
                let e = dest.entry(child).or_default();
                e.code += entry.code;
                e.comment += entry.comment;
                e.blank += entry.blank;
            }
        }

        self.git_repos += other.git_repos;
        self.gitignore_patterns.extend(other.gitignore_patterns);
    }

    pub fn into_output(self) -> OutputStats {
        let mut total = LangStats {
            n_files: 0,
            blank: 0,
            comment: 0,
            code: 0,
            children: HashMap::new(),
        };
        let mut languages = HashMap::new();

        for (name, entry) in &self.langs {
            if entry.code > 0 || entry.comment > 0 || entry.blank > 0 {
                total.n_files += entry.files;
                total.blank += entry.blank;
                total.comment += entry.comment;
                total.code += entry.code;

                let child_stats: HashMap<String, LangStats> = self
                    .children
                    .get(name)
                    .map(|child_map| {
                        child_map
                            .iter()
                            .map(|(child_name, e)| {
                                (
                                    child_name.to_string(),
                                    LangStats {
                                        n_files: 0,
                                        blank: e.blank,
                                        comment: e.comment,
                                        code: e.code,
                                        children: HashMap::new(),
                                    },
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                languages.insert(
                    name.to_string(),
                    LangStats {
                        n_files: entry.files,
                        blank: entry.blank,
                        comment: entry.comment,
                        code: entry.code,
                        children: child_stats,
                    },
                );
            }
        }

        languages.insert("SUM".to_string(), total);

        let mut patterns = self.gitignore_patterns;
        patterns.sort();
        patterns.dedup();

        OutputStats {
            languages,
            git_repos: self.git_repos,
            gitignore_patterns: patterns,
        }
    }
}

use std::collections::HashMap;

use crate::engine::fsm::LineCounts;
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
    pub git_repos: usize,
    pub gitignore_patterns: Vec<String>,
}

impl ThreadStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, lang_name: &'static str, counts: LineCounts) {
        let entry = self.langs.entry(lang_name).or_default();
        entry.files += 1;
        entry.code += counts.code as usize;
        entry.comment += counts.comment as usize;
        entry.blank += counts.blank as usize;
    }

    pub fn merge(&mut self, other: ThreadStats) {
        for (name, entry) in other.langs {
            let e = self.langs.entry(name).or_default();
            e.files += entry.files;
            e.code += entry.code;
            e.comment += entry.comment;
            e.blank += entry.blank;
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
        };
        let mut languages = HashMap::new();

        for (name, entry) in &self.langs {
            if entry.code > 0 || entry.comment > 0 || entry.blank > 0 {
                total.n_files += entry.files;
                total.blank += entry.blank;
                total.comment += entry.comment;
                total.code += entry.code;
                languages.insert(
                    name.to_string(),
                    LangStats {
                        n_files: entry.files,
                        blank: entry.blank,
                        comment: entry.comment,
                        code: entry.code,
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

use std::path::PathBuf;

pub type MatchRange = (usize, usize);

#[derive(Debug)]
pub struct FileResult {
    pub path: PathBuf,
    pub results: Vec<SearchResult>,
}

#[derive(Debug)]
pub struct SearchResult {
    pub line: usize,
    pub line_content: String,
    pub matches: Vec<MatchResult>,
}

#[derive(Debug)]
pub struct MatchResult {
    pub match_range: MatchRange,
    pub content: String,
}

impl SearchResult {
    pub fn new(line: usize, line_content: String, matches: Vec<MatchResult>) -> Self {
        SearchResult {
            line,
            line_content,
            matches,
        }
    }
}

impl FileResult {
    pub fn new(path: PathBuf, results: Vec<SearchResult>) -> Self {
        FileResult { path, results }
    }
}

impl MatchResult {
    pub fn new(start: usize, end: usize, content: String) -> Self {
        MatchResult {
            match_range: (start, end),
            content,
        }
    }
}

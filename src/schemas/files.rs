use std::path::PathBuf;

#[derive(Debug)]
pub struct FileResult {
    pub path: PathBuf,
    pub results: Vec<SearchResult>,
}

#[derive(Debug)]
pub struct SearchResult {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub content: String,
    pub line_content: String,
}

impl SearchResult {
    pub fn new(
        line: usize,
        start: usize,
        end: usize,
        content: String,
        line_content: String,
    ) -> Self {
        SearchResult {
            line,
            start,
            end,
            content,
            line_content,
        }
    }
}

impl FileResult {
    pub fn new(path: PathBuf, results: Vec<SearchResult>) -> Self {
        FileResult { path, results }
    }
}

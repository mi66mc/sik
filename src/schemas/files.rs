use std::path::PathBuf;

pub struct FileResult {
    pub path: PathBuf,
    pub results: Vec<SearchResult>,
}

pub struct SearchResult {
    pub line_n: u64,
    pub char_n: u64,
    pub content: String,
}

impl SearchResult {
    pub fn new(line_n: u64, char_n: u64, content: String) -> Self {
        SearchResult {
            line_n,
            char_n,
            content,
        }
    }
}

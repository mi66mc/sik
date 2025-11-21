use regex::Regex;

use crate::{
    errors::custom_errors::AppError,
    schemas::files::{FileResult, SearchResult},
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

pub fn process_file(
    rx: Arc<Mutex<Receiver<PathBuf>>>,
    pattern: Regex,
    tx: Sender<FileResult>,
) -> Result<(), AppError> {
    loop {
        let msg = {
            let rx = rx.lock()?;
            rx.recv()
        };

        match msg {
            Ok(path) => {
                let mut results: Vec<SearchResult> = Vec::new();

                let file = File::open(&path)?;
                let buff = BufReader::new(file);
                let mut l: usize = 0;

                for line in buff.lines() {
                    l += 1;
                    let line = line?;
                    for m in pattern.find_iter(&line) {
                        results.push(SearchResult::new(
                            l,
                            m.start(),
                            m.end(),
                            m.as_str().to_string(),
                            line.clone(),
                        ));
                    }
                }

                if !results.is_empty() {
                    tx.send(FileResult::new(path, results))?
                }
            }
            Err(_) => break,
        }
    }
    Ok(())
}

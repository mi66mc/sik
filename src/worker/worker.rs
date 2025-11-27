use regex::Regex;

use crate::{
    errors::custom_errors::AppError,
    schemas::files::{FileResult, MatchResult, SearchResult},
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

const MAX_FILE_SIZE: u64 = 2 * 1024 * 1024;

fn is_binary(file: &mut File) -> bool {
    let mut buf = [0u8; 8192]; // 8kb
    if let Ok(n) = file.read(&mut buf) {
        return buf[..n].contains(&0);
    }
    false
}

pub fn process_file(
    rx: Arc<Mutex<Receiver<PathBuf>>>,
    pattern: Regex,
    result_tx: Sender<FileResult>,
    prog_tx: Sender<()>,
) -> Result<(), AppError> {
    loop {
        let msg = {
            let rx = rx.lock()?;
            rx.recv()
        };

        if let Err(_) = msg {
            break;
        }

        let path = msg.unwrap();

        let mut results: Vec<SearchResult> = Vec::new();

        let file = File::open(&path)?;

        // FIXME: this is retarded, need to think in better handler for progress without repeating
        // :(
        prog_tx.send(())?;

        if (file.metadata()?.len() > MAX_FILE_SIZE) || {
            let mut temp = File::open(&path)?;
            is_binary(&mut temp)
        } {
            continue;
        }

        let mut buff = BufReader::new(file);
        let mut line_no = 0;
        let mut bytes = Vec::new();

        loop {
            bytes.clear();

            let n = buff.read_until(b'\n', &mut bytes)?;
            if n == 0 {
                break;
            }

            line_no += 1;

            let text = String::from_utf8_lossy(&bytes);
            let mut matches: Vec<MatchResult> = Vec::new();

            for m in pattern.find_iter(&text) {
                matches.push(MatchResult::new(m.start(), m.end(), m.as_str().to_string()));
            }

            if !matches.is_empty() {
                results.push(SearchResult::new(line_no, text.to_string(), matches));
            }
        }

        if !results.is_empty() {
            result_tx.send(FileResult::new(path, results))?
        }
    }
    Ok(())
}

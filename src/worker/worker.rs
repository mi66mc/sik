use regex::Regex;

use crate::{
    errors::custom_errors::AppError,
    schemas::files::{FileResult, SearchResult},
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
    tx: Sender<FileResult>,
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

        let mut file = File::open(&path)?;
        if file.metadata()?.len() > MAX_FILE_SIZE || is_binary(&mut file) {
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

            for m in pattern.find_iter(&text) {
                results.push(SearchResult::new(
                    line_no,
                    m.start(),
                    m.end(),
                    m.as_str().to_string(),
                    text.to_string(),
                ));
            }
        }

        if !results.is_empty() {
            tx.send(FileResult::new(path, results))?
        }
    }
    Ok(())
}

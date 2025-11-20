use regex::Regex;

use crate::{errors::custom_errors::AppError, output::printer::print_info};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, Mutex, mpsc::Receiver},
};

pub fn process_file(rx: Arc<Mutex<Receiver<PathBuf>>>, pattern: Regex) -> Result<(), AppError> {
    loop {
        let msg = {
            let rx = rx.lock()?;
            rx.recv()
        };

        match msg {
            Ok(path) => {
                print_info(&format!("FILE: {}", path.display()));

                let file = File::open(path)?;
                let buff = BufReader::new(file);

                for line in buff.lines() {
                    let line = line?;
                    if pattern.is_match(&line) {
                        print_info(&format!("CONTENT:\n{}", line));
                    }
                }
            }
            Err(_) => break,
        }
    }
    Ok(())
}

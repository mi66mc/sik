use regex::Regex;
use sik::{
    cli::args::Args,
    errors::custom_errors::AppError,
    output::printer::{print_error, print_result},
    schemas::files::FileResult,
    walker::walk,
    worker::process_file,
};

use std::{
    path::PathBuf,
    sync::{Arc, Mutex, mpsc},
    thread,
};

fn main() {
    if let Err(e) = run() {
        print_error(&e.to_string());
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let args = Args::parse();

    let path = args.path;
    let (path_tx, path_rx) = mpsc::channel::<PathBuf>();
    let (result_tx, result_rx) = mpsc::channel::<FileResult>();
    let mut workers = Vec::new();

    let walker = thread::spawn(move || -> Result<(), AppError> {
        walk(&path, &path_tx)?;
        drop(path_tx);
        Ok(())
    });

    let path_rx = Arc::new(Mutex::new(path_rx));

    for _ in 0..args.threads {
        let path_rx = Arc::clone(&path_rx);
        let result_tx = result_tx.clone();
        let p = Regex::new(&args.pattern)?;
        workers.push(thread::spawn(move || -> Result<(), AppError> {
            process_file(path_rx, p, result_tx)
        }));
    }

    walker.join().map_err(|_| AppError::ThreadPanic)??;

    for w in workers {
        w.join().map_err(|_| AppError::ThreadPanic)??;
    }

    drop(result_tx);

    for r in result_rx {
        print_result(r);
    }

    Ok(())
}

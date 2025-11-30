use regex::Regex;
use sik::{
    cli::args::Args,
    errors::custom_errors::AppError,
    output::printer::{StyledOutput, print_error, progress_bar},
    schemas::files::FileResult,
    walker::walk,
    worker::process_file,
};

use std::{
    io::{self, Write},
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
    let type_style = args.type_style;

    let path = args.path;
    let (path_tx, path_rx) = mpsc::channel::<PathBuf>();
    let (result_tx, result_rx) = mpsc::channel::<FileResult>();
    let (count_tx, count_rx) = mpsc::channel::<()>();
    let (prog_tx, prog_rx) = mpsc::channel::<()>();

    let mut workers = Vec::new();

    let walker = thread::spawn(move || -> Result<(), AppError> {
        walk(&path, &path_tx, &count_tx)?;
        drop(path_tx);
        drop(count_tx);
        Ok(())
    });

    let path_rx = Arc::new(Mutex::new(path_rx));

    for _ in 0..args.threads {
        let path_rx = Arc::clone(&path_rx);
        let result_tx = result_tx.clone();
        let prog_tx = prog_tx.clone();

        let p = Regex::new(&args.pattern)?;

        workers.push(thread::spawn(move || -> Result<(), AppError> {
            process_file(path_rx, p, result_tx, prog_tx)
        }));
    }

    drop(prog_tx);

    walker.join().map_err(|_| AppError::ThreadPanic)??;

    let mut total_files = 0;
    for () in count_rx {
        total_files += 1;
    }

    let mut processed = 0;
    for _ in prog_rx {
        processed += 1;
        progress_bar(processed, total_files)?;
    }

    // yes, this mf cleans the line after the progress bar
    print!("\r\x1b[2K");
    io::stdout().flush()?;

    drop(result_tx);

    for w in workers {
        w.join().map_err(|_| AppError::ThreadPanic)??;
    }

    for r in result_rx {
        println!("{}", StyledOutput::new(&r, type_style));
    }

    Ok(())
}

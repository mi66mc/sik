use seek::{
    cli::args::Args,
    errors::custom_errors::AppError,
    output::printer::print_error,
    walker::walk,
    worker::process_file,
};
use std::{
    sync::{mpsc, Arc, Mutex},
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
    let (tx, rx) = mpsc::channel();
    let mut workers = Vec::new();

    let walker = thread::spawn(move || -> Result<(), AppError> {
        walk(&path, &tx)?;
        drop(tx);
        Ok(())
    });

    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..args.threads {
        let rx = Arc::clone(&rx);
        workers.push(thread::spawn(move || -> Result<(), AppError> {
            process_file(rx)
        }));
    }

    walker.join().map_err(|_| AppError::ThreadPanic)??;

    for w in workers {
        w.join().map_err(|_| AppError::ThreadPanic)??;
    }

    Ok(())
}


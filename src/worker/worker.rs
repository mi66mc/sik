use crate::{errors::custom_errors::AppError, output::printer::print_info};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex, mpsc::Receiver},
};

pub fn process_file(rx: Arc<Mutex<Receiver<PathBuf>>>) -> Result<(), AppError> {
    loop {
        let msg = {
            let rx = rx.lock()?;
            rx.recv()
        };

        match msg {
            Ok(path) => {
                print_info(&format!("worker: {}", path.display()));
            }
            Err(_) => break,
        }
    }
    Ok(())
}

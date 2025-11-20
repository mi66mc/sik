use crate::errors::custom_errors::AppError;
use std::{fs, path::PathBuf, sync::mpsc};

pub fn walk(path: &str, tx: &mpsc::Sender<PathBuf>) -> Result<(), AppError> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let e_path = entry.path();

        if e_path.is_dir() {
            walk(e_path.to_str().ok_or(AppError::InvalidPath)?, tx)?;
        } else {
            tx.send(e_path)?;
        }
    }
    Ok(())
}

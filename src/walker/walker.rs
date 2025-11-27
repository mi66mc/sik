use crate::errors::custom_errors::AppError;
use std::{fs, path::PathBuf, sync::mpsc::Sender};

pub fn walk(path: &str, path_tx: &Sender<PathBuf>, count_tx: &Sender<()>) -> Result<(), AppError> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let e_path = entry.path();

        if e_path.is_dir() {
            walk(
                e_path.to_str().ok_or(AppError::InvalidPath)?,
                path_tx,
                count_tx,
            )?;
        } else {
            path_tx.send(e_path)?;
            count_tx.send(())?;
        }
    }
    Ok(())
}

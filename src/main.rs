use seek::cli::args::Args;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let path = args.path;
    let entries = fs::read_dir(path).expect("[SEEK ERROR]: Error while trying to read dir.");

    for entry in entries {
        let entry = entry.expect("[SEEK ERROR]: Couldn't read dir entry.");
        let path = entry.path();
        println!("{}", path.display());
    }

    match thread::available_parallelism() {
        Ok(non_zero_usize) => {
            let num_cpus = non_zero_usize.get();
            println!("[SEEK INFO]: Number of logical CPUs: {}", num_cpus);
        }
        Err(e) => {
            eprintln!("[SEEK ERROR]: Error getting number of CPUs: {}", e);
        }
    }

    Ok(())
}

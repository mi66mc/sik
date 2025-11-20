use crate::output::printer::{print_error, print_info};
use std::{env, num::NonZeroUsize, process::exit};

const DEFAULT_PATH: &str = ".";

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Args {
    pub pattern: String,
    pub path: String,
    pub threads: usize,
}

fn usage() {
    let program = env::args().next().unwrap_or_else(|| "seek".to_string());
    println!("Usage: {} [OPTS] <PATTERN> [PATH]", program);
    println!("\nArgs:");
    println!("  <PATTERN>             Pattern to be searched for");
    println!("  [PATH]                Path to be searched with the pattern");
    println!("\nOptions:");
    println!(
        "  -t, --threads <NUM>   Number of threads to be used, default is number of logical processors * 2",
    );
    println!("  -h, --help            Prints this message\n");

    print_info(&format!("Version: {}", VERSION));
}

impl Args {
    pub fn parse() -> Self {
        let mut args_iter = env::args().skip(1);
        let mut pattern = String::new();
        let mut path = String::new();

        let mut threads = std::thread::available_parallelism()
            .unwrap_or(NonZeroUsize::new(2).unwrap())
            .get()
            * 2;

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    usage();
                    exit(0);
                }
                "-t" | "--threads" => {
                    let num_str = match args_iter.next() {
                        Some(val) => val,
                        None => {
                            print_error("--threads is expected to receive a number");
                            usage();
                            exit(1);
                        }
                    };
                    threads = match num_str.parse() {
                        Ok(num) if num > 0 => num,
                        _ => {
                            print_error(&format!(
                                "Invalid number of threads: '{}'. Must be a positive number.",
                                num_str
                            ));
                            usage();
                            exit(1);
                        }
                    };
                }

                // unknown opt
                s if s.starts_with('-') => {
                    print_error(&format!("Unknown option: {}", s));
                    usage();
                }

                _ => {
                    if pattern.is_empty() {
                        pattern = arg;
                    } else if path.is_empty() {
                        path = arg;
                    } else {
                        print_error(&format!("Unexpected argument: {}", arg));
                        usage();
                        exit(1);
                    }
                }
            }
        }

        if pattern.is_empty() {
            print_error("Required argument <PATTERN> is missing.");
            usage();
            exit(1);
        }

        if path.is_empty() {
            path = DEFAULT_PATH.to_string();
        }

        Self {
            pattern,
            path,
            threads,
        }
    }
}

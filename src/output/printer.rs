use crate::colors::painter::{paint_blue, paint_red};

pub fn print_info(message: &str) {
    println!("{} {}", paint_blue("[SEEK INFO]:"), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", paint_red("[SEEK ERROR]:"), message);
}

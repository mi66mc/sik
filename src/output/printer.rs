use crate::{
    colors::painter::{paint_blue, paint_green, paint_red, paint_yellow},
    schemas::files::FileResult,
};

pub fn print_info(message: &str) {
    println!("{} {}", paint_blue("[SIK INFO]:"), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", paint_red("[SIK ERROR]:"), message);
}

pub fn print_result(result: FileResult) {
    println!("{}", paint_blue(result.path.to_str().unwrap()));

    for r in result.results {
        println!(
            "{}:{} {}",
            paint_green(&r.line.to_string()),
            paint_green(&r.start.to_string()),
            highlight(&r.line_content, r.start, r.end)
        );
    }

    println!();
}

fn highlight(s: &str, start: usize, end: usize) -> String {
    let before = &s[..start];
    let word = &s[start..end];
    let after = &s[end..];

    format!("{}{}{}", before, paint_yellow(word), after)
}

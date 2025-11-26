use crate::{
    colors::painter::{paint_blue, paint_green, paint_red, paint_yellow},
    schemas::files::{FileResult, MatchRange},
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
        let line = paint_green(&r.line.to_string());
        print!(
            "[{}]: {}",
            center_ansi(&line, 4),
            highlight(
                &r.line_content,
                &r.matches
                    .iter()
                    .map(|m| -> MatchRange { m.match_range })
                    .collect::<Vec<MatchRange>>(),
            )
        );
    }

    println!();
}

pub fn highlight(s: &str, matches: &Vec<MatchRange>) -> String {
    let mut matches = matches.to_owned();
    matches.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out = String::new();
    let mut last_end = 0;

    for &(start, end) in &matches {
        if start > last_end {
            out.push_str(&s[last_end..start]);
        }

        let word = &s[start..end];
        out.push_str(&paint_yellow(word));

        last_end = end;
    }

    if last_end < s.len() {
        out.push_str(&s[last_end..]);
    }

    out
}

fn visible_len(s: &str) -> usize {
    let mut count = 0;
    let mut in_esc = false;

    for c in s.chars() {
        if in_esc {
            if c == 'm' {
                in_esc = false;
            }
            continue;
        }

        if c == '\x1b' {
            in_esc = true;
            continue;
        }

        count += 1;
    }

    count
}

fn center_ansi(text: &str, width: usize) -> String {
    let vis = visible_len(text);
    if vis >= width {
        return text.to_string();
    }

    let pad = width - vis;
    let left = pad / 2;
    let right = pad - left;

    format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
}

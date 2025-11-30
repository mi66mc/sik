use crate::{
    colors::painter::{paint_blue, paint_green, paint_magenta, paint_red, paint_yellow},
    errors::custom_errors::AppError,
    schemas::files::{FileResult, MatchRange},
};

use std::{
    fmt::{self, Display},
    io::{self, Write},
};

// ----- GENERICS

pub enum DisplayMode {
    Primary,
    Secondary,
    Tertiary,
    //Enabled,
    //Disabled,
}

/// Wrapper that prints a value of type `T` using a specific [`DisplayMode`].
///
/// This type does **not** define how `T` is printed by default. Instead, the caller must
/// implement [`Display`] for `StyledOutput<'a, T>` manually for each supported type.
///
/// This allows you to define multiple visual styles (e.g. *primary*, *secondary*) for the
/// same underlying data type.
///
/// # Type Parameters
/// - `'a`: Lifetime of the borrowed value.
/// - `T`: The wrapped type.
///
/// # Example
///
/// ```rust
/// // Assume `FileResult` implements:
/// // impl Display for StyledOutput<'_, FileResult> { ... }
///
/// let file: FileResult = ...;
/// let out = StyledOutput::new(&file, DisplayMode::Primary);
/// println!("{out}");
/// ```
pub struct StyledOutput<'a, T> {
    value: &'a T,
    mode: DisplayMode,
}

impl<'a, T> StyledOutput<'a, T>
where
    Self: Display,
{
    /// Creates a new styled wrapper for a given value and display mode.
    ///
    /// This constructor is only available if [`Display`] is implemented for
    /// `StyledOutput<'a, T>`. This prevents constructing wrappers that cannot
    /// be formatted.
    pub fn new(value: &'a T, mode: DisplayMode) -> Self {
        StyledOutput { value, mode }
    }
}

// ----- FileResult

impl Display for StyledOutput<'_, FileResult> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mode {
            DisplayMode::Tertiary => {
                write!(
                    f,
                    "{}\n",
                    paint_blue(&self.value.path.to_str().ok_or(fmt::Error)?)
                )?;

                for r in &self.value.results {
                    let message_line = format!("\n-> LINE {}: ", &r.line.to_string()).to_string();
                    let line = paint_yellow(&message_line);
                    print!("{}", center_ansi(&line, 4));
                    print!(
                        "\n{}",
                        highlight(
                            &r.line_content,
                            &r.matches
                                .iter()
                                .map(|m| -> MatchRange {
                                    m.match_range
                                })
                                .collect::<Vec<MatchRange>>(),
                        )
                    );
                    let line_end = paint_red("\n<---------//---------->\n");
                    print!("{}", line_end);
                }
            }

            DisplayMode::Secondary => {
                write!(
                    f,
                    "{}\n",
                    paint_blue(&self.value.path.to_str().ok_or(fmt::Error)?)
                )?;

                for r in &self.value.results {
                    let line = paint_green(&r.line.to_string());
                    print!(
                        "[{}] {}: {}",
                        center_ansi(&line, 4),
                        paint_magenta(&format!(
                            "@({})",
                            &r.matches
                                .iter()
                                .map(|m| -> String {
                                    format!("{}-{}", m.match_range.0, m.match_range.1)
                                })
                                .collect::<Vec<String>>()
                                .join(", ")
                        )),
                        highlight(
                            &r.line_content,
                            &r.matches
                                .iter()
                                .map(|m| -> MatchRange { m.match_range })
                                .collect::<Vec<MatchRange>>(),
                        )
                    );
                }
            }

            // primary and fallback, this output can just be primary or secondary yet
            _ => {
                write!(
                    f,
                    "{}\n",
                    paint_blue(&self.value.path.to_str().ok_or(fmt::Error)?)
                )?;

                for r in &self.value.results {
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
            }
        }
        Ok(())
    }
}

// ----------------------------------------------

pub fn print_info(message: &str) {
    println!("{} {}", paint_blue("[SIK INFO]:"), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", paint_red("[SIK ERROR]:"), message);
}

//pub fn print_result(result: FileResult) {
//    println!("{}", paint_blue(result.path.to_str().unwrap()));
//
//    for r in result.results {
//        let line = paint_green(&r.line.to_string());
//        print!(
//            "[{}]: {}",
//            center_ansi(&line, 4),
//            highlight(
//                &r.line_content,
//                &r.matches
//                    .iter()
//                    .map(|m| -> MatchRange { m.match_range })
//                    .collect::<Vec<MatchRange>>(),
//            )
//        );
//    }
//
//    println!();
//}

const SYMBOLS: [&str; 8] = ["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];

pub fn progress_bar(current: usize, total: usize) -> Result<(), AppError> {
    let width = 40;

    let ratio = current as f32 / total as f32;
    let filled = (ratio * width as f32) as usize;

    // number of steps the ball will do until 100% :)
    let total_steps = 10 * SYMBOLS.len();

    let step = (ratio * total_steps as f32) as usize;

    let symbol = SYMBOLS[step % SYMBOLS.len()];

    print!(
        "\r{} [{}{}] {}/{}",
        symbol,
        "=".repeat(filled),
        " ".repeat(width - filled),
        current,
        total
    );

    io::stdout().flush()?;

    Ok(())
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

pub fn visible_len(s: &str) -> usize {
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

pub fn center_ansi(text: &str, width: usize) -> String {
    let vis = visible_len(text);
    if vis >= width {
        return text.to_string();
    }

    let pad = width - vis;
    let left = pad / 2;
    let right = pad - left;

    format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
}

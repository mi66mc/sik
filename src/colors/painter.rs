const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

pub fn paint_red(text: &str) -> String {
    format!("{}{}{}", RED, text, RESET)
}

pub fn paint_green(text: &str) -> String {
    format!("{}{}{}", GREEN, text, RESET)
}

pub fn paint_blue(text: &str) -> String {
    format!("{}{}{}", BLUE, text, RESET)
}

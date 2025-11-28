// FIXME: REFACTOR ALL THIS SHIT, HORRIBLE, NEEDS TO BE FLEXIBLE, PROBABLY CREATE A FUNCTION AND
// PASS AN ENUM OF COLORS TO PAINT IT

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const YELLOW: &str = "\x1b[1;33m";
const MAGENTA: &str = "\x1b[35m";
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

pub fn paint_yellow(text: &str) -> String {
    format!("{}{}{}", YELLOW, text, RESET)
}

pub fn paint_magenta(text: &str) -> String {
    format!("{}{}{}", MAGENTA, text, RESET)
}

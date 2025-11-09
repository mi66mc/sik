use std::env::args;

pub struct Args {
    pub pattern: String,
    pub path: String,
}

impl Args {
    pub fn parse() -> Self {
        let mut args = args().skip(1);

        let mut this = Self {
            pattern: String::new(),
            path: ".".to_string(),
        };

        while let Some(arg) = args.next() {
            this.match_arg(&arg);
        }

        if this.pattern.is_empty() {
            eprintln!("uso: meugrep <pattern> [path]");
            std::process::exit(1);
        }

        this
    }

    fn match_arg(&mut self, arg: &str) {
        if self.pattern.is_empty() {
            self.pattern = arg.to_string();
        } else {
            self.path = arg.to_string();
        }
    }
}


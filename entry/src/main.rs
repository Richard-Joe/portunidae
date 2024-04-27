use flags::{generate_help, ParseResult, Parser, SpecialMode};
use std::io::Write;

fn main() {
    let parser = Parser::new();
    let result = parser.parse(std::env::args_os());
    match result {
        ParseResult::Err(err) => {
            println!("{}", err);
            return;
        }
        ParseResult::Special(mode) => return special(mode),
        ParseResult::Ok(args) => args,
    };
}

fn special(mode: SpecialMode) {
    let output = match mode {
        SpecialMode::HelpShort => generate_help(),
        SpecialMode::HelpLong => generate_help(),
    };
    writeln!(std::io::stdout(), "{}", output.trim_end()).unwrap();
}

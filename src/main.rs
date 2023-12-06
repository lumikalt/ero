use ero::parser::{tokenize, parse};

fn main() {
    dbg!(parse(tokenize("2 x".to_string())));
}

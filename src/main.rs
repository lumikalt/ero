use ero::engine::derive;
use ero::parser::{parse, tokenize};

fn main() {
    dbg!(derive(parse(tokenize("2x".to_string()))));
}

use ero::engine::derive;
use ero::parser::parse;

fn main() {
    let expr = "2".to_string();
    let result = derive(parse(&expr.clone()), 'x');

    // println!("({})' = {}", expr.clone(), dbg!(result).to_string());
    dbg!(result);
}

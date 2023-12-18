use ero::{parser::parse, engine::{derive, simplify}};

fn main() {
    let src = "-LN(EXP(x))^3";

    let result = simplify(simplify(derive(dbg!(simplify(parse(src))), 'x')));

    println!("({})' = {}", src, result);
}

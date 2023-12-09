use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Functions {
    Plus,
    Minus,
    Times,
    Divide,
    Pow,
    Root(i32),
    E,
    Ln(i32),
}

#[derive(Debug, Clone)]
pub enum Tokens {
    Var(char),
    Number(f64),
    Function(Functions, Vec<Tokens>),
}

#[derive(Debug, Clone)]
pub enum Types {
    BinaryFn(Functions, Box<[Ast; 2]>),
    UnaryFn(Functions, Box<[Ast; 1]>),
    Constant(f64),
    Var(char),
    Todo,
}

impl From<Tokens> for Types {
    fn from(token: Tokens) -> Types {
        match token {
            Tokens::Var(v) => Types::Var(v),
            Tokens::Number(c) => Types::Constant(c),
            Tokens::Function(f, args) => Types::BinaryFn(
                f,
                Box::new([
                    Ast(args[0].clone().into()),
                    Ast(args[1].clone().into()),
                ]),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ast(pub Types);

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result<(), fmt::Error> {
        
    }
}

pub fn tokenize(input: String) -> Vec<Tokens> {
    let mut chars = input.chars().peekable();
    let mut tokens = vec![];

    while let Some(c) = chars.next() {
        let token = match c {
            '0'..='9' => {
                let mut num = String::new();
                num.push(c);
                while let Some('0'..='9') = chars.peek() {
                    num.push(dbg!(chars.next().unwrap()));
                }
                Tokens::Number(num.parse::<f64>().unwrap())
            }
            'x' => Tokens::Var(c),
            _ => unimplemented!(),
        };

        tokens.push(token);
    }

    tokens
}

pub fn parse(tokens: Vec<Tokens>) -> Ast {
    use Tokens::*;

    let mut ast = Ast(Types::Todo);
    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        ast.0 = match token {
            Number(c) => {
                if let Some(next) = tokens.next() {
                    match next {
                        Function(f, args) => {
                            if args.len() == 1 {
                                Types::UnaryFn(f, Box::new([parse(vec![args[0].clone().into()])]))
                            } else if args.len() == 2 {
                                Types::BinaryFn(
                                    f,
                                    Box::new([
                                        Ast(args[0].clone().into()),
                                        Ast(args[1].clone().into()),
                                    ]),
                                )
                            } else {
                                unreachable!()
                            }
                        }
                        Var(var) => Types::BinaryFn(
                            Functions::Times,
                            Box::new([Ast(Types::Constant(c)), Ast(Types::Var(var))]),
                        ),
                        _ => unreachable!(),
                    }
                } else {
                    break;
                }
            }
            _ => unimplemented!(),
        };
    }

    ast
}

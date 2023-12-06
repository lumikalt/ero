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

#[derive(Debug)]
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
            Tokens::Function(f, args) => Types::BinaryFn(f, Box::new([
                Ast::new(args[0].clone().into()),
                Ast::new(args[1].clone().into())
            ])),
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub branch: Option<Box<Ast>>,
    pub node: Types,
}

impl Ast {
    fn new(value: Types) -> Ast {
        Ast {
            branch: None,
            node: value,
        }
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

    let mut ast = Ast {
        branch: None,
        node: Types::Todo,
    };
    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        ast.node = match token {
            Number(c) => {
                if let Some(next) = tokens.next() {
                    match next {
                        Function(f, args) => {
                            if args.len() == 1 {
                                Types::UnaryFn(
                                    f, Box::new([parse(vec![args[0].clone().into()])])
                                )
                            } else if args.len() == 2 {
                                Types::BinaryFn(f, Box::new([Ast::new(args[0].clone().into()), Ast::new(args[1].clone().into())]))
                            } else {
                                unreachable!()
                            }
                        }
                        Var(var) => Types::BinaryFn(
                            Functions::Times,
                            Box::new([Ast::new(Types::Constant(c)), Ast::new(Types::Var(var))]),
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

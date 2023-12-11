use std::fmt::{self, Display, Formatter};

use lazy_static::lazy_static;
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Equation;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left))
            .op(Op::infix(pow, Right))
            .op(Op::prefix(neg))
    };
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    Pow,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Constant(i64),
    Var(char),
    BinaryFn {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Plus => write!(f, "+"),
            BinOp::Minus => write!(f, "-"),
            BinOp::Times => write!(f, "*"),
            BinOp::Divide => write!(f, "/"),
            BinOp::Pow => write!(f, "^"),
        }
    }
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    dbg!(PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::int => Expr::Constant(dbg!(primary.as_str().parse::<i64>().unwrap())),
            Rule::var => Expr::Var(dbg!(primary.as_str().chars().next().unwrap())),
            Rule::expr => parse_expr(primary.into_inner()),
            _ => unreachable!("{}", primary.as_str()),
        })
        .map_infix(|lhs, infix, rhs| {
            let lhs = Box::new(lhs);
            let rhs = Box::new(rhs);

            let op = match infix.as_rule() {
                Rule::add => BinOp::Plus,
                Rule::sub => BinOp::Minus,
                Rule::mul => BinOp::Times,
                Rule::div => BinOp::Divide,
                Rule::pow => BinOp::Pow,
                _ => unreachable!(),
            };

            Expr::BinaryFn { op, lhs, rhs }
        })
        .map_prefix(|prefix, rhs| {
            let op = match prefix.as_rule() {
                Rule::neg => BinOp::Minus,
                _ => unreachable!(),
            };

            let lhs = Box::new(Expr::Constant(0));
            let rhs = Box::new(rhs);

            Expr::BinaryFn { op, lhs, rhs }
        })
        .parse(pairs))
}

pub fn parse(input: &str) -> Expr {
    match Equation::parse(Rule::equation, input) {
        Ok(mut pairs) => parse_expr(pairs.next().unwrap().into_inner()),
        Err(e) => unimplemented!("{}", e),
    }
}

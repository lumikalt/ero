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
    let pairs = dbg!(pairs);
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::int => Expr::Constant(primary.as_str().parse::<i64>().unwrap()),
            Rule::var => Expr::Var(dbg!(primary.as_str().chars().next().unwrap())),
            Rule::expr => parse_expr(dbg!(primary.into_inner())),
            _ => unreachable!(),
        })
        .map_infix(|lhs, infix, rhs| {
            let lhs = Box::new(lhs);
            let rhs = Box::new(rhs);

            match infix.as_rule() {
                Rule::add => Expr::BinaryFn {
                    op: BinOp::Plus,
                    lhs,
                    rhs,
                },
                Rule::sub => Expr::BinaryFn {
                    op: BinOp::Minus,
                    lhs,
                    rhs,
                },
                Rule::mul => Expr::BinaryFn {
                    op: BinOp::Times,
                    lhs,
                    rhs,
                },
                Rule::div => Expr::BinaryFn {
                    op: BinOp::Divide,
                    lhs,
                    rhs,
                },
                Rule::pow => Expr::BinaryFn {
                    op: BinOp::Pow,
                    lhs,
                    rhs,
                },
                _ => unreachable!(),
            }
        })
        .parse(pairs)
}

pub fn parse(input: &str) -> Expr {
    Equation::parse(Rule::expr, input)
        .unwrap_or_else(|e| unreachable!("{}", e))
        .map(|pairs| parse_expr(pairs.into_inner().next().unwrap().into_inner()))
        .next()
        .unwrap()
}

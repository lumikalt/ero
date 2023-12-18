use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::grammar;

#[derive(Debug, Clone)]
pub enum Func {
    Exp,
    Ln,
    Log(usize),
    Root(usize),
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Sinh,
    Cosh,
    Tanh,
    Arcsinh,
    Arccosh,
    Arctanh,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    Power,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Constant(i64),
    Var(char),
    Neg(Box<Expr>),
    FunctionCall {
        op: Func,
        arg: Box<Expr>,
    },
    BinaryFn {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Func::Exp => write!(f, "EXP"),
            Func::Ln => write!(f, "LN"),
            Func::Log(base) => write!(f, "LOG[{}]", base),
            Func::Root(base) => write!(f, "ROOT[{}]", base),
            Func::Sin => write!(f, "SIN"),
            Func::Cos => write!(f, "COS"),
            Func::Tan => write!(f, "TAN"),
            Func::Arcsin => write!(f, "ARCSIN"),
            Func::Arccos => write!(f, "ARCCOS"),
            Func::Arctan => write!(f, "ARCTAN"),
            Func::Sinh => write!(f, "SINH"),
            Func::Cosh => write!(f, "COSH"),
            Func::Tanh => write!(f, "TANH"),
            Func::Arcsinh => write!(f, "ARCSINH"),
            Func::Arccosh => write!(f, "ARCCOSH"),
            Func::Arctanh => write!(f, "ARCTANH"),
        }
    }
}

impl FromStr for Func {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exp" => Ok(Func::Exp),
            "ln" => Ok(Func::Ln),
            "sin" => Ok(Func::Sin),
            "cos" => Ok(Func::Cos),
            "tan" => Ok(Func::Tan),
            "arcsin" => Ok(Func::Arcsin),
            "arccos" => Ok(Func::Arccos),
            "arctan" => Ok(Func::Arctan),
            "sinh" => Ok(Func::Sinh),
            "cosh" => Ok(Func::Cosh),
            "tanh" => Ok(Func::Tanh),
            "arcsinh" => Ok(Func::Arcsinh),
            "arccosh" => Ok(Func::Arccosh),
            "arctanh" => Ok(Func::Arctanh),
            _ => Err(format!("Unknown function: {}", s)),
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Plus => write!(f, "+"),
            BinOp::Minus => write!(f, "-"),
            BinOp::Times => write!(f, "*"),
            BinOp::Divide => write!(f, "/"),
            BinOp::Power => write!(f, "^"),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Constant(c) => write!(f, "{}", c),
            Expr::Var(v) => write!(f, "{}", v),
            Expr::Neg(e) => write!(f, "-({})", e),
            Expr::FunctionCall { op, arg } => write!(f, "{}({})", op, arg),
            Expr::BinaryFn { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
        }
    }
}

impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Func::Exp, Func::Exp) => true,
            (Func::Ln, Func::Ln) => true,
            (Func::Log(a), Func::Log(b)) => a == b,
            (Func::Root(a), Func::Root(b)) => a == b,
            (Func::Sin, Func::Sin) => true,
            (Func::Cos, Func::Cos) => true,
            (Func::Tan, Func::Tan) => true,
            (Func::Arcsin, Func::Arcsin) => true,
            (Func::Arccos, Func::Arccos) => true,
            (Func::Arctan, Func::Arctan) => true,
            (Func::Sinh, Func::Sinh) => true,
            (Func::Cosh, Func::Cosh) => true,
            (Func::Tanh, Func::Tanh) => true,
            (Func::Arcsinh, Func::Arcsinh) => true,
            (Func::Arccosh, Func::Arccosh) => true,
            (Func::Arctanh, Func::Arctanh) => true,
            _ => false,
        }
    }
}

impl PartialEq for BinOp {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BinOp::Plus, BinOp::Plus) => true,
            (BinOp::Minus, BinOp::Minus) => true,
            (BinOp::Times, BinOp::Times) => true,
            (BinOp::Divide, BinOp::Divide) => true,
            (BinOp::Power, BinOp::Power) => true,
            _ => false,
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Constant(a), Expr::Constant(b)) => a == b,
            (Expr::Var(a), Expr::Var(b)) => a == b,
            (Expr::Neg(a), Expr::Neg(b)) => a == b,
            (
                Expr::FunctionCall { op: a_op, arg: a_arg },
                Expr::FunctionCall { op: b_op, arg: b_arg },
            ) => a_op == b_op && a_arg == b_arg,
            (
                Expr::BinaryFn {
                    op: a_op,
                    lhs: a_lhs,
                    rhs: a_rhs,
                },
                Expr::BinaryFn {
                    op: b_op,
                    lhs: b_lhs,
                    rhs: b_rhs,
                },
            ) => a_op == b_op && a_lhs == b_lhs && a_rhs == b_rhs,
            _ => false,
        }
    }
}

pub fn parse<'src>(src: &'src str) -> Expr {
    grammar::ExprParser::new()
        .parse(src)
        .expect("Failed to parse")
}

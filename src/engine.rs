use crate::parser::{BinOp, Expr};

pub fn derive(expr: Expr, var: char) -> Expr {
    use BinOp::*;
    use Expr::*;

    match expr {
        Constant(_) => Constant(0),
        Var(c) if c == var => Constant(1),
        Var(_) => Constant(0),
        BinaryFn { op, lhs, rhs } =>{
            let lhs = *lhs.clone();
            let rhs = *rhs.clone();

             match op {
            Plus => BinaryFn {
                op: BinOp::Plus,
                lhs: Box::new(derive(lhs, var)),
                rhs: Box::new(derive(rhs, var)),
            },
            Minus => BinaryFn {
                op: BinOp::Minus,
                lhs: Box::new(derive(lhs, var)),
                rhs: Box::new(derive(rhs, var)),
            },
            Times => BinaryFn {
                op: Plus,
                lhs: Box::new(BinaryFn {
                    op: BinOp::Times,
                    lhs: Box::new(derive(lhs.clone(), var)),
                    rhs: Box::new(rhs.clone()),
                }),
                rhs: Box::new(BinaryFn {
                    op: BinOp::Times,
                    lhs: Box::new(lhs.clone()),
                    rhs: Box::new(derive(rhs, var)),
                }),
            },
            Divide => BinaryFn {
                op: BinOp::Divide,
                lhs: Box::new(BinaryFn {
                    op: BinOp::Minus,
                    lhs: Box::new(BinaryFn {
                        op: BinOp::Times,
                        lhs: Box::new(derive(lhs.clone(), var)),
                        rhs:Box::new(rhs.clone()),
                    }),
                    rhs: Box::new(BinaryFn {
                        op: BinOp::Times,
                        lhs: Box::new(lhs.clone()),
                        rhs: Box::new(derive(rhs.clone(), var)),
                    }),
                }),
                rhs: Box::new(BinaryFn {
                    op: BinOp::Pow,
                    lhs: Box::new(rhs.clone()),
                    rhs: Box::new(Constant(2)),
                }),
            },
            BinOp::Pow => BinaryFn {
                op: BinOp::Times,
                lhs: Box::new(BinaryFn {
                    op: BinOp::Times,
                    lhs: Box::new(rhs.clone()),
                    rhs: Box::new(derive(lhs.clone(), var)),
                }),
                rhs: Box::new(BinaryFn {
                    op: BinOp::Pow,
                    lhs: Box::new(lhs.clone()),
                    rhs: Box::new(BinaryFn {
                        op: BinOp::Minus,
                        lhs: Box::new(rhs.clone()),
                        rhs: Box::new(Constant(1)),
                    }),
                }),
            },
        }
    }
    }
}

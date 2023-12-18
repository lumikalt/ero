use crate::parser::{BinOp, Expr, Func};

pub fn derive(expr: Expr, var: char) -> Expr {
    use BinOp::*;
    use Expr::*;

    match expr {
        Constant(_) => Constant(0),
        Var(c) if c == var => Constant(1),
        Var(_) => Constant(0),
        FunctionCall { op, arg } => {
            let arg = *arg.clone();

            match op {
                Func::Exp => BinaryFn {
                    op: BinOp::Times,
                    lhs: Box::new(FunctionCall {
                        op: Func::Exp,
                        arg: Box::new(arg.clone()),
                    }),
                    rhs: Box::new(derive(arg, var)),
                },
                Func::Ln => BinaryFn {
                    op: BinOp::Divide,
                    lhs: Box::new(derive(arg.clone(), var)),
                    rhs: Box::new(arg),
                },
                Func::Sin => BinaryFn {
                    op: BinOp::Times,
                    lhs: Box::new(FunctionCall {
                        op: Func::Cos,
                        arg: Box::new(arg.clone()),
                    }),
                    rhs: Box::new(derive(arg, var)),
                },
                Func::Cos => Neg(Box::new(BinaryFn {
                    op: BinOp::Times,
                    lhs: Box::new(FunctionCall {
                        op: Func::Sin,
                        arg: Box::new(arg.clone()),
                    }),
                    rhs: Box::new(derive(arg, var)),
                })),
                Func::Tan => todo!(),
                Func::Arcsin => todo!(),
                Func::Arccos => todo!(),
                Func::Log(_n) => {
                    todo!()
                }
                Func::Root(_n) => {
                    todo!("root")
                }
                Func::Arctan => todo!(),
                Func::Sinh => BinaryFn {
                    op: BinOp::Times,
                    lhs: Box::new(FunctionCall {
                        op: Func::Cosh,
                        arg: Box::new(arg.clone()),
                    }),
                    rhs: Box::new(derive(arg, var)),
                },
                Func::Cosh => BinaryFn {
                    op: BinOp::Times,
                    lhs: Box::new(FunctionCall {
                        op: Func::Sinh,
                        arg: Box::new(arg.clone()),
                    }),
                    rhs: Box::new(derive(arg, var)),
                },
                Func::Tanh => todo!(),
                Func::Arcsinh => todo!(),
                Func::Arccosh => todo!(),
                Func::Arctanh => todo!(),
            }
        }
        BinaryFn { op, lhs, rhs } => {
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
                            rhs: Box::new(rhs.clone()),
                        }),
                        rhs: Box::new(BinaryFn {
                            op: BinOp::Times,
                            lhs: Box::new(lhs.clone()),
                            rhs: Box::new(derive(rhs.clone(), var)),
                        }),
                    }),
                    rhs: Box::new(BinaryFn {
                        op: BinOp::Power,
                        lhs: Box::new(rhs.clone()),
                        rhs: Box::new(Constant(2)),
                    }),
                },
                Power => BinaryFn {
                    op: BinOp::Times,
                    lhs: Box::new(BinaryFn {
                        op: BinOp::Times,
                        lhs: Box::new(rhs.clone()),
                        rhs: Box::new(derive(lhs.clone(), var)),
                    }),
                    rhs: Box::new(BinaryFn {
                        op: BinOp::Power,
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
        Neg(expr) => {
            let expr = *expr.clone();

            Neg(Box::new(derive(expr, var)))
        }
    }
}

pub fn simplify(expr: Expr) -> Expr {
    use BinOp::*;
    use Expr::*;

    match expr.clone() {
        Constant(_) => expr,
        Var(_) => expr,
        Neg(expr) => {
            let expr = *expr.clone();

            match expr {
                Constant(a) => Constant(-a),
                Neg(expr) => *expr,
                _ => Neg(Box::new(simplify(expr))),
            }
        }
        FunctionCall { op, arg } => {
            let arg = *arg.clone();

            match op {
                Func::Exp => match arg.clone() {
                    Constant(0) => Constant(1),
                    Constant(a) if a < 0 => BinaryFn {
                        op: BinOp::Divide,
                        lhs: Box::new(Constant(1)),
                        rhs: Box::new(FunctionCall {
                            op: Func::Exp,
                            arg: Box::new(Constant(-a)),
                        }),
                    },
                    BinaryFn {
                        op: BinOp::Times,
                        lhs,
                        rhs,
                    } => match (*lhs, *rhs) {
                        (Constant(a), x) | (x, Constant(a)) => BinaryFn {
                            op: BinOp::Power,
                            lhs: Box::new(FunctionCall {
                                op: Func::Exp,
                                arg: Box::new(x),
                            }),
                            rhs: Box::new(Constant(a)),
                        },
                        (lhs, rhs) => BinaryFn {
                            op: BinOp::Power,
                            lhs: Box::new(FunctionCall {
                                op: Func::Exp,
                                arg: Box::new(lhs),
                            }),
                            rhs: Box::new(rhs),
                        },
                    },
                    FunctionCall { op, arg } => match op {
                        Func::Ln => *arg,
                        _ => FunctionCall {
                            op: Func::Exp,
                            arg: Box::new(simplify(*arg)),
                        },
                    },
                    _ => FunctionCall {
                        op: Func::Exp,
                        arg: Box::new(simplify(arg)),
                    },
                },
                Func::Ln => match arg.clone() {
                    Constant(1) => Constant(0),
                    Constant(a) if a < 0 => Neg(Box::new(FunctionCall {
                        op: Func::Ln,
                        arg: Box::new(Constant(-a)),
                    })),
                    FunctionCall { op, arg } => match op {
                        Func::Exp => *arg,
                        _ => FunctionCall {
                            op: Func::Ln,
                            arg: Box::new(simplify(*arg)),
                        },
                    },
                    BinaryFn { op, lhs, rhs } => match op {
                        BinOp::Power => match (*lhs, *rhs) {
                            (lhs, Constant(a)) => BinaryFn {
                                op: BinOp::Times,
                                lhs: Box::new(Constant(a)),
                                rhs: Box::new(FunctionCall {
                                    op: Func::Ln,
                                    arg: Box::new(lhs),
                                }),
                            },
                            (lhs, rhs) => BinaryFn {
                                op: BinOp::Times,
                                lhs: Box::new(rhs),
                                rhs: Box::new(FunctionCall {
                                    op: Func::Ln,
                                    arg: Box::new(lhs),
                                }),
                            },
                        },
                        BinOp::Divide if matches!(*lhs.clone(), Constant(1)) => BinaryFn {
                            op: BinOp::Minus,
                            lhs: Box::new(FunctionCall {
                                op: Func::Ln,
                                arg: rhs,
                            }),
                            rhs: Box::new(Constant(1)),
                        },

                        _ => FunctionCall {
                            op: Func::Ln,
                            arg: Box::new(simplify(arg)),
                        },
                    },
                    _ => FunctionCall {
                        op: Func::Ln,
                        arg: Box::new(simplify(arg)),
                    },
                },
                expr => FunctionCall {
                    op: expr,
                    arg: Box::new(simplify(arg)),
                },
            }
        }
        BinaryFn { op, lhs, rhs } => {
            let lhs = *lhs.clone();
            let rhs = *rhs.clone();

            match op {
                Plus => {
                    let lhs = simplify(lhs);
                    let rhs = simplify(rhs);

                    match (lhs, rhs) {
                        (Constant(a), Constant(b)) => Constant(a + b),
                        (Constant(0), x) => x,
                        (x, Constant(0)) => x,
                        (x, y) => BinaryFn {
                            op: BinOp::Plus,
                            lhs: Box::new(x),
                            rhs: Box::new(y),
                        },
                    }
                }
                Minus => {
                    let lhs = simplify(lhs);
                    let rhs = simplify(rhs);

                    match (lhs, rhs) {
                        (Constant(a), Constant(b)) => Constant(a - b),
                        (Constant(0), x) => Neg(Box::new(x)),
                        (x, Constant(0)) => x,
                        (x, y) => BinaryFn {
                            op: BinOp::Minus,
                            lhs: Box::new(x),
                            rhs: Box::new(y),
                        },
                    }
                }
                Times => {
                    let lhs = simplify(lhs);
                    let rhs = simplify(rhs);

                    match (lhs, rhs) {
                        (Constant(a), Constant(b)) => Constant(a * b),
                        (Constant(0), _) => Constant(0),
                        (_, Constant(0)) => Constant(0),
                        (Constant(1), x) => x,
                        (x, Constant(1)) => x,
                        (x, y) => BinaryFn {
                            op: BinOp::Times,
                            lhs: Box::new(x),
                            rhs: Box::new(y),
                        },
                    }
                }
                Divide => {
                    let lhs = simplify(lhs);
                    let rhs = simplify(rhs);

                    match (lhs, rhs) {
                        (Constant(a), Constant(b)) => Constant(a / b),
                        (Constant(0), _) => Constant(0),
                        (x, Constant(1)) => x,
                        (x, Constant(-1)) => Neg(Box::new(x)),
                        (x, Neg(rhs)) => Neg(Box::new(BinaryFn {
                            op: BinOp::Divide,
                            lhs: Box::new(x),
                            rhs,
                        })),
                        (x, y) if x == y => Constant(1),
                        (x, y) => BinaryFn {
                            op: BinOp::Divide,
                            lhs: Box::new(x),
                            rhs: Box::new(y),
                        },
                    }
                }
                Power => {
                    let lhs = simplify(lhs);
                    let rhs = simplify(rhs);

                    match (lhs, rhs) {
                        (Constant(a), Constant(b)) => Constant(a.pow(b as u32)),
                        (x, Constant(1)) => x,
                        (_x, Constant(0)) => Constant(1),
                        (x, y) => BinaryFn {
                            op: BinOp::Power,
                            lhs: Box::new(x),
                            rhs: Box::new(y),
                        },
                    }
                }
            }
        }
    }
}

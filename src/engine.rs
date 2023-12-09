use crate::parser::{Ast, Types};

pub fn derive(ast: Ast) -> Ast {
    match ast {
        Ast(Types::BinaryFn(f, args)) => {
            use crate::parser::Functions::*;
            match f {
                Times => Ast(Types::BinaryFn(
                    Plus,
                    Box::new([
                        Ast(Types::BinaryFn(
                            Times,
                            Box::new([(derive(args[0].clone())), args[1].clone()]),
                        )),
                        Ast(Types::BinaryFn(
                            Times,
                            Box::new([(derive(args[1].clone())), args[0].clone()]),
                        )),
                    ]),
                )),

                _ => todo!(),
            }
        }
        Ast(Types::Constant(_)) => Ast(Types::Constant(0.0)),
        Ast(Types::Var(_)) => Ast(Types::Constant(1.0)),
        _ => unimplemented!(),
    }
}

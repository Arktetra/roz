use crate::{interpreter::Interpreter, literal::Literal};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> Literal;
}


pub mod expr;
pub mod parse;
pub mod bytecode;
pub mod interpreter;


#[cfg(test)]
mod tests {
    use crate::{interpreter::Interpreter, bytecode::ByteCode};

    #[test]
    fn it_works() {
        let path = String::from("test.br");
        let byte_code = ByteCode::from(&path);
        let mut interpreter = Interpreter::new(byte_code);

        interpreter.run();
    }
}

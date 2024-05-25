use crate::ast::{Ast};
use crate::context::Context;
use crate::lexer::Lexer;
use crate::tamper::Tamper;

mod lexer;
mod ast;
mod context;
mod tamper;
mod unparser;

fn main() {
    let tamper_demo = Tamper::new_raw("<div><p>{{ value }}</p></div>".to_string());
    let mut context = Context::new();
    context.insert("value".to_string(), "foobar".to_string());

    let html = tamper_demo.render(&mut context);
    println!("{:?}", html);
}

mod tests {
    #[test]
    fn test_ast_parsing() {}
}

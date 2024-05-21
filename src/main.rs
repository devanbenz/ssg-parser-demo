use crate::ast::{Ast};
use crate::lexer::Lexer;

mod lexer;
mod ast;

fn main() {
    let mut lxr = Lexer::new("<div><p>{{ hello }}</p><p><b>hi</b>{{ world }}</p></div>".to_string());
    let tokens = lxr.evaluate();
    let ast = Ast::from_tokens(tokens);

    if let Some(a) = ast {
        a.format(0);
    }
}

struct Context {
    ctx: Vec<(String, String)>,
}

mod tests {
    #[test]
    fn test_ast_parsing() {}
}

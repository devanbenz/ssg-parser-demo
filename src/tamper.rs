use crate::ast::Ast;
use crate::context::Context;
use crate::lexer::Lexer;
use crate::unparser::Unparser;

pub struct Tamper {
    pub ast: Option<Box<Ast>>
}

impl Tamper {
    pub fn new_raw(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.evaluate();
        let ast = Ast::from_tokens(tokens);

        Self { ast }
    }
    pub fn render(self, context: &mut Context) -> String {
        let mut ast = self.ast.unwrap();
        ast.build_from_context(context);
        let mut unparser = Unparser::new(ast);
        let html = unparser.to_html();

        html
    }
}

use std::ops::Deref;
use std::vec::IntoIter;
use crate::ast::Expr::{HtmlElement, ScalarVariable};
use crate::context::Context;
use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    ScalarVariable {
        ctx: String,
        value: Option<String>,
    },
    HtmlElement {
        tag: String
    },
}

#[derive(Debug)]
pub struct Ast {
    pub node: Expr,
    pub children: Option<Vec<Box<Ast>>>,
}

impl Ast {
    pub fn new(expr: Expr) -> Ast {
        Ast {
            node: expr,
            children: None,
        }
    }

    pub fn new_with_children(expr: Expr, child: Option<Vec<Box<Ast>>>) -> Ast {
        Ast {
            node: expr,
            children: child,
        }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Option<Box<Ast>> {
        if tokens.is_empty() {
            return None;
        }

        let mut ast: Option<Box<Ast>> = None;
        let mut token_iter = tokens.into_iter();

        Self::add_nodes(&mut ast, &mut token_iter);

        ast
    }

    fn add_nodes(ast: &mut Option<Box<Ast>>, token_iter: &mut IntoIter<Token>) {
        println!("{:?}", token_iter);
        if let Some(token) = token_iter.next() {
            println!("{:?}", token);
            match token {
                Token::HtmlOpeningTag(v) => {
                    let new_node = Box::new(Ast::new_with_children(HtmlElement {
                        tag: v
                    }, Some(Vec::new())));

                    if let Some(node) = ast {
                        node.children.as_mut().unwrap().push(new_node);
                    } else {
                        *ast = Some(new_node);
                    }

                    Self::add_nodes(ast, token_iter);
                }
                Token::StringLiteral(v) => {
                    let new_node = Box::new(Ast::new_with_children(ScalarVariable {
                        ctx: v,
                        value: None,
                    }, Some(Vec::new())));

                    if let Some(node) = ast {
                        node.children.as_mut().unwrap().push(new_node);
                    } else {
                        *ast = Some(new_node);
                    }

                    Self::add_nodes(ast, token_iter);
                }
                _ => {
                    token_iter.next();
                    Self::add_nodes(ast, token_iter);
                }
            }
        }
    }

    pub fn build_from_context(&mut self, context: &mut Context) {
        Self::traverse_and_insert_keys(&mut self.children, context);
    }

    fn traverse_and_insert_keys(node: &mut Option<Vec<Box<Ast>>>, context: &mut Context) {
        if let Some(nodes) = node {
            for mut node in nodes {
                match &mut node.node {
                    ScalarVariable { value, ctx } => {
                        if let Some(key_val) = context.get(ctx.to_owned()) {
                            node.node = ScalarVariable { value: Some(key_val), ctx: ctx.to_owned() }
                        }
                    }
                    _ => {}
                }
                Self::traverse_and_insert_keys(&mut node.children, context);
            }
        }
    }

    pub fn format(&self, tab: usize) {
        let tab_stop = "\t".repeat(tab);
        println!("{}{:?}", tab_stop, self.node);
        if let Some(children) = &self.children {
            let tab = tab + 1;
            for child in children {
                child.format(tab);
            }
        }
    }
}

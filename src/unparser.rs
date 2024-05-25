use std::collections::VecDeque;
use crate::ast::{Ast, Expr};

pub struct Unparser {
    ast: Box<Ast>
}

impl Unparser {
    pub fn new(ast: Box<Ast>) -> Self{
        Self { ast }
    }
    pub fn to_html(self) -> String {
        let mut tag_queue = VecDeque::new();
        let mut raw_html = String::new();

        Self::traverse_and_build_html(Some(vec![self.ast]), &mut raw_html, &mut tag_queue);

        raw_html
    }

    fn traverse_and_build_html(node: Option<Vec<Box<Ast>>>, html: &mut String, tag_queue: &mut VecDeque<String>) {
        if let Some(nodes) = node {
            for node in nodes {
                println!("{:?}", node);

                match &node.node {
                    Expr::ScalarVariable { value, .. } => {
                        println!("{:?}", value);
                        if let Some(v) = value {
                            html.push_str(format!("{}", v).as_str());
                        } else {
                            html.push_str("");
                        }

                        Self::traverse_and_build_html(node.children, html, tag_queue);
                    }
                    Expr::HtmlElement { tag } => {
                        println!("{:?}", tag);
                        html.push_str(format!("<{}>", tag).as_str());
                        tag_queue.push_back(tag.to_string());

                        Self::traverse_and_build_html(node.children, html, tag_queue);

                        html.push_str( format!("</{}>", tag).as_str());
                    }
                }
            }
        }
    }
}

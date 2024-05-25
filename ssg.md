---
title: Building a static site generator for fun and (no) profit 
author: Devan Benz
---

Static Site Generators
---

# What is it?  

A static site generator is a way of building static HTML pages out of a set of templates, text files, and static content such as images. 
It is often used for:
* Blogs
* Newsletters
* Simple websites to display information

Often times it will be a replacement for a CMS (Content Management System) such as wordpress, usually, being much more light weight and easier to use. 
A CMS system will consist of a database which results in more complex requirements to store data. Where as with a static site generator you can easily 
generate a set of pages to serve. 

<!-- end_slide -->

Fin
---


# The End

<!-- pause -->
Oh wait.

<!-- pause -->
There's more.

<!-- pause -->
# Static site generators are interesting
<!-- pause -->
Not neccessarily because of what they do. But because of the methodology behind them. 

<!-- end_slide -->

Compilers
---

# Static site generators are *compilers

<!-- pause -->
<!--The most interesting part about static site generators is that they technically are a compiler. They take in raw `templates` consisting of `text` proceed
to perform lexical analysis building out an Abstract Syntax Tree (AST), parse this AST to insert new data/modify text, and then write back to the filesystem in order to create a whole new thing. *Compilers take this approach too* just with syntax of your 
favorite (or least favorite) programming language to create machine code. 
-->
## Flow 
* *jinja2* or any other templates with raw text as source files

* Often has markdown or other forms of text files for content such as blog posts  

* Outputs valid HTML

```
┌───────────┐    ┌──────────┐      ┌───────────┐     ┌───────────┐
│           │    │          │      │           │     │           │
│ Raw Text  │    │  Lexer   │      │  Parser   │     │   AST     │
│           ├───►│          ├─────►│           ├────►│           │
│           │    │          │      │           │     │           │
└───────────┘    └──────────┘      └───────────┘     └─────┬─────┘
      ▲                                                    │      
      │         ┌────────────────────────────────┐         │      
      │         │   For use within application   │         │      
      └─────────┤    logic to perform work       │◄────────┘      
                │                                │                
                └────────────────────────────────┘
```
<!-- pause -->
* For this talk we will only go over capturing scalar variables in context 

<!--Generally the logic performed will involve control structures such as loops and conditionals and much more
I will only be covering injecting scalar variables with context as this talk is not meant to be a production grade SSG -->

<!-- end_slide -->
Lexer
---
<!-- column_layout: [2, 1] -->
<!-- column: 0 -->
## Tokens

* Read characters from text
<!-- pause -->
* Build token vector 
<!-- pause --> 
* Parse token vector in to an AST  
<!-- pause --> 

```rust
enum Tokens {
    OpeningTag(String),
    ClosingTag(String),
    LeftBrace,
    RightBrace,
    StringLiteral(String),
}
```
An example of a Tokens rust enum which could in theory represent `<div><p>{{ value }}</p></div>`



<!-- end_slide -->
Lexer (cont)
---
```rust
pub struct Lexer(String);

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer(input)
    }

    pub fn evaluate(&mut self) -> Vec<Tokens> {
        let mut tokens: Vec<Tokens> = vec![];
        let mut chars = self.0.chars().peekable();

        loop {
            match chars.peek() {
                None => break,
                Some(&'{') => {
                    tokens.push(Tokens::LeftBrace);
                    chars.next();
                },
                Some(&'}') => {
                    tokens.push(Tokens::RightBrace);
                    chars.next();
                },
                Some(&'<') => {
                    let mut opening_tag: String = char.by_ref().take_while(|predicate| predicate != &'>').collect();
                    // Need to push '>' since take while consumes it
                    opening_tag.push('>');
                    tokens.push(Tokens::HtmlTag(opening_tag));
                },
                _ => {
                    if chars.peek().is_some_and(|c| c.is_alphabetic()) {
                        let literal: String = chars.by_ref().take_while(|predicate| predicate.is_alphabetic()).collect();
                        tokens.push(Tokens::StringLiteral(literal));
                    } else {
                        chars.next();
                    }
                }
            }
        }

        tokens
    }
}
```
<!-- end_slide -->
AST
---
<!-- column_layout: [2, 1] -->
<!-- column: 0 -->
## Abstract Syntax Trees & Context Free Grammar
* Lexed tokens will be parsed into AST 
* ASTs are a way to define grammar using a tree 
<!-- pause -->
* Text is often ambigous 
<!-- pause --> 
* Context free grammar helps with this ambiquity 
>For example, the sentence:
>John, whose blue car was in the garage, walked to the grocery store.
>can be logically parenthesized (with the logical metasymbols [ ]) as follows:
>[John[, [whose [blue car]] [was [in [the garage]]],]] [walked [to [the [grocery store]]]].

### Grammar can be defined as an AST
```
                ┌────────┐                  
                │        │                  
                │  html  │                  
                │        │                  
                └┬──────┬┘                  
                 │      │                   
          ┌──────▼─┐  ┌─▼──────┐            
          │        │  │        │            
          │  div   │  │   div  │            
          │        │  │        │            
          └┬─────┬─┘  └────────┘            
           │     │                          
    ┌──────▼─┐ ┌─▼──────┐                   
    │        │ │        │   <html>          
    │   p    │ │   p    │     <div>         
    │        │ │        │       <p>hello</p>
    └──┬─────┘ └──┬─────┘       <p>world</p>
       │          │           </div>        
┌──────▼─┐  ┌─────▼──┐        <div>         
│        │  │        │        </div>        
│ hello  │  │ world  │      </html>         
│        │  │        │                      
└────────┘  └────────┘                      
```

<!-- column: 1 -->

```rust
enum Expr {
    ScalarVariable {
        value: Option<String>,
        ctx: String
    }
    HtmlElement {
        tag: String
    }
}

struct Ast {
    node: Expr,
    children: Option<Vec<Box<Ast>>>
}
```
AST struct with Expr enum. Expr will be parsed from Tokens. 

<!-- end_slide -->
AST (cont)
---
```rust
fn add_nodes(ast: &mut Option<Box<Ast>>, token_iter: &mut IntoIter<Token>) {
        while let Some(token) = token_iter.next() {
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
                },
                
/// Cut off for presentation purposes

                },
                _ => panic!("syntax error")
            }
        }
    }
```
<!-- end_slide -->
AST (cont)
---
<!-- column_layout: [2, 1] -->
<!-- column: 0 -->
Adding a formatter for pretty printing like so
```rust
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
```
calling `ast.format(0);` with the following html `<div><p>{{ value }}</p></div>` yields

```
HtmlElement { tag: "div" }
        HtmlElement { tag: "p" }
            ScalarVariable { ctx: "value", value: None }

```
<!-- end_slide -->
Context
---

<!-- column_layout: [2, 1] -->
<!-- column: 0 -->
# Context
* Need somewhere to store context for ScalarVariable
* Using something like a hashmap 
* Rust has two good candidates for this in `std::collections` 
<!-- pause -->
* `HashMap<T, S>`
<!-- pause -->
* `BtreeMap<T, S>`

<!-- pause -->
```rust
struct Context {
    ctx: HashMap<String, String>
}

impl Context {
    pub fn new() -> Self {
        Self {
            ctx: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.ctx.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String>{
        if let Some(value) = self.ctx.get(&key) {
            Ok(value.to_string())
        } else {
            None
        }
    }
}
```
<!-- end_slide -->
Context (cont)
---
* Need to add traversal and mutation to ScalarVariable nodes within AST
<!-- pause -->

```rust
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
```
<!-- pause -->
```
{"value": "foobar"}
```

```
HtmlElement { tag: "div" }
        HtmlElement { tag: "p" }
            ScalarVariable { ctx: "value", value: None }

```
```
Becomes
```
```
HtmlElement { tag: "div" }
        HtmlElement { tag: "p" }
            ScalarVariable { ctx: "value", value: Some("foobar") }
```
<!-- end_slide -->
Unparser
---
<!-- column_layout: [2, 1] -->
<!-- column: 0 -->
# Unparser and writing back to String
* Unparser's are basically just the opposite of a Parser
<!-- pause -->
* Writes tree back to string 
<!-- pause -->
```rust
    fn traverse_and_build_html(
        node: Option<Vec<Box<Ast>>>, 
        html: &mut String, 
        tag_queue: &mut VecDeque<String>) {
        if let Some(nodes) = node {
            for node in nodes {
                match &node.node {
                    Expr::ScalarVariable { value, .. } => {
                        if let Some(v) = value {
                            html.push_str(format!("{}", v).as_str());
                        } else {
                            html.push_str("");
                        }

                        Self::traverse_and_build_html(node.children, html, tag_queue);
                    }
                    Expr::HtmlElement { tag } => {
                        html.push_str(format!("<{}>", tag).as_str());
                        tag_queue.push_back(tag.to_string());

                        Self::traverse_and_build_html(node.children, html, tag_queue);

                        html.push_str( format!("</{}>", tag).as_str());
                    }
                }
            }
        }
    }
```

<!-- end_slide -->
Tamper
---
<!-- column_layout: [2, 1] -->
<!-- column: 0 -->

# Tamper (The builder)
* Need to implement a way to tie everything together
<!-- pause -->
* Tamper is the name of the builder since it will be "tamping" the pieces together 
<!-- pause --> 

```rust
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
```
<!-- end_slide -->
Putting it all together
---
# Topics covered
* General overview on how compilers work
* Tokenization and lexical analysis
* Building an Abstract Syntax Tree & Context free grammar
* Hash structures in Rust
* Some recursive techniques when working with Trees
* How Rust's pattern matching really helps when building out parsers

## Final main 
```rust
fn main() {
    let tamper_demo = Tamper::new_raw("<div><p>{{ value }}</p></div>".to_string());
    let mut context = Context::new();
    context.insert("value".to_string(), "foobar".to_string());

    let html = tamper_demo.render(&mut context);
    println!("{:?}", html);
}

// outputs: <div><p>foobar</p></div>
```
<!-- end_slide -->

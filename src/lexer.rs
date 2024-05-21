#[derive(Debug)]
pub enum Token {
    HtmlOpeningTag(String),
    HtmlClosingTag(String),
    LeftBrace,
    RightBrace,
    StringLiteral(String),
}
pub struct Lexer(String);

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer(input)
    }

    pub fn evaluate(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut chars = self.0.chars().peekable();

        loop {
            match chars.peek() {
                None => break,
                Some(&'{') => {
                    tokens.push(Token::LeftBrace);
                    chars.next();
                },
                Some(&'}') => {
                    tokens.push(Token::RightBrace);
                    chars.next();
                },
                Some(&'<') => {
                    chars.next();
                    if chars.peek() == Some(&'/') {
                        chars.next();
                        let closing_tag: String = chars
                            .by_ref()
                            .take_while(|predicate| predicate != &'>')
                            .collect();

                        tokens.push(Token::HtmlClosingTag(closing_tag));
                    } else {
                        let opening_tag: String = chars
                            .by_ref()
                            .take_while(|predicate| predicate != &'>')
                            .collect();

                        tokens.push(Token::HtmlOpeningTag(opening_tag));
                    }
                },
                _ => {
                    if chars.peek().is_some_and(|c| c.is_alphabetic()) {
                        let literal: String = chars
                            .by_ref()
                            .take_while(|predicate| predicate.is_alphabetic())
                            .collect();

                        tokens.push(Token::StringLiteral(literal));
                    } else {
                        chars.next();
                    }
                }
            }
        }

        tokens
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    /// "// comment"
    LineComment,
    /// `/* block comment */`
    BlockComment,
    /// Any whitespace character sequence.
    Whitespace,
    Identifier,
    InvalidIdentifier,
    Literal {
        kind: LiteralKind,
        value: String,
    },
    Newline,
    Semi,
    Comma,
    Dot,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    Int { base: Base },
    String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

pub fn parse(input: &'static str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars();
    let mut _pos = 0;
    let mut _line = 1;

    while let Some(c) = chars.next() {
        let token: Token = match c {
            '/' => {
                if let Some(c) = chars.next() {
                    match c {
                        '/' => {
                            let mut comment = String::new();
                            while let Some(c) = chars.next() {
                                if c == '\n' {
                                    break;
                                }
                                comment.push(c);
                            }
                            Token::new(TokenKind::LineComment, comment.len() + 2)
                        }
                        '*' => {
                            let mut comment = String::new();
                            while let Some(c) = chars.next() {
                                if c == '*' {
                                    if let Some(c) = chars.next() {
                                        if c == '/' {
                                            break;
                                        }
                                    }
                                }
                                comment.push(c);
                            }
                            Token::new(TokenKind::BlockComment, comment.len() + 1)
                        }
                        _ => Token::new(TokenKind::InvalidIdentifier, 1),
                    }
                } else {
                    Token::new(TokenKind::InvalidIdentifier, 1)
                }
            }
            ' ' | '\t' | '\r' => {
                let mut whitespace = String::new();
                while let Some(c) = chars.next() {
                    if c != ' ' && c != '\t' && c != '\r' {
                        break;
                    }
                    whitespace.push(c);
                }
                Token::new(TokenKind::Whitespace, whitespace.len() + 1)
            }
            '\n' => {
                _line += 1;
                Token::new(TokenKind::Newline, 1)
            }
            ';' => Token::new(TokenKind::Semi, 1),
            ',' => Token::new(TokenKind::Comma, 1),
            '.' => Token::new(TokenKind::Dot, 1),
            '(' => Token::new(TokenKind::OpenParenthesis, 1),
            ')' => Token::new(TokenKind::CloseParenthesis, 1),
            '{' => Token::new(TokenKind::OpenBrace, 1),
            '}' => Token::new(TokenKind::CloseBrace, 1),
            '[' => Token::new(TokenKind::OpenBracket, 1),
            ']' => Token::new(TokenKind::CloseBracket, 1),
            '"' => {
                let mut value = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    value.push(c);
                }

                Token::new(
                    TokenKind::Literal {
                        kind: LiteralKind::String,
                        value: value.clone(),
                    },
                    value.len(),
                )
            }
            '0'..='9' => {
                let mut value = String::new();
                value.push(c);
                let mut base_specified = false;
                while let Some(c) = chars.next() {
                    if !c.is_numeric() && c != 'x' && !base_specified {
                        break;
                    }
                    if c == 'x' {
                        base_specified = true;
                    }
                    value.push(c);
                }
                Token::new(
                    TokenKind::Literal {
                        kind: LiteralKind::Int {
                            base: if base_specified {
                                Base::Hexadecimal
                            } else {
                                Base::Decimal
                            },
                        },
                        value: value.clone(),
                    },
                    value.len(),
                )
            }
            _ => {
                if c.is_alphabetic() {
                    let mut value = String::new();
                    while let Some(c) = chars.next() {
                        if c.is_alphanumeric() {
                            value.push(c);
                        } else {
                            break;
                        }
                    }
                    Token::new(TokenKind::Identifier, value.len())
                } else if c.is_numeric() {
                    let mut value = String::new();
                    while let Some(c) = chars.next() {
                        if c.is_numeric() {
                            value.push(c);
                        } else {
                            break;
                        }
                    }
                    Token::new(
                        TokenKind::Literal {
                            kind: LiteralKind::Int {
                                base: Base::Decimal,
                            },
                            value: value.clone(),
                        },
                        value.clone().len(),
                    )
                } else {
                    Token::new(TokenKind::InvalidIdentifier, 1)
                }
            }
        };
        tokens.push(token);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_comment() {
        assert_eq!(
            parse("// comment"),
            vec![Token {
                kind: TokenKind::LineComment,
                len: 10,
            }]
        );

        assert_eq!(
            parse("/* comment */"),
            vec![Token {
                kind: TokenKind::BlockComment,
                len: 10,
            }]
        );
    }

    #[test]
    fn parse_whitespace() {
        assert_eq!(
            parse(" \t\r\n"),
            vec![Token {
                kind: TokenKind::Whitespace,
                len: 3,
            }]
        );
    }

    #[test]
    fn parse_identifier() {
        assert_eq!(
            parse("0xffff"),
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Int {
                        base: Base::Hexadecimal
                    },
                    value: "0xffff".to_string(),
                },
                6
            )]
        );
    }
}

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
                    if c.is_ascii_hexdigit() || c.is_ascii_digit() || (c == 'x' && !base_specified)
                    {
                        value.push(c);
                        if c == 'x' {
                            base_specified = true;
                        };
                    } else {
                        break;
                    }
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
                if c.is_ascii_alphabetic() {
                    let mut value = String::new();
                    value.push(c);
                    while let Some(c) = chars.next() {
                        if c.is_ascii_alphabetic() {
                            value.push(c);
                        } else {
                            break;
                        }
                    }

                    Token::new(TokenKind::Identifier, value.len())
                } else if c.is_ascii_digit() || c.is_ascii_hexdigit() {
                    let mut value = String::new();
                    let mut is_hex = false;
                    value.push(c);
                    while let Some(c) = chars.next() {
                        if c.is_ascii_digit() || c.is_ascii_hexdigit() || (c == 'x' && !is_hex) {
                            value.push(c);
                            if c == 'x' {
                                is_hex = true;
                            };
                        } else {
                            break;
                        }
                    }

                    Token::new(
                        TokenKind::Literal {
                            kind: LiteralKind::Int {
                                base: if is_hex {
                                    Base::Hexadecimal
                                } else {
                                    Base::Decimal
                                },
                            },
                            value: value.clone(),
                        },
                        value.len(),
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

    #[test]
    fn parse_semicolon() {
        assert_eq!(
            parse(";"),
            vec![Token {
                kind: TokenKind::Semi,
                len: 1,
            }]
        );
    }

    #[test]
    fn parse_comma() {
        assert_eq!(
            parse(","),
            vec![Token {
                kind: TokenKind::Comma,
                len: 1,
            }]
        );
    }

    #[test]
    fn parse_dot() {
        assert_eq!(
            parse("."),
            vec![Token {
                kind: TokenKind::Dot,
                len: 1,
            }]
        );
    }

    #[test]
    fn parse_open_parenthesis() {
        assert_eq!(
            parse("("),
            vec![Token {
                kind: TokenKind::OpenParenthesis,
                len: 1,
            }]
        );
    }

    #[test]
    fn parse_close_parenthesis() {
        assert_eq!(
            parse(")"),
            vec![Token {
                kind: TokenKind::CloseParenthesis,
                len: 1,
            }]
        );
    }

    #[test]
    fn parse_open_brace() {
        assert_eq!(
            parse("{"),
            vec![Token {
                kind: TokenKind::OpenBrace,
                len: 1,
            }]
        );
    }

    #[test]
    fn parse_close_brace() {
        assert_eq!(
            parse("}"),
            vec![Token {
                kind: TokenKind::CloseBrace,
                len: 1,
            }]
        );
    }

    #[test]
    fn parse_open_bracket() {
        assert_eq!(
            parse("["),
            vec![Token {
                kind: TokenKind::OpenBracket,
                len: 1,
            }]
        );
    }

    #[test]
    fn parse_close_bracket() {
        assert_eq!(
            parse("]"),
            vec![Token {
                kind: TokenKind::CloseBracket,
                len: 1,
            }]
        );
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            parse("\"hello\""),
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::String,
                    value: "hello".to_string(),
                },
                5
            )]
        );
    }

    #[test]
    fn parse_int() {
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

    #[test]
    fn parse_int_base_10() {
        assert_eq!(
            parse("255"),
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Int {
                        base: Base::Decimal
                    },
                    value: "255".to_string(),
                },
                3
            )]
        );
    }

    #[test]
    fn parse_full() {
        let input = r#"
object "Contract" {
    code {
        datacopy(0, dataoffset("runtime"), datasize("runtime"))
        return(0, datasize("runtime"))
    }
    object "runtime" {
        code {
            switch shr(0xf8, calldataload(0))
            // calldata shifted by 248 bits to the right
            // is equivalent of the byte slice calldata[0:1]
            case 0x00 {
                mstore(0, "Hello, World")
                return(0, 0x20)
            }
            default { 
                revert(0, 0)
            }
        }
    }
}
"#;
        println!("{:?}", parse(input));
    }
}

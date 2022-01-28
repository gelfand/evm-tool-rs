#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
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
    let mut tokens = Vec::new();
    let mut chars = input.chars();
    let mut _pos = 0;
    let mut _line = 1;

    while let Some(c) = chars.next() {
        let kind: TokenKind = match c {
            '/' => {
                if let Some(c) = chars.next() {
                    match c {
                        '/' => TokenKind::LineComment,
                        '*' => TokenKind::BlockComment,
                        _ => TokenKind::InvalidIdentifier,
                    }
                } else {
                    TokenKind::InvalidIdentifier
                }
            }
            _ => {
                todo!()
            }
        };

        let len = match kind {
            TokenKind::LineComment => {
                let mut len = 0;
                while let Some(c) = chars.next() {
                    if c == '\n' {
                        break;
                    }
                    len += 1;
                }
                len + 2
            }
            _ => todo!(),
        };

        tokens.push(Token { kind, len });
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
    }
}

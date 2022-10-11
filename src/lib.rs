mod character_stream;

use crate::character_stream::CharacterStream;

#[derive(Debug, PartialEq, Eq)]
pub enum KeywordType {
    Char,
    Double,
    Else,
    Float,
    For,
    If,
    Int,
    Long,
    Return,
    While,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperatorType {
    Plus,
    PlusEqual,
    And,
    Assign,
    BitwiseAnd,
    BitwiseOr,
    Decrement,
    NotEqual,
    Divide,
    DivideEqual,
    Equals,
    GreaterThan,
    GreaterThanOrEqual,
    Increment,
    LessThan,
    LessThanOrEqual,
    Asterisk,
    AsteriskEqual,
    ExclamationPoint,
    Or,
    Minus,
    MinusEqual,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    CloseBrace,
    CloseBracket,
    CloseParen,
    Char(char),
    Eof,
    Id(String),
    Keyword(KeywordType),
    Number(String),
    OpenBrace,
    OpenBracket,
    OpenParen,
    Operator(OperatorType),
    SemiColon,
    String(String),
    Invalid(String),
}

pub struct Scanner {
    index: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(text: &str) -> Self {
        Self {
            index: 0,
            tokens: tokenize_str(text),
        }
    }

    pub fn next_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}

fn tokenize_str(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut cs = CharacterStream::new(text);
    loop {
        cs.skip_whitespace();

        // Gambiarra da boa
        if cs.is_eof() {
            tokens.push(Token::Eof);
            break;
        }
        skip_comments(&mut cs);
        if cs.is_eof() {
            tokens.push(Token::Eof);
            break;
        }

        let token = match cs.current_char() {
            '_' | 'a'..='z' | 'A'..='Z' => lex_identifier_or_keyword(&mut cs),
            '0'..='9' | '.' => lex_number(&mut cs),
            '"' => lex_string(&mut cs),
            '\'' => lex_char(&mut cs),
            '(' => {
                cs.advance_by(1);
                Token::OpenParen
            }
            ')' => {
                cs.advance_by(1);
                Token::CloseParen
            }
            '{' => {
                cs.advance_by(1);
                Token::OpenBrace
            }
            '}' => {
                cs.advance_by(1);
                Token::CloseBrace
            }
            '[' => {
                cs.advance_by(1);
                Token::OpenBracket
            }
            ']' => {
                cs.advance_by(1);
                Token::CloseBracket
            }
            ';' => {
                cs.advance_by(1);
                Token::SemiColon
            }
            _ => lex_operator(&mut cs),
        };

        tokens.push(token);
    }

    tokens
}

fn skip_comments(cs: &mut CharacterStream) {
    if cs.current_char() == '/' {
        match cs.next_char() {
            Some(&'/') => {
                while cs.current_char() != '\n' {
                    cs.advance_by(1);
                }
            
                cs.skip_whitespace();

                if cs.check_bounds() {
                    skip_comments(cs);
                }
            }
            Some(&'*') => {
                loop {
                    cs.advance_by(1);
                    if cs.current_char() == '*' && cs.next_char() == Some(&'/') {
                        cs.advance_by(2);
                        cs.skip_whitespace();
                        break;
                    }
                }

                if cs.check_bounds() {
                    skip_comments(cs);
                }
            }
            Some(_) | None => (),
        }
    }
}

fn lex_identifier_or_keyword(cs: &mut CharacterStream) -> Token {
    let mut id = String::new();
    while cs.current_char().is_alphanumeric() || cs.current_char() == '_' {
        id.push(cs.current_char());
        cs.advance_by(1);
    }

    match id.as_str() {
        "for" => Token::Keyword(KeywordType::For),
        "while" => Token::Keyword(KeywordType::While),
        "if" => Token::Keyword(KeywordType::If),
        "else" => Token::Keyword(KeywordType::Else),
        "int" => Token::Keyword(KeywordType::Int),
        "float" => Token::Keyword(KeywordType::Float),
        "double" => Token::Keyword(KeywordType::Double),
        "long" => Token::Keyword(KeywordType::Long),
        "return" => Token::Keyword(KeywordType::Return),
        "char" => Token::Keyword(KeywordType::Char),
        _ => Token::Id(id),
    }
}

fn lex_number(cs: &mut CharacterStream) -> Token {
    let mut num = String::new();
    while cs.current_char().is_ascii_digit() {
        num.push(cs.current_char());
        cs.advance_by(1);
    }

    if cs.current_char().is_alphanumeric() {
        let mut invalid_id = String::with_capacity(num.len());
        invalid_id.push_str(&num);

        while cs.current_char().is_alphanumeric() {
            invalid_id.push(cs.current_char());
            cs.advance_by(1);
        }
        return Token::Invalid(format!("Invalid identifier \"{}\"!", invalid_id));
    }

    if cs.current_char() == '.' {
        num.push(cs.current_char());
        cs.advance_by(1);

        while cs.current_char().is_ascii_digit() {
            num.push(cs.current_char());
            cs.advance_by(1);
        }

        if cs.current_char() == 'f' {
            cs.advance_by(1);
        }
    }

    Token::Number(num)
}

fn lex_string(cs: &mut CharacterStream) -> Token {
    let mut string = String::new();
    cs.advance_by(1);

    while cs.current_char() != '"' {
        string.push(cs.current_char());
        cs.advance_by(1);
    }

    if cs.current_char() != '"' {
        return Token::Invalid("missing terminating \"".to_string());
    }
    cs.advance_by(1);

    string.push('\0');

    Token::String(string)
}

fn lex_char(cs: &mut CharacterStream) -> Token {
    cs.advance_by(1);
    let char = if cs.current_char() == '\'' {
        '\0'
    } else {
        cs.current_char()
    };
    cs.advance_by(1);

    if cs.current_char().is_ascii_alphanumeric() {
        return Token::Invalid("char literals should only have one character".to_string());
    }

    if cs.current_char() != '\'' {
        return Token::Invalid("missing terminating \'".to_string());
    }
    cs.advance_by(1);

    Token::Char(char)
}

fn lex_operator(cs: &mut CharacterStream) -> Token {
    let next_char = match cs.next_char() {
        Some(&next_char) => next_char,
        None => return Token::Eof,
    };

    let (token, advance_total) = match cs.current_char() {
        '+' => match next_char {
            '=' => (Token::Operator(OperatorType::PlusEqual), 2),
            '+' => (Token::Operator(OperatorType::Increment), 2),
            _ => (Token::Operator(OperatorType::Plus), 1),
        },
        '-' => match next_char {
            '=' => (Token::Operator(OperatorType::MinusEqual), 2),
            '-' => (Token::Operator(OperatorType::Decrement), 2),
            _ => (Token::Operator(OperatorType::Minus), 1),
        },
        '>' => match next_char {
            '=' => (Token::Operator(OperatorType::GreaterThanOrEqual), 2),
            _ => (Token::Operator(OperatorType::GreaterThan), 1),
        },
        '<' => match next_char {
            '=' => (Token::Operator(OperatorType::LessThanOrEqual), 2),
            _ => (Token::Operator(OperatorType::LessThan), 1),
        },
        '&' => match next_char {
            '&' => (Token::Operator(OperatorType::And), 2),
            _ => (Token::Operator(OperatorType::BitwiseAnd), 1),
        },
        '|' => match next_char {
            '|' => (Token::Operator(OperatorType::Or), 2),
            _ => (Token::Operator(OperatorType::BitwiseOr), 1),
        },
        '=' => match next_char {
            '=' => (Token::Operator(OperatorType::Equals), 2),
            _ => (Token::Operator(OperatorType::Assign), 1),
        },
        '!' => match next_char {
            '=' => (Token::Operator(OperatorType::NotEqual), 2),
            _ => (Token::Operator(OperatorType::ExclamationPoint), 1),
        },
        '*' => match next_char {
            '=' => (Token::Operator(OperatorType::AsteriskEqual), 2),
            _ => (Token::Operator(OperatorType::Asterisk), 1),
        },
        '/' => match next_char {
            '=' => (Token::Operator(OperatorType::DivideEqual), 2),
            _ => (Token::Operator(OperatorType::Divide), 1),
        },
        char => (Token::Invalid(format!("Invalid symbol \"{}\"!", char)), 1),
    };
    cs.advance_by(advance_total);

    token
}

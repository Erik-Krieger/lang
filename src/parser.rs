use std::collections::HashSet;
use std::{fs, process};

#[derive(Debug)]
pub enum Token {
    Bogus,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenBracket,
    CloseBracket,
    OpenAngle,
    CloseAngle,
    ExclamationMark,
    QuestionMark,
    Equals,
    Colon,
    SemiColon,
    Period,
    Comma,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Underscore,
    SingleQuote,
    DoubleQuote,
    Identifier { name: String },
    Keyword { name: String },
    CharLiteral { value: String },
    StringLiteral { value: String },
    NumericLiteral { value: String },
}

struct ParserData {
    tokens: Vec<Token>,
    current_buffer: Vec<char>,
    current_token: Token,
    period_count: u64,
    next_char_escaped: bool,
}

pub fn parse_file(file_path: &String) -> Vec<Token> {
    let file_content = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(err) => {
            dbg!(err);
            process::exit(1);
        }
    };

    let mut keywords = HashSet::new();
    keywords.insert("fn");
    keywords.insert("pub");
    keywords.insert("mut");
    keywords.insert("return");

    let mut parser = ParserData {
        tokens: Vec::new(),
        current_buffer: Vec::new(),
        current_token: Token::Bogus,
        period_count: 0,
        next_char_escaped: false,
    };

    for token in file_content.chars() {
        process_token(token, &mut parser);
    }

    return parser.tokens;
}

fn process_token(token: char, parser: &mut ParserData) {
    match parser.current_token {
        Token::Bogus => {
            if token.is_ascii_digit() {
                parser.current_token = Token::NumericLiteral {
                    value: String::new(),
                };
                parser.current_buffer.push(token);
            } else if token.is_ascii_alphabetic() || token == '_' {
                parser.current_token = Token::Identifier {
                    name: String::new(),
                };
                parser.current_buffer.push(token);
            } else if token == '"' {
                parser.current_token = Token::StringLiteral {
                    value: String::new(),
                };
            } else if token == '\'' {
                parser.current_token = Token::CharLiteral {
                    value: String::new(),
                };
            } else {
                let p_token: Token = match token {
                    '(' => Token::OpenParen,
                    ')' => Token::CloseParen,
                    '{' => Token::OpenCurly,
                    '}' => Token::CloseCurly,
                    '[' => Token::OpenBracket,
                    ']' => Token::CloseBracket,
                    '<' => Token::OpenAngle,
                    '>' => Token::CloseAngle,
                    '!' => Token::ExclamationMark,
                    '?' => Token::QuestionMark,
                    '=' => Token::Equals,
                    ':' => Token::Colon,
                    ';' => Token::SemiColon,
                    '.' => Token::Period,
                    ',' => Token::Comma,
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Asterisk,
                    '/' => Token::Slash,
                    '%' => Token::Percent,
                    '_' => Token::Underscore,
                    _ => Token::Bogus,
                };

                match p_token {
                    Token::Bogus => {}
                    _ => parser.tokens.push(p_token),
                }
            }
        }
        Token::NumericLiteral { .. } => {
            if token.is_ascii_digit() || token == '_' || token == ',' {
                return;
            } else if token == '.' && parser.period_count == 0 {
                parser.period_count += 1;
                parser.current_buffer.push(token);
            } else if token == '.' && parser.period_count > 0 {
                panic!();
            } else {
                let value: String = parser.current_buffer.iter().cloned().collect();
                let new_token: Token = Token::NumericLiteral { value: value };
                parser.current_buffer.clear();
                parser.current_token = Token::Bogus;
                parser.tokens.push(new_token);

                process_token(token, parser);
            }
        }
        Token::Identifier { .. } => {
            if token.is_ascii_alphabetic() || token.is_ascii_digit() || token == '_' {
                parser.current_buffer.push(token);
            } else {
                let value: String = parser.current_buffer.iter().cloned().collect();

                if is_keyword(&value) {
                    parser.tokens.push(Token::Keyword { name: value });
                } else {
                    parser.tokens.push(Token::Identifier { name: value });
                }

                parser.current_buffer.clear();
                parser.current_token = Token::Bogus;

                process_token(token, parser);
            }
        }
        Token::StringLiteral { .. } => {
            if !parser.next_char_escaped {
                if token == '\\' {
                    parser.next_char_escaped = true;
                    return;
                } else if token == '"' {
                    let value: String = parser.current_buffer.iter().cloned().collect();
                    let new_token: Token = Token::StringLiteral { value: value };
                    parser.current_buffer.clear();
                    parser.current_token = Token::Bogus;
                    parser.tokens.push(new_token);
                    return;
                }
            } else {
                parser.next_char_escaped = false;
            }

            parser.current_buffer.push(token);
        }
        Token::CharLiteral { .. } => {
            if !parser.next_char_escaped {
                if token == '\\' {
                    parser.next_char_escaped = true;
                    return;
                } else if token == '\'' {
                    assert!(parser.current_buffer.iter().len() == 0);

                    let value: String = parser.current_buffer.iter().cloned().collect();
                    let new_token: Token = Token::StringLiteral { value: value };
                    parser.current_buffer.clear();
                    parser.current_token = Token::Bogus;
                    parser.tokens.push(new_token);
                    return;
                }
            } else {
                parser.next_char_escaped = false;
            }

            parser.current_buffer.push(token);
        }
        _ => unreachable!(),
    };
}

fn is_keyword(token: &String) -> bool {
    match &token[..] {
        "fn" => true,
        "pub" => true,
        "mut" => true,
        "return" => true,
        _ => false,
    }
}

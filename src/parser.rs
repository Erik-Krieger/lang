use std::{fs, process};

enum ParserTokenType {
    None,
    Identifier,
    Number,
}

struct ParserData {
    tokens: Vec<String>,
    current_buffer: Vec<char>,
    current_token: ParserTokenType,
}

pub fn parse_file(file_path: &String) -> Vec<String> {
    let file_content = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(err) => {
            dbg!(err);
            process::exit(1);
        }
    };

    let mut parser = ParserData {
        tokens: Vec::new(),
        current_buffer: Vec::new(),
        current_token: ParserTokenType::None,
    };

    for token in file_content.chars() {
        process_token(token, &mut parser);
    }

    return parser.tokens;
}

fn process_token(token: char, parser: &mut ParserData) {
    match parser.current_token {
        ParserTokenType::None => {
            if token.is_ascii_digit() {
                parser.current_token = ParserTokenType::Number;
                parser.current_buffer.push(token);
            } else if token.is_ascii_alphabetic() || token == '_' {
                parser.current_token = ParserTokenType::Identifier;
                parser.current_buffer.push(token);
            } else if is_bracket(token) {
                parser.tokens.push(token.into());
            } else if is_special_char(token) {
                parser.tokens.push(token.into());
            }
        }
        ParserTokenType::Number => {
            if token.is_ascii_digit() || token == '_' {
                parser.current_buffer.push(token);
            } else {
                parser
                    .tokens
                    .push(parser.current_buffer.iter().cloned().collect());
                parser.current_buffer.clear();
                parser.current_token = ParserTokenType::None;

                process_token(token, parser);
            }
        }
        ParserTokenType::Identifier => {
            if token.is_ascii_alphabetic() || token.is_ascii_digit() || token == '_' {
                parser.current_buffer.push(token);
            } else {
                parser
                    .tokens
                    .push(parser.current_buffer.iter().cloned().collect());
                parser.current_buffer.clear();
                parser.current_token = ParserTokenType::None;

                process_token(token, parser);
            }
        }
    };
}

fn is_bracket(token: char) -> bool {
    return match token {
        '(' => true,
        ')' => true,
        '{' => true,
        '}' => true,
        '[' => true,
        ']' => true,
        '<' => true,
        '>' => true,
        _ => false,
    };
}

fn is_operator(token: char) -> bool {
    return match token {
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        '%' => true,
        _ => false,
    };
}

fn is_special_char(token: char) -> bool {
    if is_operator(token) {
        return true;
    }

    return match token {
        '=' => true,
        '!' => true,
        ';' => true,
        ':' => true,
        ',' => true,
        '.' => true,
        '^' => true,
        '&' => true,
        '|' => true,
        '~' => true,
        _ => false,
    };
}

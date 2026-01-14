use std::{fs, process};

#[derive(Debug, Clone)]
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
    Ampersand,
    Pipe,
    NewLine,
    PubModifier,

    LogicalAnd,
    LogicalOr,

    TypedVariable {
        name: String,
        data_type: DataTypeStruct,
    },

    ArrowRight,
    ReturnTypeSpecifier {
        data_type: DataType,
        is_reference: bool,
    },
    Identifier {
        name: String,
    },
    Keyword {
        name: String,
    },
    DataType {
        data_type: DataTypeStruct,
    },
    CharLiteral {
        value: String,
    },
    StringLiteral {
        value: String,
    },
    NumericLiteral {
        value: String,
    },

    Function {
        name: String,
        public: bool,
        return_type: DataType,
        params: Vec<Token>,
        body: Vec<Token>,
    },

    FunctionDefinition {
        name: String,
        public: bool,
    },

    ParamToken {
        name: String,
        data_type: DataType,
    },

    VariableToken {
        name: String,
        data_type: DataType,
        declared_with_let: bool,
    },
}

#[derive(Debug, Clone, Copy)]
enum DataType {
    None,
    I64 { size: usize },
    U64 { size: usize },
    F64 { size: usize },
    Bool { size: usize },
}

#[derive(Debug, Clone)]
struct DataTypeStruct {
    data_type: DataType,
    is_ref: bool,
}

const KEYWORD_FUNCTION: &str = "fn";
const KEYWORD_RETURN: &str = "return";
const KEYWORD_LET: &str = "let";
const KEYWORD_MUT: &str = "mut";
const KEYWORD_PUB: &str = "pub";
const KEYWORD_IF: &str = "if";
const KEYWORD_ELSE: &str = "else";

#[derive(Debug)]
pub struct Compiler {
    tokens: Vec<Token>,
    current_buffer: Vec<char>,
    current_token: Token,
    period_count: u64,
    next_char_escaped: bool,
    functions: Vec<Function>,
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    body: Vec<Token>,
    parameters: Vec<Token>,
    return_type: DataTypeStruct,
    public: bool,
}

pub fn compile(file_path: &String) -> Compiler {
    let mut compiler = Compiler {
        tokens: Vec::new(),
        current_buffer: Vec::new(),
        current_token: Token::Bogus,
        period_count: 0,
        next_char_escaped: false,
        functions: Vec::new(),
    };

    parse_file(file_path, &mut compiler);

    //dbg!(&compiler);

    merge_tokens_round_one(&mut compiler);

    dbg!(&compiler);

    split_functions(&mut compiler);

    dbg!(&compiler);

    return compiler;
}

fn parse_file(file_path: &String, compiler: &mut Compiler) {
    let file_content = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(err) => {
            dbg!(err);
            process::exit(1);
        }
    };

    for token in file_content.chars() {
        process_token(token, compiler);
    }
}

fn process_token(token: char, compiler: &mut Compiler) {
    match compiler.current_token {
        Token::Bogus => {
            if token.is_ascii_digit() {
                compiler.current_token = Token::NumericLiteral {
                    value: String::new(),
                };
                compiler.current_buffer.push(token);
            } else if token.is_ascii_alphabetic() || token == '_' {
                compiler.current_token = Token::Identifier {
                    name: String::new(),
                };
                compiler.current_buffer.push(token);
            } else if token == '"' {
                compiler.current_token = Token::StringLiteral {
                    value: String::new(),
                };
            } else if token == '\'' {
                compiler.current_token = Token::CharLiteral {
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
                    '&' => Token::Ampersand,
                    '|' => Token::Pipe,
                    '\n' => Token::NewLine,
                    _ => Token::Bogus,
                };

                match p_token {
                    Token::Bogus => {}
                    _ => compiler.tokens.push(p_token),
                }
            }
        }
        Token::NumericLiteral { .. } => {
            if token.is_ascii_digit() || token == '_' || token == ',' {
                return;
            } else if token == '.' && compiler.period_count == 0 {
                compiler.period_count += 1;
                compiler.current_buffer.push(token);
            } else if token == '.' && compiler.period_count > 0 {
                panic!();
            } else {
                let value: String = compiler.current_buffer.iter().cloned().collect();
                let new_token: Token = Token::NumericLiteral { value: value };
                compiler.current_buffer.clear();
                compiler.current_token = Token::Bogus;
                compiler.tokens.push(new_token);

                process_token(token, compiler);
            }
        }
        Token::Identifier { .. } => {
            if token.is_ascii_alphabetic() || token.is_ascii_digit() || token == '_' {
                compiler.current_buffer.push(token);
            } else {
                let value: String = compiler.current_buffer.iter().cloned().collect();

                if is_keyword(&value) {
                    if &value == "pub" {
                        compiler.tokens.push(Token::PubModifier);
                    } else {
                        compiler.tokens.push(Token::Keyword { name: value });
                    }
                } else if is_datatype(&value) {
                    compiler.tokens.push(Token::DataType {
                        data_type: get_type_from_name(&value),
                    });
                } else {
                    compiler.tokens.push(Token::Identifier { name: value });
                }

                compiler.current_buffer.clear();
                compiler.current_token = Token::Bogus;

                process_token(token, compiler);
            }
        }
        Token::StringLiteral { .. } => {
            if !compiler.next_char_escaped {
                if token == '\\' {
                    compiler.next_char_escaped = true;
                    return;
                } else if token == '"' {
                    let value: String = compiler.current_buffer.iter().cloned().collect();
                    let new_token: Token = Token::StringLiteral { value: value };
                    compiler.current_buffer.clear();
                    compiler.current_token = Token::Bogus;
                    compiler.tokens.push(new_token);
                    return;
                }
            } else {
                compiler.next_char_escaped = false;
            }

            compiler.current_buffer.push(token);
        }
        Token::CharLiteral { .. } => {
            if !compiler.next_char_escaped {
                if token == '\\' {
                    compiler.next_char_escaped = true;
                    return;
                } else if token == '\'' {
                    assert!(compiler.current_buffer.iter().len() == 0);

                    let value: String = compiler.current_buffer.iter().cloned().collect();
                    let new_token: Token = Token::StringLiteral { value: value };
                    compiler.current_buffer.clear();
                    compiler.current_token = Token::Bogus;
                    compiler.tokens.push(new_token);
                    return;
                }
            } else {
                compiler.next_char_escaped = false;
            }

            compiler.current_buffer.push(token);
        }
        _ => unreachable!(),
    };
}

fn merge_tokens_round_one(compiler: &mut Compiler) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut skip_count: usize = 0;
    let token_count: usize = (&compiler.tokens).len();

    for idx in 0..token_count {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        match jti(compiler, &mut skip_count, idx, token_count) {
            Some(token) => match token {
                Token::Bogus => {}
                _ => tokens.push(token),
            },
            None => {}
        }
    }

    compiler.tokens = tokens;

    return;

    /*match token {
        // begin comment handling
        Token::Slash => {
            assert!(old_token_count > idx + 1);
            match &compiler.tokens[idx + 1] {
                Token::Slash => {
                    for look_ahead_count in idx + 2..old_token_count {
                        match &compiler.tokens[look_ahead_count] {
                            Token::NewLine => {
                                skip_count = look_ahead_count - idx;
                                return true;
                            }
                            _ => {}
                        }
                    }
                    return false;
                }
                Token::Asterisk => {
                    for look_ahead_count in idx + 2..old_token_count {
                        match &compiler.tokens[look_ahead_count] {
                            Token::Asterisk => {
                                match &compiler.tokens[look_ahead_count + 1] {
                                    Token::Slash => {
                                        skip_count = look_ahead_count + 1 - idx;
                                        return true;
                                    }
                                    _ => {}
                                }
                                return false;
                            }
                            _ => {}
                        }
                    }
                    // If we reach this point, we are at the end of the file and have not found a closing sequence for comments.
                    // Either way we stop herer
                    // TODO: Maybe output an error.
                    return false;
                }
                _ => {}
            }
        }
        // end comment handling

        // At this point we don't need the new lines anymore
        Token::NewLine => {}
        Token::Minus => {
            assert!(old_token_count > idx + 1);
            match &compiler.tokens[idx + 1] {
                Token::CloseAngle => {
                    tokens.push(Token::ArrowRight);
                    skip_count = 1;
                    continue;
                }
                _ => {}
            }
        }
        Token::Ampersand => {
            assert!(old_token_count > idx + 1);
            match &compiler.tokens[idx + 1] {
                Token::DataType { data_type, .. } => {
                    tokens.push(Token::DataType {
                        data_type: *data_type,
                        is_reference: true,
                    });
                    skip_count = 1;
                    continue;
                }
                Token::Ampersand => {
                    tokens.push(Token::LogicalAnd);
                    skip_count = 1;
                    continue;
                }
                _ => {}
            }
        }
        Token::Pipe => {
            assert!(old_token_count > idx + 1);
            match &compiler.tokens[idx + 1] {
                Token::Pipe => {
                    tokens.push(Token::LogicalOr);
                    skip_count = 1;
                    continue;
                }
                _ => {}
            }
        }
        Token::PubModifier => {
            assert!(old_token_count > idx + 2);
            match &compiler.tokens[idx + 1] {
                Token::Keyword { name } => match &name[..] {
                    "fn" => match &compiler.tokens[idx + 2] {
                        Token::Identifier { name } => {
                            tokens.push(Token::FunctionDefinition {
                                name: name.to_string(),
                                public: true,
                            });
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        Token::Keyword { name } => match &name[..] {
            "fn" => {}
            _ => {}
        },
        tok => {
            tokens.push(tok.clone());
        }
    };*/

    //compiler.tokens = tokens;
}

fn jti(
    compiler: &Compiler,
    skip_count: &mut usize,
    idx: usize,
    old_token_count: usize,
) -> Option<Token> {
    if idx >= old_token_count {
        return None;
    }

    let token = &compiler.tokens[idx];

    match token {
        // begin comment handling
        Token::Slash => {
            assert!(old_token_count > idx + 1);
            match &compiler.tokens[idx + 1] {
                Token::Slash => {
                    for look_ahead_count in idx + 2..old_token_count {
                        match &compiler.tokens[look_ahead_count] {
                            Token::NewLine => {
                                *skip_count = look_ahead_count - idx;
                                return None;
                            }
                            _ => {}
                        }
                    }
                    return None;
                }
                Token::Asterisk => {
                    for look_ahead_count in idx + 2..old_token_count {
                        match &compiler.tokens[look_ahead_count] {
                            Token::Asterisk => {
                                match &compiler.tokens[look_ahead_count + 1] {
                                    Token::Slash => {
                                        *skip_count = look_ahead_count + 1 - idx;
                                        return None;
                                    }
                                    _ => {}
                                }
                                return None;
                            }
                            _ => {}
                        }
                    }
                    // If we reach this point, we are at the end of the file and have not found a closing sequence for comments.
                    // Either way we stop herer
                    // TODO: Maybe output an error.
                    return None;
                }
                _ => {}
            }
        }
        // end comment handling

        // At this point we don't need the new lines anymore
        Token::NewLine => return None,
        Token::Minus => {
            assert!(old_token_count > idx + 1);
            match &compiler.tokens[idx + 1] {
                Token::CloseAngle => {
                    *skip_count = 1;
                    return Some(Token::ArrowRight);
                }
                _ => {}
            }
        }
        Token::Ampersand => match &compiler.tokens[idx + 1] {
            Token::DataType { data_type, .. } => {
                *skip_count = 1;
                let mut data_type = data_type.clone();
                data_type.is_ref = true;
                return Some(Token::DataType {
                    data_type: data_type,
                });
            }
            Token::Ampersand => {
                *skip_count = 1;
                return Some(Token::LogicalAnd);
            }
            _ => return None,
        },
        Token::Pipe => {
            assert!(old_token_count > idx + 1);
            match &compiler.tokens[idx + 1] {
                Token::Pipe => {
                    *skip_count = 1;
                    return Some(Token::LogicalOr);
                }
                _ => {}
            }
        }
        Token::PubModifier => {
            match jti(compiler, skip_count, idx + 1, old_token_count) {
                Some(token) => {
                    match token {
                        Token::FunctionDefinition { name, .. } => {
                            *skip_count += 1;
                            return Some(Token::FunctionDefinition { name, public: true });
                        }
                        _ => {
                            // TODO: log error no valid type specified
                            return None;
                        }
                    }
                }
                None => {
                    // TODO: log error no valid type specified
                    return None;
                }
            }
        }
        Token::Keyword { name } => match &name[..] {
            KEYWORD_FUNCTION => match jti(compiler, skip_count, idx + 1, old_token_count) {
                Some(token) => {
                    match token {
                        Token::Identifier { name } => {
                            *skip_count = 1;
                            return Some(Token::FunctionDefinition {
                                name,
                                public: false,
                            });
                        }
                        _ => {
                            // TODO: Log error. There should be an identifier after a "fn" keyword.
                            return None;
                        }
                    }
                }
                None => {
                    // TODO: Log error. There should be an identifier after a "fn" keyword.
                    return None;
                }
            },
            _ => return Some(token.clone()),
        },
        Token::Colon => match jti(compiler, skip_count, idx + 1, old_token_count) {
            Some(token) => {
                match token {
                    Token::DataType { .. } => {
                        *skip_count = 1;
                        return Some(token);
                    }
                    _ => {
                        // TODO: log error, not valid token found after colon.
                        return None;
                    }
                }
            }
            None => {
                // TODO: log error, colon cannot be the last character.
                return None;
            }
        },
        Token::DataType { .. } => {
            *skip_count = 0;
            return Some(token.clone());
        }
        Token::Identifier { name } => match jti(compiler, skip_count, idx + 1, old_token_count) {
            Some(token) => match token {
                Token::DataType { data_type } => {
                    *skip_count += 1;
                    return Some(Token::TypedVariable {
                        name: name.to_string(),
                        data_type,
                    });
                }
                _ => {
                    *skip_count = 0;
                    return Some(Token::Identifier {
                        name: name.to_string(),
                    });
                }
            },
            None => {
                // TODO: Identifier shouldn't be the last token.
                return None;
            }
        },
        _ => {
            return Some(token.clone());
        }
    };

    dbg!(token);
    unreachable!();
}

fn split_functions(compiler: &mut Compiler) {
    let mut function_body_start_index: usize = 0;
    let mut indent_count: u64 = 0;
    let mut current_function: Option<Function> = None;

    for (idx, token) in (&compiler.tokens).iter().enumerate() {
        match token {
            Token::FunctionDefinition { name, public } => {
                current_function = Some(Function {
                    name: name.to_string(),
                    body: Vec::new(),
                    parameters: Vec::new(),
                    return_type: DataTypeStruct {
                        data_type: DataType::None,
                        is_ref: false,
                    },
                    public: *public,
                });
            }
            Token::OpenCurly => {
                if indent_count == 0 {
                    function_body_start_index = idx + 1;
                }
                indent_count += 1;
            }
            Token::CloseCurly => {
                assert!(function_body_start_index != 0);
                indent_count -= 1;
                if indent_count == 0 {
                    match current_function {
                        Some(func) => {
                            let mut func = func.clone();

                            for token in (&compiler.tokens)[function_body_start_index..idx].iter() {
                                func.body.push(token.clone());
                            }

                            compiler.functions.push(func);
                            current_function = None;
                        }
                        None => unreachable!(),
                    }
                }
            }
            Token::ArrowRight => {
                assert!(compiler.tokens.len() > idx + 1);
                if indent_count == 0 {
                    match &compiler.tokens[idx + 1] {
                        Token::DataType { data_type, .. } => match &mut current_function {
                            Some(func) => {
                                func.return_type = data_type.clone();
                            }
                            None => {}
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn is_keyword(token: &String) -> bool {
    match &token[..] {
        KEYWORD_FUNCTION => true,
        KEYWORD_PUB => true,
        KEYWORD_MUT => true,
        KEYWORD_RETURN => true,
        KEYWORD_LET => true,
        KEYWORD_IF => true,
        KEYWORD_ELSE => true,
        _ => false,
    }
}

fn is_datatype(token: &String) -> bool {
    match &token[..] {
        "i64" => true,
        "u64" => true,
        "f64" => true,
        "bool" => true,
        _ => false,
    }
}

fn get_type_from_name(name: &String) -> DataTypeStruct {
    let inner_type = match &name[..] {
        "i64" => DataType::I64 { size: 8 },
        "u64" => DataType::U64 { size: 8 },
        "f64" => DataType::F64 { size: 8 },
        "bool" => DataType::Bool { size: 1 },
        _ => DataType::None,
    };

    return DataTypeStruct {
        data_type: inner_type,
        is_ref: false,
    };
}

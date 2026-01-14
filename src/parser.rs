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
        data_type: DataType,
        is_reference: bool,
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
    Undefined,
    Void,
    I64,
    U64,
    F64,
    Bool,
}

#[derive(Debug)]
pub struct Compiler {
    tokens: Vec<Token>,
    current_buffer: Vec<char>,
    current_token: Token,
    period_count: u64,
    next_char_escaped: bool,
}

pub fn compile(file_path: &String) -> Compiler {
    let mut compiler = Compiler {
        tokens: Vec::new(),
        current_buffer: Vec::new(),
        current_token: Token::Bogus,
        period_count: 0,
        next_char_escaped: false,
    };

    parse_file(file_path, &mut compiler);

    //dbg!(&compiler);

    merge_tokens_round_one(&mut compiler);

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
                        is_reference: false,
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
    let old_token_count: usize = (&compiler.tokens).len();

    'outer: for (idx, token) in (&compiler.tokens).iter().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        match token {
            // begin comment handling
            Token::Slash => {
                assert!(old_token_count > idx + 1);
                match &compiler.tokens[idx + 1] {
                    Token::Slash => {
                        for look_ahead_count in idx + 2..old_token_count {
                            match &compiler.tokens[look_ahead_count] {
                                Token::NewLine => {
                                    skip_count = look_ahead_count - idx;
                                    continue 'outer;
                                }
                                _ => {}
                            }
                        }
                        break 'outer;
                    }
                    Token::Asterisk => {
                        for look_ahead_count in idx + 2..old_token_count {
                            match &compiler.tokens[look_ahead_count] {
                                Token::Asterisk => {
                                    match &compiler.tokens[look_ahead_count + 1] {
                                        Token::Slash => {
                                            skip_count = look_ahead_count + 1 - idx;
                                            continue 'outer;
                                        }
                                        _ => {}
                                    }
                                    break 'outer;
                                }
                                _ => {}
                            }
                        }
                        // If we reach this point, we are at the end of the file and have not found a closing sequence for comments.
                        // Either way we stop herer
                        // TODO: Maybe output an error.
                        break 'outer;
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
        }
    }

    compiler.tokens = tokens;
}

fn jti(
    compiler: &mut Compiler,
    new_tokens: &mut Vec<Token>,
    skip_count: &mut usize,
    old_token_count: usize,
) {
}

/*fn merge_tokens(compiler: &mut Compiler) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut skip_count: u64 = 0;

    for (idx, token) in (&compiler.tokens).iter().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        match token {
            Token::Minus => {
                if (&compiler.tokens).len() > idx + 1 {
                    match (&compiler.tokens)[idx + 1] {
                        Token::CloseAngle => {
                            assert!((&compiler.tokens).len() > idx + 2);
                            match &compiler.tokens[idx + 2] {
                                Token::DataType { name: data_type } => {
                                    tokens.push(Token::ReturnTypeSpecifier {
                                        data_type: get_type_from_name(&data_type),
                                        is_reference: false,
                                    });
                                    skip_count = 2;
                                    continue;
                                }
                                Token::Ampersand => {
                                    assert!((&compiler.tokens).len() > idx + 3);
                                    match &compiler.tokens[idx + 3] {
                                        Token::DataType { name: data_type } => {
                                            tokens.push(Token::ReturnTypeSpecifier {
                                                data_type: get_type_from_name(&data_type),
                                                is_reference: true,
                                            });
                                            skip_count = 3;
                                            continue;
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                // TODO: Add multiple return types i.e. unnamed tuple.
                                _ => unreachable!(),
                            }
                        }
                        _ => {}
                    }
                }
            }

            Token::Keyword { name: key_name } => match &key_name[..] {
                "let" => {
                    assert!((&compiler.tokens).len() > idx + 1);
                    match &compiler.tokens[idx + 1] {
                        Token::Identifier { name: ident_name } => {}
                        Token::Keyword { name: key_name } => if key_name == "mut" {},
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            },

            _ => unreachable!(),
        }

        tokens.push(token.clone());
    }

    compiler.tokens = tokens;
}

fn merge_tokens_old(compiler: &mut Compiler) {
    let mut tokens: Vec<Token> = Vec::new();

    let mut pub_mod_active: bool = false;
    let mut func_decl_active: bool = false;
    let mut func_param_decl_active: bool = false;
    let mut skip_iteration_count: u64 = 0;

    for (idx, token) in (&compiler.tokens).iter().enumerate() {
        if skip_iteration_count > 0 {
            skip_iteration_count -= 1;
            continue;
        }

        match token {
            Token::Keyword { name: keyword_name } => {
                if keyword_name == "pub" {
                    pub_mod_active = true;
                } else if keyword_name == "fn" {
                    func_decl_active = true;
                }
            }
            Token::Identifier { name: ident_name } => {
                if func_param_decl_active {
                    assert!(func_decl_active);
                    assert!(compiler.tokens.len() > idx + 3);
                    match &compiler.tokens[idx + 1] {
                        Token::Colon => match &compiler.tokens[idx + 2] {
                            Token::Ampersand => {}
                            Token::DataType { name: type_name } => {
                                tokens.push(Token::ParamToken {
                                    name: ident_name.to_string(),
                                    data_type: get_type_from_name(type_name),
                                });
                                match &compiler.tokens[idx + 3] {
                                    Token::Comma => {
                                        skip_iteration_count += 3;
                                        continue;
                                    }
                                    Token::CloseParen => {
                                        skip_iteration_count += 3;
                                        func_param_decl_active = false;
                                        continue;
                                    }
                                    _ => {}
                                }
                                skip_iteration_count += 2;
                                continue;
                            }
                            _ => {}
                        },
                        _ => {}
                    }

                    tokens.push(Token::ParamToken {
                        name: ident_name.to_string(),
                        data_type: DataType::Undefined,
                    });
                } else if func_decl_active {
                    tokens.push(Token::Function {
                        name: ident_name.clone(),
                        public: pub_mod_active,
                        return_type: DataType::Undefined,
                        params: Vec::new(),
                        body: Vec::new(),
                    });
                    pub_mod_active = false;
                }
            }
            Token::OpenParen => {
                if func_decl_active {
                    func_param_decl_active = true;
                } else {
                    tokens.push(token.clone());
                }
            }
            Token::CloseParen => {
                if func_param_decl_active {
                    assert!(func_decl_active);
                    func_param_decl_active = false;
                }
            }
            _ => {
                tokens.push(token.clone());
            }
        }
    }

    compiler.tokens = tokens;
}*/

fn is_keyword(token: &String) -> bool {
    match &token[..] {
        "fn" => true,
        "pub" => true,
        "mut" => true,
        "return" => true,
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

fn get_type_from_name(name: &String) -> DataType {
    match &name[..] {
        "i64" => DataType::I64,
        "u64" => DataType::U64,
        "f64" => DataType::F64,
        "bool" => DataType::Bool,
        _ => DataType::Undefined,
    }
}

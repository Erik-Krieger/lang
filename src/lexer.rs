pub enum Token {
    Bogus,
}

pub fn parse_tokens(token_strings: &Vec<String>) -> Vec<Token> {
    let tokens: Vec<Token> = Vec::new();
    let mut skipIterationCount: u64 = 0;
    for token in token_strings {
        if skipIterationCount > 0 {
            skipIterationCount -= 1;
            continue;
        }
        /*
        match token {
            "(" => tokens.push(Token::Bogus),
            _ => unreachable!(),
        };*/
    }

    return Vec::new();
}

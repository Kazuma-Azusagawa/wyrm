#[derive(Debug, Clone, Copy)]
enum TokenType {
    ProgramError, // Used as a default type because of errors
    Function,     // func
    BraceLeft,    // {
    BraceRight,   // }
    BracketLeft,  // (
    BracketRight, // )
    LiteralInt,
    LiteralString,
    Variable,   // let
    TypeDefine, // ::
    SemiColon,  // ;
    Exit,       // exit
}

impl Default for Token {
    fn default() -> Self {
        Self {
            t_type: None,
            value: Default::default(),
        }
    }
}

struct Token {
    t_type: Option<TokenType>,
    value: Option<String>,
}

fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Default::default();
    let mut token_buffer: Token = Default::default();
    let mut buffer: String = Default::default();
    for index in 0..input.len() {
        if buffer == "func" {
            token_buffer.t_type = Some(TokenType::Function);
            buffer.clear();
        }
        if !input.chars().nth(index).unwrap().is_whitespace()
            && input.chars().nth(index).unwrap() != ' '
        {
            buffer.push(input.chars().nth(index).unwrap());
        }
        if token_buffer.t_type.is_some() {
            tokens.push(token_buffer);
            token_buffer = Default::default();
        }
    }
    return tokens;
}

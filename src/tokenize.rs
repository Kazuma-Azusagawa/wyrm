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
#[derive(Clone)]
struct Token {
    t_type: Option<TokenType>,
    value: String,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut flush = |buffer: &mut String, tokens: &mut Vec<Token>| {
        if buffer.is_empty() {
            return;
        }
        let t_type = match buffer.as_str() {
            "func" => Some(TokenType::Function),
            "{" => Some(TokenType::BraceLeft),
            "}" => Some(TokenType::BraceRight),
            "(" => Some(TokenType::BracketLeft),
            ")" => Some(TokenType::BracketRight),
            ";" => Some(TokenType::SemiColon),
            "::" => Some(TokenType::TypeDefine),
            "let" => Some(TokenType::Variable),
            _ => None,
        };
        tokens.push(Token {
            value: buffer.clone(),
            t_type,
        });
        buffer.clear();
    };

    for c in input.chars() {
        match c {
            ';' => flush(&mut buffer, &mut tokens),
            c if c.is_whitespace() => flush(&mut buffer, &mut tokens),
            _ => buffer.push(c),
        }
    }
    flush(&mut buffer, &mut tokens);
    tokens
}

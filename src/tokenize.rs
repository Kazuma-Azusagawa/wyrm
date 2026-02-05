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

fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Default::default();
    let mut token_buffer: Token = Default::default();
    let mut has_type: bool = false;
    for character in input.chars() {
        if has_type {
            token_buffer.value.push(character);
        } else if (character == ';') {
            tokens.push(token_buffer);
        } else {
            if &token_buffer.value == "func" {
                token_buffer.t_type = Some(TokenType::Function);
                has_type = true;
            }
            if !character.is_ascii_whitespace() && character != ' ' {
                token_buffer.value.push(character);
            }
            if token_buffer.t_type.is_some() {
                tokens.push(token_buffer);
                token_buffer = Default::default();
            }
        }
    }
    return tokens;
}

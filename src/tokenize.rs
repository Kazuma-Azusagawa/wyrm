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
    Variable, // let
    VariableValue,
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
    let mut tokens: Vec<Token> = Default::default();
    let mut token = Token::default();
    let mut buffer = String::new();
    for c in input.chars() {
        if c != ' ' && c != ';' {
            buffer.push(c);
        } else {
            token = match buffer.as_str() {
                "func" => Token {
                    t_type: Some(TokenType::Function),
                    value: buffer,
                },
                "::" => Token {
                    t_type: Some(TokenType::TypeDefine),
                    value: buffer,
                },
                "{" => Token {
                    t_type: Some(TokenType::BraceLeft),
                    value: buffer,
                },
                "}" => Token {
                    t_type: Some(TokenType::BraceRight),
                    value: buffer,
                },
                "(" => Token {
                    t_type: Some(TokenType::BracketLeft),
                    value: buffer,
                },
                ")" => Token {
                    t_type: Some(TokenType::BracketRight),
                    value: buffer,
                },
                "exit" => Token {
                    t_type: Some(TokenType::Exit),
                    value: buffer,
                },
                "let" => Token {
                    t_type: Some(TokenType::Variable),
                    value: buffer,
                },
                _ => Token {
                    t_type: Some(TokenType::ProgramError),
                    value: buffer,
                },
            };
            tokens.push(token);
            token = Default::default();
            buffer.clear();
        }
    }
    return tokens;
}

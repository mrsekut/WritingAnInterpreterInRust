// NOTE: フィールドをall pubにしたいときは全部`pub`って書かないといけないの？
pub struct Token {
    pub Type: TokenType,
    pub Literal: String, // NOTE: 理解して必要があれば&strにしよう
}

pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

// TODO: name
fn get_token_type(t: TokenType) -> String {
    let l = match t {
        TokenType::ILLEGAL => "ILLEGAL",
        TokenType::EOF => "EOF",
        TokenType::IDENT => "IDENT",
        TokenType::INT => "INT",
        TokenType::ASSIGN => "=",
        TokenType::PLUS => "+",
        TokenType::COMMA => ",",
        TokenType::SEMICOLON => ",",
        TokenType::LPAREN => "(",
        TokenType::RPAREN => ")",
        TokenType::LBRACE => "{",
        TokenType::RBRACE => "}",
        TokenType::FUNCTION => "FUNCTION",
        TokenType::LET => "LET",
    };

    l.to_string()
}

// const ILLEGAL: String = "ILLEGAL";
// const EOF: String = "EOF";

// const IDENT: String = "IDENT";
// const INT: String = "INT";

// const ASSIGN: String = "=";
// const PLUS: String = "+";

// const COMMA: String = ",";
// const SEMICOLON: String = ";";

// const LPAREN: String = "(";
// const RAPREN: String = ")";
// const LBRACE: String = "{";
// const RBRACE: String = "}";

// const FUNTION: String = "FUNCTION";
// const LET: String = "LET";

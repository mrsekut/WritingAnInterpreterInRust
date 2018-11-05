type TokenType = String;

struct Token {
    Type: TokenType,
    Literal: String,
}

const ILLEGAL: String = "ILLEGAL";
const EOF: String = "EOF";

const IDENT: String = "IDENT";
const INT: String = "INT";

const ASSIGN: String = "=";
const PLUS: String = "+";

const COMMA: String = ",";
const SEMICOLON: String = ";";

const LPAREN: String = "(";
const PRAPREN: String = ")";
const LBRACE: String = "{";
const RBRACE: String = "}";

const FUNTION: String = "FUNCTION";
const LET: String = "LET";

// NOTE: これ両方書かないといけないの？
use lexer::token::Token;
use lexer::token::TokenType;

#[derive(Debug)]

struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: String, // NOTE: char?
}

impl Lexer {
    fn new(input: String) -> Lexer {
        // NOTE: 適当に初期化
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: "".to_string(),
        };
        l.read_char();
        l
    }

    fn get_char(&self, index: i32) -> String {
        let op = self.input.chars().nth(index as usize);
        return match op {
            Some(i) => i.to_string(),
            None => "empty".to_string(),
        };
    }

    fn read_char(&mut self) {
        self.ch = self.get_char(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&self) -> Token {
        let token: Token;

        match &*self.ch {
            "=" => new_token(TokenType::ASSIGN, self.ch),
            ";" => new_token(TokenType::SEMICOLON, self.ch),
            "(" => new_token(TokenType::LPAREN, self.ch),
            ")" => new_token(TokenType::RPAREN, self.ch),
            "," => new_token(TokenType::COMMA, self.ch),
            "+" => new_token(TokenType::PLUS, self.ch),
            "{" => new_token(TokenType::LBRACE, self.ch),
            "}" => new_token(TokenType::RBRACE, self.ch),
            "0" => {
                token.Type = TokenType::EOF;
                token.Literal = "".to_string();
                token
            }
        }
    }
}

fn new_token(token_type: TokenType, ch: String) -> Token {
    Token {
        Type: token_type,
        Literal: ch.to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();

        let l = Lexer::new(input);

        let expects = vec![
            (TokenType::ASSIGN, "="),
            (TokenType::PLUS, "+"),
            (TokenType::LPAREN, "("),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RBRACE, "}"),
            (TokenType::COMMA, ","),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        for (token_type, literal) in expects {
            let t = l.next_token();
            // assert_eq!(t.Type, token_type);
            assert_eq!(t.Literal, literal);
        }
    }
}

#[derive(Debug)]

struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: String, // char?
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: "ss".to_string(),
        };
        l.read_char();
        l
    }

    fn get_char(self, string: String, index: i32) -> String {
        let op = string.chars().nth(index as usize);
        return match op {
            Some(i) => i.to_string(),
            None => "empty".to_string(),
        };
    }

    fn read_char(&mut self) {
        self.ch = self.get_char(self.input, self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();

        let l = Lexer::new(input);

        // let tok = l.NextToken();

        assert_eq!(l.input, "=+(){},;");
    }
}

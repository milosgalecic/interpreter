use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            Some('=') => Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Some('+') => Token { token_type: TokenType::Plus, literal: "+".to_string() },
            Some('(') => Token { token_type: TokenType::LParen, literal: "(".to_string() },
            Some(')') => Token { token_type: TokenType::RParen, literal: ")".to_string() },
            Some('{') => Token { token_type: TokenType::LBrace, literal: "{".to_string() },
            Some('}') => Token { token_type: TokenType::RBrace, literal: "}".to_string() },
            Some(',') => Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Some(';') => Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Some(ch) if Self::is_letter(ch) => {
                let ident = self.read_identifier();
                return Token {
                    token_type: Self::lookup_ident(&ident),
                    literal: ident,
                };
            }
            Some(ch) if Self::is_digit(ch) => {
                let number = self.read_number();
                return Token {
                    token_type: TokenType::Int(number.parse().unwrap()),  // Assuming the number is always valid
                    literal: number,
                };
            }
            None => Token { token_type: TokenType::EOF, literal: "".to_string() },
            _ => Token { token_type: TokenType::Illegal, literal: self.ch.unwrap().to_string() },
        };

        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position < self.input.len() {
            Some(self.input[self.read_position])
        } else {
            None
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while let Some(ch) = self.ch {
            if Self::is_letter(ch) {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_pos..self.position].iter().collect()
    }

    fn is_letter(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_digit(10)
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        while let Some(ch) = self.ch {
            if Self::is_digit(ch) {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_pos..self.position].iter().collect()
    }

    fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Identifiers(ident.to_string()),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;\nlet ten = 10;\nlet add = fn(x, y) {\n    x + y;\n};\nlet result = add(five, ten);\n";
        
        let expected_tokens = vec![
            (TokenType::Let, "let"),
            (TokenType::Identifiers("five".to_string()), "five"),
            (TokenType::Assign, "="),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifiers("ten".to_string()), "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int(10), "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifiers("add".to_string()), "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Identifiers("x".to_string()), "x"),
            (TokenType::Comma, ","),
            (TokenType::Identifiers("y".to_string()), "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Identifiers("x".to_string()), "x"),
            (TokenType::Plus, "+"),
            (TokenType::Identifiers("y".to_string()), "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifiers("result".to_string()), "result"),
            (TokenType::Assign, "="),
            (TokenType::Identifiers("add".to_string()), "add"),
            (TokenType::LParen, "("),
            (TokenType::Identifiers("five".to_string()), "five"),
            (TokenType::Comma, ","),
            (TokenType::Identifiers("ten".to_string()), "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        
        for (i, (expected_type, expected_literal)) in expected_tokens.iter().enumerate() {
            let tok = lexer.next_token();
            
            assert_eq!(tok.token_type, *expected_type, "Test [{}] - TokenType mismatch", i);
            assert_eq!(tok.literal, *expected_literal, "Test [{}] - Literal mismatch", i);
        }
    }
}

use crate::errors::error;

use crate::tokens::Token;
use crate::tokens::TokenKind;

pub struct Lexer {
    input: String,
    start: usize,   //start of the current token
    current: usize, //current position in input
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn current_char(&self) -> char {
        self.input.chars().nth(self.current).unwrap_or('\0')
    }

    pub fn next_char(&self) -> char {
        self.input.chars().nth(self.current + 1).unwrap_or('\0')
    }

    pub fn advance(&mut self) -> char {
        let c = self.current_char();
        self.current += 1;
        c
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.current_char() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn number(&mut self) -> Option<Token> {
        while self.current_char().is_digit(10) {
            self.advance();
        }

        if self.current_char() == '.' && self.next_char().is_digit(10) {
            self.advance();

            while self.current_char().is_digit(10) {
                self.advance();
            }
        }

        let literal = self.input[self.start..self.current].to_string();
        //parse into a double
        literal.parse::<f64>().ok();
        Some(self.add_token_with_literal(TokenKind::Number, literal))
    }

    fn string(&mut self) -> Option<Token> {
        while self.current_char() != '"' && !self.is_at_end() {
            if self.current_char() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return None;
        }

        self.advance();

        let literal = self.input[self.start + 1..self.current - 1].to_string();

        Some(self.add_token_with_literal(TokenKind::String, literal))
    }

    fn comment(&mut self) -> Option<Token> {
        while self.current_char() != '\n' && !self.is_at_end() {
            self.skip_whitespace();
            self.advance();
        }

        Some(self.add_token(TokenKind::Comment))

    }

    fn add_token_with_literal(&mut self, kind: TokenKind, literal: String) -> Token {
        Token::new_with_literal(kind, literal, self.line)
    }

    fn add_token(&mut self, kind: TokenKind) -> Token {
        Token::new(kind, self.line)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.current_char() != expected {
            return false;
        }

        self.current += 1;
        true
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            return None;
        }

        self.skip_whitespace();

        self.start = self.current;

        match self.advance() {
            '(' => Some(self.add_token(TokenKind::LeftParen)),
            ')' => Some(self.add_token(TokenKind::RightParen)),
            '{' => Some(self.add_token(TokenKind::LeftBrace)),
            '}' => Some(self.add_token(TokenKind::RightBrace)),
            '[' => Some(self.add_token(TokenKind::RightSquare)),
            ']' => Some(self.add_token(TokenKind::LeftSquare)),
            ',' => Some(self.add_token(TokenKind::Comma)),
            '.' => Some(self.add_token(TokenKind::Dot)),
            '-' => Some(self.add_token(TokenKind::Minus)),
            '+' => Some(self.add_token(TokenKind::Plus)),
            ';' => Some(self.add_token(TokenKind::Semicolon)),
            '*' => Some(self.add_token(TokenKind::Asterisk)),

            '!' => {
                if self.current_char() == '=' {
                    self.advance();
                    Some(self.add_token(TokenKind::NotEqual))
                } else {
                    Some(self.add_token(TokenKind::Bang))
                }
            }

            '=' => {
                if self.current_char() == '=' {
                    self.advance();
                    Some(self.add_token(TokenKind::EqualEqual))
                } else {
                    Some(self.add_token(TokenKind::Equal))
                }
            }

            '<' => {
                if self.current_char() == '=' {
                    self.advance();
                    Some(self.add_token(TokenKind::LessThanEqual))
                } else {
                    Some(self.add_token(TokenKind::LessThan))
                }
            }

            '>' => {
                if self.current_char() == '=' {
                    self.advance();
                    Some(self.add_token(TokenKind::GreaterThanEqual))
                } else {
                    Some(self.add_token(TokenKind::GreaterThan))
                }
            }

            '/' => Some(self.add_token(TokenKind::Slash)),

            '#' => self.comment(),

            '"' => self.string(),

            '0'..='9' => self.number(),

            '\0' => {
                Some(self.add_token(TokenKind::EOF))
            },

            _ => {
                error(self.line, "Unexpected character.");
                None
            }
        }
    }
}
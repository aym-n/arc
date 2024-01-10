use crate::tokens::*;

pub struct Lexer {
    input: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input + "\0",
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

        Some(self.add_token_with_literal(TokenKind::Number, Object::Num(literal.parse::<f64>().unwrap())))
    }

    fn string(&mut self) -> Option<Token> {
        while self.current_char() != '"' && !self.is_at_end() {
            if self.current_char() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("{}: Unterminated string.", self.line);
            return None;
        }

        self.advance();

        let literal = self.input[self.start + 1..self.current - 1].to_string();

        Some(self.add_token_with_literal(TokenKind::String, Object::Str(literal)))
    }

    fn comment(&mut self) -> Option<Token> {
        while self.current_char() != '\n' && !self.is_at_end() {
            self.skip_whitespace();
            self.advance();
        }

        Some(self.add_token(TokenKind::Comment))

    }

    fn add_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.input[self.start..self.current].to_string(), None, self.line)
    }

    fn add_token_with_literal(&self, kind: TokenKind, literal: Object) -> Token {
        Token::new(kind, self.input[self.start..self.current].to_string(), Some(literal), self.line)
    }

    fn is_at_end(&self) -> bool {
        self.current > self.input.len() - 1
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

            'a'..='z' | 'A'..='Z' | '_' => {
                while self.current_char().is_alphanumeric() {
                    self.advance();
                }

                let text = &self.input[self.start..self.current];

                match text {
                    "and" => Some(self.add_token(TokenKind::And)),
                    "class" => Some(self.add_token(TokenKind::Class)),
                    "else" => Some(self.add_token(TokenKind::Else)),
                    "false" => Some(self.add_token(TokenKind::False)),
                    "for" => Some(self.add_token(TokenKind::For)),
                    "fn" => Some(self.add_token(TokenKind::Fn)),
                    "if" => Some(self.add_token(TokenKind::If)),
                    "nil" => Some(self.add_token(TokenKind::Nil)),
                    "or" => Some(self.add_token(TokenKind::Or)),
                    "print" => Some(self.add_token(TokenKind::Print)),
                    "return" => Some(self.add_token(TokenKind::Return)),
                    "super" => Some(self.add_token(TokenKind::Super)),
                    "this" => Some(self.add_token(TokenKind::This)),
                    "true" => Some(self.add_token(TokenKind::True)),
                    "var" => Some(self.add_token(TokenKind::Var)),
                    "while" => Some(self.add_token(TokenKind::While)),
                    _ => {
                        let literal = self.input[self.start..self.current].to_string();
                        Some(self.add_token_with_literal(TokenKind::Identifier, Object::Str(literal)))
                    },
                }
            }

            _ => {
                eprintln!("{}: Unexpected character: {}", self.line, self.current_char());
                None
            }
        }
    }
}

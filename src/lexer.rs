use crate::token::{Token, keyword_to_token};

/// ESCAPE QUOTE IS A CONSTANT FOR THE LEXER PROGRAM.
const ESCAPE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';
const DECIMAL_MARKER: char = '.';
const UNDERSCORE: char = '_';
const STAR: char = '*';
const COMMA: char = ',';
const DOT: char = '.';
const SEMICOLON: char = ';';
const NOT: char = '!';
const EQ: char = '=';
const LT: char = '<';
const GT: char = '>';
const PLUS: char = '+';
const MINUS: char = '-';
const DIVISOR: char = '/';
const MODULO: char = '%';
const PIPE: char = '|';
const LEFT_PARENTHESES: char = '(';
const RIGHT_PARENTHESES: char = ')';

/// SQL Lexer implementation.
pub(crate) struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    // Creates a new lexer with an specific input.
    // Positions the cursor of the lexer at the beginning of the chars array.
    pub(crate) fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = if chars.is_empty() {
            None
        } else {
            Some(chars[0])
        };

        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    /// Advances the cursor of the lexer to the next position.
    pub(crate) fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
    }

    /// Peek the char offseted at [offset] from  the lexer cursor position in the input string
    fn peek(&self, offset: usize) -> Option<char> {
        let pos = self.position + offset;
        if pos < self.input.len() {
            Some(self.input[pos])
        } else {
            None
        }
    }

    /// Advances the cursor until the next non-whitespace position.
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Consume a string from the input buffer.
    /// In SQL, strings are formatted between quotes.
    /// Therefore we need to skip them to get to consume the actual information.
    fn read_string(&mut self) -> String {
        let mut result = String::new();
        self.advance(); // Skip opening quote

        while let Some(ch) = self.current_char {
            if ch == ESCAPE_QUOTE {
                // Check for escaped quote
                if self.peek(1) == Some(ESCAPE_QUOTE) {
                    result.push(ESCAPE_QUOTE);
                    self.advance();
                    self.advance();
                } else {
                    self.advance(); // Skip closing quote
                    break;
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }

        result
    }

    // Read a number from the input buffer.
    // On my implementation, decimals are represented with dots.
    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == DECIMAL_MARKER {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        num_str.parse().unwrap_or(0.0)
    }

    /// Read an identifier from the input buffer.
    /// Identifiers are allowed to contain alphanumeric characters and underscores.
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == UNDERSCORE {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    /// Read the next token.
    /// First, advances the cursor to the next token position (position where the whitespace ends).
    /// Reads the next token, dispatching to the corresponding reader functio based on the input.
    /// If the next char is an [ESCAPE_QUOTE], tries to interpret everything between it and the next [ESCAPE_QUOTE] as a string.
    /// If the next char is a [DOUBLE_QUOTE] tries to interpret anything until the next  [DOUBLE_QUOTE] as an identifier.
    /// Single-char tokens are pretty easy to peek any other way.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            None => Token::Eof,
            Some(ESCAPE_QUOTE) => Token::StringLiteral(self.read_string()),
            Some(DOUBLE_QUOTE) => {
                self.advance();
                let mut ident = String::new();
                while let Some(ch) = self.current_char {
                    if ch == DOUBLE_QUOTE {
                        self.advance();
                        break;
                    }
                    ident.push(ch);
                    self.advance();
                }
                Token::Identifier(ident)
            }
            Some(ch) if ch.is_ascii_digit() => Token::NumberLiteral(self.read_number()),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let ident = self.read_identifier();
                keyword_to_token(&ident)
            }
            Some(STAR) => {
                self.advance();
                Token::Star
            }
            Some(COMMA) => {
                self.advance();
                Token::Comma
            }
            Some(DOT) => {
                self.advance();
                Token::Dot
            }
            Some(SEMICOLON) => {
                self.advance();
                Token::Semicolon
            }
            Some(LEFT_PARENTHESES) => {
                self.advance();
                Token::LParen
            }
            Some(RIGHT_PARENTHESES) => {
                self.advance();
                Token::RParen
            }
            Some(EQ) => {
                self.advance();
                Token::Eq
            }
            Some(NOT) => {
                self.advance();
                if self.current_char == Some(EQ) {
                    self.advance();
                    Token::Neq
                } else {
                    Token::Not
                }
            }
            Some(LT) => {
                self.advance();
                if self.current_char == Some(EQ) {
                    self.advance();
                    Token::Le

                // <> means not equal in ANSI-SQL.
                } else if self.current_char == Some(GT) {
                    self.advance();
                    Token::Neq
                } else {
                    Token::Lt
                }
            }
            Some(GT) => {
                self.advance();
                if self.current_char == Some(EQ) {
                    self.advance();
                    Token::Ge
                } else {
                    Token::Gt
                }
            }
            Some(PLUS) => {
                self.advance();
                Token::Plus
            }
            Some(MINUS) => {
                self.advance();
                // Check for comments
                if self.current_char == Some('-') {
                    // Skip until end of line
                    while let Some(ch) = self.current_char {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                    self.next_token()
                } else {
                    Token::Minus
                }
            }
            Some(DIVISOR) => {
                self.advance();
                Token::Slash
            }
            Some(MODULO) => {
                self.advance();
                Token::Percent
            }
            Some(PIPE) => {
                self.advance();
                if self.current_char == Some(PIPE) {
                    self.advance();
                    Token::Concat
                } else {
                    // Single | is not a valid SQL operator, treat as unknown
                    self.next_token()
                }
            }
            _ => {
                self.advance();
                self.next_token()
            }
        }
    }

    /// Peek the next token, without advancing the cursor.
    pub(crate) fn peek_token(&mut self) -> Token {
        let saved_position = self.position;
        let saved_char = self.current_char;

        let token = self.next_token();

        self.position = saved_position;
        self.current_char = saved_char;

        token
    }
}

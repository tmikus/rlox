use std::any::Any;
use std::ptr::null;
use token::Token;
use token_type::TokenType;
use token_type::TokenType::*;
use error::error;
use runtime_status::RuntimeStatus;

pub struct Scanner {
  source: String,
  tokens: Vec<Token>,
  start: i32,
  current: i32,
  line: i32,
}

impl Scanner {
  pub fn new(source: String) -> Scanner {
    Scanner {
      source,
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
    }
  }

  pub fn scan_tokens(&mut self, runtime_status: &mut RuntimeStatus) -> &Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token(runtime_status);
    }
    self.tokens.push(Token::new(EOF, "".to_string(), None, self.line));
    &self.tokens
  }

  fn scan_token(&mut self, runtime_status: &mut RuntimeStatus) {
    let c = self.advance();
    match c {
      '(' => self.add_token(LEFT_PAREN, None),
      ')' => self.add_token(RIGHT_PAREN, None),
      '{' => self.add_token(LEFT_BRACE, None),
      '}' => self.add_token(RIGHT_BRACE, None),
      ',' => self.add_token(COMMA, None),
      '.' => self.add_token(DOT, None),
      '-' => self.add_token(MINUS, None),
      '+' => self.add_token(PLUS, None),
      ';' => self.add_token(SEMICOLON, None),
      '*' => self.add_token(STAR, None),
      '!' => {
        let token = if self.match_current('=') { BANG_EQUAL } else { BANG };
        self.add_token(token, None);
      },
      '=' => {
        let token = if self.match_current('=') { EQUAL_EQUAL } else { EQUAL };
        self.add_token(token, None);
      },
      '<' => {
        let token = if self.match_current('=') { LESS_EQUAL } else { LESS };
        self.add_token(token, None);
      },
      '>' => {
        let token = if self.match_current('=') { GREATER_EQUAL } else { GREATER };
        self.add_token(token, None);
      },
      '/' => {
        if self.match_current('/') {
          while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
          }
        } else {
          self.add_token(SLASH, None);
        }
      },
      ' ' | '\r' | '\t' => { /* Ignore whitespace */ },
      '\n' => {
        self.line += 1;
      },
      '"' => self.string(runtime_status),
      _ => {
        if is_digit(c) {
          self.number();
        } else {
          let message = format!("Unexpected character: {}", c);
          error(runtime_status, self.line, &message);
        }
      },
    };
  }

  fn string(&mut self, runtime_status: &mut RuntimeStatus) {
    while self.peek() != '"' && !self.is_at_end() {
      if self.peek() == '\n' {
        self.line += 1;
      }
      self.advance();
    }
    if self.is_at_end() {
      error(runtime_status, self.line, "Unterminated string.");
      return;
    }
    self.advance();
    let value = substring(&self.source, self.start + 1, self.current - 1);
    self.add_token(STRING, Some(Box::new(value)));
  }

  fn number(&mut self) {
    while is_digit(self.peek()) {
      self.advance();
    }
    if self.peek() == '.' && is_digit(self.peek_next()) {
      self.advance();
      while is_digit(self.peek()) {
        self.advance();
      }
    }
    let number_string = substring(&self.source, self.start, self.current);
    let number = number_string.parse::<f64>().unwrap();
    self.add_token(NUMBER, Some(Box::new(number)));
  }

  fn match_current(&mut self, expected: char) -> bool {
    if self.is_at_end() { return false; }
    if char_at(&self.source, self.current) != expected { return false; }
    self.current += 1;
    true
  }

  fn peek(&self) -> char {
    if self.is_at_end() {
      '\0'
    } else {
      char_at(&self.source, self.current)
    }
  }

  fn peek_next(&self) -> char {
    let next = self.current + 1;
    if (next as usize) >= self.source.len() {
      '\0'
    } else {
      char_at(&self.source, next)
    }
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len() as i32
  }

  fn advance(&mut self) -> char {
    let character = char_at(&self.source, self.current);
    self.current += 1;
    character
  }

  fn add_token(&mut self, token_type: TokenType, literal: Option<Box<Any>>) {
    let start = self.start as usize;
    let end = self.current as usize;
    let text = self.source[start..end].to_string();
    self.tokens.push(Token::new(token_type, text, literal, self.line));
  }
}

fn char_at(value: &str, index: i32) -> char {
  value.chars().nth(index as usize).unwrap()
}

fn substring(value: &str, start: i32, end: i32) -> String {
  value[(start as usize)..(end as usize)].to_string()
}

fn is_digit(c: char) -> bool {
  c >= '0' && c <= '9'
}

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
      source: source,
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
    let token = match c {
      '(' => LEFT_PAREN,
      ')' => RIGHT_PAREN,
      '{' => LEFT_BRACE,
      '}' => RIGHT_BRACE,
      ',' => COMMA,
      '.' => DOT,
      '-' => MINUS,
      '+' => PLUS,
      ';' => SEMICOLON,
      '*' => STAR,
      _ => {
        let message = format!("Unexpected character: {}", c);
        error(runtime_status, self.line, &message);
        return;
      },
    };
    self.add_token(token, None);
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len() as i32
  }

  fn advance(&mut self) -> char {
    let character = self.source.chars().nth(self.current as usize).unwrap();
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

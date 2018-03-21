use std::any::Any;
use std::fmt;
use token_type::TokenType;

pub struct Token {
  token_type: TokenType,
  lexeme: String,
  literal: Option<Box<Any>>,
  line: i32,
}

impl Token {
  pub fn new(token_type: TokenType, lexeme: String, literal: Option<Box<Any>>, line: i32) -> Token {
    Token {
      token_type: token_type,
      lexeme: lexeme,
      literal: literal,
      line: line,
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
  }
}

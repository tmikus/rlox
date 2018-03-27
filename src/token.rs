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
    match self.literal {
      Some(ref value) => {
        if let Some(string) = value.downcast_ref::<String>() {
          format_value(formatter, &self.token_type, &self.lexeme, string)
        } else if let Some(double) = value.downcast_ref::<f64>() {
          format_value(formatter, &self.token_type, &self.lexeme, double)
        } else {
          format_value(formatter, &self.token_type, &self.lexeme, "None")
        }
      },
      None => format_value(formatter, &self.token_type, &self.lexeme, "None")
    }
  }
}

fn format_value<T: fmt::Display>(
  formatter: &mut fmt::Formatter,
  token_type: &TokenType,
  lexeme: &String,
  value: T,
) -> fmt::Result {
  write!(formatter, "{:?} {} {}", token_type, lexeme, value)
}

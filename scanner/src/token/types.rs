// NOTE: These are the enums used to make the scanner
// and achieve the regular which is the layer-2 of the Chmosky Hierarchy

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,
  Comma,
  Dot,
  Minus,
  MinusEqual,
  MinusMinus,
  Plus,
  PlusEqual,
  PlusPlus,
  Divide,
  DivideEqual,
  Multiply,
  MultiplyEqual,
  SemiColon,
  Colon,
  Question,
  Modulus,
  // One or two character tokens.
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  // Literals.
  Identifier,
  String,
  Number,
  // Keywords.
  And,
  Class,
  NullChar,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Return,
  Super,
  This,
  True,
  Var,
  While,
  Eof,
  Break,
  Continue,
  Comment,
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      // Single-character tokens
      TokenType::LeftParen => "(",
      TokenType::RightParen => ")",
      TokenType::LeftBrace => "{",
      TokenType::RightBrace => "}",
      TokenType::LeftBracket => "[",
      TokenType::RightBracket => "]",
      TokenType::Comma => ",",
      TokenType::Dot => ".",
      TokenType::Minus => "-",
      TokenType::MinusEqual => "-=",
      TokenType::MinusMinus => "--",
      TokenType::Plus => "+",
      TokenType::PlusEqual => "+=",
      TokenType::PlusPlus => "++",
      TokenType::Divide => "/",
      TokenType::DivideEqual => "/=",
      TokenType::Multiply => "*",
      TokenType::MultiplyEqual => "*=",
      TokenType::SemiColon => ";",
      TokenType::Colon => ":",
      TokenType::Question => "?",
      TokenType::Modulus => "%",

      // One or two character tokens
      TokenType::Bang => "!",
      TokenType::BangEqual => "!=",
      TokenType::Equal => "=",
      TokenType::EqualEqual => "==",
      TokenType::Greater => ">",
      TokenType::GreaterEqual => ">=",
      TokenType::Less => "<",
      TokenType::LessEqual => "<=",

      // Literals
      TokenType::Identifier => "identifier",
      TokenType::String => "string",
      TokenType::Number => "number",

      // Keywords
      TokenType::And => "and",
      TokenType::Class => "class",
      TokenType::NullChar => "null",
      TokenType::Else => "else",
      TokenType::False => "false",
      TokenType::Fun => "fun",
      TokenType::For => "for",
      TokenType::If => "if",
      TokenType::Nil => "nil",
      TokenType::Or => "or",
      TokenType::Return => "return",
      TokenType::Super => "super",
      TokenType::This => "this",
      TokenType::True => "true",
      TokenType::Var => "var",
      TokenType::While => "while",
      TokenType::Eof => "eof",
      TokenType::Break => "break",
      TokenType::Continue => "continue",
      TokenType::Comment => "comment",
    };
    write!(f, "{}", s)
  }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Literal {
  Number,
  String,
  Boolean,
  Nil,
}

impl std::fmt::Display for Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let text = match self {
      Literal::Number => "Number",
      Literal::String => "String",
      Literal::Boolean => "Boolean",
      Literal::Nil => "Nil",
    };
    write!(f, "{}", text)
  }
}

// NOTE: These are the enums used to make the scanner
// and achieve the regular which is the layer-2 of the Chmosky Hierarchy

#[derive(Debug, Clone, PartialEq)]
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
  Star,
  StarEqual,
  SemiColon,
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
  Print,
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

#[derive(Debug, Clone)]
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

// NOTE: These are the enums used to make the scanner
// and achieve the regular which is the layer-2 of the Chmosky Hierarchy

#[derive(Debug, Clone)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Divide,
  Modulus,
  Star,
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

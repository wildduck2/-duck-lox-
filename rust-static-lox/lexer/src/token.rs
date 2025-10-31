use diagnostic::diagnostic::Span;

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
  // Single-character tokens.
  Let,
  Fn,
  TrueLiteral,
  FalseLiteral,
  Nil,
  Return,
  Break,
  Continue,
  Import,
  From,
  Export,
  As,
  If,
  Else,
  While,
  For,
  Loop,
  Match,
  Type,
  Struct,
  Trait,
  Impl,
  Interface,
  Enum,
  SelfKeyword,
  Super,
  Const,
  Function,
  Await,

  // One or two character tokens.
  Plus,
  Minus,
  Star,
  Slash,
  Percent,
  Caret,
  Bang,
  BangEqual,
  EqualEqual,
  Greater,
  Less,
  GreaterEqual,
  LessEqual,
  Ampersand,
  Pipe,
  CaretEqual,
  Tilde,

  // Literals.
  Identifier,
  StringLiteral,
  IntegerLiteral,
  FloatLiteral,
  BooleanLiteral,
  NilLiteral,

  // Keywords.
  Keyword,

  // Delimiters.
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,
  Dot,
  Comma,
  Colon,
  Semicolon,
  Question,

  // Operators.
  Equal,
  LessEqualGreater,
  GreaterEqualLess,
  And,
  Or,

  // Comments
  SingleLineComment,
  MultiLineComment,

  // Misc.
  Eof,
}

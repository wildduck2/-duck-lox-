use diagnostic::diagnostic::Span;

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
  // 1️⃣ Keywords
  Let,
  Mut,
  Const,
  Fn,
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
  Function,
  Await,

  // 2️⃣ Literals
  Identifier,
  String,
  Int,
  Float,
  Bool,
  Nil,
  True,
  False,
  Void,

  // 3️⃣ Operators

  // Arithmetic
  Plus,    // +
  Minus,   // -
  Star,    // *
  Slash,   // /
  Percent, // %
  Caret,   // ^

  // Logical / Bitwise
  Bang,      // !
  Ampersand, // &
  Pipe,      // |
  Tilde,     // ~

  // Comparison
  Greater,      // >
  GreaterEqual, // >=
  Less,         // <
  LessEqual,    // <=
  EqualEqual,   // ==
  BangEqual,    // !=
  Equal,        // =

  // Compound / Misc
  CaretEqual,       // ^=
  LessEqualGreater, // <=>
  GreaterEqualLess, // >=<
  And,              // and
  Or,               // or

  // 4️⃣ Delimiters
  LeftParen,    // (
  RightParen,   // )
  LeftBrace,    // {
  RightBrace,   // }
  LeftBracket,  // [
  RightBracket, // ]
  Dot,          // .
  Comma,        // ,
  Colon,        // :
  Semicolon,    // ;
  Question,     // ?

  // 5️⃣ Comments
  SingleLineComment, // //
  MultiLineComment,  // /* ... */

  // 6️⃣ Miscellaneous
  Keyword, // For general keyword handling if needed
  Eof,     // End of file/input
}

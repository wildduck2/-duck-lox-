use diagnostic::Span;

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
  // 1️⃣ Keywords
  Break,
  Case,
  Catch,
  Class,
  Const,
  Continue,
  Debugger,
  Default,
  Delete,
  Do,
  Else,
  Enum,
  Export,
  Extends,
  False,
  Finally,
  For,
  Function,
  If,
  Import,
  In,
  Instanceof,
  New,
  Null,
  Return,
  Super,
  Switch,
  This,
  Throw,
  True,
  Try,
  Typeof,
  Var,
  Void,
  While,
  With,
  Yield,
  Await,
  As,
  Implements,
  Interface,
  Let,
  Package,
  Private,
  Protected,
  Public,
  Static,
  Any,
  Boolean,
  Constructor,
  Declare,
  Get,
  Module,
  Namespace,
  Require,
  Number,
  Set,
  String,
  Symbol,
  Type,
  From,
  Of,
  BigInt,
  Never,
  Unknown,

  // 2️⃣ Literals
  Identifier,
  Template,
  RegularExpression,
  Undefined,

  // 3️⃣ Operators
  // Arithmetic
  Plus,       // +
  Minus,      // -
  Star,       // *
  Slash,      // /
  Percent,    // %
  PlusPlus,   // ++
  MinusMinus, // --

  // Assignment
  Equal,                      // =
  PlusEqual,                  // +=
  MinusEqual,                 // -=
  StarEqual,                  // *=
  SlashEqual,                 // /=
  PercentEqual,               // %=
  AmpEqual,                   // &=
  PipeEqual,                  // |=
  CaretEqual,                 // ^=
  LessLessEqual,              // <<=
  GreaterGreaterEqual,        // >>=
  GreaterGreaterGreaterEqual, // >>>=

  // Bitwise / Logical
  Ampersand, // &
  Pipe,      // |
  Caret,     // ^
  Tilde,     // ~
  Bang,      // !
  AmpAmp,    // &&
  PipePipe,  // ||

  // Comparison
  EqualEqual,      // ==
  NotEqual,        // !=
  EqualEqualEqual, // ===
  NotEqualEqual,   // !==
  Greater,         // >
  GreaterEqual,    // >=
  Less,            // <
  LessEqual,       // <=

  // Shift
  LessLess,              // <<
  GreaterGreater,        // >>
  GreaterGreaterGreater, // >>>

  // Misc
  Question,  // ?
  Colon,     // :
  Dot,       // .
  DotDotDot, // ...
  Comma,     // ,
  Semicolon, // ;
  Backtick,  // `
  At,        // @
  Hash,      // #
  Arrow,     // =>

  // 4️⃣ Delimiters
  LeftParen,    // (
  RightParen,   // )
  LeftBrace,    // {
  RightBrace,   // }
  LeftBracket,  // [
  RightBracket, // ]

  // 5️⃣ Comments
  SingleLineComment, // //
  MultiLineComment,  // /* ... */

  // 6️⃣ End of file
  Eof,
}

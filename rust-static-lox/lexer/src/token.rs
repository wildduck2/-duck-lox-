use diagnostic::Span;

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

// ============================================================================
// SUPPORTING ENUMS
// ============================================================================

/// Style of a doc comment
///
/// # Examples
/// - `Outer`: `/// Documentation` or `/** Documentation */`
/// - `Inner`: `//! Module docs` or `/*! Module docs */`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocStyle {
  /// Outer doc comment: `///` or `/**`
  Outer,
  /// Inner doc comment: `//!` or `/*!`
  Inner,
}

/// Base of numeric literal encoding according to its prefix
///
/// # Examples
/// - `Binary`: `0b1010`
/// - `Octal`: `0o755`
/// - `Decimal`: `42`
/// - `Hexadecimal`: `0xDEADBEEF`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
  /// `0b` prefix - Binary (base 2)
  Binary = 2,
  /// `0o` prefix - Octal (base 8)
  Octal = 8,
  /// No prefix - Decimal (base 10)
  Decimal = 10,
  /// `0x` prefix - Hexadecimal (base 16)
  Hexadecimal = 16,
}

/// Errors that can occur when parsing raw string literals
///
/// Raw strings use the syntax `r#"..."#` where the number of `#`
/// characters must match on both sides
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawStrError {
  /// Non-`#` characters exist between `r` and `"`, e.g. `r#~".."`
  InvalidStarter { bad_char: char },

  /// The string was never terminated
  ///
  /// # Example
  /// `r##"hello` expects `"##` but found EOF
  NoTerminator {
    /// Number of `#` characters expected
    expected: usize,
    /// Number of `#` characters found
    found: usize,
    /// Byte offset where a possible terminator was found
    possible_terminator_offset: Option<usize>,
  },

  /// More than 65535 `#` delimiters
  ///
  /// Rust limits raw string delimiters to u16::MAX
  TooManyDelimiters { found: usize },
}

/// Enum representing all literal types supported by Rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
  /// Integer literal: `12_u8`, `0o100`, `0b1010`, `0xDEAD`, `42i64`
  ///
  /// # Fields
  /// - `base`: The numeric base (binary, octal, decimal, hex)
  /// - `empty_int`: true if no digits follow the base prefix (e.g., `0x`)
  Int { base: Base, empty_int: bool },

  /// Floating point literal: `12.34f32`, `1e3`, `3.14`, `2.5E-10`
  ///
  /// Note: `1f32` is an Int with suffix, not a Float
  ///
  /// # Fields
  /// - `base`: The numeric base (typically Decimal, but hex floats exist)
  /// - `empty_exponent`: true if exponent marker exists but no digits follow (e.g., `1e`)
  Float { base: Base, empty_exponent: bool },

  /// Character literal: `'a'`, `'\n'`, `'\u{1F980}'`, `'🦀'`
  ///
  /// # Fields
  /// - `terminated`: false if closing `'` is missing
  Char { terminated: bool },

  /// Byte literal: `b'a'`, `b'\n'`, `b'\x7F'`
  ///
  /// Must contain ASCII-only characters
  ///
  /// # Fields
  /// - `terminated`: false if closing `'` is missing
  Byte { terminated: bool },

  /// String literal: `"hello"`, `"foo\nbar"`, `"multi
  /// line"`
  ///
  /// # Fields
  /// - `terminated`: false if closing `"` is missing
  Str { terminated: bool },

  /// Byte string literal: `b"hello"`, `b"\x48\x69"`
  ///
  /// Must contain ASCII-only characters
  ///
  /// # Fields
  /// - `terminated`: false if closing `"` is missing
  ByteStr { terminated: bool },

  /// C string literal: `c"hello"`, `c"null-terminated\0"`
  ///
  /// Added in Rust 1.77, creates a `&CStr`
  ///
  /// # Fields
  /// - `terminated`: false if closing `"` is missing
  CStr { terminated: bool },

  /// Raw string literal: `r"no escapes"`, `r#"with "quotes" "#`, `r###"custom"###`
  ///
  /// No escape sequences are processed
  ///
  /// # Fields
  /// - `n_hashes`: Number of `#` delimiters used
  /// - `err`: Optional error if malformed
  RawStr {
    n_hashes: u16,
    err: Option<RawStrError>,
  },

  /// Raw byte string literal: `br"bytes"`, `br#"raw bytes"#`
  ///
  /// Combination of raw and byte string features
  ///
  /// # Fields
  /// - `n_hashes`: Number of `#` delimiters used
  /// - `err`: Optional error if malformed
  RawByteStr {
    n_hashes: u16,
    err: Option<RawStrError>,
  },

  /// Raw C string literal: `cr"c string"`, `cr#"raw c"#`
  ///
  /// Added in Rust 1.77
  ///
  /// # Fields
  /// - `n_hashes`: Number of `#` delimiters used
  /// - `err`: Optional error if malformed
  RawCStr {
    n_hashes: u16,
    err: Option<RawStrError>,
  },
}

// ============================================================================
// MAIN TOKEN KIND ENUM
// ============================================================================

/// The lexeme-level kind of token in Rust
///
/// This enum represents all possible tokens that can be produced by
/// the Rust lexer, including literals, keywords, operators, and special tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
  // ------------------------------------------------------------------------
  // COMMENTS & WHITESPACE
  // ------------------------------------------------------------------------
  /// Line comment: `// comment` or `/// doc comment` or `//! inner doc`
  ///
  /// Extends from `//` to the end of the line (not including newline)
  LineComment { doc_style: Option<DocStyle> },

  /// Block comment: `/* comment */` or `/** doc */` or `/*! inner doc */`
  ///
  /// # Fields
  /// - `doc_style`: If present, this is a documentation comment
  /// - `terminated`: false if closing `*/` is missing
  BlockComment {
    doc_style: Option<DocStyle>,
    terminated: bool,
  },

  /// Any whitespace character sequence: spaces, tabs, newlines, etc.
  ///
  /// Includes: ` `, `\t`, `\n`, `\r`, and other Unicode whitespace
  Whitespace,

  // ------------------------------------------------------------------------
  // IDENTIFIERS & KEYWORDS
  // ------------------------------------------------------------------------
  /// Identifier or keyword: `foo`, `let`, `fn`, `if`, `my_variable`
  ///
  /// The lexer doesn't distinguish between identifiers and keywords.
  /// That distinction is made during parsing by checking against a keyword list.
  ///
  /// Valid identifiers start with `a-z`, `A-Z`, `_`, or most Unicode XID_Start
  /// characters, followed by `a-z`, `A-Z`, `0-9`, `_`, or Unicode XID_Continue.
  Ident,

  /// Invalid identifier (contains characters not allowed in identifiers)
  ///
  /// This can occur when someone tries to use emoji or other invalid
  /// characters directly in an identifier position.
  InvalidIdent,

  /// Raw identifier: `r#let`, `r#type`, `r#async`
  ///
  /// Allows using keywords as identifiers. The `r#` prefix is included in the token.
  /// Used when you need a variable named after a keyword.
  RawIdent,

  // ------------------------------------------------------------------------
  // LIFETIMES
  // ------------------------------------------------------------------------
  /// Lifetime: `'a`, `'static`, `'_`
  ///
  /// # Fields
  /// - `starts_with_number`: true for invalid lifetimes like `'1lifetime`
  Lifetime { starts_with_number: bool },

  /// Raw lifetime: `'r#foo`
  ///
  /// Allows using keywords as lifetime names with the `r#` prefix
  RawLifetime,

  // ------------------------------------------------------------------------
  // LITERALS
  // ------------------------------------------------------------------------
  /// All literal values: integers, floats, strings, chars, etc.
  ///
  /// # Fields
  /// - `kind`: The specific type of literal (see `LiteralKind`)
  /// - `suffix_start`: Byte offset where the type suffix starts (e.g., `u32` in `42u32`)
  ///   If no suffix exists, this equals the literal's length.
  Literal {
    kind: LiteralKind,
    suffix_start: u32,
  },

  // ------------------------------------------------------------------------
  // SPECIAL PREFIXES & ERRORS
  // ------------------------------------------------------------------------
  /// Unknown literal prefix: `foo#`, `bar"`, `baz'`
  ///
  /// When a potential prefix is followed by a literal-starting character
  /// but isn't a recognized prefix (like `b`, `r`, `br`, `c`, `cr`).
  ///
  /// Only the prefix part (before `#`, `"`, or `'`) is in the token.
  UnknownPrefix,

  /// Unknown prefix in a lifetime context: `'foo#`
  ///
  /// Similar to UnknownPrefix but for lifetime-like tokens
  UnknownPrefixLifetime,

  /// Reserved prefix for Rust 2024+ (edition-specific feature)
  ///
  /// Prefixes like `k#`, `f#` are reserved for future literal types.
  /// The `#` is included in the token.
  ReservedPrefix,

  // ------------------------------------------------------------------------
  // SINGLE-CHARACTER PUNCTUATION
  // ------------------------------------------------------------------------
  /// `;` - Semicolon (statement terminator)
  Semi,

  /// `,` - Comma (separator)
  Comma,

  /// `.` - Dot (field access, method call, range start)
  Dot,

  /// `(` - Open parenthesis
  OpenParen,

  /// `)` - Close parenthesis
  CloseParen,

  /// `{` - Open brace (block start)
  OpenBrace,

  /// `}` - Close brace (block end)
  CloseBrace,

  /// `[` - Open bracket (array/slice)
  OpenBracket,

  /// `]` - Close bracket
  CloseBracket,

  /// `@` - At symbol (pattern binding)
  At,

  /// `#` - Pound/Hash (attributes, macros)
  Pound,

  /// `~` - Tilde (historical, rarely used in modern Rust)
  Tilde,

  /// `?` - Question mark (error propagation operator)
  Question,

  /// `:` - Colon (type annotation, struct patterns)
  Colon,

  /// `$` - Dollar sign (macro variables)
  Dollar,

  /// `=` - Equals (assignment, comparison when doubled)
  Eq,

  /// `!` - Bang/Exclamation (negation, macro invocation)
  Bang,

  /// `<` - Less than (comparison, generics)
  Lt,

  /// `>` - Greater than (comparison, generics)
  Gt,

  /// `-` - Minus (subtraction, negation)
  Minus,

  /// `&` - Ampersand (reference, bitwise AND)
  And,

  /// `|` - Pipe (bitwise OR, closure parameters)
  Or,

  /// `+` - Plus (addition, trait bounds)
  Plus,

  /// `*` - Star/Asterisk (multiplication, dereference, raw pointers)
  Star,

  /// `/` - Slash (division)
  Slash,

  /// `^` - Caret (bitwise XOR)
  Caret,

  /// `%` - Percent (modulo/remainder)
  Percent,

  // ------------------------------------------------------------------------
  // SPECIAL TOKENS
  // ------------------------------------------------------------------------
  /// Shebang line: `#!/usr/bin/env rustc`
  ///
  /// Only valid as the very first line of a file.
  /// Used for executable Rust scripts.
  Shebang,

  /// Unknown/invalid token
  ///
  /// Any character or sequence that doesn't match other token patterns.
  /// Examples: `№`, `¿`, or other unexpected Unicode characters.
  Unknown,

  /// End of input
  ///
  /// Sentinel value indicating the lexer has reached the end of the source.
  /// Useful for parsers to detect completion without special-casing.
  Eof,
}

// ============================================================================
// MULTI-CHARACTER OPERATORS (typically handled in parser, not lexer)
// ============================================================================
//
// Note: Many Rust lexers emit single-character tokens and let the parser
// combine them. Some lexers emit these as distinct tokens:
//
// - `::` - Path separator
// - `->` - Return type indicator
// - `=>` - Match arm separator
// - `..` - Range (exclusive end)
// - `..=` - Range (inclusive end)
// - `...` - (Deprecated range syntax)
// - `&&` - Logical AND
// - `||` - Logical OR
// - `<<` - Left shift
// - `>>` - Right shift
// - `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=` - Compound assignment
// - `==` - Equality
// - `!=` - Inequality
// - `<=` - Less than or equal
// - `>=` - Greater than or equal
//
// If your lexer needs these, add them to the TokenKind enum.

impl TokenKind {
  /// Returns true if this token is a trivia (whitespace or comment)
  ///
  /// Trivia tokens are typically ignored during parsing but preserved
  /// for formatting tools and IDEs.
  pub fn is_trivia(&self) -> bool {
    matches!(
      self,
      TokenKind::Whitespace | TokenKind::LineComment { .. } | TokenKind::BlockComment { .. }
    )
  }

  /// Returns true if this token can start an expression
  pub fn can_start_expr(&self) -> bool {
    matches!(
      self,
      TokenKind::Ident
            | TokenKind::RawIdent
            | TokenKind::Literal { .. }
            | TokenKind::OpenParen
            | TokenKind::OpenBracket
            | TokenKind::OpenBrace
            | TokenKind::Or  // closure
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Bang
            | TokenKind::And
    )
  }

  /// Returns true if this token represents a literal
  pub fn is_literal(&self) -> bool {
    matches!(self, TokenKind::Literal { .. })
  }

  /// Returns true if this token is an error or invalid token
  pub fn is_error(&self) -> bool {
    matches!(
      self,
      TokenKind::Unknown
        | TokenKind::InvalidIdent
        | TokenKind::UnknownPrefix
        | TokenKind::UnknownPrefixLifetime
    )
  }
}

impl LiteralKind {
  /// Returns true if this literal is a string-like type
  pub fn is_string_like(&self) -> bool {
    matches!(
      self,
      LiteralKind::Str { .. }
        | LiteralKind::ByteStr { .. }
        | LiteralKind::CStr { .. }
        | LiteralKind::RawStr { .. }
        | LiteralKind::RawByteStr { .. }
        | LiteralKind::RawCStr { .. }
    )
  }

  /// Returns true if this literal is a numeric type
  pub fn is_numeric(&self) -> bool {
    matches!(self, LiteralKind::Int { .. } | LiteralKind::Float { .. })
  }
}

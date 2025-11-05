# Complete Rust Lexer Implementation TODO

A comprehensive checklist for building a production-ready Rust lexer from scratch.

---

## Phase 1: Core Infrastructure 🏗️

### 1.1 Basic Lexer Structure
- [x] Create `Lexer` struct with:
  - [x] `input: &str` - Source code input
  - [x] `pos: usize` - Current byte position
  - [x] `chars: Peekable<CharIndices>` - Character iterator with lookahead
- [x] Implement `Lexer::new(input: &str) -> Self`
- [x] Implement `Lexer::next_token() -> Token` (main entry point)
- [x] Add helper methods:
  - [x] `peek_char() -> Option<char>` - Look at current char without consuming
  - [x] `peek_char_nth(n: usize) -> Option<char>` - Look ahead n characters
  - [x] `consume_char() -> Option<char>` - Advance and return current char
  - [x] `current_pos() -> usize` - Get current byte position
  - [x] `remaining_input() -> &str` - Get unprocessed input

### 1.2 Span/Location Tracking
- [x] Implement span calculation (start/end positions)
- [x] Add line and column tracking (optional, for better error messages)
- [x] Create `Span::new(start: usize, end: usize)` constructor
- [x] Implement `Span::merge()` for combining spans

---

## Phase 2: Whitespace & Comments 📝

### 2.1 Whitespace
- [x] Implement `lex_whitespace() -> Token`
- [x] Handle all Unicode whitespace characters:
  - [x] Space (` `)
  - [x] Tab (`\t`)
  - [x] Newline (`\n`)
  - [x] Carriage return (`\r`)
  - [x] Other Unicode whitespace (use `char::is_whitespace()`)
- [x] Test with mixed whitespace

### 2.2 Line Comments
- [x] Implement `lex_line_comment() -> Token`
- [x] Detect `//` at start
- [x] Check for doc comments:
  - [x] `///` → `DocStyle::Outer`
  - [x] `//!` → `DocStyle::Inner`
  - [x] `//` → `None` (regular comment)
- [x] Consume until end of line (not including `\n`)
- [x] Handle EOF in middle of line comment

### 2.3 Block Comments
- [x] Implement `lex_block_comment() -> Token`
- [x] Detect `/*` at start
- [x] Check for doc comments:
  - [x] `/**` (not `/**/`) → `DocStyle::Outer`
  - [x] `/*!` → `DocStyle::Inner`
  - [x] `/*` → `None` (regular comment)
- [x] **Handle nesting**: `/* /* nested */ */`
  - [x] Track nesting depth counter
  - [x] Increment on `/*`, decrement on `*/`
- [x] Set `terminated: false` if EOF before closing `*/`
- [x] Test deeply nested comments

### 2.4 Shebang
- [x] Implement `lex_shebang() -> Option<Token>`
- [ ] **Only valid as first token** (position 0)
- [ ] Must start with `#!`
- [ ] Must be followed by `[` or `/` (not `![` for inner attribute)
- [ ] Consume until end of line
- [ ] Return `None` if not at position 0 or invalid

---

## Phase 3: Identifiers & Keywords 🔤

### 3.1 Regular Identifiers
- [ ] Implement `lex_ident() -> Token`
- [ ] Validate first character:
  - [ ] `a-z`, `A-Z`, `_`
  - [ ] Unicode XID_Start characters
- [ ] Validate continuation characters:
  - [ ] `a-z`, `A-Z`, `0-9`, `_`
  - [ ] Unicode XID_Continue characters
- [ ] Use `char::is_xid_start()` and `char::is_xid_continue()`
- [ ] Mark as `InvalidIdent` if contains invalid characters (like emoji)

### 3.2 Raw Identifiers
- [ ] Implement `lex_raw_ident() -> Token`
- [ ] Detect `r#` prefix
- [ ] Parse identifier after `r#`
- [ ] Validate that it's a valid identifier
- [ ] Special cases:
  - [ ] `r#_` is invalid (underscore alone cannot be raw ident)
  - [ ] `r#crate` has special meaning

### 3.3 Keywords (Parser-level, but document here)
- [ ] Document that lexer emits `Ident` for keywords
- [ ] Parser should check against keyword list:
  - [ ] Strict keywords: `as`, `break`, `const`, `continue`, `crate`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`, `self`, `Self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`, `use`, `where`, `while`
  - [ ] Reserved keywords: `abstract`, `become`, `box`, `do`, `final`, `macro`, `override`, `priv`, `typeof`, `unsized`, `virtual`, `yield`
  - [ ] Weak keywords (contextual): `async`, `await`, `dyn`, `try`, `union`, `'static`

---

## Phase 4: Lifetimes 🕐

### 4.1 Regular Lifetimes
- [ ] Implement `lex_lifetime() -> Token`
- [ ] Detect `'` followed by identifier
- [ ] Parse identifier part (same rules as regular idents)
- [ ] Check if starts with number:
  - [ ] `'1abc` → set `starts_with_number: true` (invalid)
- [ ] Special lifetimes:
  - [ ] `'_` (anonymous lifetime)
  - [ ] `'static` (special, but treated as regular ident)

### 4.2 Raw Lifetimes
- [ ] Implement `lex_raw_lifetime() -> Token`
- [ ] Detect `'r#` prefix
- [ ] Parse identifier after `'r#`
- [ ] Allow keywords as lifetime names

### 4.3 Lifetime vs Char Literal Disambiguation
- [ ] If `'` followed by identifier start → lifetime
- [ ] If `'` followed by anything else → try char literal
- [ ] Handle edge cases like `'1` (invalid lifetime)

---

## Phase 5: Numeric Literals 🔢

### 5.1 Integer Literals
- [ ] Implement `lex_number() -> Token`
- [ ] Detect base prefix:
  - [ ] `0b` → Binary
  - [ ] `0o` → Octal
  - [ ] `0x` → Hexadecimal
  - [ ] None → Decimal
- [ ] Parse digits according to base:
  - [ ] Binary: `0-1`
  - [ ] Octal: `0-7`
  - [ ] Decimal: `0-9`
  - [ ] Hex: `0-9`, `a-f`, `A-F`
- [ ] Handle digit separators: `1_000_000`
- [ ] Set `empty_int: true` if no digits after prefix (e.g., `0x`)
- [ ] Parse optional suffix: `u8`, `i32`, `u64`, `i128`, `usize`, `isize`

### 5.2 Float Literals
- [ ] Detect float when number contains:
  - [ ] Decimal point: `3.14`
  - [ ] Exponent: `1e10`, `2E-5`
- [ ] Parse decimal point and fractional part
- [ ] Parse exponent:
  - [ ] `e` or `E` followed by optional `+`/`-`
  - [ ] Then digits
  - [ ] Set `empty_exponent: true` if no digits after `e`
- [ ] Parse optional suffix: `f32`, `f64`
- [ ] **Special case**: `1f32` is int with suffix, not float
- [ ] Hexadecimal floats (rare): `0x1.8p3`

### 5.3 Edge Cases
- [ ] `123` → Int (decimal)
- [ ] `0x` → Int with `empty_int: true`
- [ ] `1.` → Float (with empty fractional part)
- [ ] `1e` → Float with `empty_exponent: true`
- [ ] `1.foo()` → Int `1`, then `.`, then method call (not a float!)
- [ ] `1.2.3` → Float `1.2`, then `.`, then `3`

---

## Phase 6: Character & Byte Literals 📝

### 6.1 Character Literals
- [ ] Implement `lex_char() -> Token`
- [ ] Detect opening `'`
- [ ] Parse content:
  - [ ] Regular character: `'a'`
  - [ ] Escape sequences (see 6.3)
  - [ ] Unicode escapes: `'\u{1F980}'`
- [ ] Detect closing `'`
- [ ] Set `terminated: false` if EOF or newline before closing
- [ ] Validate:
  - [ ] Not empty: `''` is invalid
  - [ ] Single character (after escaping)

### 6.2 Byte Literals
- [ ] Implement `lex_byte() -> Token`
- [ ] Detect `b'` prefix
- [ ] Parse ASCII-only content
- [ ] Same escape rules as char, but restricted to ASCII
- [ ] Validate byte is in range 0-127 (after escaping)

### 6.3 Escape Sequences
- [ ] Implement `parse_escape_sequence() -> Result<char, Error>`
- [ ] Simple escapes:
  - [ ] `\n` → newline
  - [ ] `\r` → carriage return
  - [ ] `\t` → tab
  - [ ] `\\` → backslash
  - [ ] `\'` → single quote
  - [ ] `\"` → double quote
  - [ ] `\0` → null
- [ ] Byte escapes: `\xHH` (2 hex digits)
- [ ] Unicode escapes: `\u{HHHHHH}` (1-6 hex digits)
- [ ] Validate Unicode scalar values (not surrogate pairs)

---

## Phase 7: String Literals 📜

### 7.1 Regular Strings
- [ ] Implement `lex_string() -> Token`
- [ ] Detect opening `"`
- [ ] Parse content with escape sequences
- [ ] Allow multi-line strings
- [ ] Detect closing `"`
- [ ] Set `terminated: false` if EOF before closing

### 7.2 Raw Strings
- [ ] Implement `lex_raw_string() -> Token`
- [ ] Detect `r` followed by zero or more `#`, then `"`
- [ ] Count opening `#` characters → `n_hashes`
- [ ] Validate no other characters between `r` and `#*"`
  - [ ] Set `err: InvalidStarter` if found
- [ ] Parse content (no escapes processed)
- [ ] Find closing: `"` followed by same number of `#`
- [ ] Set `err: NoTerminator` if:
  - [ ] EOF reached
  - [ ] Different number of closing `#`
- [ ] Set `err: TooManyDelimiters` if > 65535 `#`

### 7.3 Byte Strings
- [ ] Implement `lex_byte_string() -> Token`
- [ ] Detect `b"` prefix
- [ ] Same as regular strings but ASCII-only
- [ ] Validate all bytes are 0-127 (after escaping)

### 7.4 Raw Byte Strings
- [ ] Implement `lex_raw_byte_string() -> Token`
- [ ] Detect `br` prefix
- [ ] Combine raw string and byte string rules
- [ ] No escapes, ASCII-only content

### 7.5 C Strings (Rust 1.77+)
- [ ] Implement `lex_c_string() -> Token`
- [ ] Detect `c"` prefix
- [ ] Same as regular strings
- [ ] Automatically null-terminated
- [ ] Cannot contain interior `\0` (except as last char)

### 7.6 Raw C Strings
- [ ] Implement `lex_raw_c_string() -> Token`
- [ ] Detect `cr` prefix
- [ ] Combine raw string and C string rules

---

## Phase 8: Operators & Punctuation ⚙️

### 8.1 Single-Character Tokens
Implement individual lexing functions for each:
- [ ] `;` → `Semi`
- [ ] `,` → `Comma`
- [ ] `.` → `Dot` (check not start of `..` or number)
- [ ] `(` → `OpenParen`
- [ ] `)` → `CloseParen`
- [ ] `{` → `OpenBrace`
- [ ] `}` → `CloseBrace`
- [ ] `[` → `OpenBracket`
- [ ] `]` → `CloseBracket`
- [ ] `@` → `At`
- [ ] `#` → `Pound`
- [ ] `~` → `Tilde`
- [ ] `?` → `Question`
- [ ] `:` → `Colon`
- [ ] `$` → `Dollar`
- [ ] `=` → `Eq`
- [ ] `!` → `Bang`
- [ ] `<` → `Lt`
- [ ] `>` → `Gt`
- [ ] `-` → `Minus`
- [ ] `&` → `And`
- [ ] `|` → `Or`
- [ ] `+` → `Plus`
- [ ] `*` → `Star`
- [ ] `/` → `Slash`
- [ ] `^` → `Caret`
- [ ] `%` → `Percent`

### 8.2 Disambiguation
- [ ] `/` → Check if followed by `/` (line comment) or `*` (block comment)
- [ ] `.` → Check if followed by digit (float literal)
- [ ] `'` → Check if lifetime or char literal
- [ ] `#` → Check if `#!` at position 0 (shebang)
- [ ] Letters → Check if prefix for literal (`b`, `r`, `br`, `c`, `cr`)

---

## Phase 9: Unknown Prefixes & Reserved Syntax 🚫

### 9.1 Unknown Literal Prefixes
- [ ] Implement `detect_unknown_prefix() -> Option<Token>`
- [ ] If identifier followed immediately by `"`, `'`, or `#"`:
  - [ ] Check if it's a known prefix (`b`, `r`, `br`, `c`, `cr`)
  - [ ] If not known → `UnknownPrefix`
  - [ ] Token contains only the prefix part

### 9.2 Unknown Lifetime Prefixes
- [ ] Implement `detect_unknown_lifetime_prefix() -> Option<Token>`
- [ ] If `'` + identifier + `#`:
  - [ ] Not `'r#` → `UnknownPrefixLifetime`

### 9.3 Reserved Prefixes (Rust 2024+)
- [ ] Implement `detect_reserved_prefix() -> Option<Token>`
- [ ] Check for single-letter + `#` patterns: `k#`, `f#`
- [ ] Reserved for future literal types
- [ ] Return `ReservedPrefix` token

---

## Phase 10: Main Lexer Loop 🔄

### 10.1 Token Dispatch
- [ ] Implement main `next_token()` logic:
  ```rust
  match self.peek_char() {
      None => Eof token,
      Some(c) => dispatch based on c
  }
  ```

### 10.2 Character-based Dispatch
- [ ] Whitespace chars → `lex_whitespace()`
- [ ] `/` → Check for comments or `Slash`
- [ ] `#` → Check for shebang or `Pound`
- [ ] `'` → Check for lifetime or char literal
- [ ] `"` → `lex_string()`
- [ ] `a-z`, `A-Z`, `_` → `lex_ident()` or check prefix
- [ ] `0-9` → `lex_number()`
- [ ] Operators → Single-char tokens
- [ ] Everything else → `Unknown`

### 10.3 Prefix Detection
- [ ] After lexing potential identifier, check next char:
  - [ ] `"` → Could be string prefix
  - [ ] `'` → Could be byte literal prefix
  - [ ] `#` → Could be raw string prefix
- [ ] Backtrack if needed or use lookahead

---

## Phase 11: Error Handling & Validation ⚠️

### 11.1 Malformed Literals
- [ ] Unterminated strings/chars: set `terminated: false`
- [ ] Invalid escape sequences: document as lexer concern or parser?
- [ ] Empty numbers after prefix: set `empty_int: true`
- [ ] Invalid raw string delimiters: set `RawStrError`

### 11.2 Invalid Characters
- [ ] Return `Unknown` token for unrecognized characters
- [ ] Unicode characters not valid in identifiers → `InvalidIdent`
- [ ] Non-ASCII in byte literals → validation error

### 11.3 Contextual Validation
- [ ] Shebang only at position 0
- [ ] Raw identifiers cannot be `_` alone
- [ ] Lifetimes cannot start with numbers (but lex anyway, mark invalid)

---

## Phase 12: Testing 🧪

### 12.1 Unit Tests Per Feature
- [ ] Whitespace: various Unicode whitespace
- [ ] Comments: nested, unterminated, doc comments
- [ ] Identifiers: ASCII, Unicode, keywords, raw
- [ ] Lifetimes: regular, raw, invalid
- [ ] Numbers: all bases, floats, empty, suffixes
- [ ] Chars/bytes: escapes, unicode, unterminated
- [ ] Strings: all variants, raw with different `#` counts
- [ ] Operators: all single-char tokens
- [ ] Unknown/invalid tokens

### 12.2 Integration Tests
- [ ] Lex complete Rust files from standard library
- [ ] Lex code with intentional errors
- [ ] Lex edge cases: empty file, only whitespace, only comments
- [ ] Performance test: large files (1MB+)

### 12.3 Fuzzing
- [ ] Set up cargo-fuzz
- [ ] Fuzz with random bytes
- [ ] Fuzz with semi-valid Rust code
- [ ] Check for panics and infinite loops

---

## Phase 13: Optimizations 🚀

### 13.1 Performance
- [ ] Profile hot paths with `cargo flamegraph`
- [ ] Optimize character lookahead (avoid repeated UTF-8 decoding)
- [ ] Use byte-based matching where possible (ASCII fast path)
- [ ] Consider SIMD for whitespace/identifier scanning

### 13.2 Memory
- [ ] Minimize allocations in hot path
- [ ] Reuse buffers where possible
- [ ] Consider arena allocation for tokens

### 13.3 Benchmarking
- [ ] Set up criterion benchmarks
- [ ] Benchmark against rustc_lexer
- [ ] Test on various file sizes and code styles

---

## Phase 14: API & Documentation 📚

### 14.1 Public API
- [ ] Clean, ergonomic API for consumers:
  ```rust
  let tokens: Vec<Token> = Lexer::new(source).collect();
  ```
- [ ] Iterator implementation
- [ ] Error recovery mode (continue after errors)

### 14.2 Documentation
- [ ] Document all public types and methods
- [ ] Add examples to struct/function docs
- [ ] Create comprehensive README
- [ ] Document differences from rustc_lexer (if any)

### 14.3 Examples
- [ ] Simple token printer example
- [ ] Syntax highlighter example
- [ ] Token statistics analyzer

---

## Phase 15: Advanced Features (Optional) ✨

### 15.1 Error Recovery
- [ ] Continue lexing after errors
- [ ] Produce sensible tokens for malformed input
- [ ] Provide helpful error messages

### 15.2 Source Maps
- [ ] Track original source locations
- [ ] Support for `#[macro_export]` and `include!()` expansions
- [ ] Map tokens back to original files

### 15.3 Proc Macro Support
- [ ] TokenStream compatible output
- [ ] Preserve token spacing information
- [ ] Support token tree construction

### 15.4 IDE Support
- [ ] Provide token semantic information
- [ ] Support incremental lexing
- [ ] Token classification for syntax highlighting

---

## Phase 16: Validation & Release 🎉

### 16.1 Compliance Testing
- [ ] Test against Rust specification
- [ ] Compare output with rustc_lexer on Rust repo
- [ ] Test edition-specific features (2015, 2018, 2021, 2024)

### 16.2 Code Quality
- [ ] Run clippy with strict lints
- [ ] Ensure no unsafe code (or justify it)
- [ ] 100% documentation coverage
- [ ] Format with rustfmt

### 16.3 CI/CD
- [ ] Set up GitHub Actions
- [ ] Test on multiple Rust versions
- [ ] Test on different platforms (Linux, macOS, Windows)
- [ ] Automated releases with cargo-release

### 16.4 Publication
- [ ] Choose appropriate license (MIT/Apache-2.0)
- [ ] Publish to crates.io
- [ ] Announce on Rust forums/Reddit
- [ ] Add to Awesome Rust list

---

## Estimated Time ⏱️

- **Phase 1-2**: 1-2 days (infrastructure + whitespace/comments)
- **Phase 3-4**: 2-3 days (identifiers + lifetimes)
- **Phase 5**: 3-4 days (numeric literals, tricky!)
- **Phase 6-7**: 4-5 days (char/byte/string literals, many variants)
- **Phase 8-9**: 1-2 days (operators + error cases)
- **Phase 10**: 1 day (main loop integration)
- **Phase 11**: 2-3 days (error handling polish)
- **Phase 12**: 3-5 days (comprehensive testing)
- **Phase 13**: 2-3 days (optimizations)
- **Phase 14**: 2-3 days (API + docs)
- **Phase 15**: Variable (optional features)
- **Phase 16**: 1-2 days (final validation)

**Total**: ~25-35 days for a solid, production-ready lexer

---

## Key Resources 📖

- [Rust Reference - Lexical Structure](https://doc.rust-lang.org/reference/lexical-structure.html)
- [rustc_lexer source code](https://github.com/rust-lang/rust/tree/master/compiler/rustc_lexer)
- [Unicode XID specification](http://www.unicode.org/reports/tr31/)
- [Rust Edition Guide](https://doc.rust-lang.org/edition-guide/)

---

Good luck building your Rust lexer! 🦀

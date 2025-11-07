# Missing Token Types Implementation Guide

This document lists all token types that are **defined** in `TokenKind` but **not yet implemented** in the lexer.

## Summary

| Category | Missing Tokens | Priority | Complexity |
|----------|---------------|----------|------------|
| **Lifetimes** | `Lifetime`, `RawLifetime` | High | Medium |
| **Prefix Errors** | `UnknownPrefix`, `UnknownPrefixLifetime`, `ReservedPrefix` | Medium | Low |
| **Error Recovery** | `Unknown` | Low | Low |

---

## 1. Lifetime Tokens

### Missing: `Lifetime` and `RawLifetime`

**Current Status**: ❌ Not implemented

**What needs to be done**:
- When encountering a single quote `'`, check if it's a lifetime or a character literal
- Lifetimes start with `'` followed by an identifier (e.g., `'a`, `'static`, `'_`)
- Character literals start with `'` followed by content and end with `'`

**Implementation Location**: 
- Add `lex_lifetime()` function in a new module `lexers/lifetime.rs` or in `scanner_utils.rs`
- Update `lex_tokens()` dispatch to handle `'` character

**Examples**:
```rust
'a          // Lifetime { starts_with_number: false }
'static     // Lifetime { starts_with_number: false }
'_          // Lifetime { starts_with_number: false }
'0invalid   // Lifetime { starts_with_number: true } (error)
r#'lifetime // RawLifetime (hypothetical)
```

**Current Behavior**: 
- Single quotes `'` are routed to `lex_string()` which handles character literals
- No distinction between lifetimes and character literals

**Implementation Steps**:
1. In `scanner_utils.rs`, when `'\''` is encountered, peek ahead
2. If next char is alphanumeric/underscore → lifetime
3. If next char is `r` → check for raw lifetime
4. Otherwise → character literal (current behavior)

---

## 2. Prefix Error Tokens

### Missing: `UnknownPrefix`, `UnknownPrefixLifetime`, `ReservedPrefix`

**Current Status**: ❌ Not implemented

**What needs to be done**:
- Detect invalid prefixes on literals (e.g., `q"invalid"`, `x'invalid`)
- Detect reserved prefixes that aren't used yet
- Emit appropriate error tokens with diagnostics

**Implementation Location**: 
- Update `lex_string()` in `lexers/literal.rs` to handle unknown prefixes
- Add lifetime prefix checking in lifetime lexer

**Examples**:
```rust
q"invalid"  // UnknownPrefix - 'q' is not a valid string prefix
x'invalid   // UnknownPrefixLifetime - only 'r' is valid before lifetimes
z"text"     // ReservedPrefix - reserved for future use
```

**Current Behavior**: 
- Unknown prefixes like `q"..."` would fall through to the default case and emit `InvalidCharacter` diagnostic
- No specific handling for prefix errors

**Implementation Steps**:
1. In `lex_string()`, check if prefix is valid (`b`, `c`, `r`, or none)
2. If invalid but looks like a prefix → `UnknownPrefix`
3. If reserved prefix → `ReservedPrefix`
4. For lifetimes, check if prefix before `'` is valid → `UnknownPrefixLifetime`

---

## 3. Error Recovery Token

### Missing: `Unknown`

**Current Status**: ❌ Not implemented

**What needs to be done**:
- Emit `Unknown` token for truly unparseable input
- Used for error recovery in the parser

**Implementation Location**: 
- Update `lex_tokens()` in `scanner_utils.rs` default case

**Examples**:
```rust
// When encountering completely invalid input that can't be classified
// Currently emits diagnostic but returns None
```

**Current Behavior**: 
- Invalid characters emit a diagnostic and return `None`
- No token is emitted, which might cause parser issues

**Implementation Steps**:
1. In `lex_tokens()` default case, emit `TokenKind::Unknown` instead of returning `None`
2. Still emit diagnostic for the error
3. This allows parser to continue with error recovery

## Quick Reference: All Token Types


### ❌ Not Implemented
- `Lifetime { starts_with_number: bool }`
- `RawLifetime`
- `UnknownPrefix`
- `UnknownPrefixLifetime`
- `ReservedPrefix`
- `Unknown`

---

## Testing Checklist

When implementing each missing token:

- [ ] Add lexer function that returns the token
- [ ] Add dispatch case in `lex_tokens()`
- [ ] Emit appropriate diagnostics for error cases
- [ ] Add test cases in `lexer/tests/`
- [ ] Verify token appears in token stream
- [ ] Check that spans are correct
- [ ] Ensure error recovery works properly


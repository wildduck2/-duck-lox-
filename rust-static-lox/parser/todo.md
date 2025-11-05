# Rust Parser Implementation TODO



### 2.1 Primary Expressions
- [ ] Parse literals (integers, floats, strings, chars, bools)
- [ ] Parse identifiers and paths (`foo`, `std::vec::Vec`)
- [ ] Parse grouped expressions `(expr)`
- [ ] Parse tuple expressions `(a, b, c)`
- [ ] Parse array expressions `[1, 2, 3]` and `[0; 10]`
- [ ] Parse underscore expression `_`

### 2.2 Pratt Parser for Binary Operations
- [ ] Implement Pratt parsing algorithm
- [ ] Define operator precedence table
  ```rust
  fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
    match op {
      Token::Or => Some((1, 2)),           // ||
      Token::And => Some((3, 4)),          // &&
      Token::EqEq | Token::NotEq => Some((5, 6)),  // ==, !=
      Token::Less | Token::LessEq => Some((7, 8)), // <, <=
      // ... etc
    }
  }
  ```

- [ ] Parse binary operations (arithmetic, comparison, logical, bitwise)
- [ ] Parse unary operations (`-`, `!`, `*`, `&`, `&&`)
- [ ] Parse cast expressions (`as Type`)
- [ ] Parse type ascription (`expr: Type`)

### 2.3 Postfix Operations
- [ ] Parse field access (`obj.field`, `obj.0`)
- [ ] Parse method calls (`obj.method()`, `obj.method::<T>()`)
- [ ] Parse function calls (`func(args)`)
- [ ] Parse indexing (`arr[i]`)
- [ ] Parse try operator (`expr?`)
- [ ] Parse await (`expr.await`)

### 2.4 Range Expressions
- [ ] Parse range operators (`..`, `..=`)
- [ ] Parse full range (`..`)
- [ ] Parse ranges with bounds (`a..b`, `a..=b`, `..b`, `a..`)

### 2.5 Closures
- [ ] Parse closure syntax
  - [ ] `|x| expr`
  - [ ] `|x: i32| -> i32 { expr }`
  - [ ] `move |x| expr`
  - [ ] `async |x| expr`

### 2.6 Block Expressions
- [ ] Parse basic blocks `{ stmts; expr }`
- [ ] Parse unsafe blocks `unsafe { ... }`
- [ ] Parse const blocks `const { ... }`
- [ ] Parse async blocks `async { ... }`
- [ ] Parse try blocks `try { ... }`
- [ ] Parse labeled blocks `'label: { ... }`

## Phase 3: Control Flow (Week 5)

### 3.1 Conditional Expressions
- [ ] Parse if expressions
  - [ ] `if cond { ... }`
  - [ ] `if cond { ... } else { ... }`
  - [ ] `if cond { ... } else if { ... }`
- [ ] Parse let expressions in conditions `if let pat = expr`

### 3.2 Match Expressions
- [ ] Parse match expressions `match expr { arms }`
- [ ] Parse match arms with patterns
- [ ] Parse match guards `pat if expr =>`
- [ ] Handle trailing commas

### 3.3 Loop Expressions
- [ ] Parse `loop { ... }`
- [ ] Parse `while cond { ... }`
- [ ] Parse `for pat in expr { ... }`
- [ ] Parse labeled loops `'label: loop { ... }`
- [ ] Parse break with labels and values `break 'label expr`
- [ ] Parse continue with labels `continue 'label`

### 3.4 Other Control Flow
- [ ] Parse return expressions `return expr`
- [ ] Parse yield expressions `yield expr`
- [ ] Parse become expressions `become expr`

## Phase 4: Pattern Parsing (Week 6)

### 4.1 Basic Patterns
- [ ] Parse wildcard pattern `_`
- [ ] Parse rest pattern `..`
- [ ] Parse literal patterns
- [ ] Parse identifier patterns `x`, `mut x`, `ref x`, `ref mut x`
- [ ] Parse path patterns `Some`, `Option::None`

### 4.2 Compound Patterns
- [ ] Parse tuple patterns `(a, b, c)`
- [ ] Parse slice patterns `[a, b, .., c]`
- [ ] Parse struct patterns `Point { x, y }`
- [ ] Parse tuple struct patterns `Some(x)`
- [ ] Parse reference patterns `&pat`, `&mut pat`
- [ ] Parse box patterns `box pat`

### 4.3 Advanced Patterns
- [ ] Parse or patterns `A | B | C`
- [ ] Parse range patterns `1..=10`, `'a'..='z'`
- [ ] Parse @ bindings `x @ Some(y)`
- [ ] Parse grouped patterns `(pat)`

## Phase 5: Type Parsing (Week 7-8)

### 5.1 Primitive Types
- [ ] Parse integer types (`i8`, `i16`, ..., `usize`)
- [ ] Parse float types (`f32`, `f64`)
- [ ] Parse `bool`, `char`, `str`
- [ ] Parse never type `!`

### 5.2 Compound Types
- [ ] Parse array types `[T; N]`
- [ ] Parse slice types `[T]`
- [ ] Parse tuple types `(T1, T2, T3)`
- [ ] Parse reference types `&T`, `&'a T`, `&mut T`
- [ ] Parse raw pointer types `*const T`, `*mut T`

### 5.3 Named Types
- [ ] Parse path types `Vec<T>`, `std::vec::Vec<T>`
- [ ] Parse qualified paths `<T as Trait>::Type`
- [ ] Parse trait object types `dyn Trait`, `dyn Trait + 'a`
- [ ] Parse impl trait types `impl Trait`

### 5.4 Function Types
- [ ] Parse bare function types
  - [ ] `fn(i32) -> i32`
  - [ ] `unsafe fn()`
  - [ ] `extern "C" fn()`
  - [ ] `for<'a> fn(&'a str)`

### 5.5 Special Types
- [ ] Parse infer type `_`
- [ ] Parse parenthesized types `(Type)`
- [ ] Parse macro invocations in type position
- [ ] Parse typeof types `typeof(expr)` (unstable)

## Phase 6: Items & Declarations (Week 9-11)

### 6.1 Function Items
- [ ] Parse function signatures
  - [ ] Visibility (`pub`, `pub(crate)`, etc.)
  - [ ] Qualifiers (`const`, `async`, `unsafe`, `extern`)
  - [ ] Generic parameters `<T, U>`
  - [ ] Parameters with patterns
  - [ ] Return type
  - [ ] Where clauses
- [ ] Parse function bodies
- [ ] Parse variadic functions `...`

### 6.2 Struct Items
- [ ] Parse named structs `struct S { fields }`
- [ ] Parse tuple structs `struct S(T1, T2);`
- [ ] Parse unit structs `struct S;`
- [ ] Parse struct fields with attributes and visibility
- [ ] Parse generic structs with where clauses

### 6.3 Enum Items
- [ ] Parse enum declarations
- [ ] Parse unit variants
- [ ] Parse tuple variants
- [ ] Parse struct variants
- [ ] Parse discriminants `= expr`

### 6.4 Trait Items
- [ ] Parse trait declarations
  - [ ] `trait Trait`
  - [ ] `unsafe trait`
  - [ ] `auto trait`
  - [ ] Supertraits `: Trait1 + Trait2`
- [ ] Parse trait methods (with/without bodies)
- [ ] Parse associated types (GATs)
- [ ] Parse associated constants

### 6.5 Impl Blocks
- [ ] Parse inherent impls `impl Type { ... }`
- [ ] Parse trait impls `impl Trait for Type { ... }`
- [ ] Parse negative impls `impl !Trait for Type`
- [ ] Parse default impls `default impl ...`
- [ ] Parse unsafe impls `unsafe impl ...`

### 6.6 Other Items
- [ ] Parse const items `const NAME: Type = value;`
- [ ] Parse static items `static NAME: Type = value;`
- [ ] Parse type aliases `type Name<T> = Type;`
- [ ] Parse modules `mod name { ... }` and `mod name;`
- [ ] Parse use declarations with all variants
  - [ ] `use path::*;`
  - [ ] `use path::{A, B, C};`
  - [ ] `use path as alias;`
- [ ] Parse extern crate `extern crate name;`
- [ ] Parse unions `union U { fields }`
- [ ] Parse extern types `extern type T;`

## Phase 7: Generics System (Week 12-13)

### 7.1 Generic Parameters
- [ ] Parse type parameters `<T>`
- [ ] Parse lifetime parameters `<'a>`
- [ ] Parse const parameters `<const N: usize>`
- [ ] Parse parameter bounds `<T: Trait>`
- [ ] Parse default values `<T = DefaultType>`

### 7.2 Generic Arguments
- [ ] Parse angle-bracketed args `<T, U, V>`
- [ ] Parse parenthesized args `Fn(i32) -> i32`
- [ ] Parse lifetime arguments `<'a>`
- [ ] Parse type arguments
- [ ] Parse const arguments
- [ ] Parse associated type bindings `<Item = Type>`
- [ ] Parse associated type constraints `<Item: Trait>`

### 7.3 Where Clauses
- [ ] Parse type predicates `where T: Trait`
- [ ] Parse lifetime predicates `where 'a: 'b`
- [ ] Parse equality predicates `where T = Type`
- [ ] Parse higher-ranked trait bounds `where for<'a> F: Fn(&'a T)`

### 7.4 Trait Bounds
- [ ] Parse simple bounds `T: Trait`
- [ ] Parse multiple bounds `T: Trait1 + Trait2`
- [ ] Parse lifetime bounds `T: 'a`
- [ ] Parse maybe bounds `T: ?Sized`
- [ ] Parse const bounds `T: ?const Trait`
- [ ] Parse higher-ranked bounds `for<'a> T: Trait<'a>`

## Phase 8: Attributes & Macros (Week 14-15)

### 8.1 Attributes
- [ ] Parse outer attributes `#[attr]`
- [ ] Parse inner attributes `#![attr]`
- [ ] Parse attribute content
  - [ ] Simple paths `#[derive(Debug)]`
  - [ ] Name-value pairs `#[cfg(feature = "std")]`
  - [ ] Lists `#[allow(dead_code, unused)]`
- [ ] Parse doc comments as attributes
- [ ] Parse cfg attributes with predicates
- [ ] Parse cfg_attr attributes

### 8.2 Macro Rules
- [ ] Parse `macro_rules!` declarations
- [ ] Parse macro matchers with token trees
- [ ] Parse macro transcribers
- [ ] Parse repetition operators `*`, `+`, `?`
- [ ] Parse metavariables `$name:ty`
- [ ] Parse fragment specifiers

### 8.3 Macro 2.0
- [ ] Parse `macro` declarations
- [ ] Parse macro parameters
- [ ] Parse macro bodies

### 8.4 Macro Invocations
- [ ] Parse macro calls `macro!()`
- [ ] Parse macro calls with different delimiters
  - [ ] `macro!(args)`
  - [ ] `macro![args]`
  - [ ] `macro!{args}`
- [ ] Parse macros in all positions (expr, type, item, pattern)

## Phase 9: Advanced Features (Week 16-17)

### 9.1 Foreign Items
- [ ] Parse `extern` blocks
- [ ] Parse foreign functions
- [ ] Parse foreign statics
- [ ] Parse foreign types
- [ ] Parse ABI strings

### 9.2 Inline Assembly
- [ ] Parse `asm!` macro syntax
- [ ] Parse assembly templates
- [ ] Parse assembly operands
- [ ] Parse assembly options

### 9.3 Visibility Modifiers
- [ ] Parse `pub`
- [ ] Parse `pub(crate)`
- [ ] Parse `pub(super)`
- [ ] Parse `pub(self)`
- [ ] Parse `pub(in path)`

### 9.4 Special Expressions
- [ ] Parse struct literals with all features
- [ ] Parse format string macros (if expanding)
- [ ] Parse inline const expressions
- [ ] Parse box expressions
- [ ] Parse labeled block expressions

## Phase 10: Testing & Refinement (Week 18-20)

### 10.1 Unit Tests
- [ ] Write tests for each expression type
- [ ] Write tests for each pattern type
- [ ] Write tests for each type
- [ ] Write tests for each item type
- [ ] Write tests for operator precedence
- [ ] Write tests for edge cases

### 10.2 Integration Tests
- [ ] Parse real Rust files from std library
- [ ] Parse popular crates (tokio, serde, etc.)
- [ ] Compare with rustc's parser output
- [ ] Test error recovery

### 10.3 Error Handling
- [ ] Improve error messages
- [ ] Add error recovery (continue parsing after errors)
- [ ] Add suggestions for common mistakes
- [ ] Add help messages for confusing syntax

### 10.4 Performance
- [ ] Profile parser performance
- [ ] Optimize hot paths
- [ ] Reduce allocations
- [ ] Add benchmarks

### 10.5 Documentation
- [ ] Document parser API
- [ ] Write usage examples
- [ ] Document grammar extensions
- [ ] Write contributor guide

## Phase 11: Advanced Error Recovery (Week 21)

### 11.1 Synchronization Points
- [ ] Recover at semicolons
- [ ] Recover at closing braces
- [ ] Recover at item boundaries
- [ ] Skip to next valid token

### 11.2 Partial Parsing
- [ ] Continue parsing after missing tokens
- [ ] Insert synthetic tokens when needed
- [ ] Build partial AST nodes
- [ ] Mark error nodes in AST

## Phase 12: Optional Features (Future)

### 12.1 IDE Support
- [ ] Incremental parsing
- [ ] Preserve whitespace and comments in AST
- [ ] Provide source mapping
- [ ] Support incomplete code

### 12.2 Macro Expansion
- [ ] Implement declarative macro expansion
- [ ] Handle recursive macros
- [ ] Hygiene system
- [ ] Macro debugging

### 12.3 Name Resolution
- [ ] Build symbol tables
- [ ] Resolve paths
- [ ] Handle imports
- [ ] Scope analysis

---

## Getting Started Checklist

**Start here for immediate progress:**

1. [ ] Set up project with cargo
2. [ ] Copy AST definitions to `src/ast.rs`
3. [ ] Create `src/lexer.rs` and define Token enum
4. [ ] Implement basic tokenizer for identifiers and keywords
5. [ ] Write first test: tokenize `fn main() {}`
6. [ ] Create `src/parser.rs` with Parser struct
7. [ ] Implement `parse_expr()` for literals
8. [ ] Write test: parse `42`
9. [ ] Implement Pratt parser for binary operations
10. [ ] Write test: parse `1 + 2 * 3`

**Recommended order of implementation:**
1. Lexer (tokens, literals, keywords)
2. Simple expressions (literals, paths, binary ops)
3. Statements (let, expression statements)
4. Blocks and control flow
5. Patterns (for let statements and match)
6. Types (for let statements and functions)
7. Function items (simplest item to parse)
8. Other items (structs, enums, traits, impls)
9. Generics and where clauses
10. Attributes and macros

Good luck! 🦀

# Complete Language Implementation Plan V2

## Project Vision

**Your Second Language After Lox - Tree-Walking Interpreter Edition**

This is your production-grade, statically-typed language built on what you learned from Lox. Since this will be a **tree-walking interpreter** (not bytecode), you'll get:
- Faster implementation time
- Easier debugging
- Clearer architecture
- Better learning experience for type systems

The focus shifts from bytecode optimization to elegant AST traversal and sophisticated type checking.

---

## Key Decision: Static vs Dynamic Typing?

### Why Static Typing From Scratch

After building Lox (dynamically typed), choose **static typing** for your second language:

**Better for learning advanced concepts:**
- Type inference algorithms (Hindley-Milner)
- Generic programming
- Trait systems
- Compile-time guarantees

**Real-world applicability:**
- Most production languages are statically typed (Rust, TypeScript, Go, Swift)
- Better tooling support (IDEs, LSP)
- Catch bugs at compile time
- Self-documenting code

**You already understand dynamic typing:**
- Lox taught you the interpreter basics
- Time to level up your skills
- More challenging and rewarding

### Language Feature Comparison

| Feature | Lox (Your First) | This Language (Your Second) |
|---------|------------------|----------------------------|
| Type System | Dynamic | **Static with inference** |
| Execution | Tree-walk interpreter | **Tree-walk interpreter** |
| Functions | First-class | First-class + Generics |
| OOP | Classes | **Structs + Traits** |
| Pattern Matching | No | **Yes** |
| Modules | No | **Yes** |
| Standard Library | Minimal | **Rich** |
| Error Messages | Basic | **Detailed with suggestions** |

---

## Phase 1: Scanner/Lexer

**Goal:** Convert source code into tokens

**Difficulty:** Easy (you've done this in Lox!)

**Time Estimate:** 1-2 days

### Core Structure

```rust
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

pub struct Token {
    kind: TokenKind,
    lexeme: String,
    span: Span,
    line: usize,
}

pub struct Span {
    start: usize,
    end: usize,
}
```

### Token Types to Support

**Keywords - Type System (NEW compared to Lox!):**
- `type`, `struct`, `enum`, `trait`, `impl`, `interface`
- `int`, `float`, `string`, `bool`, `void`

**Keywords - Declarations:**
- `let`, `const`, `fn`, `return`

**Keywords - Control Flow:**
- `if`, `else`, `while`, `for`, `loop`, `break`, `continue`, `match`

**Keywords - Values:**
- `true`, `false`, `nil`, `self`, `super`

**Keywords - Modules (NEW!):**
- `import`, `export`, `mod`, `pub`, `use`

**Operators - Arithmetic:**
- `+`, `-`, `*`, `/`, `%`, `**` (power)

**Operators - Comparison:**
- `==`, `!=`, `<`, `<=`, `>`, `>=`

**Operators - Logical:**
- `and`, `or`, `not`, `&&`, `||`, `!`

**Operators - Type Annotations (NEW!):**
- `:` (type annotation)
- `::` (path separator)
- `->` (function return type)
- `=>` (match arms)
- `|` (union types or patterns)

**Delimiters:**
- `(`, `)`, `{`, `}`, `[`, `]`
- `,`, `.`, `;`, `?`

**Literals:**
- Identifiers
- Integer literals (with underscores: `1_000_000`)
- Float literals (`3.14`, `1e10`)
- String literals (with escape sequences)

### Example Input/Output

```
Input: let x: int = 42;

Output:
Token { kind: Let, lexeme: "let", span: Span(0, 3), line: 1 }
Token { kind: Identifier, lexeme: "x", span: Span(4, 5), line: 1 }
Token { kind: Colon, lexeme: ":", span: Span(5, 6), line: 1 }
Token { kind: Int, lexeme: "int", span: Span(7, 10), line: 1 }
Token { kind: Equal, lexeme: "=", span: Span(11, 12), line: 1 }
Token { kind: IntegerLiteral, lexeme: "42", span: Span(13, 15), line: 1 }
Token { kind: Semicolon, lexeme: ";", span: Span(15, 16), line: 1 }
```

### Implementation Tips

- Reuse your Lox scanner logic - it's mostly the same
- Add support for `::` and `->` (multi-character operators)
- Handle string escapes (`\n`, `\t`, `\"`)
- Support number underscores for readability: `1_000_000`
- Track column numbers for better error messages

### What You'll Learn

- Multi-character token recognition
- Better error reporting with spans
- Handling more complex syntax

---

## Phase 2: Parser & AST

**Goal:** Build Abstract Syntax Tree from tokens

**Difficulty:** Medium (more complex than Lox due to types)

**Time Estimate:** 3-5 days

### AST Node Types

#### Statements (Don't produce values)

```rust
pub enum Stmt {
    VarDecl {
        name: String,
        type_annotation: Type,
        initializer: Option<Expr>,
        is_mutable: bool,
        span: Span,
    },
    
    FnDecl {
        name: String,
        params: Vec<Param>,
        return_type: Type,
        body: Vec<Stmt>,
        span: Span,
    },
    
    StructDecl {
        name: String,
        fields: Vec<Field>,
        span: Span,
    },
    
    TraitDecl {
        name: String,
        methods: Vec<FnSignature>,
        span: Span,
    },
    
    ImplBlock {
        trait_name: Option<String>,
        type_name: String,
        methods: Vec<Stmt>,
        span: Span,
    },
    
    If { 
        condition: Expr, 
        then_branch: Box<Stmt>, 
        else_branch: Option<Box<Stmt>>,
        span: Span,
    },
    
    While { 
        condition: Expr, 
        body: Box<Stmt>,
        span: Span,
    },
    
    For { 
        variable: String,
        iterable: Expr,
        body: Box<Stmt>,
        span: Span,
    },
    
    Loop { 
        body: Box<Stmt>,
        span: Span,
    },
    
    Return { value: Option<Expr>, span: Span },
    Break { span: Span },
    Continue { span: Span },
    ExprStmt { expr: Expr, span: Span },
    Block { stmts: Vec<Stmt>, span: Span },
}
```

#### Expressions (Produce values)

```rust
pub enum Expr {
    Integer { value: i64, span: Span },
    Float { value: f64, span: Span },
    String { value: String, span: Span },
    Bool { value: bool, span: Span },
    Nil { span: Span },
    
    Identifier { name: String, span: Span },
    
    Binary { 
        left: Box<Expr>, 
        op: BinaryOp, 
        right: Box<Expr>,
        span: Span,
    },
    
    Unary { 
        op: UnaryOp, 
        expr: Box<Expr>,
        span: Span,
    },
    
    Call { 
        callee: Box<Expr>, 
        args: Vec<Expr>,
        span: Span,
    },
    
    Member { 
        object: Box<Expr>, 
        field: String,
        span: Span,
    },
    
    Index { 
        object: Box<Expr>, 
        index: Box<Expr>,
        span: Span,
    },
    
    Assign { 
        target: Box<Expr>, 
        value: Box<Expr>,
        span: Span,
    },
    
    Array { 
        elements: Vec<Expr>,
        span: Span,
    },
    
    Object { 
        type_name: String, 
        fields: Vec<(String, Expr)>,
        span: Span,
    },
    
    Lambda { 
        params: Vec<Param>, 
        return_type: Type, 
        body: Vec<Stmt>,
        span: Span,
    },
    
    Match { 
        expr: Box<Expr>, 
        arms: Vec<MatchArm>,
        span: Span,
    },
    
    Grouping { 
        expr: Box<Expr>,
        span: Span,
    },
}

pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod, Power,
    Eq, NotEq, Less, LessEq, Greater, GreaterEq,
    And, Or,
}

pub enum UnaryOp {
    Neg,  // -x
    Not,  // !x
}
```

#### Type Representations

```rust
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Void,
    
    Array(Box<Type>),
    Tuple(Vec<Type>),
    
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    
    Named(String),
    
    Generic {
        name: String,
        type_params: Vec<Type>,
    },
    
    TypeVar(String),  // For inference
}
```

### Supporting Types

```rust
pub struct Param {
    name: String,
    type_annotation: Type,
    default_value: Option<Expr>,
}

pub struct Field {
    name: String,
    type_annotation: Type,
    default_value: Option<Expr>,
}

pub struct FnSignature {
    name: String,
    params: Vec<Param>,
    return_type: Type,
}

pub struct MatchArm {
    pattern: Pattern,
    guard: Option<Expr>,
    body: Expr,
}

pub enum Pattern {
    Wildcard,
    Literal(Expr),
    Identifier(String),
    Tuple(Vec<Pattern>),
    Struct { name: String, fields: Vec<(String, Pattern)> },
}
```

### Grammar (BNF-style)

```
program        → declaration* EOF ;

declaration    → varDecl | fnDecl | structDecl | traitDecl | implBlock | statement ;

varDecl        → "let" IDENTIFIER ":" type ( "=" expression )? ";" ;
fnDecl         → "fn" IDENTIFIER "(" parameters? ")" "->" type block ;
structDecl     → "struct" IDENTIFIER "{" fields "}" ;
traitDecl      → "trait" IDENTIFIER "{" fnSignatures "}" ;
implBlock      → "impl" IDENTIFIER ("for" IDENTIFIER)? "{" fnDecl* "}" ;

statement      → exprStmt | ifStmt | whileStmt | forStmt | returnStmt | block ;

exprStmt       → expression ";" ;
ifStmt         → "if" expression block ( "else" block )? ;
whileStmt      → "while" expression block ;
forStmt        → "for" IDENTIFIER "in" expression block ;
returnStmt     → "return" expression? ";" ;
block          → "{" declaration* "}" ;

expression     → assignment ;
assignment     → ( call "." )? IDENTIFIER "=" assignment | logicOr ;
logicOr        → logicAnd ( "or" logicAnd )* ;
logicAnd       → equality ( "and" equality )* ;
equality       → comparison ( ( "==" | "!=" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "+" | "-" ) factor )* ;
factor         → unary ( ( "*" | "/" | "%" ) unary )* ;
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" | "." IDENTIFIER | "[" expression "]" )* ;
primary        → INTEGER | FLOAT | STRING | "true" | "false" | "nil"
               | IDENTIFIER | "(" expression ")" | array | object | lambda | match ;

type           → "int" | "float" | "string" | "bool" | "void"
               | "[" type "]"
               | "(" type ( "," type )* ")" "->" type
               | IDENTIFIER
               | IDENTIFIER "<" type ( "," type )* ">"
```

### Example Programs to Parse

```
// Example 1: Simple function
fn add(a: int, b: int) -> int {
    return a + b;
}

// Example 2: Struct with methods
struct Point {
    x: float,
    y: float,
}

impl Point {
    fn new(x: float, y: float) -> Point {
        return Point { x: x, y: y };
    }
    
    fn distance(self: Point) -> float {
        return sqrt(self.x * self.x + self.y * self.y);
    }
}

// Example 3: Pattern matching
fn describe(x: int) -> string {
    return match x {
        0 => "zero",
        1 => "one",
        _ => "other",
    };
}

// Example 4: Generic function
fn first<T>(arr: [T]) -> T {
    return arr[0];
}

// Example 5: Arrays and loops
fn sum(numbers: [int]) -> int {
    let total: int = 0;
    for num in numbers {
        total = total + num;
    }
    return total;
}
```

### Implementation Tips

- Use **recursive descent parsing** (like Lox)
- Add **error recovery** - continue parsing after errors
- Build **operator precedence** into the grammar
- Track **spans** for every node for error messages
- Consider **Pratt parsing** for expressions (cleaner than recursive descent)

### What You'll Learn

- Parsing type annotations
- Handling generic syntax
- Building more complex AST nodes
- Better error recovery strategies

---

## Phase 3: Semantic Analysis & Type Checking

**Goal:** Validate the program and infer/check types

**Difficulty:** Hard (this is the new challenge!)

**Time Estimate:** 5-7 days

This is where your language differs most from Lox. You need to:

1. Build symbol tables
2. Resolve all names
3. Check type correctness
4. Infer types where not specified
5. Validate trait implementations

### Symbol Table Structure

```rust
pub struct SemanticAnalyzer {
    scopes: Vec<Scope>,
    current_function: Option<FunctionContext>,
    in_loop: bool,
    errors: Vec<SemanticError>,
}

pub struct Scope {
    symbols: HashMap<String, Symbol>,
}

pub struct Symbol {
    name: String,
    kind: SymbolKind,
    type_info: Type,
    is_mutable: bool,
    is_initialized: bool,
    span: Span,
}

pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
    Struct,
    Trait,
    TypeAlias,
}

pub struct FunctionContext {
    name: String,
    return_type: Type,
}
```

### Type Checking Rules

**Variable Declaration:**
```
let x: int = "hello";  // ERROR: Type mismatch
let y: int = 42;       // OK
let z = 42;            // OK: infer type as int
```

**Function Calls:**
```
fn add(a: int, b: int) -> int { ... }

add(1, 2);      // OK
add(1, "hi");   // ERROR: Expected int, got string
add(1);         // ERROR: Wrong number of arguments
```

**Binary Operations:**
```
1 + 2        // OK: int + int -> int
1 + 2.5      // OK: int + float -> float (auto-cast)
"hi" + 5     // ERROR: Cannot add string and int
```

**Member Access:**
```
struct Point { x: int, y: int }
let p = Point { x: 1, y: 2 };
p.x          // OK: int
p.z          // ERROR: Point has no field 'z'
```

### Type Inference Algorithm Overview

Use **constraint-based type inference** (simplified Hindley-Milner):

**Steps:**

1. **Assign type variables** to unknowns
   ```
   let x = 42;
   // x: 'a (unknown type)
   // 42: int
   ```

2. **Generate constraints**
   ```
   'a = int  (from initialization)
   ```

3. **Unify constraints**
   ```
   Solve: 'a = int
   Result: x has type int
   ```

4. **Substitute** type variables with concrete types

**Example with functions:**
```
fn identity(x) {
    return x;
}

// Inference:
// 1. x: 'a (parameter type unknown)
// 2. return type: 'b
// 3. Constraint: 'b = 'a (return x)
// 4. Solution: fn identity(x: 'a) -> 'a
// This becomes a GENERIC function!
```

### Semantic Errors to Detect

```rust
pub enum SemanticError {
    TypeMismatch { 
        expected: Type, 
        got: Type, 
        span: Span 
    },
    
    UnknownType { 
        type_name: String, 
        span: Span 
    },
    
    UndefinedVariable { 
        name: String, 
        span: Span 
    },
    
    UndefinedFunction { 
        name: String, 
        span: Span 
    },
    
    DuplicateDefinition { 
        name: String, 
        original_span: Span,
        duplicate_span: Span 
    },
    
    InvalidBinaryOp { 
        left: Type, 
        op: BinaryOp, 
        right: Type, 
        span: Span 
    },
    
    InvalidUnaryOp { 
        op: UnaryOp, 
        operand: Type, 
        span: Span 
    },
    
    WrongArgumentCount { 
        expected: usize, 
        got: usize, 
        span: Span 
    },
    
    WrongArgumentType {
        param_name: String,
        expected: Type,
        got: Type,
        span: Span,
    },
    
    MissingReturn { 
        function: String, 
        span: Span 
    },
    
    BreakOutsideLoop { span: Span },
    ContinueOutsideLoop { span: Span },
    
    NoSuchField { 
        type_name: String, 
        field: String, 
        span: Span 
    },
    
    NoSuchMethod { 
        type_name: String, 
        method: String, 
        span: Span 
    },
    
    MissingField {
        type_name: String,
        field: String,
        span: Span,
    },
    
    TraitNotImplemented { 
        trait_name: String, 
        type_name: String, 
        span: Span 
    },
}
```

### Multi-Pass Analysis Strategy

**Pass 1: Symbol Collection**
- Walk AST and collect all top-level declarations
- Build initial symbol table
- Check for duplicate names
- Register all structs, traits, functions

**Pass 2: Type Resolution**
- Resolve all type names to concrete types
- Check that all referenced types exist
- Build type table for structs and traits
- Validate struct field types

**Pass 3: Type Inference**
- Assign type variables to unknowns
- Generate constraints from expressions
- Unify constraints to solve for types
- Handle generic type instantiation

**Pass 4: Type Checking**
- Verify all expressions have valid types
- Check function calls match signatures
- Validate binary/unary operations
- Check assignments are type-compatible
- Ensure struct field access is valid

**Pass 5: Advanced Validation**
- Ensure all code paths return
- Check trait implementations are complete
- Validate generic type usage
- Check for unreachable code
- Validate pattern matching exhaustiveness

### Example Error Messages

Good error messages are crucial for user experience:

```
Error: Type mismatch
  --> main.txt:5:13
   |
 5 |     let x: int = "hello";
   |            ^^^   ^^^^^^^ expected int, got string
   |            |
   |            variable declared as int here
   |
   = note: cannot assign string to int variable
   = help: change type to 'string' or convert value to int

Error: Undefined variable
  --> main.txt:10:11
   |
10 |     print(y);
   |           ^ undefined variable 'y'
   |
   = note: no variable named 'y' in current scope
   = help: did you mean 'x'? (similar name found on line 5)

Error: Missing return statement
  --> main.txt:15:1
   |
15 | fn calculate(x: int) -> int {
   |                         ^^^ function declared to return int
16 |     let result = x * 2;
17 | }
   | ^ missing return statement
   |
   = note: all code paths must return a value
   = help: add 'return result;' before closing brace

Error: Wrong number of arguments
  --> main.txt:20:5
   |
20 |     add(1);
   |     ^^^^^^ expected 2 arguments, got 1
   |
   = note: function 'add' is defined here:
15 | fn add(a: int, b: int) -> int {
   |        ^^^^^^^^^^^^^^^ requires 2 parameters
```

### What You'll Learn

- Type inference algorithms
- Constraint solving
- Multi-pass compilation
- Advanced error reporting
- Symbol table management

---

## Phase 4: Tree-Walking Interpreter

**Goal:** Execute the AST directly

**Difficulty:** Medium (similar to Lox but with type info)

**Time Estimate:** 3-5 days

### Interpreter Structure

```rust
pub struct Interpreter {
    globals: Environment,
    locals: HashMap<ExprId, usize>,
}

pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}
```

### Runtime Values

```rust
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Nil,
    
    Array(Vec<Value>),
    
    Struct {
        type_name: String,
        fields: HashMap<String, Value>,
    },
    
    Function {
        name: String,
        params: Vec<Param>,
        body: Vec<Stmt>,
        closure: Environment,
    },
    
    NativeFunction {
        name: String,
        arity: usize,
        func: fn(&[Value]) -> Result<Value, RuntimeError>,
    },
}
```

### Execution Overview

The interpreter needs to handle:

**Statements:**
- Variable declarations (initialize and store in environment)
- Function declarations (create closures)
- Struct declarations (register type information)
- Control flow (if, while, for, loop)
- Return, break, continue (use Rust's Result for control flow)
- Blocks (push/pop scopes)

**Expressions:**
- Literals (return as values)
- Binary/Unary operations (apply operators to values)
- Function calls (create new environment, bind parameters, execute body)
- Member access (lookup fields in structs)
- Array indexing (bounds checking)
- Assignments (update environment)

### Runtime Errors

```rust
pub enum RuntimeError {
    TypeError { message: String, span: Span },
    DivisionByZero { span: Span },
    UndefinedVariable { name: String, span: Span },
    WrongArgumentCount { expected: usize, got: usize, span: Span },
    IndexOutOfBounds { index: i64, length: usize, span: Span },
    NoSuchField { field: String, span: Span },
    NotCallable { span: Span },
    InvalidBinaryOp { left_type: String, op: String, right_type: String, span: Span },
    InvalidUnaryOp { op: String, operand_type: String, span: Span },
    
    // Special control flow errors
    Return(Value),   // Not really an error, used for return flow
    Break,           // For break statements
    Continue,        // For continue statements
}
```

### Key Concepts

**Environment Management:**
- Global environment for top-level declarations
- Local environments created for blocks and functions
- Closure capture: functions remember their defining environment

**Type-Safe Operations:**
- Even though types are checked during semantic analysis, runtime still needs to handle operations safely
- Type information helps optimize certain operations

**Memory Management:**
- Use Rust's ownership system
- Consider reference counting (Rc) for shared values
- Clone values when needed (simple but works)

### What You'll Learn

- Environment chaining
- Closure implementation
- Control flow via exceptions/results
- Runtime value representation
- Balancing safety and performance

---

## Phase 5: Standard Library

**Goal:** Provide useful built-in functions

**Difficulty:** Easy-Medium

**Time Estimate:** 2-4 days

### Core Modules to Implement

#### I/O Module
```
fn print(value: any) -> void
fn println(value: any) -> void
fn input(prompt: string) -> string
fn read_file(path: string) -> string
fn write_file(path: string, content: string) -> void
```

#### Math Module
```
fn sqrt(x: float) -> float
fn pow(base: float, exp: float) -> float
fn abs(x: float) -> float
fn floor(x: float) -> int
fn ceil(x: float) -> int
fn round(x: float) -> int
fn sin(x: float) -> float
fn cos(x: float) -> float
fn tan(x: float) -> float
fn min(a: float, b: float) -> float
fn max(a: float, b: float) -> float
```

#### String Module
```
fn len(s: string) -> int
fn substr(s: string, start: int, end: int) -> string
fn split(s: string, delimiter: string) -> [string]
fn join(arr: [string], separator: string) -> string
fn to_upper(s: string) -> string
fn to_lower(s: string) -> string
fn trim(s: string) -> string
fn contains(s: string, substr: string) -> bool
fn replace(s: string, old: string, new: string) -> string
fn starts_with(s: string, prefix: string) -> bool
fn ends_with(s: string, suffix: string) -> bool
```

#### Array Module
```
fn len(arr: [T]) -> int
fn push(arr: [T], value: T) -> void
fn pop(arr: [T]) -> T
fn map(arr: [T], f: (T) -> U) -> [U]
fn filter(arr: [T], f: (T) -> bool) -> [T]
fn reduce(arr: [T], f: (T, T) -> T, init: T) -> T
fn sort(arr: [T]) -> [T]
fn reverse(arr: [T]) -> [T]
fn slice(arr: [T], start: int, end: int) -> [T]
fn contains(arr: [T], value: T) -> bool
```

#### Conversion Module
```
fn to_int(s: string) -> int
fn to_float(s: string) -> float
fn to_string(value: any) -> string
```

### Native Function Registration

Register native functions at interpreter startup:
- Create NativeFunction values
- Add to global environment
- Each function checks argument types at runtime
- Return appropriate values or errors

### What You'll Learn

- FFI-style function binding
- Wrapping Rust functions for your language
- Standard library design patterns
- Error handling at boundaries

---

## Phase 6: Advanced Features

### Feature 1: Generics

**Goal:** Write reusable code for multiple types

**Time Estimate:** 3-5 days

**Syntax:**
```
fn identity<T>(x: T) -> T {
    return x;
}

struct Box<T> {
    value: T,
}

impl<T> Box<T> {
    fn new(value: T) -> Box<T> {
        return Box { value: value };
    }
    
    fn get(self: Box<T>) -> T {
        return self.value;
    }
}
```

**Implementation Strategy:**
- **Monomorphization** (like Rust/C++): Generate separate code for each type used
- `Box<int>` and `Box<string>` become separate types during semantic analysis
- Type checker resolves generic parameters
- Interpreter works with concrete types only

**Challenges:**
- Type parameter resolution
- Constraint checking
- Generic function instantiation
- Error messages with generic types

### Feature 2: Traits (Interfaces)

**Goal:** Define shared behavior across types

**Time Estimate:** 3-5 days

**Syntax:**
```
trait Printable {
    fn to_string(self: Self) -> string;
}

impl Printable for int {
    fn to_string(self: int) -> string {
        // Convert int to string
    }
}

impl Printable for Point {
    fn to_string(self: Point) -> string {
        return "Point(" + float_to_string(self.x) + ", " + float_to_string(self.y) + ")";
    }
}

fn print_all<T: Printable>(items: [T]) -> void {
    for item in items {
        print(item.to_string());
    }
}
```

**Implementation Strategy:**
- Trait as interface definition (method signatures)
- Impl blocks associate traits with types
- Check all required methods are implemented
- Type checker validates trait bounds on generics

**Challenges:**
- Trait method resolution
- Multiple trait bounds
- Trait inheritance
- Default implementations (optional)

### Feature 3: Pattern Matching Enhancements

**Goal:** Exhaustive pattern matching

**Time Estimate:** 2-3 days

**Syntax:**
```
enum Option<T> {
    Some(T),
    None,
}

fn unwrap<T>(opt: Option<T>) -> T {
    return match opt {
        Some(value) => value,
        None => panic("unwrap on None"),
    };
}

fn process(x: int) -> string {
    return match x {
        0 => "zero",
        1 | 2 | 3 => "small",
        n if n < 10 => "medium",
        _ => "large",
    };
}
```

**Implementation Strategy:**
- Add enum support to parser and AST
- Pattern exhaustiveness checking during semantic analysis
- Runtime pattern matching execution
- Guard expressions support

**Challenges:**
- Exhaustiveness algorithm
- Nested pattern matching
- Variable binding in patterns
- Guard expression evaluation

### Feature 4: Module System

**Goal:** Organize code into multiple files

**Time Estimate:** 3-4 days

**Syntax:**
```
// math.lang
pub fn add(a: int, b: int) -> int {
    return a + b;
}

fn internal_helper() -> int {
    return 42;
}

// main.lang
import math;
// or: import math::add;

fn main() -> void {
    let result = math::add(1, 2);
    print(result);
}
```

**Implementation Strategy:**
- File-based modules (one file = one module)
- Module path resolution
- Public/private visibility
- Import statement handling
- Separate compilation per module

**Challenges:**
- Circular dependency detection
- Module search paths
- Symbol visibility rules
- Cross-module type checking

---

## Phase 7: Memory Management

**Goal:** Decide how to manage memory

**Difficulty:** Medium-Hard

**Time Estimate:** 5-7 days

### Three Options

#### Option 1: Reference Counting (Easiest)

**How it works:**
- Each value has a reference count
- Count increments when referenced
- Count decrements when reference dropped
- Free when count reaches zero

**Pros:**
- Simple to implement
- Deterministic cleanup
- No pause times

**Cons:**
- Can't handle cycles
- Overhead on every assignment
- Reference counting operations

**Recommendation:** Start with this using Rust's `Rc<RefCell<>>`

#### Option 2: Tracing Garbage Collection (Medium)

**How it works:**
- Periodically scan for reachable objects
- Mark all reachable objects
- Sweep (free) unreachable objects

**Pros:**
- Handles cycles naturally
- Lower per-operation overhead
- More memory efficient

**Cons:**
- Pause times during collection
- More complex to implement
- Needs root set tracking

**Recommendation:** Add this later if performance matters

#### Option 3: Ownership System (Hardest)

**How it works:**
- Compile-time borrow checking
- Move semantics by default
- Explicit borrows

**Pros:**
- Zero runtime overhead
- Memory safety guaranteed
- No GC pauses

**Cons:**
- Very complex type system
- Difficult for users to learn
- Significantly harder to implement

**Recommendation:** Don't do this unless you want a PhD-level challenge

### Recommended Approach

**Phase 1:** Use Rust's `Rc<RefCell<>>` for all values
- Simple, works immediately
- Good enough for learning

**Phase 2 (Optional):** Implement mark-and-sweep GC
- Better performance
- More realistic for production language

---

## Phase 8: Error Messages & Developer Experience

**Goal:** Make your language pleasant to use

**Difficulty:** Medium

**Time Estimate:** 3-5 days

### Error Message Best Practices

**Show context with source code:**
```
Error: Type mismatch
  --> main.lang:5:13
   |
 5 |     let x: int = "hello";
   |            ^^^   ^^^^^^^ expected int, got string
   |            |
   |            variable declared as int here
```

**Provide helpful suggestions:**
```
Error: Undefined variable 'cout'
  --> main.lang:10:5
   |
10 |     cout << "Hello";
   |     ^^^^ undefined variable
   |
   = help: did you mean 'print'?
```

**Explain the problem:**
```
Error: Missing return statement
  --> main.lang:15:1
   |
15 | fn calculate(x: int) -> int {
   |                         ^^^ function declared to return int
16 |     let result = x * 2;
17 | }
   | ^ not all code paths return a value
   |
   = note: consider adding 'return result;' at the end
```

**Group related errors:**
```
Error: Multiple issues found in function 'process'
  --> main.lang:20:1
   |
20 | fn process(x: string) -> int {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this function
21 |     let y = x + 5;           // Type mismatch
   |             ^^^^^ cannot add string and int
22 |     print(z);                // Undefined variable
   |           ^ undefined variable 'z'
23 | }
   | ^ missing return statement
```

### Implementation Tips

- Store source code for error display
- Track spans for every AST node
- Use colors in terminal output (if supported)
- Limit cascading errors (stop after 10)
- Show multiple suggestions when ambiguous

---

## Phase 9: Testing Strategy

**Goal:** Ensure your language works correctly

**Difficulty:** Easy-Medium

**Time Estimate:** Ongoing (2-3 days for framework)

### Test Categories

#### 1. Scanner Tests
```rust
#[test]
fn test_scan_keywords() {
    let source = "let fn if else while";
    let tokens = scan(source);
    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert_eq!(tokens[1].kind, TokenKind::Fn);
    // ... etc
}

#[test]
fn test_scan_operators() {
    let source = "::->=>==";
    let tokens = scan(source);
    assert_eq!(tokens[0].kind, TokenKind::DoubleColon);
    // ... etc
}
```

#### 2. Parser Tests
```rust
#[test]
fn test_parse_function() {
    let source = "fn add(a: int, b: int) -> int { return a + b; }";
    let ast = parse(source);
    // Assert AST structure is correct
}

#[test]
fn test_parse_error_recovery() {
    let source = "let x: int = ; let y: int = 42;";
    let result = parse(source);
    // Should continue parsing after error
}
```

#### 3. Type Checker Tests
```rust
#[test]
fn test_type_mismatch() {
    let source = "let x: int = \"hello\";";
    let errors = type_check(parse(source));
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::TypeMismatch { .. }));
}

#[test]
fn test_type_inference() {
    let source = "let x = 42;";
    let types = type_check(parse(source));
    assert_eq!(types.get("x"), Some(&Type::Int));
}
```

#### 4. Integration Tests (End-to-End)
```rust
#[test]
fn test_factorial() {
    let source = r#"
        fn factorial(n: int) -> int {
            if n <= 1 {
                return 1;
            }
            return n * factorial(n - 1);
        }
        print(factorial(5));
    "#;
    let output = run(source);
    assert_eq!(output, "120\n");
}

#[test]
fn test_fibonacci() {
    let source = r#"
        fn fib(n: int) -> int {
            if n <= 1 {
                return n;
            }
            return fib(n - 1) + fib(n - 2);
        }
        print(fib(10));
    "#;
    let output = run(source);
    assert_eq!(output, "55\n");
}
```

#### 5. Error Message Tests
```rust
#[test]
fn test_helpful_error_messages() {
    let source = "let x: int = \"hello\";";
    let error_msg = get_error_message(source);
    assert!(error_msg.contains("Type mismatch"));
    assert!(error_msg.contains("expected int, got string"));
}
```

### Test-Driven Development

Write tests BEFORE implementing features:
1. Write test for desired behavior
2. Run test (it fails)
3. Implement feature
4. Run test (it passes)
5. Refactor if needed

---

## Phase 10: REPL (Interactive Mode)

**Goal:** Allow interactive programming

**Difficulty:** Easy

**Time Estimate:** 1-2 days

### REPL Features

**Basic REPL:**
```
$ mylang
> let x = 42;
> print(x);
42
> fn double(n: int) -> int { return n * 2; }
> double(x);
84
> exit
```

**Enhanced REPL:**
```
$ mylang
> let x = 42;
> :type x
int
> :show x
x: int = 42
> fn add(a: int, b: int) -> int {
... return a + b;
... }
> add(1, 2)
3
```

### REPL Commands

- `:type <expr>` - Show type of expression
- `:show <name>` - Show value and type of variable
- `:help` - Show available commands
- `:quit` or `:exit` - Exit REPL
- `:load <file>` - Load and execute file
- `:reset` - Clear all definitions

### Implementation Notes

- Maintain persistent environment between inputs
- Handle multi-line input (functions, blocks)
- Pretty-print results automatically
- History support (up/down arrows)

---

## Phase 11: Optimization Opportunities

**Goal:** Make your language faster (optional)

**Difficulty:** Medium-Hard

**Time Estimate:** Variable (weeks)

### Optimization Ideas

#### 1. Constant Folding
```
// Before:
let x = 2 + 3 * 4;

// After optimization:
let x = 14;
```

#### 2. Dead Code Elimination
```
// Before:
fn test() -> int {
    let x = 42;
    let y = 100;  // Never used
    return x;
}

// After:
fn test() -> int {
    let x = 42;
    return x;
}
```

#### 3. Tail Call Optimization
```
fn factorial(n: int, acc: int) -> int {
    if n <= 1 {
        return acc;
    }
    return factorial(n - 1, n * acc);  // Tail call - can be optimized
}
```

#### 4. Inline Small Functions
```
// Before:
fn add(a: int, b: int) -> int { return a + b; }
let x = add(1, 2);

// After:
let x = 1 + 2;
```

#### 5. Type Specialization
```
// Generic function:
fn identity<T>(x: T) -> T { return x; }

// Generate specialized versions:
fn identity_int(x: int) -> int { return x; }
fn identity_string(x: string) -> string { return x; }
```

### When to Optimize

- Get it working first
- Profile to find bottlenecks
- Optimize hot paths only
- Measure improvements

---

## Phase 12: Tooling & Ecosystem

**Goal:** Make development easier

**Difficulty:** Medium-Hard

**Time Estimate:** Variable (ongoing)

### Tools to Consider

#### 1. Formatter
Automatically format code to consistent style:
```
$ mylang fmt main.lang
Formatted main.lang
```

#### 2. Linter
Detect common mistakes and bad practices:
```
$ mylang lint main.lang
Warning: Unused variable 'x' on line 10
Warning: Function 'calculate' is too complex
```

#### 3. Language Server (LSP)
Enable IDE integration:
- Autocomplete
- Go to definition
- Find references
- Rename symbol
- Inline errors
- Hover documentation

#### 4. Package Manager
Manage dependencies:
```
$ mylang init myproject
Created new project 'myproject'

$ mylang add http@1.0.0
Added dependency: http v1.0.0

$ mylang build
Building project...
```

#### 5. Documentation Generator
Generate docs from code:
```
/// Calculates the factorial of n
/// 
/// # Arguments
/// * `n` - The number to calculate factorial for
/// 
/// # Returns
/// The factorial of n
fn factorial(n: int) -> int {
    ...
}
```

#### 6. Debugger
Interactive debugging:
```
$ mylang debug program.lang
Breakpoint 1 at line 10
> step
> print x
42
> continue
```

---

## Recommended Implementation Order

### Phase 1: Core Language (Weeks 1-4)
1. Scanner - 1-2 days
2. Parser - 3-5 days
3. Semantic Analysis - 5-7 days
4. Interpreter - 3-5 days
5. Basic Standard Library - 2-4 days

**Milestone:** Can run simple programs with functions and control flow

### Phase 2: Type System (Weeks 5-6)
1. Type inference - 3-4 days
2. Generic types - 3-5 days
3. Better error messages - 2-3 days

**Milestone:** Full static type system working

### Phase 3: Advanced Features (Weeks 7-9)
1. Traits/Interfaces - 3-5 days
2. Pattern matching - 2-3 days
3. Module system - 3-4 days
4. Enhanced standard library - 3-5 days

**Milestone:** Production-ready language features

### Phase 4: Polish & Tools (Weeks 10-12)
1. REPL - 1-2 days
2. Comprehensive testing - 3-4 days
3. Documentation - 2-3 days
4. Example programs - 2-3 days
5. Performance tuning - 3-5 days

**Milestone:** Complete, polished language ready for use

---

## Key Differences from Lox

| Aspect | Lox | Your Language |
|--------|-----|---------------|
| Types | Dynamic | Static with inference |
| Type checking | Runtime | Compile time |
| OOP | Classes with inheritance | Structs with traits |
| Generics | No | Yes |
| Pattern matching | No | Yes |
| Modules | No | Yes |
| Error messages | Basic | Detailed with suggestions |
| Standard library | Minimal | Rich with modules |
| Memory | GC (Java-style) | Your choice |

---

## Common Pitfalls to Avoid

### 1. Feature Creep
- Don't add every feature you can think of
- Focus on core features first
- Get them working well before adding more

### 2. Poor Error Messages
- Users will spend most time fixing errors
- Invest time in clear, helpful messages
- Show source context and suggestions

### 3. Skipping Tests
- Write tests as you go
- Don't wait until the end
- Test-driven development works well

### 4. Premature Optimization
- Make it work first
- Make it right second
- Make it fast last (if needed)

### 5. Ignoring Edge Cases
- Handle errors gracefully
- Consider boundary conditions
- Test with invalid input

### 6. Incomplete Type System
- Type inference is tricky
- Handle all cases properly
- Don't half-implement generics

---

## Resources & References

### Books
- "Crafting Interpreters" by Bob Nystrom (you've read this!)
- "Types and Programming Languages" by Benjamin Pierce
- "Advanced Compiler Design and Implementation" by Steven Muchnick

### Papers
- "Algorithm W" - Hindley-Milner type inference
- "Essentials of Programming Languages" - semantic analysis
- "A Theory of Type Polymorphism" - generic types

### Language Inspirations
- **Rust** - ownership, traits, pattern matching
- **TypeScript** - gradual typing, type inference
- **Swift** - modern syntax, type safety
- **Go** - simplicity, interfaces
- **OCaml** - type inference, pattern matching

### Online Resources
- Rust compiler source (excellent reference)
- LLVM documentation (if you add compilation)
- Programming language design forums

---

## Success Criteria

Your language is successful when:

1. **It works** - Can execute real programs correctly
2. **Types help** - Catch real bugs at compile time
3. **Errors are clear** - Users understand what went wrong
4. **It's usable** - REPL and good stdlib make it practical
5. **You learned** - Deep understanding of type systems
6. **It's extensible** - Easy to add new features

---

## Next Steps After Completion

### Option 1: Add Compilation
- Generate machine code or LLVM IR
- Dramatically faster execution
- Learn about codegen and optimization

### Option 2: Build Something Real
- Web framework
- Game engine
- Data processing tool
- Show what your language can do

### Option 3: Create Community
- Open source your language
- Write documentation and tutorials
- Help others learn from your work

### Option 4: Advanced Type Features
- Higher-kinded types
- Dependent types
- Effect systems
- Push the boundaries further

---

## Final Thoughts

Building a statically-typed language is significantly more challenging than Lox, but you'll learn:

- **Type theory** - How type systems work
- **Program analysis** - Static checking and inference
- **Compiler design** - Multi-pass compilation
- **Language design** - Trade-offs and decisions

Take your time, build incrementally, and don't be afraid to:
- Simplify features if needed
- Ask for help from communities
- Iterate on your design
- Start over if you learn something fundamental

Good luck with your language! Remember: the goal is learning, not perfection.

---

## Appendix: Quick Reference

### Minimal Working Example

Here's a complete program in your language:

```
// Define a struct
struct Point {
    x: float,
    y: float,
}

// Implement methods
impl Point {
    fn new(x: float, y: float) -> Point {
        return Point { x: x, y: y };
    }
    
    fn distance_from_origin(self: Point) -> float {
        return sqrt(self.x * self.x + self.y * self.y);
    }
}

// Generic function
fn max<T>(a: T, b: T) -> T {
    return if a > b { a } else { b };
}

// Main function
fn main() -> void {
    let p1 = Point::new(3.0, 4.0);
    let d = p1.distance_from_origin();
    println("Distance: " + to_string(d));
    
    let bigger = max(10, 20);
    println("Max: " + to_string(bigger));
}
```

This demonstrates:
- Structs with fields
- Implementation blocks
- Methods with self parameter
- Generic functions
- Type annotations
- Standard library usage

Start here and build up!

## Study Notes: Chapter 6: Parsing Expressions

### 1\. Goal: From Tokens to Syntax Trees

The primary goal of this chapter is to build a **parser**. A parser's job is to take the flat list of tokens produced by the scanner and transform it into a structured **Abstract Syntax Tree (AST)**. This tree represents the code's grammatical structure, making it easy for the interpreter to understand and execute.

This chapter builds a complete, robust, hand-written parser for Lox's expression grammar.

-----

### 2\. The Problem: Ambiguous Grammars

Our simple grammar from Chapter 5 (e.g., `binary -> expression operator expression`) is **ambiguous**.

  * **Ambiguity** means there is more than one way to parse the same sequence of tokens, resulting in different ASTs.
  * For example, the expression `6 / 3 - 1` could be parsed as `(6 / 3) - 1` or `6 / (3 - 1)`, which evaluate to different results.

#### 2.1. Solution: Precedence & Associativity

To solve ambiguity, we must formally define operator rules, just as they exist in mathematics.

  * **Precedence:** Determines which operator is evaluated first in a mix of operators.
      * Example: `*` and `/` have higher precedence than `+` and `-`. `1 + 2 * 3` is `1 + (2 * 3)`.
  * **Associativity:** Determines which operator is evaluated first in a series of the *same* operator.
      * **Left-associative:** Operators on the left evaluate first. Most arithmetic operators are left-associative.
          * Example: `5 - 3 - 1` becomes `(5 - 3) - 1`.
      * **Right-associative:** Operators on the right evaluate first. Assignment is a common example.
          * Example: `a = b = c` becomes `a = (b = c)`.

-----

### 3\. Implementing Precedence: A Stratified Grammar

**Key Idea:** We solve ambiguity by rewriting the grammar. We "stratify" it by creating one distinct rule for *each precedence level*.

The single `expression` rule is expanded into a chain, from lowest precedence to highest:

```
expression -> equality
equality   -> comparison ( ( "!=" | "==" ) comparison )*
comparison -> term       ( ( ">" | ">=" | "<" | "<=" ) term )*
term       -> factor     ( ( "-" | "+" ) factor )*
factor     -> unary      ( ( "/" | "*" ) unary )*
unary      -> ( "!" | "-" ) unary | primary
primary    -> NUMBER | STRING | "true" | "false" | "nil"
           | "(" expression ")"
```

  * Each rule matches expressions at its own precedence level *or any level higher*.
  * For example, the `term` rule (for `+` and `-`) calls the `factor` rule (for `*` and `/`) to get its operands. This ensures that multiplication is evaluated before addition.
  * This new grammar is **unambiguous** and directly encodes the precedence and associativity rules.

-----

### 4\. The Technique: Recursive Descent Parsing

**Key Decision:** We will hand-write our parser using a technique called **Recursive Descent**.

  * It is a **top-down** parser, meaning it starts from the "topmost" grammar rule (`expression`) and works its way down to the "leaves" (`primary`).
  * It's simple, fast, robust, and used by major production compilers (e.g., GCC, V8, Roslyn).

**The Core Concept:** We translate the grammar *directly* into a series of functions.

  * **Each Grammar Rule** $\rightarrow$ **A Function/Method** in the `Parser` class (e.g., the `equality` rule becomes the `equality()` method).
  * **Nonterminal Symbol** (e.g., `comparison`) $\rightarrow$ **A Function Call** (e.g., a call to `comparison()`). This is the "recursive" part.
  * **Terminal Symbol** (e.g., `==`) $\rightarrow$ **Code to Match a Token** (e.g., `match(EQUAL_EQUAL)`).
  * **Choice (`|`)** $\rightarrow$ An `if` or `switch` statement.
  * **Repetition (`*` or `+`)** $\rightarrow$ A `while` loop.
  * **Optional (`?`)** $\rightarrow$ An `if` statement.

-----

### 5\. Parser Implementation (`Parser.java`)

The `Parser` class is built around a few helper methods that manage the token stream:

  * **Core Helpers:**
      * `match(TokenType... types)`: Checks if the current token matches any of the given types. If it does, it **consumes the token** and returns `true`.
      * `check(TokenType type)`: Checks the current token type **without consuming it** (a "lookahead").
      * `advance()`: Consumes the current token and returns it.
      * `peek()`, `previous()`, `isAtEnd()`: Other primitive helpers.
  * **Binary Operator Parsing:** The methods for binary operators (like `equality()`, `term()`, etc.) follow a specific pattern that elegantly handles left-associativity:
    1.  Call the method for the *next higher* precedence level to get the left-hand operand (e.g., `equality()` calls `comparison()`).
    2.  Start a `while` loop that runs as long as `match()` finds an operator at the *current* precedence level (e.g., `!=` or `==`).
    3.  Inside the loop, consume the operator, parse the right-hand operand (by calling the *next higher* precedence rule again), and create a new `Expr.Binary` node.
    4.  The *newly created node* becomes the *left-hand operand* for the next iteration of the loop.

-----

### 6\. Error Handling & Recovery

A parser has two jobs: parse valid code and report errors for invalid code.

  * **Goal:** We must report errors clearly, not crash, and **minimize cascaded errors** (phantom errors that are just side effects of an earlier, real error).

#### 6.1. Strategy: "Panic Mode" Recovery

When the parser encounters a syntax error, it enters **"panic mode."**

  * **Synchronization:** To get back on track, the parser discards tokens until it finds a "synchronization point"—a place where it's safe to resume parsing.
  * We will synchronize at **statement boundaries** (e.g., after a `;` or at the start of a keyword like `if`, `var`, or `for`). This prevents a single error from generating a dozen "cascaded" error messages.

#### 6.2. Implementation

  * `consume(TokenType type, String message)`: This method *expects* a token of a specific type. If the current token doesn't match, it calls `error()`.
  * `error(Token token, String message)`: This helper reports the error to the user and returns a `ParseError`.
  * `ParseError`: A private, nested `RuntimeException` class. It's a **sentinel** used purely to unwind the parser's state.
  * **Unwinding:** We `throw` this `ParseError` to unwind the Java call stack (which represents the parser's nested state).
  * **Catching:** A `try...catch` block in a high-level rule (in this chapter, the main `parse()` method) catches the `ParseError`.
  * `synchronize()`: After catching the error, this method is called. It discards tokens until it finds the next statement boundary, allowing the parser to continue.

-----

### 7\. Design Note: Logic Versus History

  * **The Dilemma:** C's precedence for bitwise operators (`&`, `|`) is logically flawed (it's lower than `==`, forcing users to write `(flags & MASK) == FLAG`).
  * **The Trade-off:**
      * **Logic:** We *could* "fix" this precedence in Lox to be more intuitive and internally consistent. This is better for new users learning the language from scratch.
      * **History:** We *could* keep the flawed C precedence because *millions of programmers* already know it. This makes the language more familiar and easier to adopt for experienced developers.
  * **Conclusion:** There's no perfect answer. Familiarity is a powerful tool for language adoption, but it often means inheriting the "mistakes" of the past.

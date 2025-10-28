## Study Notes: Chapter 5: Representing Code

### 1\. The Goal: From Tokens to Trees

  * **From:** A flat list of tokens (from the scanner).
  * **To:** A richer, higher-level representation that understands the code's nested structure (grammar).
  * **Method:** This new representation will be a tree structure that mirrors the grammar of the language, known as an **Abstract Syntax Tree (AST)**.
  * **Purpose:** The AST is simple for the **parser** to produce and easy for the **interpreter** to "walk" and consume.

-----

### 2\. Theory: Context-Free Grammars (CFGs)

To define the *syntax* of our language (what the AST represents), we need a more powerful tool than the "regular languages" used by the scanner.

  * **Scanner (Lexical Grammar):**
      * Uses **Regular Languages**.
      * "Alphabet" is *characters*.
      * "String" is a *lexeme/token* (e.g., `123`, `*`).
  * **Parser (Syntactic Grammar):**
      * Uses **Context-Free Grammars (CFGs)**.
      * "Alphabet" is *tokens*.
      * "String" is an *expression* (e.g., a sequence of tokens like `123`, `*`, `45.6`).

**Grammar Rules (Productions):**
A grammar is defined by a set of rules (productions) that generate valid "strings" (expressions).

  * **Terminals:** These are the "letters" from the alphabet. For our parser, terminals are the **tokens** from the scanner (e.Read, `NUMBER`, `+`, `STRING`).
  * **Nonterminals:** These are named references to *other rules* in the grammar, allowing for composition and recursion (e.g., `expression`, `literal`).

-----

### 3\. Grammar Notation: Backus-Naur Form (BNF)

We use a formal notation (a variant of BNF) to *write down* the rules of our grammar.

  * **Basic Rule:** `name -> body ;`
  * **Syntactic Sugar:** The book adds convenient operators (similar to regular expressions) to this notation:
      * **`|` (Pipe):** Alternation/choice (e.g., `rule -> "a" | "b" ;`).
      * **`()` (Parentheses):** Grouping.
      * **`*` (Star):** Repeat zero or more times.
      * **`+` (Plus):** Repeat one or more times.
      * **`?` (Question Mark):** Optional (zero or one time).

**Initial Lox Expression Grammar:**
Using this notation, the first subset of Lox's grammar is defined:

```
expression -> literal | unary | binary | grouping ;
literal    -> NUMBER | STRING | "true" | "false" | "nil" ;
grouping   -> "(" expression ")" ;
unary      -> ( "-" | "!" ) expression ;
binary     -> expression operator expression ;
operator   -> "==" | "!=" | "<" | "<=" | ">" | ">="
            | "+"  | "-"  | "*"  | "/" ;
```

-----

### 4\. Implementation: The `Expr` AST Classes

The recursive nature of the grammar (e.g., `grouping` contains an `expression`) maps perfectly to a tree data structure in code.

  * **Approach:** An Object-Oriented one.
  * **Base Class:** An abstract class `Expr` is created.
  * **Subclasses:** For each rule under `expression`, a **nested static class** is created (e.g., `Expr.Binary`, `Expr.Unary`, `Expr.Literal`, `Expr.Grouping`).
  * **Fields:** Each subclass is a simple data container. Its fields store the terminals (tokens) and nonterminals (other `Expr` objects) from its grammar rule.
      * `Expr.Binary` has: `Expr left`, `Token operator`, `Expr right`.
      * `Expr.Literal` has: `Object value`.

-----

### 5\. Design Challenge: The "Expression Problem"

A core design conflict arises from this OO approach.

  * **The Problem:** We will have many *operations* to perform on our `Expr` classes (interpreting, resolving, type-checking, etc.).
  * **Object-Oriented Style (Java):** Groups code by *type* (in classes).
      * **Easy to:** Add new types (just add a new `Expr` subclass).
      * **Hard to:** Add new operations (you must edit *every single* `Expr` subclass to add the new method).
  * **Functional Style (ML/Haskell):** Groups code by *operation* (in functions with pattern matching).
      * **Easy to:** Add new operations (just add a new function).
      * **Hard to:** Add new types (you must edit *every single* function to handle the new type).

Since our `Expr` types are stable, but our *operations* will grow, the OO style is awkward.

-----

### 6\. Design Solution: The Visitor Pattern

The **Visitor Pattern** is an OO design pattern that solves the "Expression Problem" by effectively emulating the functional style.

  * **Goal:** To add new operations *without* modifying the existing `Expr` classes.
  * **How it Works (Double Dispatch):**
    1.  **Define a `Visitor<R>` interface:** This interface has one `visit...()` method for *every* `Expr` subclass (e.g., `visitBinaryExpr(Expr.Binary expr)`). The `R` is a generic for the return type.
    2.  **Add `accept(Visitor<R> visitor)` to `Expr`:** The base `Expr` class gets this single, abstract `accept` method.
    3.  **Implement `accept` in subclasses:** Each subclass (e.g., `Expr.Binary`) implements the `accept` method with a single line that calls the *correct* method on the visitor: `return visitor.visitBinaryExpr(this);`.
  * **The Result:** We can now create a new operation (like an interpreter) as a *new class* that implements `Expr.Visitor`. We never have to touch the `Expr` classes again.

-----

### 7\. Tooling: Metaprogramming the AST

Writing all the `Expr` subclasses, their fields, constructors, and `accept` methods is highly repetitive boilerplate.

  * **Solution:** We create a simple **metaprogramming** script (`GenerateAst.java`).
  * **Function:** This command-line tool reads a simple, string-based description of the AST (e.g., `"Binary : Expr left, Token operator, Expr right"`).
  * **Output:** It **auto-generates the entire `Expr.java` file** for us, complete with all nested classes, fields, constructors, and the Visitor pattern implementation.

-----

### 8\. Practical Application: `AstPrinter`

To test the generated `Expr` classes and the Visitor pattern, a test class is created.

  * **Class:** `AstPrinter`
  * **Implements:** `Expr.Visitor<String>`
  * **Purpose:** To "pretty print" an AST, but in a way that makes the tree structure explicit for debugging.
  * **Output Format:** A Lisp-style S-expression (e.g., `(* (- 123) (group 45.67))`).
  * **How it Works:** It recursively calls `accept()` on its sub-expressions, wrapping the results in parentheses. This demonstrates the Visitor pattern in action.

## Study Notes: Chapter 8: Statements and State

### 1. The Goal: From Calculator to Program

The interpreter so far is a simple calculator; it can only evaluate single expressions. To make it a "real" language, we must introduce two concepts:

* **State:** The ability for the interpreter to "remember" data. This is achieved by binding names to values (i.e., **variables**).
* **Statements:** Instructions that perform work and have **side effects**. Unlike expressions, statements do not evaluate to a value. Their purpose is to *change* something, like printing to the console or modifying state.

This chapter introduces the infrastructure for statements, variables (global and local), assignment, and scope.

---

### 2. Basic Statements

We begin by adding the first two statement types.

* **Grammar:**
    * The grammar's new starting point is `program -> statement* EOF ;` (a program is a list of statements).
    * `printStmt -> "print" expression ";" ;` (Evaluates an expression and prints it).
    * `exprStmt -> expression ";" ;` (Evaluates an expression and discards the result. Used for expressions that have side effects, like assignment).
* **AST (`Stmt.java`):**
    * We create a **new, separate AST base class `Stmt`**. This is distinct from `Expr` because statements and expressions are disjoint; they are never allowed in the same grammatical locations.
    * This is automated by adding a new call to our `GenerateAst` tool.
    * **New Nodes:** `Stmt.Print(Expr expression)` and `Stmt.Expression(Expr expression)`.
* **Parsing:**
    * The main `parser.parse()` method is updated to loop, calling a new `statement()` method until it hits `EOF`, and returns a `List<Stmt>`.
    * `statement()` method checks for a `PRINT` token. If not found, it "falls through" to parsing an `expressionStatement()`.
* **Interpretation:**
    * The `Interpreter` class now also implements `Stmt.Visitor<Void>`.
    * The return type is `Void` (Java's boxed `void`) because statements don't produce a value.
    * `visitPrintStmt()`: Evaluates the expression, calls `stringify()`, and prints to `System.out.println()`.
    * `visitExpressionStmt()`: Evaluates the expression and simply discards the result.
    * The main `interpret()` method now takes a `List<Stmt>` and calls `execute()` on each one.

---

### 3. Global Variables

To introduce state, we add variable declarations and access.

* **Grammar Refinement (Declarations):**
    * We split the grammar to distinguish between "declarations" and other "statements."
    * `program -> declaration* EOF ;`
    * `declaration -> varDecl | statement ;`
    * `statement -> printStmt | exprStmt ;`
    * **Reason:** This split is crucial. Some contexts (like an `if` branch) will allow *statements* but not *declarations*. `var` is a "low-precedence" statement, and this structure allows us to control where it's allowed.
* **New Syntax:**
    * `varDecl -> "var" IDENTIFIER ( "=" expression )? ";" ;` (A `var` keyword, a name, an optional initializer, and a semicolon).
    * `primary -> ... | IDENTIFIER ;` (Using a variable's name as an expression).
* **New AST Nodes:**
    * `Stmt.Var(Token name, Expr initializer)`
    * `Expr.Variable(Token name)`
* **Parsing:**
    * The parser's `declaration()` method is now the main synchronization point for error recovery, containing the `try...catch(ParseError)` block.
    * It checks for the `var` keyword. If found, it calls `varDeclaration()`. If not, it falls through to `statement()`.
    * `primary()` is updated to check for an `IDENTIFIER` token and create an `Expr.Variable` node.
* **Core Concept: The Environment:**
    * We create a new `Environment` class to store the state (the variable bindings).
    * It internally uses a `HashMap<String, Object>`.
    * `define(String name, Object value)`: Adds a new variable. It **allows redefinition**, which is useful for the REPL.
    * `get(Token name)`: Retrieves a variable's value. If the variable is not found, it throws a **`RuntimeError`**. This is a runtime error (not a static/parse error) to allow for defining mutually recursive functions later.
* **Interpretation:**
    * The `Interpreter` gets a permanent `Environment` field for global variables.
    * `visitVarStmt()`: If there's an initializer, it's evaluated. If not, the variable is implicitly initialized to `nil` (Java `null`). The result is passed to `environment.define()`.
    * `visitVariableExpr()`: Simply calls `environment.get(expr.name)`.

---

### 4. Assignment

Now that variables exist, we need to be able to change them.

* **Syntax:**
    * Assignment (`=`) is an **expression**, not a statement.
    * `expression -> assignment ;`
    * `assignment -> IDENTIFIER "=" assignment | equality ;`
    * It has the lowest precedence and is **right-associative** (so `a = b = c` groups as `a = (b = c)`).
* **Core Concept: L-values vs. R-values:**
    * **R-value:** An expression that produces a value (e.g., `5`, `a + 1`).
    * **L-value:** An expression that represents a *storage location* (e.g., `a`). An l-value appears on the *left* side of an assignment.
* **Parsing "The Trick":**
    * A recursive descent parser with one-token lookahead doesn't know if `a` is a variable *access* or *assignment* until *after* it has parsed `a`.
    * **The Trick:**
        1.  The `assignment()` parser first parses the left-hand side as a normal expression (an r-value, like `Expr.Variable`).
        2.  It *then* checks if an `=` token follows.
        3.  If it does, it checks if the parsed expression is a valid l-value (e.g., is it an `Expr.Variable`?).
        4.  If yes, it "transforms" that expression into a new `Expr.Assign` node, parsing the right-hand side for the value.
        5.  If no (e.g., `a + b = c`), it reports an "Invalid assignment target" error.
* **Interpretation:**
    * `visitAssignExpr()`:
        1.  Evaluates the right-hand side `value`.
        2.  Calls a *new* method, `environment.assign()`.
        3.  **Returns the `value`**, since assignment is an expression.
    * `Environment.assign(Token name, Object value)`:
        * This method is *different* from `define`. It's a **runtime error to assign to an undefined variable**. Lox does not do implicit variable declaration.

---

### 5. Lexical Scope and Blocks

We introduce local variables using `{ ... }` blocks.

* **Core Concept: Lexical (Static) Scope:**
    * You can determine which variable a name refers to simply by *reading the source code*. The scope is defined by the text's nesting, not by the runtime call stack.
    * **Shadowing:** A local variable with the same name as a variable in an outer scope "hides" (or "shadows") the outer one.
* **New Syntax:**
    * `statement -> ... | block ;`
    * `block -> "{" declaration* "}" ;`
* **New AST Node:** `Stmt.Block(List<Stmt> statements)`
* **Core Implementation: Chained Environments:**
    * This is the key to lexical scope. The `Environment` class is modified to "chain" together.
    * `Environment` gets a new field: `final Environment enclosing;`.
    * The global environment has `enclosing = null`. A local environment is created with `new Environment(currentEnvironment)`.
    * `get(Token name)`: Is updated. If the name isn't in the *current* environment's map, it **recursively calls `get()` on its `enclosing` environment**.
    * `assign(Token name, Object value)`: Is updated. If the name isn't in the *current* environment, it **recursively calls `assign()` on `enclosing`**.
    * `define(String name, Object value)`: Is **unchanged**. It *always* defines the variable in the *current*, innermost environment.
* **Interpretation of Blocks:**
    * `visitBlockStmt(Stmt.Block stmt)`: This method is simple:
        1.  It creates a *new* environment: `new Environment(this.environment)`.
        2.  It calls a new helper, `executeBlock(stmt.statements, newEnvironment)`.
    * `executeBlock(List<Stmt> statements, Environment environment)`: This is the critical part for managing scope:
        1.  It saves a reference to the *current* (outer) environment: `Environment previous = this.environment;`.
        2.  It uses a **`try...finally`** block.
        3.  **`try`:** It sets the interpreter's *current* environment to the new, inner one (`this.environment = environment;`) and executes the statements in the block.
        4.  **`finally`:** It **restores** the interpreter's environment to the outer one (`this.environment = previous;`). This ensures the scope is "exited" correctly, even if a `RuntimeError` is thrown from inside the block.

---

### 6. Design Note: Implicit vs. Explicit Declaration

* **The Dilemma:** Should Lox use `var` (explicit), or just let `a = 1` create a variable (implicit, like Python/Ruby)?
* **The Problem with Implicit:** It seems simpler but creates ambiguity. Does `a = 1` in a function create a new local `a` or assign to a global `a`?
    * Typos (`varaible = 1`) are silently treated as *new* variable declarations, leading to bugs.
    * Languages that use it (Python, Ruby, JS) had to add *more* features (`global`, `nonlocal`, strict mode) to work around these ambiguities, losing the initial simplicity.
* **Lox's Choice:** Lox uses **explicit declaration** (`var`). It's clearer, less error-prone, and avoids the "which scope did I just create this in?" problem.

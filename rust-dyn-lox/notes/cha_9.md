## Study Notes: Chapter 9: Control Flow

### 1\. Goal: Achieving Turing-Completeness

The primary goal of this chapter is to add **control flow** to Lox. This is the final piece of machinery needed to make our language **Turing-complete**, meaning it can solve the same class of problems as any other "real" programming language.

  * **Branching Control Flow:** Code that *might* not be executed (e.g., `if` statements, logical operators).
  * **Looping Control Flow:** Code that can be executed more than once (e.g., `while` and `for` loops).

-----

### 2\. Conditional Execution: `if` Statements

The `if` statement is the simplest form of branching.

  * **Grammar:**
    ```
    statement -> ... | ifStmt ;
    ifStmt    -> "if" "(" expression ")" statement
                 ( "else" statement )? ;
    ```
  * **AST Node:** `Stmt.If(Expr condition, Stmt thenBranch, Stmt elseBranch)`
      * `elseBranch` is `null` if there is no `else` clause.
  * **Interpretation (`visitIfStmt()`):**
    1.  The `condition` is evaluated.
    2.  The result is passed to `isTruthy()`.
    3.  If `true`, the `thenBranch` is executed.
    4.  If `false` and `elseBranch` is not `null`, the `elseBranch` is executed.

#### 2.1. Key Problem: The "Dangling Else"

A nested `if` statement creates an ambiguity:
`if (first) if (second) print "a"; else print "b";`

  * **The Ambiguity:** Does the `else` belong to `if (first)` or `if (second)`?
  * **The Solution:** Lox, like C and Java, resolves this by binding the `else` to the **nearest preceding `if`**.
  * **Implementation:** Our recursive descent parser handles this automatically. The `ifStatement()` parser eagerly looks for an `else` token after parsing the `then` statement, so the *innermost* `if` (in this case, `if (second)`) "claims" the `else` before the outer `if` gets a chance.

-----

### 3\. Logical Operators: `and` and `or`

These are considered control flow because they **short-circuit**. They don't necessarily evaluate both operands.

  * **Grammar:**
      * New, low-precedence rules are inserted between `assignment` and `equality`.
      * `or` has lower precedence than `and`.
      * `assignment -> logic_or ;`
      * `logic_or -> logic_and ( "or" logic_and )* ;`
      * `logic_and -> equality ( "and" equality )* ;`
  * **AST Node:** `Expr.Logical(Expr left, Token operator, Expr right)`
      * A new, separate node (not `Expr.Binary`) is created so that it gets its own `visitLogicalExpr()` method to handle the short-circuiting logic.
  * **Interpretation (`visitLogicalExpr()`):**
    1.  The `left` operand is evaluated first.
    2.  **Short-circuit logic:**
          * If the operator is **`or`**:
              * If `isTruthy(left)` is `true`, the `left` value is returned immediately (the `right` operand is **never** evaluated).
          * If the operator is **`and`**:
              * If `isTruthy(left)` is `false`, the `left` value is returned immediately (the `right` operand is **never** evaluated).
    3.  If the loop does not short-circuit, the `right` operand is evaluated and its value is returned.
  * **Key Semantic:** The operators return the *value of the operand* that determined the result, not necessarily a boolean `true` or `false`.
      * `"hi" or 2` evaluates to `"hi"`.
      * `nil and "yes"` evaluates to `nil`.

-----

### 4\. Looping: `while` Statements

The `while` loop is the simplest looping construct.

  * **Grammar:** `whileStmt -> "while" "(" expression ")" statement ;`
  * **AST Node:** `Stmt.While(Expr condition, Stmt body)`
  * **Interpretation (`visitWhileStmt()`):**
      * This maps directly to a Java `while` loop.
      * The interpreter loops as long as `isTruthy(evaluate(stmt.condition))` is `true`.
      * Inside the loop, it calls `execute(stmt.body)` on each iteration.

-----

### 5\. Looping: `for` Loops and Desugaring

Lox also supports C-style `for` loops, but they are implemented in a special way.

  * **Grammar:** `forStmt -> "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;`
      * This complex rule allows for an initializer (var declaration, expression, or empty), a condition (expression or empty), and an increment (expression or empty).

#### 5.1. Key Concept: Syntactic Sugar & Desugaring

  * **Syntactic Sugar:** A feature (like `for` loops) that makes a language sweeter to use but doesn't add any new expressive *power*. A `for` loop is just a more convenient way to write a `while` loop.
  * **Desugaring:** The process of translating this "sugar" syntax into its more primitive, underlying form.
  * **Implementation:** We will **not** create a `Stmt.For` AST node. Instead, the **parser** will perform this desugaring *while it is parsing*.

#### 5.2. The Desugaring Process

The `forStatement()` method in the `Parser` transforms this `for` loop:

```lox
for (var i = 0; i < 10; i = i + 1) {
  print i;
}
```

...into the *AST equivalent* of this `while` loop:

```lox
{
  var i = 0; // The initializer
  while (i < 10) { // The condition
    {
      print i;     // The original body
      i = i + 1; // The increment
    }
  }
}
```

  * **How it Works:** The `forStatement()` parser:
    1.  Parses the `initializer`, `condition`, `increment`, and `body` clauses into local `Stmt` and `Expr` variables.
    2.  Builds the new, desugared `Stmt` tree from the inside out:
    3.  If an `increment` exists, it's appended to the `body` inside a new `Stmt.Block`.
    4.  That `body` (with increment) is made the body of a new `Stmt.While` node, using the `condition`. (If the `condition` is `null`, it uses `new Expr.Literal(true)` to create an infinite loop).
    5.  If an `initializer` exists, the entire `Stmt.While` is wrapped in *another* `Stmt.Block` with the `initializer` statement coming first.
  * **Result:** The `Interpreter` class **is not changed at all**. It never knows a `for` loop existed; it only sees the `Stmt.Block` and `Stmt.While` nodes that the parser produced.

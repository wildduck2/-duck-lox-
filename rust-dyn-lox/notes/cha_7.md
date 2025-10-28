## Study Notes: Chapter 7: Evaluating Expressions

### 1. The Goal: An Interpreter That "Executes"

This is the chapter where our interpreter "comes alive." ⚡ The goal is to take the Abstract Syntax Tree (AST) from the parser and execute it. Since we only have expressions, "executing" means **evaluating** them to produce a final, single **value**.

---

### 2. Key Challenge 1: Representing Lox Values

How do we represent Lox's dynamically-typed values (numbers, strings, booleans, nil) inside our statically-typed Java interpreter?

* **The Problem:** A Lox variable can hold a `number`, then a `string`, then `nil`. A Java variable cannot.
* **The Solution:** We will use the Java "root" class, `java.lang.Object`, as the single static type for all Lox values.
* **Mapping:**
    * Lox `number` $\rightarrow$ Java `Double`
    * Lox `string` $\rightarrow$ Java `String`
    * Lox `Boolean` $\rightarrow$ Java `Boolean`
    * Lox `nil` $\rightarrow$ Java `null`
* **Dynamic Typing:** We will use Java's `instanceof` operator to check the *runtime* type of an `Object`. This is the core mechanism we'll use to implement Lox's dynamic typing.

---

### 3. Key Challenge 2: Organizing the Evaluation Code

How do we write the code to evaluate each `Expr` type?

* **The Strategy:** We will **not** use the "Interpreter Pattern" (i.e., adding an `interpret()` method to each `Expr` subclass). This would "smush" different concerns (like parsing, resolving, and interpreting) into the same files.
* **The Solution:** We will reuse the **Visitor Pattern** from Chapter 5.
* **Implementation:**
    1.  We create a new class: `Interpreter`.
    2.  This class implements `Expr.Visitor<Object>`.
    3.  The generic type `Object` signifies that every `visit...()` method will return a Lox value (represented as a Java `Object`).
    4.  Instead of building a string (like `AstPrinter`), the visit methods will perform computations and return the resulting value.

---

### 4. Implementation: Visiting the `Expr` Nodes

Evaluation is a **post-order traversal**: we evaluate an expression's children (operands) first, then we evaluate the expression itself.

* **`evaluate(Expr expr)`:** A simple helper method that just calls `expr.accept(this)`. This is the "recursive step" of the interpreter.
* **`visitLiteralExpr(Expr.Literal expr)`:** The base case. This is the simplest to evaluate: it just returns the value that the parser already stored (e.g., `123`, `"hello"`).
* **`visitGroupingExpr(Expr.Grouping expr)`:** Also simple. It just recursively calls `evaluate()` on the expression *inside* the parentheses.
* **`visitUnaryExpr(Expr.Unary expr)`:**
    1.  First, recursively `evaluate()` the right-hand operand.
    2.  `switch` on the operator:
        * **`-` (Negation):** Casts the operand to a `(double)` and negates it.
        * **`!` (Logical Not):** This introduces the concept of "truthiness."

---

### 5. Core Concept: Truthiness and Falsiness

The `!` operator (and later, `if` statements) needs to decide if a value is "true" or "false."

* **The Rule:** Lox follows Ruby's simple rule: **`nil` and `false` are "falsey."**
* **Everything Else:** All other values (numbers, strings, `true`, etc.) are "truthy."
* **Implementation:** This logic is encapsulated in a private helper method: `isTruthy(Object object)`.

---

### 6. Implementation: Binary Expressions

* **Evaluation Order:** The interpreter *first* calls `evaluate(expr.left)`, and *then* calls `evaluate(expr.right)`. This formally defines Lox's **left-to-right evaluation order**.
* **`visitBinaryExpr(Expr.Binary expr)`:** A `switch` on the operator:
    * **Arithmetic (`-`, `*`, `/`):** Evaluate both operands, cast them to `(double)`, and perform the math.
    * **Comparisons (`>`, `>=`, `<`, `<=`):** Evaluate both operands, cast them to `(double)`, perform the comparison, and return a `Boolean`.
    * **Equality (`==`, `!=`):** These operators work on *all* types.
        * Uses a helper `isEqual(Object a, Object b)`.
        * This helper correctly handles `null` (for `nil`) and then uses Java's built-in `.equals()` method.
    * **Special Case (`+`):** This operator is **overloaded**.
        * If *both* operands are `Double`s, it performs **addition**.
        * If *both* operands are `String`s, it performs **concatenation**.

---

### 7. Handling Runtime Errors

What happens if the user tries to run `-"muffin"`? Java will throw a `ClassCastException`, which is an ugly implementation detail we must hide.

* **The Problem:** A raw Java exception (like `ClassCastException`) crashes the interpreter and exposes its internal workings (Java) to the Lox user.
* **The Goal:** We must detect, report, and gracefully handle these errors as **Lox runtime errors**.
* **The Solution (A 3-Step Process):**
    1.  **Check Before Casting:** We add helper methods (e.g., `checkNumberOperand(Token, Object)`) that `instanceof` check an operand's type *before* we try to cast it.
    2.  **Throw Custom Exception:** If the type check fails, we `throw` a new, custom exception: `RuntimeError`.
    3.  **`RuntimeError` Class:** We create a new `RuntimeError` class that extends `RuntimeException` but also stores the `Token` that caused the error. This is crucial for reporting the **line number** of the error to the user.

---

### 8. Hooking Up the Interpreter

We wire the new `Interpreter` into the main `Lox.java` class.

* **Public API:** We add a public `interpret(Expr expression)` method to the `Interpreter` class.
* **Error Handling:** This `interpret` method wraps the call to `evaluate()` inside a `try...catch` block.
    * **On Success:** It prints the resulting value.
    * **On Failure:** It `catch`es the `RuntimeError` and passes it to `Lox.runtimeError()` to be reported.
* **`stringify(Object object)`:** A new helper method that converts a Lox value (as a Java `Object`) back into a user-friendly string (e.g., Java `null` becomes Lox `"nil"`, and `123.0` becomes `"123"`).
* **Exit Codes:** The `Lox` class now tracks a `hadRuntimeError` flag to exit with code `70` if a runtime error occurs in a script.

---

### 9. Design Note: Static and Dynamic Typing

* **The Topic:** Lox is **dynamically typed** (checks types at runtime), while Java is **statically typed** (checks types at compile time).
* **Key Insight:** This is a **continuum**, not a black-and-white choice.
* **The Proof:** Even "statically typed" languages like Java still perform many *runtime* type checks (e.g., `ClassCastException` on a downcast, or `ArrayStoreException` when putting the wrong type into an `Object[]` that is *really* an `Integer[]`).
* **The Trade-off:** Language designers (like Java's) often trade perfect static safety for more programmer flexibility (like covariant arrays).

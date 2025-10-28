## Study Notes: Chapter 10: Functions

### 1\. The Goal: User-Defined Functions

This chapter's goal is to implement a complete, working function system. This is the final and most complex piece required to make Lox a truly powerful, general-purpose language.

The implementation is broken into several parts:

1.  **Function Calls:** The syntax and interpretation for *calling* something.
2.  **Native Functions:** "Built-in" functions (like `clock()`) implemented in Java.
3.  **Function Declarations:** The syntax for *defining* new functions in Lox.
4.  **Function Objects:** The *runtime representation* of a Lox function.
5.  **Return Statements:** How functions send values back to the caller.
6.  **Closures:** Ensuring functions "remember" the variables from the scope where they were defined.

-----

### 2\. Function Call Syntax & Semantics

We start by implementing the ability to *call* a function, even before we can define one.

#### 2.1. Grammar and Parsing

  * **Syntax:** A function call is a postfix-style operation. The "callee" can be any expression, followed by parentheses: `any_expression(arguments)`.
  * **Precedence:** Function calls have the highest precedence, even higher than unary operators.
  * **Grammar:**
    ```
    unary -> ( "!" | "-" ) unary | call ;
    call  -> primary ( "(" arguments? ")" )* ;
    arguments -> expression ( "," expression )* ;
    ```
      * The `call` rule loops (`*`) to allow for chained calls like `get_fn()(1)`.
  * **AST Node:** `Expr.Call(Expr callee, Token paren, List<Expr> arguments)`
  * **Arity Limit:** The parser enforces a maximum of **255 arguments**. This is for compatibility with the C interpreter we'll build later.

#### 2.2. The `LoxCallable` Interface

To interpret a call, we need a way to represent "callable things" in Java.

  * **Key Concept:** We create a new interface, `LoxCallable`. Any Java object that implements this interface is something that Lox can "call."
  * **Interface Definition:**
    ```java
    interface LoxCallable {
      int arity();
      Object call(Interpreter interpreter, List<Object> arguments);
    }
    ```
      * `arity()`: Returns the number of arguments the function expects.
      * `call()`: Executes the function's logic and returns a value.

#### 2.3. Interpreting Calls (`visitCallExpr`)

This method is the "caller." It is responsible for:

1.  **Evaluating the Callee:** First, it evaluates the `callee` expression.
2.  **Evaluating Arguments:** It evaluates each argument expression *in order* (Lox defines a left-to-right evaluation order).
3.  **Type Checking:** It checks if the resulting callee object is an `instanceof LoxCallable`. If not (e.g., `"a string"()`), it throws a `RuntimeError`.
4.  **Arity Checking:** It compares the `arguments.size()` to the callee's `arity()`. If they don't match, it throws a `RuntimeError`.
5.  **Invoking the Call:** Finally, it invokes `callee.call()` and returns the result.

-----

### 3\. Native Functions

Before implementing *user-defined* functions, we add a **native function** (a function implemented in Java) to test our `LoxCallable` machinery.

  * **Purpose:** To provide core runtime services that Lox code cannot implement itself (e.g., file I/O, user input, time).
  * **Implementation (`clock()`):**
    1.  The `Interpreter` gets a `globals` environment field that *always* points to the global scope.
    2.  In the `Interpreter`'s constructor, we `define()` a new variable named `"clock"`.
    3.  The *value* of this variable is a new **Java anonymous class** that implements `LoxCallable`.
    4.  This anonymous class's methods are:
          * `arity()`: Returns `0`.
          * `call()`: Returns `(double)System.currentTimeMillis() / 1000.0`.

-----

### 4\. Function Declarations

Now, we allow users to define their own functions in Lox.

  * **Grammar:** This is a new type of declaration.
    ```
    declaration -> funDecl | varDecl | statement ;
    funDecl     -> "fun" function ;
    function    -> IDENTIFIER "(" parameters? ")" block ;
    parameters  -> IDENTIFIER ( "," IDENTIFIER )* ;
    ```
      * The `function` rule is separate so it can be reused later for class methods.
  * **AST Node:** `Stmt.Function(Token name, List<Token> params, List<Stmt> body)`

-----

### 5\. `LoxFunction`: The Runtime Object

When the interpreter *visits* a `Stmt.Function` node, it needs to create a runtime object.

  * **Implementation:** We create a new class, `LoxFunction`, that implements `LoxCallable`.
  * This class is a "wrapper" around the `Stmt.Function` syntax node.

#### 5.1. Implementing `LoxCallable`

  * **`arity()`:** Returns `declaration.params.size()`.
  * **`call()`:** This is the core of function execution.
    1.  **Create an Environment:** It creates a *new* `Environment` for the function's scope.
    2.  **Bind Parameters:** It loops through the `params` (from the `Stmt.Function`) and the `arguments` (from the `call` method) and `define()`s each parameter as a variable in the new environment.
    3.  **Execute Body:** It calls `interpreter.executeBlock(declaration.body, environment)`.
    4.  **Return:** It returns `null` (Lox `nil`). This will be updated.

#### 5.2. Interpreting Declarations (`visitFunctionStmt`)

This method is simple:

1.  It creates a new `LoxFunction` object, wrapping the `Stmt.Function` syntax node.
2.  It calls `environment.define()` to bind this new object to the function's name in the *current* environment.

-----

### 6\. `return` Statements

Functions need a way to send data back.

  * **Grammar:** `returnStmt -> "return" expression? ";" ;` (The return value is optional).
  * **AST Node:** `Stmt.Return(Token keyword, Expr value)`
      * `value` is `null` if the return value is omitted.

#### 6.1. Implementation: Exceptions for Control Flow

When a `return` is hit, we need to *immediately* stop executing and jump out of the function body, potentially unwinding past many nested statements.

  * **Key Concept:** We use **exceptions** as a control-flow mechanism.
  * **The `Return` Exception:** We create a new, lightweight exception class: `class Return extends RuntimeException`. It has one field: `final Object value;`. We disable JVM stack trace machinery for performance.
  * **`visitReturnStmt(Stmt.Return stmt)`:**
    1.  It evaluates `stmt.value` (or uses `null` if omitted).
    2.  It `throw`s a new `Return` exception, wrapping the value: `throw new Return(value);`.
  * **Updating `LoxFunction.call()`:** The `call` method is modified to *catch* this exception.
    ```java
    try {
      interpreter.executeBlock(declaration.body, environment);
    } catch (Return returnValue) {
      return returnValue.value; // This is the function's return value
    }
    return null; // Implicit return of nil
    ```

-----

### 7\. Closures: Functions that Remember

The final, and most advanced, feature.

#### 7.1. The Problem: Local & Nested Functions

Consider this code:

```lox
fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1; // 'i' is defined in the enclosing function
    print i;
  }
  return count;
}
var counter = makeCounter();
counter(); // Fails!
```

  * **Why it Fails:** The `count()` function's environment has `globals` as its parent. The `makeCounter` environment (where `i` lives) is lost and garbage collected after `makeCounter` returns.

#### 7.2. The Solution: Closures

A "closure" is a function that "closes over" and *remembers* the environment where it was **declared**.

  * **Key Concept:** A function object must store a reference to the environment that was active *when and where* it was defined.

#### 7.3. Implementation

Three small changes are needed:

1.  **`LoxFunction` gets a new field:**
      * `private final Environment closure;`
2.  **`LoxFunction`'s constructor is updated:**
      * It now takes the declaration *and* the environment: `LoxFunction(Stmt.Function declaration, Environment closure)` and stores the environment in its new field.
3.  **`visitFunctionStmt(Stmt.Function stmt)` is updated:**
      * When it creates the `LoxFunction`, it now "captures" the *current* environment:
        `LoxFunction function = new LoxFunction(stmt, environment);`
4.  **`LoxFunction.call()` is updated:**
      * When it creates the new environment for the function's body, it now uses the *captured closure* as the parent, not `globals`:
        `Environment environment = new Environment(closure);`

With this change, the environment chain is correct (`call` $\rightarrow$ `closure` $\rightarrow$ `globals`), and the `makeCounter` example works perfectly.

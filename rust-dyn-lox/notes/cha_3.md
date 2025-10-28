## Study Notes: Chapter 3 - The Lox Language

This chapter is a high-level, informal tour of Lox. It covers the language's core design, data types, and all of its major features.

### 1. Core Design Philosophy

* **Syntax:** C-family (e.g., semicolons, curly braces). This is chosen for **familiarity**, not elegance.
* **Type System:** Lox is **dynamically typed**.
    * Variables can hold values of any type.
    * Type errors (e.g., `"a" / 10`) are detected and reported at **runtime**.
    * This was chosen to keep the implementation simple for the book.
* **Memory:** Lox has **automatic memory management**.
    * It uses a **tracing garbage collector (GC)**, not reference counting (which has issues with cycles).
    * We will implement the GC ourselves in Part III.

---

### 2. Lox Data Types

Lox has four simple, built-in data types:

* **Booleans:** `true` and `false`.
* **Numbers:** Only *one* number type: **double-precision floating point** (a 64-bit `double`). This can represent both integers (`123`) and decimals (`12.34`).
* **Strings:** `"`Enclosed in double quotes`"`.
* **Nil:** `nil`. This special value represents "no value." It is the default for uninitialized variables.

---

### 3. Expressions (Produce a Value)

* **Arithmetic:** `+`, `-`, `*`, `/`.
    * The `+` operator can also be used to concatenate two strings.
    * The `-` operator can be a prefix (negation: `-5`) or an infix (subtraction: `10 - 5`).
* **Comparison:** `>`, `>=`, `<`, `<=`. These *only* work on numbers.
* **Equality:** `==` (equal) and `!=` (not equal).
    * These work on values of *any* type.
    * **Key Rule:** Values of different types are *never* considered equal (e.g., `123 == "123"` is **false**).
* **Logical:** `!`, `and`, `or`.
    * `!` (not) is a prefix operator that inverts a boolean.
    * `and` and `or` use "short-circuit" evaluation.
        * **`and`**: If the left side is false, it returns that value *without* evaluating the right side.
        * **`or`**: If the left side is true, it returns that value *without* evaluating the right side.
* **Grouping:** `( )` are used to control precedence.

---

### 4. Statements (Produce an Effect)

Statements are the "verbs" of the language; they perform an action and do not produce a value.

* **`print` Statement:** `print "Hello";`
    * This is a built-in statement, not a function.
* **Expression Statement:** Any valid expression followed by a semicolon. `1 + 2;`
* **Block:** `{ ... }`
    * A block groups zero or more statements.
    * Blocks also introduce a new scope.
* **`var` Statement:** `var myVar = "value";`
    * Declares a variable.
    * If no initializer is provided (`var myVar;`), the variable's value defaults to `nil`.
* **Control Flow:**
    * **`if`:** `if (condition) { ... } else { ... }`
    * **`while`:** `while (condition) { ... }`
    * **`for`:** A C-style for loop. `for (var i = 0; i < 10; i = i + 1) { ... }`

---

### 5. Functions and Closures

* **Definition:** `fun printSum(a, b) { print a + b; }`
* **Calling:** `printSum(1, 2);`
    * Parentheses are *mandatory* for a call, even with no arguments: `doSomething();`
* **Terminology:**
    * **Argument:** The *value* passed to a function (e.g., `1` and `2`).
    * **Parameter:** The *variable* that holds the value inside the function (e.g., `a` and `b`).
* **Return:** `return "value";`. If execution finishes without a `return`, the function implicitly returns `nil`.
* **First-Class Functions:** Functions are real values. They can be stored in variables and passed as arguments.
* **Closures:** A function can be declared inside another function. It "closes over" and "holds on" to variables from its surrounding scope, even after the outer function has returned.

---

### 6. Classes (Object-Oriented Programming)

Lox is also object-oriented. It uses a class-based system (like Java/C++) rather than a prototype-based one (like JavaScript).

* **Declaration:** `class Breakfast { ... }`
* **Methods:** Declared like functions inside the class, but without the `fun` keyword. `cook() { print "Eggs"; }`
* **Instances:** You create an instance by calling the class itself. `var myBreakfast = Breakfast();`
* **Fields:** State is stored in fields. You can create and access them on an instance. `myBreakfast.meat = "sausage";`
* **`this`:** Inside a method, `this` refers to the instance the method was called on.
* **`init()` (Initializer):** A method named `init` is the initializer (constructor). It's called automatically when an instance is created. `init(meat) { this.meat = meat; }`
* **Inheritance:** `class Brunch < Breakfast { ... }`
    * Uses the `<` operator.
    * A subclass (Brunch) inherits all methods from its superclass (Breakfast).
* **`super`:** `super.cook();`
    * Used inside a method to access the superclass's version of that method.

---

### 7. The (Tiny) Standard Library

The Lox standard library is almost non-existent. It only includes:

1.  The `print` statement (built-in).
2.  The `clock()` function (returns the number of seconds since the program started).

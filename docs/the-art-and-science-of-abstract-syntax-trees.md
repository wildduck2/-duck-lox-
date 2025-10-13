# The Art and Science of Abstract Syntax Trees: A Journey Through Language Implementation

## Table of Contents

**Part I: Foundations**
- Chapter 1: Understanding the Problem Space
- Chapter 2: What is an Abstract Syntax Tree?
- Chapter 3: Your First AST in Rust

**Part II: The Design Challenge**
- Chapter 4: The Expression Problem
- Chapter 5: Why Traditional Approaches Fall Short
- Chapter 6: Object-Oriented vs Functional Paradigms

**Part III: Solutions and Patterns**
- Chapter 7: The Visitor Pattern Explained
- Chapter 8: Implementing Visitors in Different Languages
- Chapter 9: Modern Approaches in Rust

**Part IV: Practical Implementation**
- Chapter 10: Building Your Interpreter
- Chapter 11: Advanced AST Techniques
- Chapter 12: Performance and Optimization

---

## Part I: Foundations

### Chapter 1: Understanding the Problem Space

When you set out to build a programming language, even a simple one, you're embarking on a journey that requires solving several interconnected puzzles. Imagine you're building a translator that needs to understand English sentences and convert them into actions. Before you can translate, you need to understand the structure of what you're reading. Is "I saw the man with the telescope" about you using a telescope to see a man, or about seeing a man who happens to have a telescope? The structure matters enormously.

This is precisely the challenge we face when building interpreters and compilers. When someone writes code like `result = (5 + 3) * 2`, we need to understand not just the individual symbols, but their relationships and the order in which operations should happen. This understanding needs to be represented in a form that our program can work with systematically.

The journey from raw text to executable code involves several stages. First, we scan the text and break it into tokens, which are the meaningful units like numbers, operators, and keywords. You've already got this represented in your code with the `Token` struct. But tokens alone aren't enough. They're like having a pile of puzzle pieces without knowing how they fit together. We need to understand the structure, the grammar, the relationships between these tokens. That's where Abstract Syntax Trees come in.

But before we dive into ASTs themselves, I want you to understand why they exist and what problem they solve. Consider the expression `2 + 3 * 4`. You immediately know this equals 14, not 20, because you understand that multiplication has higher precedence than addition. You're mentally grouping it as `2 + (3 * 4)`. Your brain has constructed a hierarchical understanding of this expression. An Abstract Syntax Tree is simply making that mental hierarchy explicit and concrete in a data structure.

### Chapter 2: What is an Abstract Syntax Tree?

An Abstract Syntax Tree, commonly abbreviated as AST, is a tree representation of the abstract syntactic structure of source code. Let me unpack that definition word by word, because each part is important.

First, it's a "tree." Trees are hierarchical data structures where you have a root node, and nodes can have children. Think of a family tree, or the folder structure on your computer. Each node in the tree represents something meaningful, and the parent-child relationships show how things are composed together.

Second, it represents "syntax." The syntax is the formal structure of the language, the rules about what can go where. It's not about the actual characters or tokens themselves, but about how they relate to each other according to the grammar of your language.

Third, it's "abstract." This is crucial. The tree doesn't include every single detail from the source code. It leaves out things like whitespace, comments, the exact placement of parentheses when they're just for grouping, and other syntactic details that don't affect the meaning. It focuses on what matters for understanding what the code actually does.

Let me give you a concrete example. Consider this simple arithmetic expression: `(5 + 3) * 2`. When we build an AST for this, we might end up with a tree that looks conceptually like this:

```
        *
       / \
      +   2
     / \
    5   3
```

The multiplication operator is at the root because it's the operation that happens last, the one that combines everything. Its left child is the addition operation, and its right child is the literal value 2. The addition operation, in turn, has two children: the literals 5 and 3.

Notice what's missing from this tree. The parentheses are gone. We don't need them in the tree because the tree structure itself shows the grouping. That's what makes it "abstract"—we've abstracted away the syntactic details and kept only the semantic meaning.

Now, let's look at your Rust code and see that you've already started building exactly this kind of structure:

```rust
pub enum Expr {
  Literal(Token),
  Unary {
    operator: Token,
    right: Box<Expr>,
  },
  Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
  },
  Grouping(Box<Expr>),
}
```

This is beautiful, elegant AST design. You're using Rust's enum feature to say "an expression can be one of several different things." Let me walk through each variant and explain what it represents.

The `Literal` variant represents the simplest kind of expression: a single value like a number, a string, or a boolean. It wraps a `Token`, which contains the actual value and information about where it came from in the source code.

The `Unary` variant represents operations that have one operand, like negation (`-5`) or logical NOT (`!true`). It stores the operator token and a boxed expression that represents what the operator applies to. We use `Box<Expr>` because Rust needs to know the size of types at compile time, and without the box, the size would be recursive and infinite.

The `Binary` variant is for operations with two operands, like addition, subtraction, multiplication, comparison, and so on. It has a left expression, an operator, and a right expression. This is how we represent that tree structure I showed you earlier. When you have `5 + 3`, the `Binary` variant would have `Literal(5)` as the left, a plus token as the operator, and `Literal(3)` as the right.

The `Grouping` variant is interesting. It represents an expression wrapped in parentheses. You might wonder why we need this if the tree structure already handles precedence. The reason is that during parsing, grouping helps us handle precedence correctly, and in some languages, you might want to preserve the information that the user explicitly grouped something, perhaps for error messages or for pretty-printing the code back out.

### Chapter 3: Your First AST in Rust

Now that we understand what an AST is conceptually, let's think about how you actually build one in practice. This process is called parsing, and it's one of the most intellectually satisfying parts of building a language.

Parsing takes your flat sequence of tokens and builds the hierarchical tree structure. There are many ways to do this, but for simple expression languages, a technique called "recursive descent parsing" works beautifully and is easy to understand.

The basic idea is that you write a function for each level of precedence in your grammar. Let's think about how we'd parse arithmetic expressions with addition and multiplication. We know multiplication has higher precedence, so we want it to bind more tightly. We might have functions like this:

```rust
// Handles the lowest precedence: addition and subtraction
fn parse_addition(&mut self) -> Result<Expr, ParseError> {
    // First, parse the left side (which might be a multiplication)
    let mut expr = self.parse_multiplication()?;
    
    // Then, while we see + or - operators, keep building Binary nodes
    while self.current_token_is_plus_or_minus() {
        let operator = self.advance_and_get_token();
        let right = self.parse_multiplication()?;
        expr = Expr::Binary {
            left: Box::new(expr),
            operator,
            right: Box::new(right),
        };
    }
    
    Ok(expr)
}

// Handles higher precedence: multiplication and division
fn parse_multiplication(&mut self) -> Result<Expr, ParseError> {
    // Parse the left side (which might be a unary expression)
    let mut expr = self.parse_unary()?;
    
    // While we see * or / operators, keep building Binary nodes
    while self.current_token_is_multiply_or_divide() {
        let operator = self.advance_and_get_token();
        let right = self.parse_unary()?;
        expr = Expr::Binary {
            left: Box::new(expr),
            operator,
            right: Box::new(right),
        };
    }
    
    Ok(expr)
}

// Handles unary operators like negation
fn parse_unary(&mut self) -> Result<Expr, ParseError> {
    if self.current_token_is_unary_operator() {
        let operator = self.advance_and_get_token();
        let right = self.parse_unary()?; // Recursive! Handles -(-5)
        return Ok(Expr::Unary {
            operator,
            right: Box::new(right),
        });
    }
    
    // If it's not a unary operator, try to parse a primary expression
    self.parse_primary()
}

// Handles the highest precedence: literals and grouped expressions
fn parse_primary(&mut self) -> Result<Expr, ParseError> {
    // If we see a number, string, or boolean, it's a literal
    if self.current_token_is_literal() {
        let token = self.advance_and_get_token();
        return Ok(Expr::Literal(token));
    }
    
    // If we see a left parenthesis, parse what's inside
    if self.current_token_is_left_paren() {
        self.advance(); // consume the '('
        let expr = self.parse_addition()?; // Parse the inside
        self.expect_right_paren()?; // make sure there's a ')'
        return Ok(Expr::Grouping(Box::new(expr)));
    }
    
    Err(ParseError::UnexpectedToken)
}
```

Notice the beautiful recursion here. When we're parsing addition, we call down to multiplication. When we're parsing multiplication, we call down to unary. When we're parsing unary, we might recursively call unary again to handle things like `--5`, or we call down to primary. And when we're parsing primary, if we find parentheses, we call all the way back up to addition to parse what's inside.

This recursive structure naturally builds the right tree shape. Let's trace through what happens when we parse `2 + 3 * 4`:

We start in `parse_addition`. It calls `parse_multiplication` to get the left side. In `parse_multiplication`, we call `parse_unary`, which calls `parse_primary`, which returns `Literal(2)`. Back in `parse_multiplication`, we don't see a multiply operator, so we just return that literal. Back in `parse_addition`, we now have our left side: `Literal(2)`.

Now we see a plus operator. We consume it and call `parse_multiplication` again to get the right side. This time, in `parse_multiplication`, we get `Literal(3)` from the primary, but then we DO see a multiply operator. We consume it, parse the right side (`Literal(4)`), and build a `Binary` node: `Binary { left: Literal(3), operator: *, right: Literal(4) }`.

Back in `parse_addition`, we now have everything we need. Our left is `Literal(2)`, our operator is plus, and our right is that whole multiplication subtree. We build the final Binary node that represents the complete expression.

This is the magic of recursive descent parsing. The recursion structure mirrors the precedence structure of your grammar, and the result is a correctly shaped AST that respects operator precedence.

## Part II: The Design Challenge

### Chapter 4: The Expression Problem

Now we arrive at one of the most profound and subtle problems in programming language design. You've built your AST. You have a beautiful tree structure representing your code. What now?

Well, you need to actually do things with these trees. In a complete language implementation, you might need to interpret the trees to execute the code, print them out in a readable format for debugging, check them for type errors, optimize them by transforming the tree into a better shape, or compile them into machine code. Each of these is a different operation you want to perform on the same underlying tree structure.

The question becomes: how do you organize this code? This might seem like a simple software engineering question, but it turns out to be surprisingly deep. The challenge is called the "expression problem," and it's been written about extensively in programming language research.

Let me paint a picture of the problem. Imagine you have a table. The rows of the table are the different types of expressions in your AST: Binary, Unary, Literal, Grouping, and perhaps others you'll add later. The columns are the different operations you want to perform: evaluate, print, type check, optimize, and so on. Each cell in this table needs a specific implementation.

For example, the cell at row "Binary" and column "evaluate" contains the code that knows how to evaluate a binary expression. It would look at the operator, recursively evaluate the left and right sides, then combine the results with the appropriate operation. The cell at row "Literal" and column "print" would contain code that knows how to convert a literal value into a nice string representation.

The expression problem asks: how do we organize our code so that it's easy to add both new rows (new expression types) and new columns (new operations) without having to modify large amounts of existing code?

Here's why this is tricky. Most programming paradigms naturally support extending in one direction but make the other direction painful.

In object-oriented languages like Java or even Rust with its methods, the natural grain of the code is along the rows. You define a class or type for each expression kind, and you put the methods for all operations on that type together. Your `Expr` enum might have methods like:

```rust
impl Expr {
    fn evaluate(&self) -> Value { ... }
    fn print(&self) -> String { ... }
    fn type_check(&self) -> Result<Type, Error> { ... }
}
```

Inside each method, you'd use pattern matching to handle each variant:

```rust
fn evaluate(&self) -> Value {
    match self {
        Expr::Literal(token) => { /* evaluate literal */ },
        Expr::Binary { left, operator, right } => { /* evaluate binary */ },
        Expr::Unary { operator, right } => { /* evaluate unary */ },
        Expr::Grouping(expr) => { /* evaluate grouping */ },
    }
}
```

This approach makes adding new operations relatively easy. You just add a new method to `impl Expr`. All the existing code still works. But what if you want to add a new expression type? Maybe you want to add function calls or array indexing. Now you have to find every single one of these methods and add a new match arm. If you forget even one place, your code is broken. And if you have many operations, that's a lot of places to update.

The Java example in your document shows an even worse version of this problem, where instead of pattern matching, you'd use a chain of `instanceof` checks. That's slow and ugly, but it's conceptually the same issue.

On the flip side, functional languages with pattern matching naturally organize code along the columns. You'd have separate functions for each operation, and each function pattern-matches on all the expression types:

```rust
// All the evaluation logic in one place
fn evaluate(expr: &Expr) -> Value {
    match expr {
        Expr::Literal(token) => { /* ... */ },
        Expr::Binary { left, operator, right } => { /* ... */ },
        Expr::Unary { operator, right } => { /* ... */ },
        Expr::Grouping(expr) => { /* ... */ },
    }
}

// All the printing logic in one place
fn print(expr: &Expr) -> String {
    match expr {
        Expr::Literal(token) => { /* ... */ },
        Expr::Binary { left, operator, right } => { /* ... */ },
        Expr::Unary { operator, right } => { /* ... */ },
        Expr::Grouping(expr) => { /* ... */ },
    }
}
```

This makes adding new operations trivial. Just write a new function. You don't have to touch any existing code. But adding a new expression type? Now you have to find every function that matches on `Expr` and add a new match arm. Again, if you forget one, your code is broken.

Neither approach is perfect. Both have a natural direction of extension and a painful direction of extension.

### Chapter 5: Why Traditional Approaches Fall Short

Let me dig deeper into why these traditional approaches cause problems in practice, especially as your language implementation grows.

Consider the object-oriented approach where we put all methods on the `Expr` type. In the beginning, this feels clean. You have your expression types, and each one knows how to do various things with itself. There's a nice conceptual unity: everything about Binary expressions is in one place.

But as your project grows, something uncomfortable starts to happen. Your `Expr` definition and its implementation start to become a giant dumping ground for code from wildly different concerns.

You have interpretation code that deals with runtime values, variable scopes, and execution flow. You have pretty-printing code that deals with formatting, indentation, and string building. You have type checking code that deals with type inference, constraint solving, and error reporting. You have optimization code that deals with constant folding, dead code elimination, and tree transformations.

Each of these concerns has its own vocabulary, its own helper types and functions, its own error handling patterns. But if you put all the methods on `Expr`, you're mushing them all together. Your `Expr` file imports types from the runtime system, the formatter, the type checker, and the optimizer. It becomes this massive, tangled mess where everything depends on everything else.

This violates a principle called "separation of concerns." The idea is that your code should be organized around conceptual boundaries. All the printing logic should live together because it shares common helpers and a common purpose. All the type checking logic should live together for the same reason. But when you put methods on `Expr`, you're forcibly organizing everything around the expression types instead of around these natural conceptual boundaries.

There's another practical problem too. As your team grows, multiple people want to work on different aspects of the language. One person is improving the interpreter, another is adding better error messages with prettier formatting, and a third is implementing a new optimization pass. With everything in the `Expr` methods, they're all editing the same file. You get merge conflicts. You get code review confusion where one person's changes to the interpreter accidentally break the pretty printer because they're all jumbled together.

Now let's consider the other extreme: separate functions for each operation, with pattern matching inside. This solves the separation of concerns problem beautifully. All your interpretation code is in one file or module. All your printing code is elsewhere. They don't tangle together.

But you've traded one problem for another. Now every time you add a new expression type, you're on a scavenger hunt through your codebase. You need to find every function that matches on `Expr` and add a new arm. And the compiler might not help you find all of them, depending on your language. You might think you've updated everything, ship your code, and then discover at runtime that you forgot to handle the new expression type in some obscure code path.

There's a subtler problem too. Sometimes the code for handling a particular expression type across different operations has something in common. Maybe Binary expressions always need to recursively process their left and right children, regardless of whether you're evaluating, printing, or type checking. With the functional approach, this commonality is scattered across multiple functions. You end up duplicating patterns, and if you want to change how Binary expressions work fundamentally, you have to hunt through all those functions.

### Chapter 6: Object-Oriented vs Functional Paradigms

The document you're reading introduces the expression problem by contrasting object-oriented and functional approaches to this problem. I want to take some time to really explore this contrast because it illuminates something deep about programming language design.

Object-oriented programming, at its heart, is about bundling data and behavior together. You create objects that know how to do things to themselves. The classic example is a `BankAccount` object with `deposit()` and `withdraw()` methods. The data (the balance) and the operations on that data live together.

This works beautifully when your operations are closely tied to the data and when you have a relatively stable set of operations but a growing number of types. In a graphical user interface, for example, you might have many different kinds of widgets (buttons, text boxes, drop-downs, sliders), but they all support a common set of operations (draw, handle mouse clicks, handle keyboard input). OOP lets you define a widget interface and then add new widgets by just creating new classes that implement that interface.

Functional programming, on the other hand, is about defining transformations on data. Data structures are often immutable and passive. Functions take data, examine its structure, and produce new data. The classic example is list processing: you have a list data structure, and you apply functions like map, filter, and fold to transform it.

This works beautifully when you have a relatively stable set of types but a growing number of operations. In a compiler for a simple language, you might have a fixed set of syntax tree node types, but you're constantly adding new analysis passes: dead code detection, constant propagation, common subexpression elimination, and so on. The functional approach lets you add each new pass as a new function without touching the existing code.

The expression problem arises because we're building something—an interpreter or compiler—that falls right in between these two sweet spots. We have multiple types (expression nodes) AND multiple operations (interpret, print, type check, optimize). We need to extend in both directions. Neither the pure OOP approach nor the pure functional approach feels natural.

This is why the document says that different languages have different "grain." The grain is the direction in which extension is easy. OOP languages have a grain that runs along types. Functional languages have a grain that runs along operations. When you're building something where you need to extend in the direction against the grain, you feel friction.

Rust is interesting because it's a bit of a hybrid. It has algebraic data types (enums) and pattern matching, which gives it a functional flavor. But it also has methods and trait implementations, which gives it an object-oriented flavor. This means you can choose your grain, but you still have to choose. You can't have it both ways at once.

Or can you? This is where design patterns come in, specifically the Visitor pattern.

## Part III: Solutions and Patterns

### Chapter 7: The Visitor Pattern Explained

The Visitor pattern is one of the most misunderstood patterns in software engineering, and I think that's because it's usually explained backwards. People start with the mechanics—the `accept` methods and the `visit` methods and the double dispatch—and it all seems like needless complexity. Let me try to explain it from first principles.

The core insight behind the Visitor pattern is this: what if we could have our cake and eat it too? What if we could organize code by operations (keeping all the interpretation code together, all the printing code together) while still getting the benefits of polymorphism and type safety?

The problem with the functional approach is that pattern matching happens at runtime. The compiler doesn't know which branch will be taken. This means it's harder to catch errors and harder for the compiler to optimize. The problem with the OOP approach is that methods are scattered across classes, making it hard to keep related operations together.

The Visitor pattern is a clever trick that uses polymorphism to achieve something that looks like pattern matching but happens at compile time. It gives us the organizational benefits of the functional approach while maintaining type safety.

Let me start with a simple example before we get to the abstract syntax trees. The document uses pastries—beignets and crullers—which is delightful. Let's stick with that.

You have two types of pastries. You want to define operations on them: maybe you want to cook them, eat them, or describe them. Here's how you'd normally do it in Java with methods:

```java
abstract class Pastry {
    abstract void cook();
    abstract void eat();
    abstract void describe();
}

class Beignet extends Pastry {
    void cook() { /* beignet cooking logic */ }
    void eat() { /* beignet eating logic */ }
    void describe() { /* beignet description */ }
}

class Cruller extends Pastry {
    void cook() { /* cruller cooking logic */ }
    void eat() { /* cruller eating logic */ }
    void describe() { /* cruller description */ }
}
```

Adding a new operation means editing both classes. The code for each operation is split across files.

Now let's use the Visitor pattern. First, we define an interface that represents an operation:

```java
interface PastryVisitor {
    void visitBeignet(Beignet beignet);
    void visitCruller(Cruller cruller);
}
```

Each operation we want to perform is a class that implements this interface:

```java
class CookingVisitor implements PastryVisitor {
    void visitBeignet(Beignet beignet) {
        // All the beignet cooking logic
    }
    
    void visitCruller(Cruller cruller) {
        // All the cruller cooking logic
    }
}

class EatingVisitor implements PastryVisitor {
    void visitBeignet(Beignet beignet) {
        // All the beignet eating logic
    }
    
    void visitCruller(Cruller cruller) {
        // All the cruller eating logic
    }
}
```

Now all the cooking logic lives together in one class. All the eating logic lives together in another class. We've achieved the functional organization pattern: operations are grouped together.

But how do we actually use this? Here's the clever part. We add one method to the pastry classes:

```java
abstract class Pastry {
    abstract void accept(PastryVisitor visitor);
}

class Beignet extends Pastry {
    void accept(PastryVisitor visitor) {
        visitor.visitBeignet(this);
    }
}

class Cruller extends Pastry {
    void accept(PastryVisitor visitor) {
        visitor.visitCruller(this);
    }
}
```

To perform an operation on a pastry, you call its `accept` method and pass in the visitor for that operation:

```java
Pastry myPastry = new Beignet();
PastryVisitor cooker = new CookingVisitor();
myPastry.accept(cooker);  // This cooks the beignet
```

Here's what happens under the hood, and this is the magic: When you call `myPastry.accept(cooker)`, polymorphism selects the right `accept` method based on the runtime type of `myPastry`. Since it's actually a `Beignet`, the `Beignet.accept` method runs. That method calls `visitor.visitBeignet(this)`. Now polymorphism selects the right `visitBeignet` method based on the runtime type of `visitor`. Since it's a `CookingVisitor`, the `CookingVisitor.visitBeignet` method runs.

We've used polymorphism twice—once on the pastry type and once on the visitor type. This is called "double dispatch." The net effect is that we've routed to the exact right method based on both the pastry type and the operation type, all at compile time.

The beauty is that adding a new operation is now easy: just create a new visitor class. You don't have to modify any of the pastry classes. And the code for each operation lives together in one place.

What about adding a new pastry type? That's still somewhat invasive—you need to add a new `visit` method to the visitor interface, which means updating all existing visitors. But this is actually less bad than it sounds, because the compiler will force you to implement that new method in every visitor. You can't forget it. And often, the implementations are simple or can use default behavior.

### Chapter 8: Implementing Visitors in Different Languages

The Java implementation of the Visitor pattern is quite verbose, as Java often is. Let me show you how this pattern looks in different languages, including Rust, to give you a feel for the variations.

In Java, you need the full machinery: interfaces, classes, explicit `accept` methods with double dispatch. The document you're reading shows code that generates all this boilerplate automatically, which tells you something about how tedious it is to write by hand.

In a language with pattern matching like ML, OCaml, or Haskell, you don't need the Visitor pattern at all. You just write functions with pattern matching, and the compiler ensures exhaustiveness. If you forget to handle a case, the compiler tells you. This is the functional approach working in its natural element.

Rust is interesting because it's in the middle. Rust has algebraic data types and exhaustive pattern matching, which means you get compiler help when you forget to handle a case. But Rust also has traits and trait objects, which give you some of the flexibility of object-oriented polymorphism.

For your AST, you could use the pattern matching approach:

```rust
pub fn evaluate(expr: &Expr) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Literal(token) => {
            // Convert the token to a runtime value
            Ok(token_to_value(token))
        }
        Expr::Unary { operator, right } => {
            // Recursively evaluate the right side
            let right_val = evaluate(right)?;
            // Apply the unary operator
            apply_unary_op(operator, right_val)
        }
        Expr::Binary { left, operator, right } => {
            // Recursively evaluate both sides
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            // Apply the binary operator
            apply_binary_op(operator, left_val, right_val)
        }
        Expr::Grouping(expr) => {
            // Just evaluate what's inside
            evaluate(expr)
        }
    }
}
```

This is clean and simple. All the evaluation logic is in one function. Pattern matching is exhaustive, so if you add a new `Expr` variant and forget to handle it here, the compiler will tell you.

But what if you want more flexibility? What if you want to be able to pass around evaluators as values, or swap between different evaluation strategies? Then you might want something more like the Visitor pattern. Here's how you might do it in Rust with traits:

```rust
// The visitor trait represents an operation on expressions
pub trait ExprVisitor {
    type Output;
    
    fn visit_literal(&mut self, token: &Token) -> Self::Output;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Output;
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output;
    fn visit_grouping(&mut self, expr: &Expr) -> Self::Output;
}

// Add an accept method to Expr
impl Expr {
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expr::Literal(token) => visitor.visit_literal(token),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
        }
    }
}

// Now we can implement different operations as different visitors
pub struct Evaluator {
    // Maybe some context or state here
}

impl ExprVisitor for Evaluator {
    type Output = Result<Value, RuntimeError>;
    
    fn visit_literal(&mut self, token: &Token) -> Self::Output {
        Ok(token_to_value(token))
    }
    
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Output {
        let right_val = right.accept(self)?;
        apply_unary_op(operator, right_val)
    }
    
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output {
        let left_val = left.accept(self)?;
        let right_val = right.accept(self)?;
        apply_binary_op(operator, left_val, right_val)
    }
    
    fn visit_grouping(&mut self, expr: &Expr) -> Self::Output {
        expr.accept(self)
    }
}

pub struct PrettyPrinter {
    // Maybe formatting options here
}

impl ExprVisitor for PrettyPrinter {
    type Output = String;
    
    fn visit_literal(&mut self, token: &Token) -> Self::Output {
        format!("{}", token.lexeme)
    }
    
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Output {
        let right_str = right.accept(self);
        format!("({}{})", operator.lexeme, right_str)
    }
    
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output {
        let left_str = left.accept(self);
        let right_str = right.accept(self);
        format!("({} {} {})", left_str, operator.lexeme, right_str)
    }
    
    fn visit_grouping(&mut self, expr: &Expr) -> Self::Output {
        let inner = expr.accept(self);
        format!("(group {})", inner)
    }
}
```

Now you have the best of both worlds. Each operation is a separate type implementing the `ExprVisitor` trait. All the code for that operation lives together. And you can pass visitors around as trait objects if needed, making your system very flexible.

The `accept` method on `Expr` does use pattern matching, but it's just doing mechanical dispatch. You write it once and never touch it again. All the actual logic lives in the visitor implementations.

### Chapter 9: Modern Approaches in Rust

Rust gives us unique capabilities that weren't available when the Gang of Four wrote Design Patterns in 1994. Let's explore how Rust's modern features let us approach the expression problem in ways that feel more natural while still maintaining the benefits we're after.

#### The Power of Exhaustive Matching

First, let's talk about why Rust's pattern matching is so powerful. In Java, if you use `instanceof` checks or a switch statement, the compiler doesn't know if you've handled all cases. You could easily forget one, and the bug won't show up until runtime. But in Rust, when you match on an enum, the compiler enforces exhaustiveness. If you don't handle every variant, your code won't compile.

This fundamentally changes the calculation about whether you need the Visitor pattern. In Java, the Visitor pattern gives you compile-time safety: if you add a new expression type, the compiler forces you to implement the new `visit` method in every visitor. Without it, you're relying on runtime checks and tests to catch missing cases.

In Rust, you get that same compile-time safety from pattern matching. If you add a new variant to `Expr` and you have functions that match on it, the compiler will tell you everywhere you need to add a new match arm. This is huge. It means you can use simple functions with pattern matching and still have confidence.

Here's what's interesting though: Rust's exhaustiveness checking is location-aware. When you add a new variant, the compiler tells you about every function in your current crate that matches on `Expr` and needs updating. But if someone else's code in another crate is matching on your `Expr` enum, their code breaks when they upgrade. This is actually a feature, not a bug—it prevents subtle runtime errors—but it means you need to think carefully about your public API.

#### The #[non_exhaustive] Attribute

Rust has an attribute called `#[non_exhaustive]` that you can put on enums. When you do this, code outside your crate can't exhaustively match on the enum. They have to include a wildcard pattern (`_ => ...`) to handle future variants. This is useful for library APIs where you want to reserve the right to add variants without breaking everyone's code.

But for an AST in an interpreter you're building, you probably don't want this. You want exhaustiveness checking. You want the compiler to force you to handle every case. This is one of the reasons why Rust is such a great language for building interpreters—the compiler becomes your assistant, making sure you don't forget anything.

#### Associated Types and Generic Visitors

Let's look more carefully at how we can use Rust's trait system to implement flexible visitors. The key insight is using associated types. In the example I showed earlier, the `ExprVisitor` trait has an associated type `Output`:

```rust
pub trait ExprVisitor {
    type Output;
    
    fn visit_literal(&mut self, token: &Token) -> Self::Output;
    // ... other methods
}
```

This is powerful because different visitors can produce different types. An interpreter visitor might return `Result<Value, RuntimeError>`. A pretty printer might return `String`. A type checker might return `Result<Type, TypeError>`. A tree transformer might return `Expr`, producing a new, modified tree.

The `accept` method can be generic over any visitor:

```rust
impl Expr {
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expr::Literal(token) => visitor.visit_literal(token),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
        }
    }
}
```

When you call `expr.accept(&mut evaluator)`, the Rust compiler monomorphizes this. It generates a specialized version of `accept` for `Evaluator`. This means zero runtime cost for the abstraction. It's as if you had written the pattern match directly in the evaluator code, but with the organizational benefits of the visitor pattern.

#### Visitor State and Context

One thing that's often glossed over in visitor pattern explanations is state. Real visitors often need to carry context with them. An interpreter might need a variable environment. A type checker might need a type context. A code generator might need symbol tables and label counters.

The visitor pattern handles this beautifully. The visitor is an object (a struct in Rust) that can have fields. You can store whatever state you need:

```rust
pub struct Interpreter {
    environment: Environment,
    globals: HashMap<String, Value>,
}

impl ExprVisitor for Interpreter {
    type Output = Result<Value, RuntimeError>;
    
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output {
        let left_val = left.accept(self)?;
        let right_val = right.accept(self)?;
        
        // We have access to self.environment and self.globals here
        // This is where we can look up variables, call functions, etc.
        
        apply_binary_op(operator, left_val, right_val)
    }
    
    // ... other methods
}
```

Notice that the visit methods take `&mut self`, so they can modify the interpreter's state. When you recursively call `left.accept(self)`, you're passing the same interpreter with its environment and globals. This creates a natural recursive traversal where state is threaded through the entire tree walk.

#### The Problem with Deep Recursion

There's a subtle issue with recursive tree traversal that you should be aware of. Every time you recursively call `accept`, you're adding a frame to the call stack. For most programs, this is fine. But if someone writes code with extremely deep nesting, you could overflow the stack.

Consider an expression like: `1 + (2 + (3 + (4 + (5 + ... ))))`. If this nesting is thousands of levels deep, your stack might overflow.

There are a few solutions:

1. **Ignore it**: For many use cases, this is fine. Real code doesn't usually nest thousands of levels deep. If someone writes code that deep, running out of stack is a reasonable failure mode.

2. **Iterative traversal with an explicit stack**: Instead of using the call stack, you can use an explicit stack data structure and iterate. This is more complex but handles arbitrary depth.

3. **Trampolining**: Return a thunk (a function to call later) instead of recursing directly. This lets you bounce back and forth between the caller and callee without growing the stack.

For a first implementation, I'd recommend ignoring the problem. You can always optimize later if it becomes an issue.

#### Combining Visitors with Other Patterns

The visitor pattern doesn't have to be used in isolation. You can combine it with other design patterns to solve more complex problems.

For example, you might want to transform an AST by walking it and building a new tree. You could have a `TransformerVisitor` where each visit method returns a new `Expr`:

```rust
pub trait TransformerVisitor {
    fn visit_literal(&mut self, token: &Token) -> Expr;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Expr;
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Expr;
    fn visit_grouping(&mut self, expr: &Expr) -> Expr;
}

pub struct ConstantFolder;

impl TransformerVisitor for ConstantFolder {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Expr {
        // First, recursively transform the children
        let new_left = left.accept_transformer(self);
        let new_right = right.accept_transformer(self);
        
        // If both sides are now literals and the operator is addition,
        // we can fold this into a single literal
        if let (Expr::Literal(left_tok), Expr::Literal(right_tok)) = (&new_left, &new_right) {
            if operator.token_type == TokenType::Plus {
                if let (Literal::Number(l), Literal::Number(r)) = (&left_tok.literal, &right_tok.literal) {
                    // Create a new literal token with the sum
                    return Expr::Literal(Token {
                        token_type: TokenType::Number,
                        lexeme: (l + r).to_string(),
                        literal: Literal::Number(l + r),
                        position: operator.position,
                    });
                }
            }
        }
        
        // Otherwise, return a new Binary node with the transformed children
        Expr::Binary {
            left: Box::new(new_left),
            operator: operator.clone(),
            right: Box::new(new_right),
        }
    }
    
    // ... other methods, most of which just reconstruct the node with transformed children
}
```

This pattern—transforming a tree by walking it and building a new tree—is extremely common in compilers. You might do multiple passes: constant folding, dead code elimination, inlining, etc. Each pass is a visitor that produces a new, optimized tree.

#### Builder Pattern with Visitors

Another useful combination is the builder pattern. Sometimes when you're walking a tree, you want to build up some complex result incrementally. For example, when generating code, you might be building up a list of instructions:

```rust
pub struct CodeGenerator {
    instructions: Vec<Instruction>,
    next_temp: usize,
}

impl CodeGenerator {
    fn allocate_temp(&mut self) -> TempRegister {
        let temp = TempRegister(self.next_temp);
        self.next_temp += 1;
        temp
    }
    
    fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

impl ExprVisitor for CodeGenerator {
    type Output = TempRegister;
    
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> TempRegister {
        // Generate code for the left side, getting the temp register where the result is stored
        let left_temp = left.accept(self);
        
        // Generate code for the right side
        let right_temp = right.accept(self);
        
        // Allocate a new temp for the result
        let result_temp = self.allocate_temp();
        
        // Emit an instruction to perform the operation
        match operator.token_type {
            TokenType::Plus => self.emit(Instruction::Add {
                dest: result_temp,
                left: left_temp,
                right: right_temp,
            }),
            TokenType::Minus => self.emit(Instruction::Sub {
                dest: result_temp,
                left: left_temp,
                right: right_temp,
            }),
            // ... other operators
            _ => panic!("Unsupported operator"),
        }
        
        result_temp
    }
    
    // ... other methods
}
```

After walking the entire tree, `self.instructions` contains the generated code in the correct order. The visitor pattern makes this natural because the visitor object carries the state (the instruction list) through the entire traversal.

#### When Not to Use Visitors

It's worth noting that the visitor pattern isn't always the right choice. Here are some situations where simpler approaches work better:

1. **Small projects**: If your AST has only a few node types and a few operations, and you're not expecting it to grow much, simple functions with pattern matching are clearer and more direct.

2. **Operations that don't fit the visitor shape**: Some operations need to walk the tree in non-standard ways. Maybe you're doing control flow analysis where you need to jump around the tree. Maybe you're doing backward analysis where you process children before parents. Shoehorning these into the visitor pattern can make them more complicated than necessary.

3. **When you control all the code**: If your AST and all operations on it live in the same crate, and you're the only developer, exhaustive pattern matching gives you everything you need. The visitor pattern adds indirection that might not be worth it.

4. **Performance-critical inner loops**: The visitor pattern adds some indirection and function call overhead. For most code, this is negligible and the compiler can often optimize it away. But if you have a performance-critical operation that's called millions of times per second, you might want to inline the pattern match directly.

That said, for medium to large interpreters and compilers, the visitor pattern (or something like it) is incredibly valuable. It gives you a clean way to separate concerns, makes your code more maintainable, and scales well as your language grows.

## Part IV: Practical Implementation

### Chapter 10: Building Your Interpreter

Now that we understand the theory, let's talk about how to actually build an interpreter for your language using the AST you've defined. This is where everything comes together.

An interpreter walks the AST and executes it directly, without compiling to machine code or any intermediate representation. This is simpler than building a compiler, and it's how many scripting languages like Python and Ruby work under the hood (though production implementations add layers of optimization).

#### Defining Runtime Values

The first thing you need is a representation of runtime values. When you evaluate an expression, what do you get? You need a `Value` type:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
```

This represents the possible values in your language at runtime. Your tokens have literals, but those are static data from the source code. Values are what you compute during execution.

You'll also need error types for things that can go wrong at runtime:

```rust
#[derive(Debug)]
pub enum RuntimeError {
    TypeError {
        expected: String,
        got: String,
        position: (u32, u32),
    },
    DivisionByZero {
        position: (u32, u32),
    },
    UndefinedVariable {
        name: String,
        position: (u32, u32),
    },
    // ... other error types
}
```

#### The Interpreter Structure

Your interpreter will be a struct that implements the visitor trait. It needs to carry runtime state—at minimum, an environment for variable bindings:

```rust
pub struct Interpreter {
    environment: Environment,
}

pub struct Environment {
    values: HashMap<String, Value>,
    // Later, you might add an `enclosing` field for nested scopes
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
    
    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        self.values.get(name)
            .cloned()
            .ok_or_else(|| RuntimeError::UndefinedVariable {
                name: name.to_string(),
                position: (0, 0), // You'd want to pass in the actual position
            })
    }
    
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(RuntimeError::UndefinedVariable {
                name: name.to_string(),
                position: (0, 0),
            })
        }
    }
}
```

#### Implementing the Visitor

Now we implement the visitor. Let's start with literals, which are straightforward:

```rust
impl ExprVisitor for Interpreter {
    type Output = Result<Value, RuntimeError>;
    
    fn visit_literal(&mut self, token: &Token) -> Self::Output {
        // Convert the token's literal to a runtime Value
        match &token.literal {
            Literal::Number(n) => Ok(Value::Number(*n)),
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Boolean(b) => Ok(Value::Boolean(*b)),
            Literal::Nil => Ok(Value::Nil),
        }
    }
    
    fn visit_grouping(&mut self, expr: &Expr) -> Self::Output {
        // Grouping just evaluates what's inside
        expr.accept(self)
    }
    
    // ... more methods to come
}
```

For unary expressions, we need to handle operators like negation and logical NOT:

```rust
fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Output {
    // First, evaluate the operand
    let right_val = right.accept(self)?;
    
    // Then apply the operator
    match operator.token_type {
        TokenType::Minus => {
            // Unary minus: negate a number
            match right_val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::TypeError {
                    expected: "number".to_string(),
                    got: format!("{:?}", right_val),
                    position: operator.position,
                }),
            }
        }
        TokenType::Bang => {
            // Logical NOT: negate a boolean (or "truthiness" of any value)
            Ok(Value::Boolean(!self.is_truthy(&right_val)))
        }
        _ => Err(RuntimeError::TypeError {
            expected: "unary operator".to_string(),
            got: operator.lexeme.clone(),
            position: operator.position,
        }),
    }
}
```

Notice the error handling. We use Rust's `?` operator to propagate errors up the call stack. If evaluating the right side fails, we immediately return that error. If we successfully evaluate it but it's the wrong type (like trying to negate a string), we return a type error.

The `is_truthy` helper method defines what counts as true and false in your language:

```rust
impl Interpreter {
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,  // Everything else is truthy
        }
    }
}
```

This is a design decision. In your language, should the number 0 be falsy (like in C) or truthy (like in Python)? Should empty strings be falsy? You get to decide.

#### Binary Operators

Binary operators are where things get interesting. You have arithmetic operators, comparison operators, and logical operators:

```rust
fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output {
    // Evaluate both operands
    let left_val = left.accept(self)?;
    let right_val = right.accept(self)?;
    
    // Apply the operator
    match operator.token_type {
        TokenType::Plus => {
            // Plus can add numbers or concatenate strings
            match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => {
                    Ok(Value::String(format!("{}{}", l, r)))
                }
                _ => Err(RuntimeError::TypeError {
                    expected: "two numbers or two strings".to_string(),
                    got: format!("{:?} and {:?}", left_val, right_val),
                    position: operator.position,
                }),
            }
        }
        TokenType::Minus => {
            self.expect_numbers(&left_val, &right_val, operator)?;
            if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                Ok(Value::Number(l - r))
            } else {
                unreachable!()
            }
        }
        TokenType::Star => {
            self.expect_numbers(&left_val, &right_val, operator)?;
            if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                Ok(Value::Number(l * r))
            } else {
                unreachable!()
            }
        }
        TokenType::Slash => {
            self.expect_numbers(&left_val, &right_val, operator)?;
            if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                if r == 0.0 {
                    return Err(RuntimeError::DivisionByZero {
                        position: operator.position,
                    });
                }
                Ok(Value::Number(l / r))
            } else {
                unreachable!()
            }
        }
        TokenType::Greater => {
            self.expect_numbers(&left_val, &right_val, operator)?;
            if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                Ok(Value::Boolean(l > r))
            } else {
                unreachable!()
            }
        }
        TokenType::GreaterEqual => {
            self.expect_numbers(&left_val, &right_val, operator)?;
            if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                Ok(Value::Boolean(l >= r))
            } else {
                unreachable!()
            }
        }
        TokenType::Less => {
            self.expect_numbers(&left_val, &right_val, operator)?;
            if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                Ok(Value::Boolean(l < r))
            } else {
                unreachable!()
            }
        }
        TokenType::LessEqual => {
            self.expect_numbers(&left_val, &right_val, operator)?;
            if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                Ok(Value::Boolean(l <= r))
            } else {
                unreachable!()
            }
        }
        TokenType::EqualEqual => {
            Ok(Value::Boolean(self.is_equal(&left_val, &right_val)))
        }
        TokenType::BangEqual => {
            Ok(Value::Boolean(!self.is_equal(&left_val, &right_val)))
        }
        _ => Err(RuntimeError::TypeError {
            expected: "binary operator".to_string(),
            got: operator.lexeme.clone(),
            position: operator.position,
        }),
    }
}
```

Helper methods make this cleaner:

```rust
impl Interpreter {
    fn expect_numbers(&self, left: &Value, right: &Value, operator: &Token) 
        -> Result<(), RuntimeError> {
        match (left, right) {
            (Value::Number(_), Value::Number(_)) => Ok(()),
            _ => Err(RuntimeError::TypeError {
                expected: "numbers".to_string(),
                got: format!("{:?} and {:?}", left, right),
                position: operator.position,
            }),
        }
    }
    
    fn is_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Nil, Value::Nil) => true,
            (Value::Nil, _) | (_, Value::Nil) => false,
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Boolean(l), Value::Boolean(r)) => l == r,
            _ => false,
        }
    }
}
```

#### Short-Circuit Evaluation

One thing to note: if you add logical AND and OR operators (`&&` and `||`), you need to implement short-circuit evaluation. That means you can't evaluate both sides upfront. Instead, you evaluate the left side, and only evaluate the right side if necessary:

```rust
TokenType::And => {
    // For AND, if left is false, don't evaluate right
    let left_val = left.accept(self)?;
    if !self.is_truthy(&left_val) {
        return Ok(left_val);
    }
    right.accept(self)
}
TokenType::Or => {
    // For OR, if left is true, don't evaluate right
    let left_val = left.accept(self)?;
    if self.is_truthy(&left_val) {
        return Ok(left_val);
    }
    right.accept(self)
}
```

This is important not just for efficiency, but for correctness. Consider: `x != 0 && 10 / x > 1`. If `x` is zero, we should never evaluate the right side because it would cause a division by zero error. Short-circuit evaluation prevents this.

### Chapter 11: Advanced AST Techniques

As your language grows, you'll discover that the simple expression AST we've been working with needs to be extended. Let's talk about some common patterns and techniques for more sophisticated AST designs.

#### Statements vs Expressions

So far we've only talked about expressions, which are pieces of code that evaluate to a value. But real programming languages also have statements, which are pieces of code that perform actions but don't necessarily produce values. Things like variable declarations, print statements, if statements, while loops, and so on.

The typical approach is to have two separate AST node types: one for expressions and one for statements:

```rust
#[derive(Debug)]
pub enum Stmt {
    Expression {
        expr: Expr,
    },
    Print {
        expr: Expr,
    },
    VarDeclaration {
        name: Token,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    // ... more statement types
}
```

Now you have two visitors: one for expressions and one for statements. The statement visitor can call the expression visitor when it needs to evaluate an expression:

```rust
impl StmtVisitor for Interpreter {
    type Output = Result<(), RuntimeError>;
    
    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Output {
        let value = expr.accept(self)?;  // Uses the ExprVisitor
        println!("{}", self.stringify(&value));
        Ok(())
    }
    
    fn visit_var_declaration(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::Output {
        let value = if let Some(init) = initializer {
            init.accept(self)?  // Uses the ExprVisitor
        } else {
            Value::Nil
        };
        
        self.environment.define(name.lexeme.clone(), value);
        Ok(())
    }
    
    // ... more methods
}
```

#### Handling Variables

When you add variables, you need to extend your expression enum to include variable references:

```rust
pub enum Expr {
    // ... existing variants
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
}
```

And you implement the visitor methods:

```rust
fn visit_variable(&mut self, name: &Token) -> Self::Output {
    self.environment.get(&name.lexeme)
}

fn visit_assign(&mut self, name: &Token, value: &Expr) -> Self::Output {
    let val = value.accept(self)?;
    self.environment.assign(&name.lexeme, val.clone())?;
    Ok(val)
}
```

One subtle issue: assignment is an expression in many languages, not a statement. In Python or C, you can write `x = y = 5`, which assigns 5 to both `x` and `y`. The assignment evaluates to the assigned value. That's why `visit_assign` returns the value.

#### Scoping and Environments

When you add block statements and functions, you need nested scopes. A variable defined inside a block should shadow variables with the same name in outer scopes, and should disappear when the block ends.

The standard way to handle this is to make environments nested:

```rust
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }
    
    pub fn with_enclosing(enclosing: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }
    
    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }
        
        if let Some(ref enclosing) = self.enclosing {
            return enclosing.get(name);
        }
        
        Err(RuntimeError::UndefinedVariable {
            name: name.to_string(),
            position: (0, 0),
        })
    }
    
    // Similar for assign: try this scope, then recurse to enclosing
}
```

When you enter a block, you create a new environment with the current one as its enclosing environment. When you exit the block, you restore the previous environment:

```rust
fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
    // Save the current environment and create a new nested one
    let previous = std::mem::replace(
        &mut self.environment,
        Environment::with_enclosing(self.environment.clone())
    );
    
    // Execute all statements in the block
    let result = statements.iter()
        .try_for_each(|stmt| stmt.accept(self));
    
    // Restore the previous environment
    self.environment = previous;
    
    result
}
```

This is a bit tricky because we need to restore the environment even if an error occurs. The code above handles that correctly because `try_for_each` stops on the first error, and we unconditionally restore the environment afterward.

#### Control Flow

Control flow constructs like `if`, `while`, and `return` add interesting challenges. Let's look at `if` statements first:

```rust
fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Box<Stmt>>) 
    -> Result<(), RuntimeError> {
    let condition_value = condition.accept(self)?;
    
    if self.is_truthy(&condition_value) {
        then_branch.accept(self)?;
    } else if let Some(else_stmt) = else_branch {
        else_stmt.accept(self)?;
    }
    
    Ok(())
}
```

This is straightforward. `while` loops are similar:

```rust
fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), RuntimeError> {
    while self.is_truthy(&condition.accept(self)?) {
        body.accept(self)?;
    }
    Ok(())
}
```

But `return` statements are trickier. When you execute a `return`, you need to immediately exit not just the current visitor method, but potentially many levels of nested function calls, all the way back to the function call site. The standard way to handle this is with an exception (or in Rust, a special error type):

```rust
#[derive(Debug)]
pub enum RuntimeError {
    // ... other error types
    Return {
        value: Value,
    },
}

fn visit_return_stmt(&mut self, value: &Option<Expr>) -> Result<(), RuntimeError> {
    let val = if let Some(expr) = value {
        expr.accept(self)?
    } else {
        Value::Nil
    };
    
    Err(RuntimeError::Return { value: val })
}
```

Then, when you call a function, you catch this special "error":

```rust
fn call_function(&mut self, function: &Function, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
    // Set up the function's environment, bind parameters, etc.
    
    // Execute the function body
    match self.execute_block(&function.body) {
        Ok(()) => Ok(Value::Nil),  // Function ended without explicit return
        Err(RuntimeError::Return { value }) => Ok(value),  // Function returned a value
        Err(e) => Err(e),  // Some other error occurred
    }
}
```

This uses Rust's error handling as a control flow mechanism, which is elegant even if it bends the "errors are for errors" philosophy slightly.

# CHAPTER 12: PERFORMANCE AND OPTIMIZATION

Now we arrive at what might seem like a premature concern—after all, shouldn't you get your interpreter working correctly before worrying about making it fast? That's absolutely true, and I want to be clear that premature optimization is a real danger. You should always profile before optimizing, and you should always optimize the hottest paths first, not what you guess might be slow.

That said, understanding performance characteristics helps you make good architectural decisions from the beginning. Some optimizations are easy to add later, but others require fundamental changes to your design. Let me walk you through the landscape of interpreter performance, from the basic concerns you should think about early, to the sophisticated optimizations you might add later.

## Understanding Interpreter Performance

When people say "interpreters are slow," what they really mean is that tree-walking interpreters—the kind we've been building—have certain inherent performance characteristics that make them slower than compiled code. Let me explain why, because understanding the problem is the first step toward addressing it.

Every time you evaluate an expression in a tree-walking interpreter, you're doing a tremendous amount of work that feels wasteful. Consider a simple expression like `a + b`. In compiled code, this might become just a few machine instructions: load the value of `a` into a register, load the value of `b` into another register, add them, and store the result. Three instructions, maybe a dozen CPU cycles.

But in your tree-walking interpreter, here's what happens. You have a `Binary` node in your AST. To evaluate it, you call the visitor's `visit_binary` method. That method recursively evaluates the left child by calling `accept`, which dispatches to the appropriate visitor method based on the node type—let's say it's a `Variable` node, so we call `visit_variable`. That method looks up the variable name in the environment, which involves a hash table lookup, string comparison, and possibly walking up the chain of enclosing environments. Then we do the whole thing again for the right child. Then we have to match on the operator type to figure out what operation to perform. Then we have to check the types of the operands to make sure they're both numbers. Then finally we can add them.

Count the function calls, the memory allocations, the dynamic dispatches, the type checks. What should be three machine instructions has become dozens of function calls and hundreds of instructions. This overhead is present at every single node in your AST. When you're executing a tight loop that runs a million times, this overhead multiplies and becomes crushing.

There's another, more subtle problem. Modern CPUs are incredibly sophisticated. They can execute multiple instructions in parallel, they predict which way branches will go, they prefetch data from memory before it's needed. But they can only do these tricks when the code is predictable and when the data access patterns are regular. Tree-walking interpreters are the opposite of predictable. Every node might be a different type, requiring different code paths. Every lookup might hit different locations in memory. The CPU's branch predictor gets confused, the prefetcher doesn't know what to fetch, and the instruction pipeline stalls. Your code is fighting against the CPU instead of working with it.

## Low-Hanging Fruit: Simple Optimizations

Before we talk about sophisticated compilation techniques, let's talk about simple optimizations that can make a meaningful difference without changing your overall architecture. These are the kinds of things you might add after your interpreter is working and you've profiled it to find bottlenecks.

### Interning Strings

One common source of overhead is string operations. Every time you look up a variable, you're comparing strings. Every time you define a new variable, you're cloning strings and inserting them into hash maps. String operations are slow, especially if your strings are long.

String interning is a technique where you ensure that there's only ever one copy of each unique string in memory. Instead of passing strings around, you pass small integer identifiers that represent those strings. Comparing identifiers is much faster than comparing strings—it's just comparing two numbers.

Here's how you might implement this:

```rust
pub struct StringInterner {
    strings: Vec<String>,
    indices: HashMap<String, usize>,
}

impl StringInterner {
    pub fn new() -> Self {
        StringInterner {
            strings: Vec::new(),
            indices: HashMap::new(),
        }
    }
    
    pub fn intern(&mut self, string: String) -> StringId {
        // If we've already seen this string, return its ID
        if let Some(&id) = self.indices.get(&string) {
            return StringId(id);
        }
        
        // Otherwise, add it to our collection
        let id = self.strings.len();
        self.strings.push(string.clone());
        self.indices.insert(string, id);
        StringId(id)
    }
    
    pub fn resolve(&self, id: StringId) -> &str {
        &self.strings[id.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId(usize);
```

Now your tokens store `StringId` instead of `String` for identifiers and string literals. Your environment uses `StringId` as keys instead of strings. Lookups become integer comparisons, which are lightning fast. This single change can make variable lookups several times faster.

### Caching Environment Lookups

Another common bottleneck is variable lookups, especially in nested scopes. If you have deeply nested scopes and you're frequently accessing variables from outer scopes, you're walking that chain of environments over and over again.

A technique called "scope resolution" or "static distance" can eliminate this overhead. The idea is that during a separate analysis pass before interpretation, you walk the AST and figure out exactly how many scopes up each variable reference needs to look. You record this distance, and then at runtime, instead of searching by name, you just index directly into the right environment at the right depth.

This requires a resolver pass that builds a table mapping each variable use to its declaration:

```rust
pub struct Resolver {
    scopes: Vec<HashMap<StringId, bool>>, // Track which variables are declared in each scope
    locals: HashMap<*const Expr, usize>,  // Map each variable expression to its scope depth
}

impl Resolver {
    fn resolve_local(&mut self, expr: &Expr, name: StringId) {
        // Walk backwards through scopes to find where this variable is declared
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name) {
                // Record that this expression needs to look i scopes up
                self.locals.insert(expr as *const Expr, i);
                return;
            }
        }
        // If we didn't find it, it's either global or undefined
    }
}
```

Then at interpretation time, instead of calling `environment.get(name)`, you call `environment.get_at(depth)`, which directly indexes into the right scope without any searching. This can dramatically speed up code that accesses outer variables frequently.

### Reducing Allocations

Every time you create a boxed expression node, you're allocating heap memory. Every time you clone a value, you might be allocating. Allocations are expensive—they involve acquiring locks, finding free memory, and potentially asking the operating system for more memory.

One optimization is to use an arena allocator for your AST nodes. Instead of boxing each node individually, you allocate them all in one big contiguous block of memory. This makes allocation much faster and improves cache locality because related nodes are stored near each other in memory.

Rust has crates like `bumpalo` or `typed-arena` that make this easy:

```rust
use typed_arena::Arena;

pub struct Parser<'arena> {
    arena: &'arena Arena<Expr>,
    // ... other fields
}

impl<'arena> Parser<'arena> {
    fn parse_binary(&mut self, ...) -> &'arena Expr {
        let left = self.parse_primary();
        let right = self.parse_primary();
        
        // Allocate in the arena instead of boxing
        self.arena.alloc(Expr::Binary {
            left,
            operator,
            right,
        })
    }
}
```

Now all your expressions are in one arena, and when you're done with the entire AST, you can free the whole arena at once. This is much faster than freeing each box individually.

For runtime values, you might also want to avoid cloning when possible. Instead of storing `Value` directly, you could use reference-counted values:

```rust
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(Rc<String>),  // Reference-counted string
    Boolean(bool),
    Nil,
}
```

Now cloning a string value just increments a reference count instead of copying the entire string. This is especially beneficial for large strings or if you're passing values around frequently.

## Bytecode Compilation: The Next Level

If you profile your interpreter and find that the overhead of tree-walking is really killing you, the next step is to compile to bytecode. This is what languages like Python, Ruby, and Lua do. You transform your AST into a linear sequence of simple instructions, and then you interpret those instructions with a much tighter, faster loop.

The key insight is that bytecode eliminates the tree-walking overhead. Instead of recursively traversing an AST with virtual method calls and dynamic dispatch, you have a simple loop that fetches the next instruction and executes it. Each instruction is just a byte or two, and you can use a giant switch statement or jump table to dispatch to the right handler. This is much more CPU-friendly.

Here's what bytecode might look like for `(5 + 3) * 2`:

```
LOAD_CONSTANT 0    // Push 5 onto the stack
LOAD_CONSTANT 1    // Push 3 onto the stack
ADD                // Pop two values, add them, push result
LOAD_CONSTANT 2    // Push 2 onto the stack
MULTIPLY           // Pop two values, multiply them, push result
```

Your bytecode interpreter has a stack where it pushes and pops values, and it has an instruction pointer that walks through this linear sequence of instructions. The execution loop looks something like this:

```rust
pub struct VM {
    stack: Vec<Value>,
    chunk: Chunk,  // The bytecode instructions
    ip: usize,     // Instruction pointer
}

impl VM {
    pub fn run(&mut self) -> Result<Value, RuntimeError> {
        loop {
            let instruction = self.chunk.code[self.ip];
            self.ip += 1;
            
            match instruction {
                OpCode::LoadConstant => {
                    let constant_index = self.chunk.code[self.ip];
                    self.ip += 1;
                    let value = self.chunk.constants[constant_index as usize].clone();
                    self.stack.push(value);
                }
                OpCode::Add => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    let result = self.add_values(left, right)?;
                    self.stack.push(result);
                }
                OpCode::Multiply => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    let result = self.multiply_values(left, right)?;
                    self.stack.push(result);
                }
                OpCode::Return => {
                    return Ok(self.stack.pop().unwrap());
                }
                // ... many more instruction types
            }
        }
    }
}
```

This is much faster than tree-walking because the inner loop is tight, predictable, and cache-friendly. The CPU can fetch the next instruction while executing the current one. The branch predictor learns the common patterns. The code and data are both laid out linearly in memory, which the prefetcher loves.

Converting your AST to bytecode requires a compiler pass. You walk your AST and emit instructions:

```rust
pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Literal(token) => {
                // Add the constant to our constant pool and emit an instruction to load it
                let constant_index = self.chunk.add_constant(token_to_value(token));
                self.emit_byte(OpCode::LoadConstant as u8);
                self.emit_byte(constant_index as u8);
            }
            Expr::Binary { left, operator, right } => {
                // Compile the left side (which will leave its result on the stack)
                self.compile_expr(left)?;
                // Compile the right side (which will leave its result on the stack)
                self.compile_expr(right)?;
                // Emit the appropriate operator instruction
                match operator.token_type {
                    TokenType::Plus => self.emit_byte(OpCode::Add as u8),
                    TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
                    TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
                    TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
                    // ... other operators
                }
            }
            // ... other expression types
        }
        Ok(())
    }
}
```

Notice how the recursive structure of the AST naturally produces the right bytecode sequence. Compiling a binary expression means compiling the left side, compiling the right side, and then emitting the operator. The evaluation order is preserved in the bytecode.

## Advanced Optimizations: When You Need Even More Speed

Even with bytecode, you might hit performance walls, especially if you're running computationally intensive code. At this point, you're venturing into serious compiler territory. These techniques are complex and usually only worth it if you're building a production language implementation, but they're fascinating to understand.

### Just-In-Time Compilation

Just-in-time compilation, or JIT, is what makes modern JavaScript engines like V8 so fast. The idea is that you start by interpreting bytecode normally, but you profile the code as it runs. When you detect that a particular function or loop is being executed frequently—a "hot" code path—you compile that bytecode to native machine code on the fly.

The challenge is doing this efficiently. Compiling to machine code takes time, so you only want to do it for code that will run enough times to amortize that cost. And the compilation itself needs to be fast, which means you can't do the kinds of slow, sophisticated optimizations that ahead-of-time compilers do.

The typical approach is tiered compilation. You start with a simple interpreter. If code gets warm, you compile it with a fast, simple compiler that does minimal optimization. If code gets hot, you recompile it with a slower but more sophisticated optimizing compiler. Languages like Java (with its C1 and C2 compilers) and JavaScript (with multiple tiers in V8) use this approach.

### Type Specialization

One of the big sources of overhead in dynamic languages is type checking. Every operation has to check what types its operands are and dispatch to the right code. If you're adding two numbers in a tight loop, you're doing that type check on every iteration even though the types never change.

Type specialization is the technique of generating different versions of code for different type combinations. When you JIT-compile a hot function and you notice that a particular variable is always a number, you can generate specialized code that assumes it's a number and doesn't do the type check. If your assumption turns out to be wrong later, you fall back to the unspecialized version or recompile with different assumptions.

This is how modern JavaScript engines can make code that operates on numbers almost as fast as statically-typed code. They speculate about types, generate specialized code based on those speculations, and guard the specialized code with quick checks that their assumptions still hold.

### Inline Caching

Another powerful technique is inline caching. When you access a property on an object, you typically have to do a hash table lookup. But in practice, the same property access in your code often hits the same type of object over and over. Inline caching takes advantage of this by caching the result of the lookup.

The first time you execute `obj.x`, you do the full lookup and figure out where property `x` is stored in objects of this type. Then you cache that location right there in the code. The next time you execute the same line, you first check if the object is the same type you saw last time. If it is, you use the cached location directly, skipping the hash table lookup entirely. This turns an expensive dynamic property access into a quick type check and a direct memory access.

When objects of different types flow through the same code, you can extend this to polymorphic inline caches that remember the locations for several different types, checking each one in turn.

### Escape Analysis and Stack Allocation

In most interpreters, objects and values are allocated on the heap because you don't know their lifetime statically. But heap allocation is expensive. Escape analysis is a compiler technique that determines whether a value ever "escapes" the current function—whether a reference to it is stored somewhere that outlives the function call. If a value doesn't escape, you can allocate it on the stack instead, which is much faster and automatically freed when the function returns.

For example, if you create a temporary object in a loop, use it to compute something, and then discard it, escape analysis might determine that the object never escapes and allocate it on the stack or even just keep it in registers.

## The Wisdom of Knowing When to Stop

I want to end this chapter with a word of caution. Everything I've described in the advanced section—JIT compilation, type specialization, inline caching—represents thousands of hours of engineering work by expert compiler writers. These techniques are well understood in academia and industry, but they're complex to implement correctly and hard to debug when they go wrong.

For most projects, you should stop well before this point. A carefully implemented tree-walking interpreter or bytecode interpreter is perfectly adequate for many use cases. Scripting languages, configuration languages, embedded languages, domain-specific languages—these often don't need to be blazingly fast. Correctness, simplicity, and good error messages matter more than raw speed.

Even when performance does matter, simple optimizations can get you surprisingly far. Interning strings, caching lookups, reducing allocations, and using better data structures can speed up an interpreter by factors of two or three without fundamental changes to the architecture. Switching to bytecode can give you another factor of five or ten. Only after exhausting these simpler approaches should you even consider JIT compilation and the associated complexity.

Remember that performance is not just about raw execution speed. Startup time matters—a JIT compiler that takes seconds to warm up might be slower than a simple interpreter for short-running scripts. Memory usage matters—native code takes more memory than bytecode. Debuggability matters—JIT-compiled code is harder to step through in a debugger. Portability matters—native code generation requires different code for each CPU architecture.

The right approach depends on your goals. If you're building an educational language to teach programming concepts, a simple tree-walking interpreter is perfect—it's easy to understand and easy to explain. If you're building a scripting language for a game engine, a bytecode interpreter might be the sweet spot—it's fast enough and doesn't complicate your build process. If you're building a general-purpose language that you hope will compete with Python or JavaScript, then yes, you'll eventually need JIT compilation and sophisticated optimizations, but you should still start simple and add complexity only as needed.

The most important lesson from this chapter is to measure before you optimize. Don't assume you know what's slow. Use a profiler. Find the actual bottlenecks. Often you'll discover that the performance problem is not where you expected it to be. Maybe it's not the interpreter at all—maybe it's that your parser is doing too much work, or your standard library functions are inefficiently implemented, or you're doing unnecessary work in your algorithms.

Build it, measure it, understand it, and then—only then—optimize what matters.

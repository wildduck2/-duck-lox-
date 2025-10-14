# METHOD DISPATCH, OVERLOADING, AND OVERRIDING: WHEN THE COMPILER KNOWS AND WHEN IT DOESN'T

## The Fundamental Question: Which Code Runs?

Imagine you're writing a graphics program. You have different shapes—circles, rectangles, triangles. Each shape knows how to draw itself. You write code like this:

```rust
shape.draw();
```

That single line of code poses a question that might seem trivial but is actually profound: which `draw` method should run? Is it the circle's drawing code? The rectangle's? The triangle's? And here's the crucial follow-up question: when do we figure this out? Do we know at compile time, when we're translating your source code into something executable? Or do we only figure it out at runtime, when the program is actually running and we can see what kind of shape we actually have?

This question—when we know which code to run—fundamentally shapes how programming languages work, how they perform, and what kinds of errors they can catch for you before your code ever runs.

## Overloading: When the Compiler Knows Everything

Let's start with the simpler case: overloading. This is when you have multiple functions or methods with the same name but different parameter types. The classic example comes from languages like Java, C++, or even modern languages like Swift and Kotlin. Here's what it looks like:

```java
class Calculator {
    int add(int a, int b) {
        return a + b;
    }
    
    double add(double a, double b) {
        return a + b;
    }
    
    String add(String a, String b) {
        return a + b;
    }
}
```

We have three different methods, all called `add`, but they work on different types. When you write code like `calculator.add(5, 3)`, the compiler looks at the arguments—they're integers—and says "aha, you must mean the integer version of add." If you write `calculator.add(5.5, 3.2)`, it picks the double version. If you write `calculator.add("hello", "world")`, it picks the string version.

The absolutely critical thing to understand about overloading is that this decision happens at compile time. The compiler knows the types of your arguments, or can figure them out through type inference, and it uses those types to decide which method you're calling. By the time your program runs, the decision has already been made. The compiled code doesn't say "call add and figure out which one at runtime." It says "call this specific add function at this specific address in memory."

This is sometimes called "static dispatch" or "early binding." Static because the binding between the method name and the actual code happens statically, before the program runs. Early because it happens early in the compilation process.

Let me show you what this means in practice. When you write this Java code:

```java
Calculator calc = new Calculator();
int result = calc.add(5, 3);
double result2 = calc.add(5.5, 3.2);
```

The compiler transforms it into something more like this conceptually:

```java
Calculator calc = new Calculator();
int result = Calculator_add_int_int(calc, 5, 3);
double result2 = Calculator_add_double_double(calc, 5.5, 3.2);
```

The method names get "mangled" to encode the parameter types, and the compiler generates direct calls to these specific functions. There's no ambiguity, no decision-making at runtime. The CPU just jumps directly to the right piece of code.

This has beautiful implications for performance. There's zero overhead for method dispatch. The CPU can inline the function if it wants to, it can optimize aggressively because it knows exactly what's being called. And you get safety too—if you try to call `add` with incompatible arguments, like `add(5, "hello")`, the compiler catches this mistake immediately. You don't have to run your program and hope you hit that code path to discover the bug.

But overloading has a limitation. The compiler can only make its decision based on information it has at compile time. It looks at the static types of your arguments—the types the variables are declared with, not necessarily what's actually in those variables at runtime. This is fine for simple cases but becomes limiting when you want polymorphism.

## Overriding: When We Need Runtime Flexibility

Now let's talk about overriding, which is a completely different beast. Overriding is what happens when you have inheritance and a subclass provides its own implementation of a method defined in a parent class. This is the heart of object-oriented polymorphism.

Here's a classic example:

```java
abstract class Shape {
    abstract double area();
    abstract void draw();
}

class Circle extends Shape {
    private double radius;
    
    Circle(double radius) {
        this.radius = radius;
    }
    
    double area() {
        return Math.PI * radius * radius;
    }
    
    void draw() {
        System.out.println("Drawing a circle with radius " + radius);
    }
}

class Rectangle extends Shape {
    private double width;
    private double height;
    
    Rectangle(double width, double height) {
        this.width = width;
        this.height = height;
    }
    
    double area() {
        return width * height;
    }
    
    void draw() {
        System.out.println("Drawing a rectangle " + width + "x" + height);
    }
}
```

Both `Circle` and `Rectangle` override the `area` and `draw` methods from `Shape`. Now here's where it gets interesting. Consider this code:

```java
Shape shape = getRandomShape(); // Might return a Circle or Rectangle
double area = shape.area();
shape.draw();
```

Look at that second line carefully. We're calling `area()` on a variable of type `Shape`. But which `area` method runs? Is it the circle's area calculation or the rectangle's? The compiler can't know because the actual object stored in `shape` depends on what `getRandomShape()` returns, and that might depend on user input, random numbers, database queries, or any number of things that can't possibly be known until the program runs.

This is where we need dynamic dispatch, also called late binding. The decision about which method to call has to be made at runtime, based on the actual type of the object, not the declared type of the variable. When your program runs and executes `shape.area()`, it looks at the actual object stored in `shape`, discovers it's a Circle, and calls the Circle's area method. If it's a Rectangle, it calls the Rectangle's area method.

This is the power of polymorphism. You can write code that works with the abstract `Shape` type, and that same code automatically does the right thing for circles, rectangles, triangles, or any other shape you add later. You don't need to check the type yourself or write a big switch statement. The language handles it for you.

## How Dynamic Dispatch Actually Works: Virtual Method Tables

But how does this magic happen? How does the computer know, at runtime, which method to call? The answer is one of the most elegant data structures in computer science: the virtual method table, or vtable for short.

When you create a class with virtual methods (methods that can be overridden), the compiler creates a table that lists all the virtual methods for that class and their addresses in memory. Every object of that class contains a hidden pointer to its class's vtable. When you call a virtual method, the compiled code does something like this:

```
1. Look at the object
2. Follow its vtable pointer to find its class's method table
3. Look up the method in that table (usually by a fixed offset)
4. Jump to the address you found
```

Let me show you what this looks like in practice. When the compiler sees your Shape hierarchy, it creates vtables that might look conceptually like this:

```
Shape vtable:
  [0] area    -> (abstract, no implementation)
  [1] draw    -> (abstract, no implementation)

Circle vtable:
  [0] area    -> address of Circle.area()
  [1] draw    -> address of Circle.draw()

Rectangle vtable:
  [0] area    -> address of Rectangle.area()
  [1] draw    -> address of Rectangle.draw()
```

When you create a Circle object, that object contains a pointer to the Circle vtable. When you create a Rectangle, it contains a pointer to the Rectangle vtable. Now when you call `shape.area()`, the compiled code does this:

```
1. Load the vtable pointer from the shape object
2. Look at slot 0 in that vtable (because area is the first method)
3. Jump to whatever address is stored there
```

If the shape is actually a Circle, slot 0 contains the address of Circle's area method. If it's a Rectangle, slot 0 contains the address of Rectangle's area method. The same compiled code works for both cases because it's looking up the address at runtime.

This is incredibly clever, but it does have a cost. Instead of a direct function call (jump to a fixed address), you have several memory loads: load the vtable pointer, load the method address from the vtable, then jump to that address. Modern CPUs are good at this, but it's still slower than a direct call. This is the performance price you pay for the flexibility of polymorphism.

## The Subtle Difference in What They Dispatch On

Here's something that trips people up: overloading and overriding dispatch on different things. Overloading dispatches on the static types of the arguments. Overriding dispatches on the dynamic type of the receiver (the object you're calling the method on).

Let me illustrate this with an example that shows both at once:

```java
class Printer {
    void print(Shape s) {
        System.out.println("Printing a shape");
        s.draw();  // This uses dynamic dispatch (overriding)
    }
    
    void print(Circle c) {
        System.out.println("Printing a circle specifically");
        c.draw();  // This also uses dynamic dispatch
    }
}

// Usage:
Printer printer = new Printer();
Shape shape = new Circle(5.0);

printer.print(shape);  // Calls print(Shape), not print(Circle)!
                       // But inside, s.draw() calls Circle.draw()
```

This might surprise you. Even though `shape` actually contains a Circle, the call to `printer.print(shape)` calls the `print(Shape)` overload, not the `print(Circle)` overload. Why? Because overload resolution happens at compile time based on the static type of the argument. The static type of `shape` is `Shape`, so the compiler picks `print(Shape)`.

But inside that method, when we call `s.draw()`, dynamic dispatch kicks in. The runtime looks at the actual object, sees it's a Circle, and calls Circle's draw method.

This is sometimes called "single dispatch" because only the receiver (the object before the dot) is dispatched dynamically. The arguments are dispatched statically through overload resolution. Some languages, like Common Lisp with its multimethods or Julia with its multiple dispatch, dispatch on all arguments dynamically, which is much more powerful but also more complex and slower.

## Rust's Approach: Traits and Static vs Dynamic Dispatch

Rust takes an interesting middle path that gives you explicit control over when dispatch is static and when it's dynamic. Let me show you how this works because it really clarifies the tradeoffs.

In Rust, you define shared behavior using traits:

```rust
trait Shape {
    fn area(&self) -> f64;
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}
```

Now, when you want to write code that works with any Shape, you have two choices. You can use static dispatch with generics:

```rust
fn print_area<T: Shape>(shape: &T) {
    println!("Area: {}", shape.area());
}

// Usage:
let circle = Circle { radius: 5.0 };
let rectangle = Rectangle { width: 3.0, height: 4.0 };

print_area(&circle);     // Compiler generates print_area_for_Circle
print_area(&rectangle);  // Compiler generates print_area_for_Rectangle
```

With this approach, the compiler generates a separate version of `print_area` for each concrete type you use it with. This is called monomorphization. When you call `print_area(&circle)`, the compiler knows at compile time that you're passing a Circle, so it generates a specialized version of the function that directly calls Circle's area method. No vtables, no indirection, just a direct function call. This is as fast as it gets.

But there's a tradeoff. The compiler has to know all the types at compile time, which means you can't have a collection that mixes different shapes. You can't write `Vec<Shape>` because Shape isn't a concrete type, it's a trait. Each element of a vector has to be the same size, and different shapes have different sizes.

If you need that flexibility, Rust gives you dynamic dispatch with trait objects:

```rust
fn print_area_dynamic(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

// Usage:
let circle = Circle { radius: 5.0 };
let rectangle = Rectangle { width: 3.0, height: 4.0 };

print_area_dynamic(&circle);
print_area_dynamic(&rectangle);

// You can also have heterogeneous collections:
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle { width: 3.0, height: 4.0 }),
];

for shape in &shapes {
    shape.draw();  // Uses vtable dispatch
}
```

The `dyn` keyword means "use dynamic dispatch." Now Rust will use vtables just like Java or C++. The function receives a fat pointer that contains both the address of the object and the address of its vtable. Method calls go through the vtable.

This approach is more flexible but slower. You pay the cost of indirection on every method call. Rust makes you choose explicitly which approach you want, and that choice is visible in the type signature. If you see `&dyn Trait`, you know you're getting dynamic dispatch. If you see a generic `<T: Trait>`, you know you're getting static dispatch.

## Method Resolution in the Presence of Inheritance

Things get more complex when you have deep inheritance hierarchies. Consider this:

```java
class Animal {
    void makeSound() {
        System.out.println("Some generic animal sound");
    }
    
    void eat() {
        System.out.println("Eating...");
    }
}

class Mammal extends Animal {
    void makeSound() {
        System.out.println("Mammal sound");
    }
    
    void breathe() {
        System.out.println("Breathing air");
    }
}

class Dog extends Mammal {
    void makeSound() {
        System.out.println("Woof!");
    }
    
    void eat() {
        System.out.println("Eating dog food");
    }
}
```

When you have a Dog and call `makeSound()`, which implementation runs? The answer is that the language searches up the inheritance chain starting from the most specific class. It looks at Dog first, finds `makeSound()`, and calls that. If Dog didn't override `makeSound()`, it would look at Mammal, find it there, and call that. If Mammal didn't override it either, it would go all the way to Animal.

This search happens at runtime for virtual methods, but the vtable makes it fast. When the compiler builds Dog's vtable, it already does this search and fills in the vtable with the final, most specific version of each method. So at runtime, it's still just one vtable lookup, not a search up the inheritance chain.

For methods that aren't overridden at all, like `breathe()`, the vtable just points to the implementation from whichever class defined it. When a Dog calls `breathe()`, it jumps to Mammal's implementation.

## The Performance Story: Why This Matters

Let me be concrete about the performance implications because this often determines architectural decisions in real systems.

A static dispatch through overloading is typically just a few CPU cycles. The CPU can inline the function, it can optimize aggressively, it can predict branches perfectly because there are no branches. Modern processors can execute these calls at essentially zero cost.

A dynamic dispatch through overriding is more expensive. On a modern x86 CPU, loading the vtable pointer and then the method address takes maybe ten to twenty cycles, and that's if everything is in the cache. If the vtable isn't in cache, you're looking at hundreds of cycles. The CPU also has a harder time predicting where you're jumping to, which can cause pipeline stalls.

For most code, this doesn't matter. Twenty cycles is nothing compared to the actual work the method does. But in tight inner loops that run millions of times, this overhead multiplies. This is why performance-critical code, like game engines or scientific computing libraries, often avoids virtual methods in hot paths.

Some languages and compilers can optimize away virtual calls through a process called devirtualization. If the compiler can prove that a variable will always contain a specific concrete type, it can replace the virtual call with a direct call. Modern JIT compilers like those in the JVM and JavaScript engines are very good at this. They profile the code as it runs, notice that a particular call site always sees the same concrete type, and speculatively compile a version that directly calls that type's method with a quick guard check.

## Multiple Dispatch: When One Dimension Isn't Enough

Most object-oriented languages only do single dispatch. They dispatch dynamically on the receiver but statically on the arguments. But sometimes you want to dispatch on multiple arguments simultaneously.

The classic example is collision detection in a game. You have different types of game objects, and collisions between different types need different handling. A bullet hitting a wall is different from a bullet hitting a player, which is different from a player hitting a wall. You want code like:

```
collide(object1, object2)
```

And you want the right collision handling code to run based on the types of both objects. With single dispatch, you end up with awkward patterns like the visitor pattern or double dispatch, where you manually chain two virtual method calls to effectively dispatch on both objects.

Some languages, like Common Lisp and Julia, support multiple dispatch natively. In Julia, you can write:

```julia
collide(bullet::Bullet, wall::Wall) = ...
collide(bullet::Bullet, player::Player) = ...
collide(player::Player, wall::Wall) = ...
```

The language automatically picks the right method based on the runtime types of all arguments. This is more flexible and often more natural for certain problem domains. The performance cost is higher because you need more complex dispatch logic, but modern implementations make this quite fast.

## Compile-Time vs Runtime: The Fundamental Tradeoff

The overarching theme here is a fundamental tradeoff in programming language design between flexibility and performance, between decisions made at compile time and decisions made at runtime.

Compile-time decisions are faster because they're already made by the time your code runs. The CPU just follows direct instructions. The compiler can optimize aggressively because it knows exactly what's happening. But compile-time decisions are inflexible. Once your code is compiled, the behavior is fixed.

Runtime decisions are flexible. The same code can handle many different types. You can load plugins, deserialize objects, create heterogeneous collections. But runtime decisions cost performance. Every decision at runtime is work the CPU has to do while your program is running.

Different languages make different choices about where on this spectrum they want to sit. C++ leans toward compile-time decisions with templates and inline functions, accepting that this makes the compiler slower and binary sizes larger. Java and C# use dynamic dispatch extensively but try to optimize it with JIT compilation. Rust gives you explicit control, letting you choose static or dynamic dispatch on a case-by-case basis. Python does almost everything at runtime for maximum flexibility at the cost of speed.

Understanding this tradeoff helps you make better design decisions. When you're choosing whether to use inheritance and virtual methods versus generics and static polymorphism, you're not just making a style choice. You're making a decision about when flexibility is worth the performance cost.

## Practical Implications for Your Code

Let me end with some practical advice about when to use overloading versus overriding, and when to worry about the performance implications.

Use overloading when you have genuinely different operations that happen to have the same conceptual name but work on different types. The classic example is output operators or conversion functions. A function that converts an integer to a string is fundamentally different from one that converts a floating point number to a string, even though they share the same name. Overloading is perfect for this.

Use overriding when you have the same abstract operation that needs different implementations for different subtypes. Drawing shapes, processing different types of events, handling different file formats—these are perfect for inheritance and virtual methods. The key is that you want to write code that works with the abstract type and automatically does the right thing for concrete types.

Don't worry about the performance cost of virtual methods unless you've profiled and found them to be a bottleneck. Modern CPUs are fast enough that virtual method calls are essentially free for most code. Optimize for clarity and good design first.

But if you do find that virtual method calls are a bottleneck, you have options. You can use static polymorphism with templates or generics instead. You can restructure your code to make the types known at compile time. You can batch operations to reduce the number of virtual calls. Or you can accept the cost because the flexibility is worth it.

The most important thing is to understand what's happening under the hood so you can make informed decisions. When you write `shape.draw()`, you now know exactly what work the computer has to do to figure out which draw method to call. That understanding makes you a better programmer, whether you're building interpreters, designing class hierarchies, or just trying to make your code faster.

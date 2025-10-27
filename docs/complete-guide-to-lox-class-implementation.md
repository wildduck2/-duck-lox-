# Complete Guide to Lox Class Implementation

## Table of Contents
1. [Overview](#overview)
2. [Class Structure](#class-structure)
3. [How Classes Work](#how-classes-work)
4. [Method Binding & `this`](#method-binding--this)
5. [Static Methods](#static-methods)
6. [Initializers (init)](#initializers-init)
7. [Getters & Setters](#getters--setters)
8. [Closures in Methods](#closures-in-methods)
9. [Full Execution Flow](#full-execution-flow)
10. [Better Approaches](#better-approaches)

---

## Overview

Your Lox interpreter implements classes with:
- Instance methods (with `this` binding)
- Static methods (no `this`)
- Fields (dynamic properties)
- Initializers (`init()` method)
- Property access (`.` operator)

---

## Class Structure

### Core Types

```rust
// The class definition
pub struct LoxClass {
  pub name: String,
  pub methods: HashMap<String, Arc<LoxFunction>>,      // Instance methods
  pub static_methods: HashMap<String, Arc<LoxFunction>>, // Static methods
}

// An instance of a class
pub struct LoxClassInstance {
  pub class: Arc<LoxClass>,                    // Reference to the class
  pub fields: HashMap<String, LoxValue>,       // Instance fields
}

// A method is just a LoxFunction
pub struct LoxFunction {
  pub params: Vec<Token>,
  pub body: Vec<Stmt>,
  pub closure: Rc<RefCell<Env>>,  // Captured environment
  pub is_initializer: bool,        // Special flag for init()
}
```

---

## How Classes Work

### Step 1: Class Declaration (Parser)
```lox
class Dog {
  init(name) {
    this.name = name;
  }
  
  bark() {
    print(this.name + " says woof!");
  }
  
  static info() {
    print("Dogs are animals");
  }
}
```

Parser creates:
```rust
Stmt::Class(
  Expr::Identifier("Dog"),
  vec![init_method, bark_method],  // Instance methods
  vec![info_method]                 // Static methods
)
```

### Step 2: Semantic Analysis (Resolver)

```rust
// In resolver
Stmt::Class(name, methods, static_methods) => {
  self.current_class = ClassType::Class;
  
  // Declare class name in current scope
  self.declare(name);
  
  // Resolve INSTANCE methods with 'this' in scope
  self.begin_scope();
  self.scopes.insert("this", ...);  // Add 'this' to scope
  for method in methods {
    self.resolve_stmt(method);      // Can use 'this'
  }
  self.end_scope();
  
  // Resolve STATIC methods WITHOUT 'this'
  self.current_class = ClassType::StaticMethod;
  for method in static_methods {
    self.resolve_stmt(method);      // 'this' causes error
  }
}
```

**Key insight**: Static methods are resolved in a different context where `this` is not available.

### Step 3: Interpretation (Runtime)

```rust
fn eval_class(...) {
  // 1. Extract class name
  let class_name = "Dog";
  
  // 2. Convert method Stmts into LoxFunctions
  let mut methods_map = HashMap::new();
  for method in methods {
    let function = Arc::new(LoxFunction {
      params: extract_params(method),
      body: extract_body(method),
      closure: env.clone(),  // Capture CURRENT environment
      is_initializer: (method_name == "init"),
    });
    methods_map.insert(method_name, function);
  }
  
  // 3. Create the class
  let class = Arc::new(LoxClass {
    name: class_name,
    methods: methods_map,
    static_methods: static_methods_map,
  });
  
  // 4. Store in environment
  env.define("Dog", LoxValue::Class(class));
}
```

---

## Method Binding & `this`

### The Problem
When you write:
```lox
var dog = Dog("Buddy");
dog.bark();  // How does bark() know which dog?
```

The `bark` method needs access to the specific `dog` instance.

### The Solution: Method Binding

```rust
// When accessing dog.bark
fn eval_get(object: Expr, name: Token) {
  let object_val = eval_expr(object);  // Get the instance
  
  if let LoxValue::Instance(instance) = object_val {
    // Look up the method
    if let Some(method) = instance.class.methods.get("bark") {
      // BIND 'this' to the instance
      let bound_method = method.bind(instance);
      return LoxValue::Function(bound_method);
    }
  }
}
```

### How `bind()` Works

```rust
impl LoxFunction {
  pub fn bind(&self, instance: Rc<RefCell<LoxClassInstance>>) -> Arc<LoxFunction> {
    // 1. Create a NEW environment
    let mut bound_env = Env::new();
    
    // 2. Set the method's closure as parent
    bound_env.enclosing = Some(self.closure.clone());
    
    // 3. Define 'this' in this new environment
    let env_rc = Rc::new(RefCell::new(bound_env));
    env_rc.borrow_mut().define("this", LoxValue::Instance(instance));
    
    // 4. Return new function with bound environment
    Arc::new(LoxFunction {
      params: self.params.clone(),
      body: self.body.clone(),
      closure: env_rc,           // NEW closure with 'this'
      is_initializer: self.is_initializer,
    })
  }
}
```

### Visual Representation

```
Original method closure:
┌─────────────────┐
│ Method's Env    │
│ (class scope)   │
└─────────────────┘

After binding:
┌─────────────────┐
│ Bound Env       │
│ this -> dog1    │  ← NEW layer
├─────────────────┤
│ Method's Env    │
│ (class scope)   │  ← Original closure
└─────────────────┘

Different instance:
┌─────────────────┐
│ Bound Env       │
│ this -> dog2    │  ← Different 'this'
├─────────────────┤
│ Method's Env    │
│ (class scope)   │  ← SAME original closure
└─────────────────┘
```

**Key insight**: Each instance gets its own binding, but they all share the same underlying method closure.

---

## Static Methods

### How They're Different

```rust
// Static method access: Dog.info()
fn eval_get(object: Expr, name: Token) {
  let object_val = eval_expr(object);
  
  // Check if accessing CLASS (not instance)
  if let LoxValue::Class(class) = object_val {
    if let Some(static_method) = class.static_methods.get("info") {
      // NO BINDING - return the function as-is
      return LoxValue::Function(static_method.clone());
    }
  }
  
  // Check if accessing INSTANCE
  if let LoxValue::Instance(instance) = object_val {
    // Instance methods get bound...
  }
}
```

### Key Differences

| Feature | Instance Method | Static Method |
|---------|----------------|---------------|
| Access | `instance.method()` | `Class.method()` |
| `this` | Bound to instance | Not available |
| Binding | Creates new closure with `this` | Used as-is |
| Resolver | `this` in scope | `this` causes error |

---

## Initializers (init)

### Special Rules for `init()`

1. **Always returns the instance** (even if you `return` something else)
2. **Called automatically** when you call the class like a function
3. **Can't return a value** (in strict Lox)

### Implementation

```rust
impl LoxCallable for LoxClass {
  fn call(&self, interpreter: &mut Interpreter, arguments: Vec<...>) {
    // 1. Create the instance
    let instance = Rc::new(RefCell::new(LoxClassInstance {
      class: Arc::new(self.clone()),
      fields: HashMap::new(),
    }));
    
    // 2. Look for init() method
    if let Some(initializer) = self.find_method("init") {
      // 3. Bind 'this' to the new instance
      let bound_init = initializer.bind(instance.clone());
      
      // 4. Call init() with arguments
      bound_init.call(interpreter, arguments, engine)?;
      // Note: Return value is IGNORED
    }
    
    // 5. ALWAYS return the instance
    Ok(LoxValue::Instance(instance))
  }
}
```

### In LoxFunction

```rust
impl LoxCallable for LoxFunction {
  fn call(&self, interpreter: &mut Interpreter, arguments: Vec<...>) {
    // ... execute function body ...
    
    // Special handling for initializers
    if self.is_initializer {
      // Return 'this' instead of the return value
      return Ok(self.closure.borrow().get("this").unwrap().clone());
    }
    
    // Normal return
    Ok(result)
  }
}
```

### Example Flow

```lox
var dog = Dog("Buddy");
```

1. `Dog(...)` triggers `LoxClass::call()`
2. Creates empty instance: `{ class: Dog, fields: {} }`
3. Finds `init` method
4. Binds `this` to the instance
5. Calls `init("Buddy")` → sets `this.name = "Buddy"`
6. Returns the instance (fields now: `{ name: "Buddy" }`)

---

## Getters & Setters

### Property Access (Get)

```rust
// dog.name
fn eval_get(object: Expr, name: Token) {
  let object_val = eval_expr(object);  // Evaluate 'dog'
  
  if let LoxValue::Instance(instance) = object_val {
    // 1. Check FIELDS first
    if let Some(field) = instance.borrow().fields.get("name") {
      return Ok(field.clone());  // Return field value
    }
    
    // 2. Check METHODS second
    if let Some(method) = instance.borrow().class.methods.get("name") {
      let bound = method.bind(instance.clone());
      return Ok(LoxValue::Function(bound));
    }
    
    // 3. Not found
    return Err("Undefined property");
  }
}
```

### Property Assignment (Set)

```rust
// dog.name = "Rex"
fn eval_set(object: Expr, name: Token, value: Expr) {
  let object_val = eval_expr(object);
  let value_result = eval_expr(value);
  
  if let LoxValue::Instance(instance) = object_val {
    // Set the field (creates if doesn't exist)
    instance.borrow_mut().fields.insert(
      name.lexeme,
      value_result.clone()
    );
    return Ok(value_result);
  }
}
```

### Important Notes

1. **Fields shadow methods**: If you set a field with the same name as a method, the field takes precedence
2. **Dynamic fields**: You can add fields at runtime (not declared in class)
3. **No getters/setters**: Simple field access, no computed properties

```lox
class Example {
  method() {
    print("method");
  }
}

var e = Example();
e.method();        // Prints "method" (calls method)
e.method = "hi";   // Sets field 'method' to "hi"
e.method();        // ERROR: "hi" is not callable
```

---

## Closures in Methods

### What Gets Captured

```lox
var x = "outer";

class Test {
  method() {
    print(x);  // What is 'x'?
  }
}
```

### The Closure Chain

```rust
// When eval_class runs:
fn eval_class(env: &mut Env, ...) {
  let function = Arc::new(LoxFunction {
    closure: env.clone(),  // Captures CURRENT environment
    // This env contains 'x' variable
  });
}
```

### When Called

```
Call stack when method() runs:

┌─────────────────────┐
│ Method Call Env     │  ← Local variables of method
│ (params, locals)    │
├─────────────────────┤
│ Bound Env           │  ← Contains 'this'
│ this -> instance    │
├─────────────────────┤
│ Method's Closure    │  ← Environment where class was defined
│ x -> "outer"        │  ← Can access 'x'
├─────────────────────┤
│ Global Env          │
└─────────────────────┘
```

### Example with Nested Scope

```lox
var factory = nil;
{
  var counter = 0;
  
  class Counter {
    increment() {
      counter = counter + 1;  // Closes over 'counter'
      return counter;
    }
  }
  
  factory = Counter;
}

var c = factory();
print(c.increment());  // 1
print(c.increment());  // 2
// 'counter' still accessible!
```

This works because:
1. `Counter` class captures the environment with `counter`
2. All methods share this closure
3. The environment stays alive as long as the class exists

---

## Full Execution Flow

### Complete Example

```lox
class Dog {
  init(name) {
    this.name = name;
  }
  
  bark() {
    print(this.name + " says woof!");
  }
}

var dog = Dog("Buddy");
dog.bark();
```

### Step-by-Step Execution

#### 1. Class Declaration
```rust
eval_class() {
  // Create LoxFunction for init
  let init_fn = LoxFunction {
    params: [name],
    body: [this.name = name],
    closure: current_env.clone(),  // Captures global scope
    is_initializer: true,
  };
  
  // Create LoxFunction for bark
  let bark_fn = LoxFunction {
    params: [],
    body: [print(...)],
    closure: current_env.clone(),  // Same closure
    is_initializer: false,
  };
  
  // Create class
  let dog_class = LoxClass {
    name: "Dog",
    methods: {"init": init_fn, "bark": bark_fn},
    static_methods: {},
  };
  
  // Store in environment
  env.define("Dog", LoxValue::Class(dog_class));
}
```

#### 2. Instantiation: `Dog("Buddy")`
```rust
// eval_call sees Dog("Buddy")
LoxClass::call(arguments: [("Buddy", token)]) {
  // Create instance
  let instance = LoxClassInstance {
    class: Dog,
    fields: {},  // Empty initially
  };
  
  // Find init
  let init = dog_class.methods.get("init");
  
  // Bind 'this'
  let bound_init = init.bind(instance);
  // bound_init.closure now has 'this' -> instance
  
  // Call init("Buddy")
  LoxFunction::call(bound_init, [("Buddy", token)]) {
    // Create function env
    let fn_env = Env::with_enclosing(bound_init.closure);
    // fn_env chain: fn_env -> bound_env (has 'this') -> global
    
    // Bind parameter
    fn_env.define("name", "Buddy");
    
    // Execute: this.name = name
    eval_set(this, "name", name) {
      // Get 'this' from bound_env
      let instance = fn_env.get("this");  // Gets the instance
      
      // Get 'name' parameter
      let value = fn_env.get("name");  // "Buddy"
      
      // Set field
      instance.fields.insert("name", "Buddy");
    }
    
    // is_initializer = true, so return 'this'
    return fn_env.get("this");
  }
  
  // Return instance (now has name field)
  return instance;  // { class: Dog, fields: {name: "Buddy"} }
}
```

#### 3. Method Call: `dog.bark()`
```rust
// eval_get: dog.bark
eval_get(dog, "bark") {
  let instance = eval_expr(dog);  // Get the instance
  
  // Look up method
  let bark_method = instance.class.methods.get("bark");
  
  // Bind 'this'
  let bound_bark = bark_method.bind(instance);
  // bound_bark.closure chain: bound_env (has 'this') -> global
  
  return LoxValue::Function(bound_bark);
}

// eval_call: (dog.bark)()
LoxFunction::call(bound_bark, []) {
  let fn_env = Env::with_enclosing(bound_bark.closure);
  // fn_env chain: fn_env -> bound_env (has 'this') -> global
  
  // Execute: print(this.name + " says woof!")
  eval_expr(Binary {
    lhs: Get(this, "name"),
    op: "+",
    rhs: " says woof!"
  }) {
    // Get 'this' from bound_env
    let instance = fn_env.get("this");
    
    // Get field 'name'
    let name = instance.fields.get("name");  // "Buddy"
    
    // Concatenate
    return "Buddy" + " says woof!" = "Buddy says woof!";
  }
  
  // print() is called
  // Output: "Buddy says woof!"
}
```

---

## Better Approaches

### Current Implementation: Pros & Cons

#### ✅ Pros
1. **Simple and readable** - Easy to understand the flow
2. **Correct semantics** - Implements Lox spec properly
3. **Efficient binding** - Only creates closures when needed
4. **Shared method storage** - All instances share method code

#### ❌ Cons
1. **Runtime overhead** - Method binding happens at every access
2. **No optimization** - Could cache bound methods
3. **Memory usage** - Separate Arc for each method
4. **No inheritance** - Can't extend classes

### Improvement #1: Method Caching

```rust
pub struct LoxClassInstance {
  pub class: Arc<LoxClass>,
  pub fields: HashMap<String, LoxValue>,
  pub method_cache: HashMap<String, Arc<LoxFunction>>,  // ADD THIS
}

fn eval_get(instance, name) {
  // Check cache first
  if let Some(cached) = instance.borrow().method_cache.get(name) {
    return Ok(LoxValue::Function(cached.clone()));
  }
  
  // Not cached, bind and cache
  if let Some(method) = instance.borrow().class.methods.get(name) {
    let bound = method.bind(instance.clone());
    instance.borrow_mut().method_cache.insert(name, bound.clone());
    return Ok(LoxValue::Function(bound));
  }
}
```

**Benefit**: Methods only bound once per instance.

### Improvement #2: Vtable (Virtual Method Table)

```rust
pub struct LoxClass {
  pub name: String,
  pub vtable: Vec<Arc<LoxFunction>>,  // Array of methods
  pub method_indices: HashMap<String, usize>,  // name -> index
}

fn eval_get(instance, name) {
  let index = instance.class.method_indices.get(name)?;
  let method = &instance.class.vtable[*index];
  return method.bind(instance);
}
```

**Benefit**: Faster lookup (array index vs hash map).

### Improvement #3: Inline Caching

```rust
// Store last accessed property per call site
struct PropertyCache {
  last_class: Option<Arc<LoxClass>>,
  last_result: Option<LoxValue>,
}

// At each property access site
static CACHE: PropertyCache = ...;

fn eval_get(instance, name) {
  // Check if same class as last time
  if CACHE.last_class == Some(instance.class.clone()) {
    return CACHE.last_result;  // Super fast!
  }
  
  // Miss - do normal lookup and update cache
  let result = normal_lookup(instance, name);
  CACHE.last_class = Some(instance.class.clone());
  CACHE.last_result = Some(result.clone());
  return result;
}
```

**Benefit**: Near-zero overhead for repeated property access.

### Improvement #4: Prototype Chain (JS-style)

```rust
pub struct LoxClass {
  pub name: String,
  pub prototype: HashMap<String, LoxValue>,  // Methods stored here
  pub parent: Option<Arc<LoxClass>>,         // For inheritance
}

pub struct LoxClassInstance {
  pub class: Arc<LoxClass>,
  pub properties: HashMap<String, LoxValue>,  // Own properties
}

fn eval_get(instance, name) {
  // 1. Check own properties
  if let Some(prop) = instance.properties.get(name) {
    return prop;
  }
  
  // 2. Walk prototype chain
  let mut current_class = Some(instance.class.clone());
  while let Some(class) = current_class {
    if let Some(method) = class.prototype.get(name) {
      return method.bind(instance);
    }
    current_class = class.parent.clone();
  }
  
  return Err("Property not found");
}
```

**Benefit**: Enables inheritance and prototype-based OOP.

### Improvement #5: Separate Field/Method Namespaces

```rust
pub struct LoxClassInstance {
  pub class: Arc<LoxClass>,
  pub fields: HashMap<String, LoxValue>,
  // Methods accessed separately, never stored here
}

fn eval_get(instance, name) {
  // Try methods FIRST (can't be shadowed)
  if let Some(method) = instance.class.methods.get(name) {
    return method.bind(instance);
  }
  
  // Then try fields
  if let Some(field) = instance.fields.get(name) {
    return field;
  }
}
```

**Benefit**: Prevents accidental method shadowing, more predictable.

### Best Overall Approach (Production-Quality)

```rust
pub struct LoxClass {
  pub name: String,
  pub methods: Vec<Arc<LoxFunction>>,      // Vtable
  pub method_map: HashMap<String, usize>,  // name -> vtable index
  pub static_methods: HashMap<String, Arc<LoxFunction>>,
  pub parent: Option<Arc<LoxClass>>,       // For inheritance
}

pub struct LoxClassInstance {
  pub class: Arc<LoxClass>,
  pub fields: Vec<LoxValue>,               // Array, not HashMap
  pub field_map: HashMap<String, usize>,   // name -> field index
  pub bound_methods: Vec<Option<Arc<LoxFunction>>>,  // Cache
}

impl LoxClass {
  fn find_method(&self, name: &str) -> Option<(usize, &Arc<LoxFunction>)> {
    // Try this class
    if let Some(&index) = self.method_map.get(name) {
      return Some((index, &self.methods[index]));
    }
    
    // Try parent (inheritance)
    if let Some(parent) = &self.parent {
      return parent.find_method(name);
    }
    
    None
  }
}

fn eval_get(instance, name) {
  // 1. Check field by index
  if let Some(&field_index) = instance.field_map.get(name) {
    return Ok(instance.fields[field_index].clone());
  }
  
  // 2. Check method cache
  if let Some((method_index, _)) = instance.class.find_method(name) {
    if let Some(bound) = &instance.bound_methods[method_index] {
      return Ok(LoxValue::Function(bound.clone()));
    }
    
    // 3. Not cached - bind and cache
    let method = &instance.class.methods[method_index];
    let bound = method.bind(instance.clone());
    instance.borrow_mut().bound_methods[method_index] = Some(bound.clone());
    return Ok(LoxValue::Function(bound));
  }
  
  Err("Property not found")
}
```

**Benefits**:
- Array access (faster than HashMap)
- Method caching (bind once)
- Inheritance support
- Separate namespaces
- Predictable performance

---

## Summary

### What You Implemented
- ✅ Basic classes with fields
- ✅ Instance methods with `this` binding
- ✅ Static methods (no `this`)
- ✅ Initializers (`init()`)
- ✅ Dynamic property access (`.` operator)
- ✅ Proper closures in methods
- ✅ Compile-time `this` validation

### What's Missing
- ❌ Inheritance (superclasses)
- ❌ Method caching (performance)
- ❌ Private fields/methods
- ❌ Computed properties (getters/setters)
- ❌ Static fields
- ❌ Class methods vs instance methods distinction

### Next Steps
1. **Add inheritance**: `class Cat extends Animal`
2. **Add super**: `super.method()` to call parent methods
3. **Optimize**: Add method caching to instances
4. **Add private fields**: `#privateField` syntax
5. **Add static fields**: `static count = 0`

Your implementation is solid and follows the Crafting Interpreters approach closely. It's clear, correct, and educational. For a production interpreter, you'd want the optimizations mentioned above, but for learning purposes, what you have is excellent!

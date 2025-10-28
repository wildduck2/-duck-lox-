## The Big Picture of Compiler Representations

| Compiler Stage    | Main Purpose                     | Common Representations / Structures                                                                                   |
| ----------------- | -------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| **1. Frontend**   | Parse and understand source code | **AST (Abstract Syntax Tree)**, **Parse Tree**, **Symbol Table**, **Type Environment**                                |
| **2. Middle-end** | Analyze and optimize             | **3AC**, **CFG**, **SSA**, **CPS**, **DAG (Dataflow Graph)**, **HIR/MIR (High/Mid-Level IRs)**                        |
| **3. Backend**    | Generate efficient machine code  | **LIR (Low-Level IR)**, **Instruction Selection Trees**, **Register Allocation Graph**, **Machine CFG**, **Assembly** |

---

## Let’s break down the most important IR families

### 1. **AST (Abstract Syntax Tree)**

* Hierarchical structure representing the **syntax** of the program.
* Built by the parser.
* Good for **semantic analysis** and **early transformations**.
* Still very “high-level” (no explicit control or data flow yet).

### 2. **3AC (Three-Address Code)**

* Linear, low-level-ish IR.
* Shows **explicit operations** like load/store, add, compare.
* Simple to generate from AST.
* Used by many compilers as an early optimization and canonicalization step.

### 3. **CFG (Control Flow Graph)**

* Graph structure where nodes = **basic blocks**, edges = **jumps/branches**.
* Used to analyze **reachability, loops, dominators, liveness, etc.**
* The backbone for most middle-end optimizations.

### 4. **SSA (Static Single Assignment)**

* Data-flow form of IR.
* Adds φ (phi) nodes at merge points to track variable versions.
* Makes dataflow optimizations simple and fast.
* The **standard IR in optimizing compilers** like LLVM.

### 5. **CPS (Continuation-Passing Style)**

* Expresses control explicitly via *continuations*.
* Great for functional languages, advanced optimizations, or control constructs like coroutines, exceptions, async/await.
* Harder to generate or read, but powerful.

### 6. **DAG (Directed Acyclic Graph)**

* Used inside compilers for *expression trees* or *data dependencies*.
* Example: combine `x + y` and `y + x` into the same node.
* Used for **common subexpression elimination**, **instruction selection**, etc.

### 7. **HIR / MIR / LIR (High-, Mid-, Low-level IR)**

Many real compilers use *multiple IR layers*:

* **HIR (High-Level IR):** still close to source language (loops, variables, functions).

  * e.g. JavaScriptCore’s HIR, V8’s “Ignition bytecode.”
* **MIR (Mid-Level IR):** roughly corresponds to SSA / 3AC with explicit control flow.
* **LIR (Low-Level IR):** close to assembly; includes registers and memory details.

---

## Specialized representations used internally

| Structure                          | Purpose                                    |
| ---------------------------------- | ------------------------------------------ |
| **Symbol Table**                   | Keeps types, scopes, variable metadata     |
| **Call Graph**                     | Represents inter-procedural function calls |
| **Dominator Tree**                 | Shows which blocks dominate others in CFG  |
| **Interference Graph**             | Used in register allocation                |
| **Data Dependence Graph (DDG)**    | Used in instruction scheduling             |
| **PDG (Program Dependence Graph)** | Used in slicing, parallelization           |
| **SSA + CFG combined**             | What LLVM IR effectively is                |

---

## So… should you use all of them?

No — you **don’t need all of them**, but you’ll almost always have **a combination** of:

1. **AST** (frontend)
2. **CFG** (middle)
3. **SSA** (optimizations)
4. **Low-level IR / machine IR** (backend)

Optional:

* **CPS**, if you want to model control explicitly (functional/async languages)
* **DAG**, for local expression optimization or instruction selection

---

## In short

| Category   | Name | What it focuses on | Typical use                        |
| ---------- | ---- | ------------------ | ---------------------------------- |
| High-level | AST  | Program structure  | Parsing, semantic checks           |
| Mid-level  | 3AC  | Simple code form   | Easy linear transformations        |
| Mid-level  | CFG  | Control flow       | Loop analysis, reachability        |
| Mid-level  | SSA  | Data flow          | Powerful optimizations             |
| Mid-level  | CPS  | Control as data    | Functional control transformations |
| Low-level  | LIR  | Hardware mapping   | Register allocation, codegen       |

---

## What’s “best”?

There’s no single “best” — they complement each other.

| Goal                                          | Best Representation |
| --------------------------------------------- | ------------------- |
| Easy to generate                              | 3AC                 |
| Dataflow optimization                         | SSA                 |
| Control flow analysis                         | CFG                 |
| Functional / async / coroutine transformation | CPS                 |
| Source-level checks                           | AST                 |
| Machine-level codegen                         | LIR                 |

---

**Conclusion:**
A real optimizing compiler *uses multiple IRs in stages*.
Example (like LLVM):

```
AST → 3AC → CFG → SSA → Optimizations → LIR → Machine Code
```

So no — 3AC, CFG, SSA, and CPS aren’t *the only* things,
but they are the **core analytical IRs** you must understand.
Around them, you have **trees (ASTs)** above and **machine IRs** below.


## Study Notes: Chapter 2 - A Map of the Territory

### 1. The Core Pipeline: Climbing the Mountain

The process of turning source code into a runnable program is like a pipeline. The text visualizes it as climbing a mountain:

* **Ascending (Front End):** We analyze the user's code, transforming it from a flat string of text into a high-level representation of its *meaning*.
* **Descending (Back End):** We transform that high-level representation into lower-level instructions that a machine can actually execute.

---

### 2. The "Front End" (Understanding the Code)

1.  **Scanning (or Lexing):**
    * **Input:** Raw source code (a long string of characters).
    * **Output:** A list of **tokens** (like "words").
    * **Job:** Chunks characters together (e.g., `var`, `123`, `(`). It also discards "meaningless" things like whitespace and comments.

2.  **Parsing:**
    * **Input:** The flat list of tokens from the scanner.
    * **Output:** A tree structure called an **Abstract Syntax Tree (AST)**.
    * **Job:** Takes the flat list and gives it a nested, grammatical structure. This is where **syntax errors** are caught (e.g., `(1 + 2`).

3.  **Static Analysis:**
    * **Input:** The AST.
    * **Output:** An *annotated* AST or other data structure (like a symbol table).
    * **Job:** Figures out the *meaning* (semantics) of the code without running it.
        * **Binding (or Resolution):** Finds where each variable is defined (this is **scope**).
        * **Type Checking:** For statically-typed languages, this phase checks if operations are valid (e.g., you can't add a number to a string).

---

### 3. The "Middle & Back End" (Executing the Code)

4.  **Intermediate Representation (IR):**
    * A "middle-ground" representation of the code that isn't tied to the *source* language (like Lox) or the *target* machine (like x86).
    * **Key Benefit:** Portability. You can write one **Front End** (for Lox) that produces an IR, and multiple **Back Ends** that consume it (one for x86, one for ARM). This is much less work than writing a full compiler for each combination.

5.  **Optimization:**
    * This phase transforms the IR into a *new* IR that does the same thing, but more efficiently.
    * A simple example is **constant folding**: replacing `2 + 3` with `5` at compile time.

6.  **Code Generation:**
    * **Job:** Takes the (optimized) IR and translates it into a low-level language.
    * **Option A: Native Machine Code:** Instructions for a *real* CPU (e.g., x86, ARM). It's very fast but not portable.
    * **Option B: Bytecode:** Instructions for a "hypothetical" or *virtual* machine. This is very portable.

7.  **Virtual Machine (VM):**
    * If you generate **bytecode**, you need a VM. The VM is a program that *simulates* the hypothetical machine at runtime.
    * This is the approach our second interpreter, `clox`, will use.

8.  **Runtime:**
    * A set of services the program needs *while it's running*.
    * Examples: a **garbage collector** to manage memory, or the code that checks an object's type.

---

### 4. Alternate Routes (The "Shortcuts")

Not all implementations follow all 8 steps.

* **Tree-Walk Interpreter:** Goes straight from the **AST** (step 2/3) to executing the program. It "walks" the tree and evaluates each node.
    * **Pro:** Simple to build.
    * **Con:** Slow.
    * **This is our first project: `jlox`!**

* **Transpiler (Source-to-Source Compiler):**
    * A compiler whose **Back End** (step 6) outputs code in *another high-level language* (e.g., compiling TypeScript to JavaScript).

* **Just-in-Time (JIT) Compilation:**
    * A hybrid approach. It starts by interpreting, but *during runtime*, it compiles "hot spots" (frequently used code) to fast, native machine code.
    * This is complex but gives the best of both worlds (portability + speed). Used by modern JavaScript and Java VMs.

---

### 5. Key Concept: Compiler vs. Interpreter

This isn't a strict "either/or" question.

* **Compiler:** A program that *translates* source code into another form (e.g., machine code, bytecode, or JavaScript). `gcc` is a classic compiler.
* **Interpreter:** A program that *executes* code directly from source. Our first interpreter, `jlox`, is a "pure" tree-walk interpreter.

**The "Both" Category:** Many modern "interpreters" (like Python, Ruby, and our upcoming `clox`) are a mix. They first **compile** the source code into internal **bytecode**, and then **interpret** that bytecode with a **VM**.

## Study Notes: Chapter 1: Introduction

### 1. Core Philosophy & Goal

* **Primary Goal:** To build intuition and a tangible understanding of how programming languages work.
* **Methodology:** "Learn by doing." The book is practical and implementation-focused, intentionally light on dense theory.
* **Approach:** We will build two complete, runnable interpreters from scratch. The idea is that after *building* one, theoretical concepts (from other books) will be easier to understand.

---

### 2. Why Learn Language Implementation?

It's not just about creating the next "big" language (which is rare).

* **Domain-Specific Languages (DSLs):** "Little languages" are everywhere (e.g., config files, template engines, markup) and are a common part of large software projects.
* **Programming "Exercise":** Implementing a language is a significant challenge that forces mastery of fundamental computer science:
    * Recursion
    * Trees and Graphs
    * Hash Tables (which we'll build from scratch)
    * Performance-critical code
* **Demystification:** It removes the "magic" from languages and compilers. It shows they are just complex programs, not "arcane arts."

---

### 3. The Project: Two Interpreters for "Lox"

The book is structured around building two implementations of the same language, "Lox".

**Key Decision: No "Compiler-Compilers"**
* We will **not** use tools like **Lex** or **Yacc**.
* Every part (scanner, parser, executor) will be written by hand to ensure there are no "black boxes" and we understand every line of code.

#### Interpreter 1: `jlox` (in Java)

* **Language:** Java
* **Focus:** Simplicity, correctness, and object-oriented concepts.
* **Architecture:** A straightforward **tree-walk interpreter**.
* **Goal:** To understand the core concepts and semantics of the language clearly. It relies on the JVM for runtime features like garbage collection.

#### Interpreter 2: `clox` (in C)

* **Language:** C
* **Focus:** Performance and low-level implementation details.
* **Architecture:** A more advanced **bytecode virtual machine (VM)**. This involves:
    1.  **Compiler:** Translates Lox source code into efficient **bytecode**.
    2.  **Executor:** A VM that runs the bytecode.
* **Goal:** To learn how languages are implemented "all the way down." We will have to build our own data structures (dynamic arrays, hash tables) and a **garbage collector (GC)**.

---

### 4. Book & Chapter Structure

* **Part I:** Introduction (this section).
* **Part II:** The `jlox` (Java) interpreter.
* **Part III:** The `clox` (C) compiler and VM.
* **Incremental Chapters:** Each chapter adds a single language feature. The interpreter remains in a **runnable state** at the end of every chapter.

**Key Page Elements:**

* **Code Snippets:** Precise, complete code blocks showing exactly what to add or replace.
* **Asides:** Interesting (but skippable) details on history, people, or related concepts.
* **Challenges:** Exercises at the end of chapters to explore *beyond* the book's main path. (Note: These should be done in a separate copy of the code).
* **Design Notes:** Short essays on the "human" or design-choice aspect of language creation (e.g., how to name things).

---

### 5. Design Note: Naming a Language

A good language name is:
1.  **Not in use:** Avoids legal/social issues.
2.  **Easy to pronounce.**
3.  **Searchable:** "Go" is okay, "for" would be terrible.
4.  **Not offensive:** Avoids negative connotations in other cultures (e.g., "Nimrod" was renamed to "Nim").

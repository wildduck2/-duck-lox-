# Syntax-Directed Translation (SDT) in Compiler Design

## Definition

**Syntax-Directed Translation (SDT)** is a compiler design method that integrates **syntax analysis** with **semantic actions** to systematically translate source code into another form, such as **intermediate code**, **machine code**, or **optimized instructions**.

In SDT, each grammar production rule is associated with **semantic actions** that define how translation and computation occur. These actions may perform tasks such as:

* Evaluating expressions
* Checking data types
* Generating intermediate code
* Handling semantic and syntactic errors

This integration allows the compiler to analyze structure (syntax) while simultaneously performing translation (semantics).

---

## Core Concepts of SDT

### 1. Key Elements

SDT relies on the following core components:

1. **Lexical values** – basic tokens like variable names and constants.
2. **Constants** – fixed values used in computations.
3. **Attributes** – semantic information attached to grammar symbols, representing computed values, types, or code fragments.

The translation process involves **constructing a parse tree or syntax tree** and evaluating attributes by visiting its nodes in a defined order. In many modern compilers, translation is performed **on the fly during parsing**, without explicitly constructing the tree.

---

## Syntax-Directed Definition (SDD) vs. Syntax-Directed Translation (SDT)

| Aspect               | Syntax-Directed Definition (SDD)                                                                           | Syntax-Directed Translation (SDT)                                                       |
| :------------------- | :--------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- |
| **Definition**       | A context-free grammar with attributes and semantic rules associated with grammar symbols and productions. | A translation process that embeds semantic actions directly within grammar productions. |
| **Nature**           | Descriptive — specifies what computations to perform.                                                      | Procedural — specifies both what and **when** to perform computations.                  |
| **Implementation**   | Uses attribute grammars.                                                                                   | Uses translation schemes (semantic actions inside rules).                               |
| **Example**          | `E → E1 + T { E.val = E1.val + T.val }`                                                                    | `E → E + T { print('+'); }`                                                             |
| **Action Placement** | Always at the end of production.                                                                           | Can appear anywhere in the production; order matters.                                   |
| **Readability**      | More readable.                                                                                             | More efficient.                                                                         |
| **Purpose**          | Defines non-terminal attributes and dependencies.                                                          | Used to generate intermediate code.                                                     |
| **Evaluation**       | Left-to-right (attribute-based).                                                                           | Left-to-right (action-based).                                                           |

---

## Attributes in Syntax-Directed Translation

An **attribute** is any value associated with a symbol in the parse tree. Attributes carry semantic information and help in computing or enforcing language semantics.

### Examples of Attributes

* Data type of a variable
* Instruction or code fragment
* Line number for error handling
* Temporary storage or register information

### Types of Attributes

#### 1. Synthesized Attributes

* Computed from the attributes of child nodes.
* Associated with **bottom-up evaluation**.
* Example: computing an expression value from its operands.

#### 2. Inherited Attributes

* Computed from the attributes of the **parent** or **siblings**.
* Associated with **top-down evaluation**.
* Example: passing type information from declarations to identifiers.

---

## Attribute Grammars

An **attribute grammar** extends a context-free grammar with semantic information.
It provides a formal way to define how attributes are computed and propagated through the parse tree, enabling tasks such as **type checking**, **symbol table management**, and **semantic validation**.

### Example: Attribute Grammar for Type Declarations

| Production Rule | Semantic Rule                              |
| :-------------- | :----------------------------------------- |
| `D → T L`       | `L.in := T.type` (Pass type info downward) |
| `T → int`       | `T.type := integer`                        |
| `T → real`      | `T.type := real`                           |
| `L → L1 , id`   | `L1.in := L.in; addtype(id.entry, L.in)`   |
| `L → id`        | `addtype(id.entry, L.in)`                  |

**Explanation:**

* `T` defines the data type (`int` or `real`).
* `L` passes this type information to all declared identifiers.
* `addtype()` updates the symbol table with the type of each identifier.

---

## Grammar and Translation Rules

### SDT Scheme Example

```
E → E + T { print('+') }
E → E - T { print('-') }
E → T
T → 0 { print('0') }
T → 1 { print('1') }
...
T → 9 { print('9') }
```

### Equivalent SDD Scheme

```
E → E + T     E.code = E.code || T.code || '+'
E → E - T     E.code = E.code || T.code || '-'
E → T         E.code = T.code
T → 0         T.code = '0'
T → 1         T.code = '1'
...
T → 9         T.code = '9'
```

The SDT scheme embeds executable actions (`print`) directly into grammar rules, while SDD describes attribute computations declaratively.

---

## Example: Evaluating an Arithmetic Expression

### Grammar

```
E → E + T     { E.val = E.val + T.val }     // PR#1
E → T         { E.val = T.val }             // PR#2
T → T * F     { T.val = T.val * F.val }     // PR#3
T → F         { T.val = F.val }             // PR#4
F → INTLIT    { F.val = INTLIT.lexval }     // PR#5
```

### Example Input

`2 + 3 * 4`

### Parse Tree

```
        E
       /|\
      E + T
     /   /|\
    T   T * F
   /   /   |
  F   F    4
  |   |
  2   3
```

### Bottom-Up Evaluation

| Step | Production  | Computation           |
| :--- | :---------- | :-------------------- |
| 1    | `F → 2`     | `F.val = 2`           |
| 2    | `F → 3`     | `F.val = 3`           |
| 3    | `F → 4`     | `F.val = 4`           |
| 4    | `T → F`     | `T.val = 3`           |
| 5    | `T → T * F` | `T.val = 3 * 4 = 12`  |
| 6    | `E → T`     | `E.val = 12`          |
| 7    | `E → E + T` | `E.val = 2 + 12 = 14` |

**Final Computed Value:** `14`

---

## Advantages of Syntax-Directed Translation

1. **Ease of Implementation**
   Provides a clear and systematic way to specify translations using grammar rules.

2. **Separation of Concerns**
   Distinguishes parsing from translation, improving modularity and maintainability.

3. **Efficient Code Generation**
   Supports intermediate code generation and optimization during parsing.

---

## Disadvantages of Syntax-Directed Translation

1. **Limited Expressiveness**
   Cannot represent all complex semantic relationships easily compared to general attribute grammars.

2. **Inflexibility**
   Difficult to express complex dependencies that require reordering or multiple passes.

3. **Limited Error Recovery**
   Error handling and recovery mechanisms are weaker, potentially reducing diagnostic quality.

---

## Summary

**Syntax-Directed Translation (SDT)** integrates parsing with semantic evaluation to generate intermediate or target code efficiently.
By associating actions and attributes with grammar productions, SDT enables compilers to:

* Evaluate expressions during parsing
* Perform semantic checks
* Generate code incrementally

It forms the conceptual foundation for **syntax-directed definitions**, **attribute grammars**, and **intermediate code generation**, ensuring both correctness and efficiency in compiler implementation.

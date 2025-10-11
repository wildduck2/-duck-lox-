## Chapter 1: Introduction to Three-Address Code

### 1.1 Overview and Learning Goals

Three-address code (TAC) serves as a crucial **intermediate representation** within compilers, acting as a bridge between the high-level source code we write and the low-level machine code computers execute. While compilers typically generate TAC automatically, this guide focuses on **manual conversion** to provide a clear, step-by-step understanding of its structure and generation.

Our goal is to learn how to hand-write TAC from given source code, demystifying the process of **decomposition** and the instruction-level structure. We will explore the specific instruction set used and walk through a detailed example of translating a loop-based source code into its TAC equivalent.

You might encounter various abbreviations for three-address code, sometimes with hyphens and sometimes without. For clarity, we'll generally use the abbreviation that includes the digit '3' (e.g., **3AC**), as it visually emphasizes the "three" aspect of this code.

For pedagogical simplicity in our demonstrations, we'll assume that each instruction in our three-address code has an **equal size**, meaning that instruction addresses will increment by a uniform amount (e.g., by one). In reality, instruction sizes vary, and addresses increment based on their byte sizes. However, this simplification allows us to focus on the core concepts of decomposition, conditional jumps, placeholders, and address replacement without getting bogged down in complex byte-size arithmetic.

---

### 1.2 The Three-Address Instruction Set and Valid Formats

The instruction set for three-address code adheres to strict rules regarding the number of operands (or "addresses") per instruction. No instruction should use more than three registers overall. Let's look at the valid formats:

#### 1.2.1 Two-Address Assignment Form: `A = B`

This form allows an assignment where a value from one register or variable (`B`) is assigned to another (`A`). While it appears to have only two entities, it still conforms to the three-address code limitations because it represents two operands involved in an operation.

* **Example:** `a = b`
    * Here, 'a' and 'b' represent either registers or distinct variables in the intermediate representation.

#### 1.2.2 Unary Operator Assignment Form: `A = op B`

This format demonstrates operations involving a single operand (`B`) and a unary operator (`op`), with the result assigned to `A`.

* **Example:** `A = -B` (negation)
    * Unary instructions like this are considered to use two registers, fitting within the instruction size constraints.

#### 1.2.3 Binary Operator Assignment Form: `A = B op C`

This is the most common form, showcasing operations between two operands (`B` and `C`) using a binary operator (`op`), with the result assigned to `A`.

* **Example:** `A = B + C`
    * Common binary operators include `+` (plus), `-` (minus), `*` (multiplication), and `/` (division).
    * It's important to remember that operator precedence (e.g., multiplication before addition) will influence how complex expressions are decomposed into multiple TAC instructions using temporary variables.

---

### 1.3 Constraints on Instructions, Conditions, and Register Usage

Maintaining the "three-address" constraint is vital for all instructions, including jumps and complex conditions.

#### 1.3.1 Unconditional Jump: `goto L`

An **unconditional jump** transfers control flow directly to a specified label (`L`).

* **Syntax:** `goto L`
* The target address `L` can either be a direct memory address or stored in a register. For our manual conversion, we'll initially use **placeholders** for jump targets and replace them with actual instruction addresses once the entire code sequence is laid out.

#### 1.3.2 Conditional Jump: `if condition goto L`

**Conditional jumps** execute based on the evaluation of a condition. The condition itself must adhere to strict operand limits.

* **Condition Form:** The `condition` part of the instruction must involve **at most two registers**.
* **Example:** `if a >= b goto L` is valid, as it uses only two operands (`a` and `b`) for the comparison.
* **Complex Conditions:** If a boolean expression is more complex and involves more than two registers (e.g., `if (a > b AND c < d) goto L`), it **must be decomposed** into multiple simpler conditional statements to satisfy the three-address constraint. We'll see this in our walkthrough.
* Similar to unconditional jumps, we'll use **placeholders** for the target address `L` initially.

#### 1.3.3 Register Limits and Temporary Variables

To manage complex expressions and adhere to the three-address limit per instruction, we introduce **temporary variables** (often denoted as `T1`, `T2`, etc.).

* **Purpose:** Temporaries are used to hold the intermediate results of computations when an expression cannot be represented in a single three-address instruction. For example, in an expression like `X = A + B * C`, the multiplication `B * C` would first be computed and stored in a temporary (`T1 = B * C`), and then the addition would follow (`X = A + T1`).
* **Constraint Reminder:** Every decomposed step and every instruction, including those involving temporaries, must explicitly respect the rule of having no more than three registers.

---

### 1.4 Step-by-Step Conversion of Source Code into Three-Address Code

Now, let's put it all together by manually converting a source code example. This process involves identifying statements, breaking down complex expressions, managing control flow, and then assigning and replacing addresses.

#### 1.4.1 Initial Mapping and Straightforward Assignments

We begin by examining the source code and identifying statements that can be directly translated into three-address instructions without further decomposition. These are often simple assignments.

* **Direct Conversion:** A statement like `i = z` is already in a valid three-address form and can be written as a single instruction in our intermediate representation.
* **Order and Placement:** It's crucial to distinguish between **initializations** that happen once before a loop starts and operations that occur repeatedly within a loop's body. For instance, an initialization like `i = j` would be placed as a separate instruction executed *before* any loop condition checks.

#### 1.4.2 Loop Condition Translation

The loop condition is a critical part of control flow. We translate it using a conditional jump.

* **Conditional Jump with Placeholder:** A loop test like `if i < k` is translated into an instruction such as `if i < k goto A`. Here, `A` is a **placeholder address** that will point to the start of the loop body.
* **False Path:** If the condition `i < k` is false, execution should proceed to the instructions immediately following the loop.
* **Control Flow Reinforcement:** At the end of each loop iteration, an **unconditional jump** (`goto L`) will be used to direct control back to the instruction that checks the loop condition, ensuring the loop continues as long as the condition is true.

#### 1.4.3 Loop Body Decomposition and If-Else Logic

Statements within the loop body, especially compound expressions or conditional (`if-else`) logic, require careful decomposition into multiple three-address instructions, often utilizing temporary variables.

* **If-Branch Translation:** Consider an `if` block, such as `if i > f { j = i + (k * f) }`. This would be broken down:
    1.  First, the condition: `if i > f goto [True_Branch_Address]`
    2.  If true, calculate the multiplication: `T1 = k * f`
    3.  Then, the addition and assignment: `j = i + T1`
* **Else-Branch Translation:** For an `else` block (e.g., `else { j = i * (k - f) }`), similar decomposition occurs:
    1.  A jump would skip the `if` branch if its condition was true.
    2.  If in the `else` block, calculate the subtraction: `T2 = k - f`
    3.  Then, the multiplication and assignment: `j = i * T2`
    4.  Crucially, after the `if` or `else` branch completes its operations, an **unconditional jump** is needed to skip any remaining alternative branch code and proceed to the end-of-iteration steps.
* **End-of-Iteration Steps:** Finally, within the loop body, any update to the loop variable (e.g., `i = i + 1`) is translated into a separate three-address instruction. After all operations for the current iteration are complete, an unconditional jump directs control back to the loop's conditional test.

---

## Chapter 2: Address Numbering and Placeholder Replacement 

### 2.1 Assigning Instruction Addresses

Once all the three-address instructions have been generated, the next crucial step is to assign a unique numeric address to each instruction.

* **Address Format:** Addresses are typically written as a number followed by a colon (e.g., `1:`).
* **Sequential Increment:** Based on our pedagogical assumption of uniform instruction size, we will increment addresses sequentially by one for each instruction. This simplifies the process of tracking where each instruction resides in the code.
* **Placeholders for Forward References:** During the initial translation, we used placeholders (like `A`, `B`, `C`) for jump targets. These placeholders are essential for **forward references**, where a jump points to an instruction that hasn't been assigned a concrete address yet.

---

### 2.2 Replacing Placeholders with Concrete Addresses

With all instructions numbered, we can now go back and replace all our temporary placeholders with their corresponding concrete numeric addresses.

* **Initial Mapping:** We'll fill in placeholders, such as replacing 'A' with address 6 (if that's where the loop body starts) and other placeholders with the computed addresses after the entire instruction sequence is laid out.
* **Corrections and Verification:** It's common during manual conversion to identify minor errors or miscalculations in addresses. The process of replacing placeholders provides an opportunity to review and correct these, ensuring all jumps point to the correct instructions. After replacement, you'll have a complete three-address code listing where all control flow is clearly defined by explicit addresses.




```c
i = 0;

for (i=j; i < k; i++) {

  if (i > f) {
    j = i + k * f;
  } else {
    j = i * (k - f);
  }
}

k = m;
```

1: i = 0
1: i = j  
1: if i < k [GOTO]:A
1: k = m
1: [GOTO]:B
A: if i > f [GOTO]:C
A+1: t1 = k -f
A+2: j = i * t1
1: [GOTO]:A+3
C: t2 = k * f
C+1: j = i + t2
C+2: i = i + 1
C+3: [GOTO]:A
B: 



this is the final result of a `3AC`

1: i = 0
2: i = j  
3: if i < k [GOTO]:6
4: k = m
5: [GOTO]:14
6: if i > f [GOTO]:10
7: t1 = k - f
8: j = i * t1
9: [GOTO]:6
10: t2 = k * f
11: j = i + t2
12: i = i + 1
13: [GOTO]:6
14:

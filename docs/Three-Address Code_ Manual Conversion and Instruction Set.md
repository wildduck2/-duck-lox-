# [Three-Address Code: Manual Conversion and Instruction Set](https://www.youtube.com/watch?v=dwb32J9rNMY)

## Overview and video goal for learning three-address code

### Presenter states three-address code is an intermediate representation compilers use syntax translation to generate it, but this video demonstrates hand-written conversion for understanding.

- Video intent: convert given source code into three-address code by hand to make the representation clear and understandable for learners.

- Speaker will write code manually rather than relying on compiler-generated output to explain decomposition and instruction-level structure step by step.

- The presentation includes both the instruction set used and a walkthrough of translating a specific loop-based source code example into three-address code.

### Speaker notes common abbreviations for three-address code, mentions both styles, and prefers the digit-containing abbreviation because it visually includes a numeral within the name.

- Terminology: three-address code is sometimes abbreviated with or without hyphens, and the presenter references both variants during the presentation carefully.

- Abbreviation preference: the presenter likes the abbreviation that includes a digit because it explicitly shows the 'three' as a numeral.

- This naming detail does not affect semantics but is mentioned to clarify verbal references used consistently throughout the tutorial examples.

### Presenter explains the learning approach: assume each instruction has equal size and addresses increment by a uniform amount for pedagogical simplicity in the video.

- Assumption: treat all instructions as identical size so instruction addresses increment by one for clarity during the manual conversion demonstration.

- Speaker acknowledges reality differs, noting actual instruction sizes vary and addresses would normally increment based on instruction byte sizes instead of uniform steps.

- Simplification allows focus on decomposition, conditional jumps, placeholders, and address replacement without dealing with byte-size arithmetic complexities in this video.

## Three-address instruction set and valid instruction formats

### Two-address assignment form described as 'A = B' allowed, where A and B represent registers or variables and count as two operands within an instruction.

- Example: 'a = b' is presented as a valid instruction because it uses only two registers and conforms to three-address code limitations.

- Terminology: operands are sometimes called registers but may refer to distinct variables in three-address intermediate representation depending on implementation details.

- Instruction limit: instructions must not use more than three registers overall, ensuring each instruction remains within the three-address code constraint.

### Unary operator assignment form 'A = op B' illustrated with examples such as negation to show single-operand operator usage within instructions.

- Example operator: unary minus is shown as 'A = -B' to demonstrate allowable unary operations in three-address code instruction set.

- Formality: unary instructions count as using two registers, which fits within the instruction size constraints described earlier in the lecture.

- Speaker emphasizes operator examples to clarify that unary and binary forms must adhere to register count limitations for validity in representation.

### Binary operator assignment form 'A = B op C' provided, with typical operators like plus, minus, and multiplication used as examples.

- Example: 'A = B + C' demonstrates addition as a binary operation requiring at most three registers in the instruction.

- Operators such as plus, minus, and times are listed as valid binary operator examples in the instruction set explanation section.

- Speaker notes multiplication precedence will affect decomposition into temporary computations when translating compound expressions into three-address code during the example translation.

## Constraints on instructions, conditions, and register usage

### Unconditional jump 'goto L' form explained as using one register when the target address is stored in a register or directly specified.

- Syntax: 'go to' instruction is part of the three-address instruction set and may use a register to hold the jump address.

- Usage: unconditional jumps are used to transfer control flow between labeled addresses in the manually written three-address code during the example translation.

- Presenter places placeholders for jump targets initially and intends to replace them with actual addresses after laying out instructions sequentially.

### Conditional jump 'if condition goto L' requires the condition to have at most two registers and may need decomposition depending on complexity.

- Condition form example: 'if a >= b goto L' is valid because it uses only two operands for the comparison operation.

- Complex boolean expressions with more than two registers must be broken into multiple conditions to conform to three-address constraints as illustrated.

- In the walkthrough the presenter splits compound conditions as needed and uses conditional branches with placeholders for addresses during translation.

### Register limits and temporary variable usage described, including temporary variables T1, T2 to hold intermediate computation results during expression decomposition.

- Temporaries: presenter introduces T1 and T2 as example temporary variables to store multiplication or subtraction intermediate results during the loop body.

- Purpose: using temporaries lets the speaker decompose complex expressions into valid binary or unary three-address instructions throughout the demonstration.

- Constraint reminder: no instruction should have more than three registers and each decomposed step must respect this rule explicitly here.

## Step-by-step conversion of source code into three-address code

### Initial mapping: presenter begins by identifying top-level statements and translating straightforward assignments such as 'i = z' directly into three-address instructions without decomposition.

- Direct conversion: the statement 'i = z' is already in a valid three-address form and is written as a single instruction in the IR.

- Order: presenter highlights that some assignments occur before loop entry, while others execute on each iteration, distinguishing initializations from loop body operations.

- Placement: the initialization 'i = j' is moved outside the loop as a separate three-address instruction executed before loop condition checking.

### Loop condition translation: presenter writes the loop test 'if i < k goto ...' using a conditional jump with a placeholder address for the loop body start.

- Placeholder usage: the presenter writes 'if i < k goto A' initially and later replaces 'A' with the actual instruction address after laying out the loop body.

- False path: when the condition is false, execution proceeds to the instruction after the loop, demonstrated by writing subsequent instructions outside the loop.

- Control flow: presenter inserts unconditional jumps to skip loop body when appropriate and to return to the loop condition at the end of each iteration.

### Loop body decomposition: presenter breaks compound statements inside the loop into temporaries and conditional branches representing if-else logic within the loop.

- If-branch translation: when 'if i > f' is true, presenter computes 'T1 = k * f' then updates 'j = i + T1' using two three-address instructions.

- Else-branch translation: in the else case presenter computes 'T2 = k - f' then updates 'j = i * T2' and ensures a jump skips the if-branch code.

- End-of-iteration steps: presenter shows incrementing loop variable 'i = i + 1' as a separate three-address instruction and then jumps back to the loop condition.

## Address numbering, placeholders, and final address replacements

### Instruction addresses: presenter assigns unique numeric addresses to each three-address instruction, incrementing sequentially based on assumption of uniform instruction size.

- Address format: addresses are written as numbers followed by a colon, and the presenter increments them by one for each instruction in the example.

- Practical note: real systems vary instruction sizes, but the video assumes equal size to simplify address calculation and replacement tasks.

- Placeholders are used for forward references to loop body and jump targets until the entire instruction sequence is numbered completely.

### Address replacement: presenter replaces placeholders with concrete numeric addresses and adjusts some addresses when mistakes are identified during walkthrough corrections.

- Initial mapping: the presenter fills in placeholder 'A' with address 6 and subsequent placeholders with addresses computed after laying out instructions.

- Corrections: as numbering proceeds the presenter realizes some addresses need correction, and updates B and C placeholders to reflect accurate instruction indices.

- Finalization: after replacement presenter presents a complete three-address code listing and acknowledges possible mistakes while encouraging understanding for learners nonetheless.

### Summary and closing: presenter reiterates the goal, invites viewers to understand despite mistakes, and signs off until the next video.

- Speaker caveat: presenter admits there were mistakes during the example and asks viewers to focus on conceptual understanding rather than perfect listing accuracy.

- Expectation: viewers are encouraged to use the walkthrough to learn decomposition, placeholder use, and address replacement in manual three-address code writing.

- Closing remark: presenter says 'see you in the next video' ending the tutorial segment on manual three-address code conversion now.


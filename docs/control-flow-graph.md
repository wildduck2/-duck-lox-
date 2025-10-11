# Basic Blocks & Control-Flow Graphs (Three-Address Code)

A compact, practical reference for identifying basic blocks and building a control-flow graph (CFG) from three-address code.

---

# 1. What is a Basic Block? (Definition & core idea)

* **Definition:** A **basic block** is the *longest* contiguous sequence of instructions such that control can only enter at the first instruction and can only leave at the last instruction (no internal jumps or entry points).
* **Why it matters:** Basic blocks group straight-line code so analyses and optimizations can operate on coarse, predictable units rather than individual instructions.
* **Maximality:** You must include as many consecutive instructions as possible until a branch or a branch target forces a block boundary.

---

# 2. Block invariants (single entry / single exit)

* **Single entry:** Every control transfer into the block must target the **first** instruction. No instruction inside the block may be the target of an outside jump.

  * If an instruction is targeted by a jump, it **must** begin a block.
* **Single exit:** Control leaves the block only at the last instruction.

  * If a branch (conditional or unconditional) appears, it must be the **last** instruction in its block.
* **Consequence:** Any violation (a mid-block target or mid-block branch) forces you to split and create new blocks so invariants hold.

---

# 3. Preparation: mark branches and targets

Before grouping instructions, scan the code and mark:

1. **All branch instructions** (conditional and unconditional) — these must become block **terminators** (last instruction in their block).
2. **All target instructions** (labels/jump destinations) — these must become block **entry points** (first instruction in a block).

**Why:** Pre-highlighting ensures you form *maximal* blocks without accidentally including a branch inside or missing a target as a block start.

---

# 4. Algorithm: form basic blocks (step-by-step)

1. Start at the first instruction (program entry).
2. Create a new block and add successive instructions until you encounter a marked instruction.

   * If you hit a **branch**, include it and end the current block.
   * If you hit a **target**, start a new block at that instruction (do **not** include it in the previous block).
3. If an instruction is **both** a branch and a target, it must be a **single-instruction** block (it’s forced to be the last of one block and the first of another).
4. Continue scanning sequentially until all instructions are assigned to blocks.

**Notes:**

* Most blocks are multi-instruction; some will be single-instruction due to dual roles.
* This produces the set of CFG nodes directly (each block → one node).

---

# 5. Build the Control-Flow Graph (CFG)

* **Nodes:** each basic block becomes a vertex in the CFG.
* **Edges:** for each block, create directed edges that represent possible control transfers from that block’s **last** instruction to the **first** instruction of successor blocks:

  * **Conditional branch:** edge to the branch target block and edge to the fall-through successor block (if any).
  * **Unconditional goto:** edge to the target block only.
  * **Fall-through (no branch):** edge to the next sequential block.
* **Special nodes:**

  * **Entry node:** block containing the program’s first instruction.
  * **Exit nodes:** blocks with no outgoing edges (return/terminate).
* **Loops:** appear as back edges from a block to an earlier block (e.g., a for-loop where the loop body or footer jumps back to the loop header).

---

# 6. Quick checklist (practical)

* [ ] Mark every branch instruction.
* [ ] Mark every jump/label target.
* [ ] Start blocks at program entry and every target.
* [ ] End blocks at branch instructions.
* [ ] Make single-instruction blocks for instructions that are both branch *and* target.
* [ ] Create CFG edges from block last-instructions to successor block entries.
* [ ] Identify entry and exit blocks and any loop back-edges.



this is the basic blocks

1: i = 0
2: i = j  
3: if i < k [GOTO]:6 [branch]
4: k = m
5: [GOTO]:14 [branch]
6: if i > f [GOTO]:10 [target][branch]
7: t1 = k - f
8: j = i * t1
9: [GOTO]:6 [branch]
10: t2 = k * f [target]
11: j = i + t2
12: i = i + 1
13: [GOTO]:6 [branch]
14:



This is the final Control Flow Graph (CFG)


      ┌──────────────────────────────┐
      │   B1:                        │
      │   i = 0                      │
      │   i = j                      │
      │   if (i < k) goto B3         │
      └──────┬──────────────┬────────┘
             │True          │False
             │              ▼
             │       ┌─────────────────────┐
             │       │   B2:               │
             │       │   k = m             │
             │       │   goto B6           │
             │       └────────┬────────────┘
             │                │
             ▼                ▼
      ┌──────────────────────────────┐
      │   B3:                        │
      │   if (i > f) goto B5         │
      └──────┬──────────────┬────────┘
             │True          │False
             │              ▼
             │       ┌─────────────────────┐
             │       │   B4:               │
             │       │   t1 = k - f        │
             │       │   j = i * t1        │
             │       │   goto B3           │◄──┐
             │       └─────────────────────┘   │
             │                                 │
             ▼                                 │
      ┌──────────────────────────────┐         │
      │   B5:                        │         │
      │   t2 = k * f                 │         │
      │   j = i + t2                 │         │
      │   i = i + 1                  │         │
      │   goto B3                    │─────────┘
      └──────────────────────────────┘
                      │ 
                      ▼
              ┌──────────────────────┐
              │   B6:                │
              │   (exit / end)       │
              └──────────────────────┘


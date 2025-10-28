# Dead Code Elimination — a practical guide for compiler writers

Dead Code Elimination (DCE) is a core optimization that removes instructions, statements, or whole blocks that have no effect on program observable behavior. It’s cheap, effective, and a huge enabler for further optimizations (smaller CFGs, better register allocation, fewer memory accesses, improved inlining outcomes).

Below is a compact, practical reference covering what “dead code” means, detection algorithms, common variants, correctness caveats, implementation tips, and test ideas.

---

# What is dead code?

Dead code can appear in several forms:

* **Unreachable code:** Control never reaches a statement or basic block (e.g., code after `return`, or blocks with no incoming edges).
* **Dead stores (dead assignments):** A computed value is never used (no later read before redefinition or function exit). Example: `x = 5; x = 6;` the first store is dead.
* **Dead temporaries / unused side-effect-free computations:** Computations whose results are unused and that have no side effects.
* **Redundant computations removed via DCE + other passes:** Things that become dead after other optimizations (LICM, GVN) run.

Note: code that appears unused but has *observable side effects* (I/O, volatile loads/stores, calls that throw, memory fencing, synchronization) is **not** dead.

---

# Why DCE matters

* Reduces instruction count and code size.
* Decreases register pressure and spill risk.
* Makes hot paths faster and smaller.
* Enables and amplifies other optimizations (CSE, inlining, LICM, register allocator).
* Simplifies program reasoning and diagnostics.

---

# Core techniques

## 1) Unreachable code elimination (UCE)

* Compute reachability from entry (DFS/BFS) on the CFG.
* Remove blocks not visited.
* Also remove edges and fix up phi nodes / exception edges.

This is straightforward and usually first in the DCE pipeline.

## 2) Local Dead Store Elimination (basic)

* Within a basic block, walk instructions backwards.
* Track which variables/temps are “live” (used later in the block).
* If an instruction writes `v` and `v` is not live afterwards and the instruction has no side effects, remove it; otherwise mark its operands live.
* Very cheap and often removes most trivial dead stores.

## 3) Global Dead Code Elimination (dataflow)

* Perform a **live-variable analysis** across the CFG (backward dataflow):

  * `live_out[B] = ⋃_{s ∈ succ(B)} live_in[s]`
  * `live_in[B] = use[B] ∪ (live_out[B] − def[B])`
* For each instruction: if it defines a variable that is not in the instruction’s `live_out` and the instruction has no side effects → delete it.
* Iterate to fixed-point (worklist) if needed. Complexity is linear in practice.

## 4) SSA-based DCE (Sparse DCE)

* On SSA form, dead definitions are easy to find: start from program outputs/side-effecting ops and mark reachable SSA defs; remove unmarked defs.
* This is often implemented as a mark-and-sweep on the def-use graph:

  * Mark all instructions that are side-effecting or otherwise live (returns, prints, volatile accesses).
  * BFS/DFS over uses to mark dependent defs reachable.
  * Delete unmarked instructions (and their now-useless phi nodes).
* SSA-based approach is precise and often cheaper (sparse) than repeated dataflow.

## 5) Interprocedural DCE

* With interprocedural analysis (call graphs), remove dead functions or dead parameter assignments when safe (requires whole-program or module-level analysis).
* Conservative when calls have unknown side effects or via dynamic dispatch — use points-to/call-graph and alias info.

---

# Pseudocode — simple backward local DCE (per basic block)

```
for each basic block B:
  live = live_out[B] (from global liveness) // or empty for local
  for instr in B.instructions reversed:
    if instr has side-effects:
      mark instr as live
      live |= uses(instr)
    else if instr.def in live:
      // instruction needed
      live = (live - {instr.def}) ∪ uses(instr)
    else:
      // instr.def not live and no side-effects -> remove instr
```

On SSA:

```
worklist = {all side-effecting instructions and returns}
mark = set(worklist)
while worklist not empty:
  i = pop(worklist)
  for each operand def d of i:
    if d not in mark:
      mark.add(d); worklist.push(d)
delete all instructions not in mark
```

---

# Correctness rules & caveats

Do **not** remove:

* Instructions that can throw observable exceptions in a way that affects program behavior (careful: some compilers allow reordering/folding under strict flags).
* I/O, volatile memory accesses, memory fences, `synchronized` monitors, atomic operations.
* Calls to unknown or impure functions (unless you have side-effect info / purity analysis).
* Memory loads if alias analysis cannot prove no intervening store.
* Anything that affects dynamic semantics (class initialization, reflection side-effects).

Be conservative where language rules or memory models are strict (C-style UB, IEEE FP exceptions).

---

# Interactions with other passes

* **LICM / Strength reduction / GVN** can create new dead definitions; run DCE afterwards.
* **GVN/CSE** may replace expressions with reused values making old computations dead.
* **Register allocation** benefits from DCE (reduced live ranges).
* **Inlining** may expose more DCE opportunities — but can also increase code size; run DCE after inlining to trim.

Ordering tip: UCE → LICM/GVN → SSA → SSA-DCE → other cleanups is a common pattern; but pipelines differ.

---

# Heuristics & performance considerations

* Prefer SSA-based (sparse) DCE when using SSA IR — it scales well and is precise.
* Avoid DCE that increases register pressure (e.g., remove computations that reduce later spills?) Usually removal helps, but creating temporaries to enable other passes can backfire.
* Use profile-guided info: don’t aggressively remove code that is live only on cold paths if doing size vs speed tradeoffs.
* For large modules, incremental or modular DCE keeps memory bounded.

---

# Testing & verification

* Unit tests: tiny IR snippets covering unreachable code, dead stores, side-effecting calls, volatile accesses.
* Regression: round-trip preserving semantics on random IR via an interpreter/fuzzer — run original and optimized and compare outputs for many inputs.
* Edge cases: exception behavior, floating-point NaNs/infinities, integer overflow semantics (C UB).
* Integration tests: check that subsequent passes (register allocator, codegen) are still valid and that the resulting binary passes functional tests.

---

# Implementation checklist

* [ ] UCE: remove unreachable blocks and fix φ nodes.
* [ ] Local DCE: implement backward pass per block.
* [ ] Global DCE: implement liveness dataflow or SSA sparse DCE.
* [ ] Respect side-effects, volatile, atomic, and memory fence semantics.
* [ ] Add alias analysis hooks to be conservative for loads/stores.
* [ ] Run DCE after optimizations that create dead defs (GVN, LICM).
* [ ] Add diagnostics / debug printing to show deletions for verification.
* [ ] Add fuzz/regression tests covering tricky semantics.

---

# Short examples

Before:

```c
int f() {
  int x = 1;
  int y = 2;
  int z = x + y;   // z never used
  return y;
}
```

After DCE:

```c
int f() {
  int y = 2;
  return y;
}
```

SSA-based example:

```
a1 = const 1
b1 = const 2
c1 = add a1 b1   // removed if c1 unused
print b1         // marks b1 live, a1 used to compute b1? (not in this snippet)
```

---

# Final notes

DCE is among the most reliable, low-risk optimizations in a compiler. The SSA-based sparse approach is recommended if your IR supports SSA — it’s precise and efficient. Always be conservative about side-effects, memory ordering, and language-specific semantics (FP, UB). Combine DCE with GVN / LICM / inlining to multiply wins, and add good diagnostics and tests to gain confidence.

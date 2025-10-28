# Loop-Invariant Code Motion (LICM) — a practical guide for compiler writers

Loop-Invariant Code Motion (LICM) is a classic and high-impact optimization: move computations that produce the same result on every iteration out of the loop so they execute once instead of many times. This reduces runtime work, exposes further optimizations, and often yields measurable speedups on hot loops.

Below I explain what LICM is, how compilers detect safe hoisting opportunities, implementation strategies (including SSA), correctness caveats, practical heuristics, and common extensions (subexpression hoisting, if/else motion, repeat loops). I finish with a compact SSA-based pseudocode you can adapt.

---

# What LICM does (intuitively)

Given a loop:

```
while (cond) {
  t = A + B      // A and B never change inside the loop
  body_uses_t
}
```

LICM transforms the program to:

```
t = A + B        // computed once before the loop (preheader)
while (cond) {
  body_uses_t
}
```

The result is the same, but the expensive expression `A + B` runs only once, not every iteration.

Many real compilers insert a *loop preheader* (a new basic block that precedes the loop header and is executed exactly once before looping begins) — LICM places hoisted code in the preheader. For loops whose entry condition itself depends on invariants, some compilers generate an `if` guarding hoisted code (as in your example).

---

# Why LICM matters

* Removes redundant computation in hot loops (big payoff for expensive ops: divisions, pow, memory loads).
* Reduces instruction count and CPU cycles.
* Exposes additional optimizations: constant folding, CSE, dead-code elimination.
* Often yields dramatic speedups on numerical code or inner loops.

---

# When a computation is loop-invariant (safety conditions)

A statement/expression `S` is safe to hoist out of loop `L` if **all** of the following hold:

1. **Value independence** — `S`’s operands are not modified inside the loop (they are loop-invariant themselves or constants).
2. **No side effects** — `S` does not have side effects that change program semantics if moved (no I/O, no stores to volatile memory, no heap mutation unless proven safe).
3. **Exception semantics preserved** — moving `S` must not change whether/when exceptions occur in ways observable by the program (e.g., integer divide-by-zero, traps). Conservative compilers avoid hoisting expressions that can throw unless it’s provably safe.
4. **Memory / alias safety** — if `S` is a load from memory, ensure no store inside the loop may alias that address; if unsure, do not hoist.
5. **Loop entry semantics** — the hoisted instruction must be executed on all paths that reach the original loop body (preheader must dominate original position) — the canonical way to ensure this is to place code in the loop preheader.

If all those conditions hold, moving `S` to the loop preheader preserves semantics.

---

# Typical implementation strategy

1. **Identify loops**
   Build a control-flow graph (CFG) and compute loop structures (natural loops) using backedges and dominance.

2. **Create/ensure a loop preheader**
   If the loop header has multiple predecessors, insert a unique preheader block where hoisted code will be placed. This block executes once immediately before the loop header.

3. **Find loop-invariant instructions**
   For each instruction in the loop:

   * Check whether all operands are either constants or defined outside the loop or already known invariant.
   * Check side-effects, exceptions, and memory aliasing.
   * If safe, mark the instruction invariant.

4. **Hoist invariants**

   * Move invariant instructions to the preheader. When the instruction uses temps created inside the loop that are now hoisted, ensure those temps are also hoisted (transitively).
   * If an instruction is used in multiple places, ensure correct SSA/rename handling (in non-SSA IR, introduce a new temp and replace uses).

5. **Repeat until fixed point**
   One hoist can create new invariants (an invariant depends on another hoisted expression). Iterate until no more instructions can be hoisted.

6. **Post-passes**
   Run constant folding, CSE/GVN, and dead-code elimination to clean up and exploit newly exposed opportunities.

---

# LICM in SSA form (recommended)

SSA makes LICM simpler and more precise:

* Each definition is unique; you reason about values by SSA names, not by variable aliases.
* Hoisting an SSA instruction means cloning or moving a single SSA definition to the preheader; uses in the loop continue to refer to the SSA name.
* For phi nodes: invariants involving phi nodes require care — an SSA value is invariant only if all incoming values to the phi are the *same* invariant.

Many modern compilers (LLVM, GCC) implement LICM directly on SSA IR.

---

# Example (your R-like snippet)

Original:

```
i <- 0
n <- 1000
y <- rnorm(1)
z <- rnorm(1)
a <- c()
while (i < n) {
  x <- y + z
  a[i] <- 6 * i + x * x
  i <- i + 1
}
```

Hoisted (preheader/if approach):

```
i <- 0
n <- 1000
y <- rnorm(1)
z <- rnorm(1)
a <- c()
if (i < n) {
  x <- y + z      # hoisted once before loop
}
while (i < n) {
  a[i] <- 6 * i + x * x
  i <- i + 1
}
```

Notes:

* We guarded hoisted code with `if (i < n)` so we only compute `x` if the loop will run at least once (preserves semantics when `i >= n`).
* A canonical compiler would place `x <- y + z` in a preheader that is executed only when the loop is entered.

---

# Memory loads, pointers and alias analysis

Loads are trickier: `v = load [p]` can only be hoisted if no store in the loop might write to memory that aliases `p`. That requires alias analysis:

* **Conservative approach:** hoist loads only if the pointer is provably non-aliasing and not written in the loop.
* **Aggressive approach:** use flow-sensitive alias analysis to prove safety; combine with GVN/CSE to reuse loads instead of hoisting in some cases.

---

# Exceptions, volatile and concurrency

* Volatile loads/stores and accesses under a concurrent memory model cannot be hoisted unless language memory model allows it (in many languages it doesn’t).
* For languages with observable exceptions or signals, hoisting could change the point where an exception is raised — be conservative.

---

# Subexpression hoisting & related optimizations

LICM can operate at two granularities:

* **Whole-instruction hoisting** — move complete instructions that are invariant.
* **Subexpression hoisting** — pull out invariant parts of larger expressions:

  ```
  while (...) {
    t = (x * y) + f(i)
  }
  ```

  If `x*y` is invariant, hoist it:

  ```
  tmp = x*y
  while (...) {
    t = tmp + f(i)
  }
  ```

  Subexpression hoisting often requires splitting expressions into temporaries (and enabling later passes like CSE/GVN).

Related optimizations: strength reduction (e.g., convert `i*k` to incrementing accumulator), induction variable elimination — often done together with LICM.

---

# If/else motion, repeat loops and special cases (your TODOs)

* **if/else motion**
  Moving an `if (cond) {...}` entirely out of a loop is safe only if:

  1. The `if` body contains only loop-invariant statements, and
  2. The condition of the `if` does not depend on loop-variant variables or on state changed by the loop.
     If both hold you can hoist the `if` to the preheader (or wrap the hoisted code inside an `if` guarded by the loop entry condition as you showed). This requires checking both the `if` condition and the body for invariance.

* **repeat/until loops**
  `repeat { ... if (cond) break }` is harder because the loop may execute the body before any preheader code. You can only hoist code that is safe to execute *before* the first iteration — which means the hoisted code must preserve semantics if executed and not break intended behavior (for example, moving code that influences the exit condition can change behavior). Many compilers avoid hoisting out of such loops unless they can prove the loop has a detectable precondition.

* **To implement your example:** transforming `repeat` by inserting an `if` that guards hoisted computations is possible only if you can prove the `if` (the negated exit condition) holds for the first iteration; otherwise semantics change.

---

# Heuristics and cost model

Hoisting is not always beneficial:

* **Register pressure**: hoisting creates live values across loop iterations, which increases register pressure and can cause spills that negate benefits.
* **Code size**: cloning instructions into preheader increases code size — sometimes not worthwhile for tiny cheap ops.
* **Cost model**: prefer hoisting expensive computations (divides, pow, calls to pure functions, memory loads with proven no writes).
* Use profile information: hoist when loop is hot.

Typical heuristic: only hoist when the instruction cost > cost of keeping an extra live temp (spill probability weighted).

---

# Correctness checklist (before hoisting)

* Operands invariant in the loop.
* No side-effects or they are safe to duplicate/execute earlier.
* Memory loads: proven non-aliased by loop stores.
* The hoisted instruction executes on every original path (place in preheader or guarded by `if`).
* Moving it does not change exception ordering in a way the program can observe.

---

# Pseudocode — SSA friendly LICM (simplified)

```
compute dominator tree and natural loops
for each loop L (innermost to outermost):
  ensure L has a single preheader (create if necessary)

  changed = true
  while changed:
    changed = false
    for each instruction I in L (in topological/CFG order):
      if I is already hoisted or I has side-effects -> continue
      if all operands of I are:
         - defined outside L, or
         - are constants, or
         - are definitions already marked invariant
      and I is safe to execute earlier (no mem alias/store, no volatile, exception-safe):
         mark I as invariant
         move/clone I to preheader
         replace uses inside L with the hoisted definition
         changed = true
```

Notes:

* Iterate to fixed point because hoisting an instruction may make others invariant.
* In SSA, moving an instruction keeps it as one definition; in non-SSA IR you may need to create a new temp in the preheader and replace uses.

---

# Testing & verification

* Unit tests for canonical cases: arithmetic invariants, nested loops, hoisting across nested loops (must hoist to the nearest preheader dominating uses).
* Edge cases: loads with stores, volatile accesses, exceptions, pointer aliasing.
* Fuzz IR with interpreter check: verify semantics unchanged on random inputs.
* Profile before and after to confirm performance gains on hot loops.

---

# Interactions with other passes

* LICM + GVN/CSE: hoisted expressions may unify with other values (GVN will find equivalences).
* LICM + induction variable analysis: combined passes can eliminate multiplications by incrementing accumulators.
* LICM followed by dead-code elimination: move-only code that becomes unused can be removed.
* LICM should run after inlining (so hoisting can see across callee bodies) but before register allocation (so register pressure is considered).

---

# Practical pitfalls & gotchas

* Floating-point semantics: hoisting may change rounding/exception behavior — be conservative.
* Multi-threaded code / shared memory: must respect memory models (no hoisting of non-atomic loads that could race).
* Calls with observable side effects must not be hoisted (unless proven pure).
* Hoisting increases lifetime of values — may worsen register allocation.

---

# Summary

LICM moves loop-invariant computations out of loops to cut redundant work. Implementing it safely requires:

* accurate detection of invariance,
* checks for side effects and aliasing,
* inserting a preheader or guarding hoisted code with appropriate conditionals,
* a cost model to avoid hurting performance via register pressure.

In SSA form LICM is simpler and more powerful; combine it with GVN, CSE, and DCE to maximize benefit.

# Constant Propagation — a practical guide for compiler writers

Constant propagation is a fundamental local (and sometimes global) optimization used in compilers to replace variables known to hold constant values with those constants. Doing so simplifies expressions, exposes further optimization opportunities (like constant folding and dead-code elimination), and reduces runtime work.

This article explains what constant propagation is, why it matters, how compilers compute it, common algorithms and pitfalls, and gives practical examples and pseudocode you can use when implementing it.

---

# What constant propagation does (and why)

Constant propagation replaces occurrences of a variable by a constant when the compiler can be sure that the variable always holds that constant at that program point.

Simple benefits:

* Eliminates repeated computation (e.g., `pi = 22/7` → use `3.142857...` or precomputed `3.14`).
* Enables constant folding: `3 * (x + 2)` becomes `3 * 5` if `x` is `3`, then `15`.
* Reduces register/memory traffic by removing needless copies.
* Exposes dead code and simplifies control flow (e.g., `if (true)` branches become unconditional).

---

# When a value is “a constant”

A value is considered constant at a program point if **every reaching definition** of that variable assigns the *same* constant. If some reaching definition is unknown or different, the variable is not a constant there.

Two common scopes:

* **Local (basic-block) constant propagation** — simplest: within a basic block, values defined earlier are known and can be replaced.
* **Global constant propagation** — uses data-flow analysis (reaching definitions or a constant propagation lattice) across basic blocks and control-flow joins.

---

# Formal view: lattice and dataflow

Constant propagation can be framed as a forward dataflow analysis over a lattice per variable:

Lattice for each variable:

* `⊥` (bottom / UNDEF) — not yet assigned
* `c` (a specific constant)
* `⊤` (top / NAC: Not A Constant / varying or unknown)

Transfer function for statement `x = expr`:

* Evaluate `expr` under current mapping:

  * if `expr` evaluates to a concrete constant `c` (all operands constants), set `x := c`
  * else set `x := ⊤` (NAC)
    For other statements that don't redefine `x`, `x` keeps its value.

Meet at control-flow joins = lattice **meet** (greatest lower bound):

* meet(`c`, `c`) = `c`
* meet(`c1`, `c2` where c1 ≠ c2) = `⊤`
* meet(`⊥`, `c`) = `c` (if you treat ⊥ as initial)
* meet with `⊤` yields the other if `⊤` is top? (implementation conventions vary; commonly we initialize to `⊥` and treat `⊤` as NAC)

Analysis iterates until a fixed point (worklist, monotone framework). This formulation is equivalent to "constant propagation via abstract interpretation."

---

# Simple examples (step-by-step)

Example 1 — local/basic-block:

```
a = 30
b = 20 - a / 2
c = b * (30 / a + 2) - a
```

Propagate `a = 30`:

```
a = 30
b = 20 - 30 / 2      // replace a
c = b * (30 / 30 + 2) - 30
```

Then fold constants:

```
b = 20 - 15          // 30/2 = 15
b = 5
c = b * (1 + 2) - 30 // 30/30 = 1
c = 5 * 3 - 30
c = 15 - 30
c = -15
```

Example 2 — basic folding:

```
x = 12.4
y = x / 2.3
```

Replace `x`:

```
y = 12.4 / 2.3
```

Then fold constant division at compile-time if FP semantics allow it.

Example 3 — control-flow join (why global analysis is needed):

```
if (cond) { x = 1; } else { x = 2; }
y = x + 3;
```

At `y = x + 3`, `x` is not a constant (it can be 1 or 2), so no propagation. If both branches assigned `1`, then `x` could be propagated.

---

# Algorithms & implementation notes

1. **Local (single basic block)**

   * Walk statements top-to-bottom.
   * Keep a map `constValue[var]` (either constant or NAC).
   * When you see `v = const` → set `constValue[v] = const`.
   * When `v` assigned a non-constant expression involving NAC variables → set `constValue[v] = NAC`.
   * Replace uses where `constValue` has a concrete constant.
   * Very cheap and effective.

2. **Global (dataflow) — worklist approach**

   * Represent each variable’s abstract value in the lattice.
   * Initialize variables to `⊥` (or NAC/unknown depending on convention).
   * For each basic block, compute transfer of incoming mapping to outgoing mapping; enqueue successors when mapping changes.
   * Iterate until fixed point.
   * Complexity: number of updates × cost per update; practical and widely used.

3. **SSA (Static Single Assignment) makes it easier**

   * Each variable is assigned exactly once; uses of a variable directly refer to an SSA name.
   * Constant propagation reduces to substituting SSA names known to be constants.
   * Phi-nodes need special handling: `x = phi(a,b)` is constant only if all incoming values are the *same* constant.

4. **Sparse conditional constant propagation (SCCP)**

   * Combines constant propagation with control-flow information: if an `if` condition becomes a compile-time constant, unreachable branches can be pruned and more constants discovered.
   * More powerful: it propagates constants and marks unreachable code, then repeats.

---

# Pseudocode (simple global worklist)

```text
// mapping: var -> lattice value (Const(c) | NAC | UNDEF)
initialize mapping[var] = UNDEF for all vars
worklist = all basic blocks (or entry)

while worklist not empty:
  B = pop(worklist)
  inMap = meet of outgoing maps of predecessors
  outMap = transfer(inMap, B)
  if outMap changed:
    for succ in successors(B):
      add succ to worklist
```

`transfer(inMap, B)` = simulate statements in B using abstract values; when you see `x = expr`:

* evaluate expr under `inMap`:

  * if expr evaluates to a concrete constant `c` → set `x = c`
  * else set `x = NAC`
* update map as you go.

---

# Practical issues & caveats

* **Floating-point arithmetic & IEEE semantics**
  Constant folding/propagation of floats may change program behavior (signaling NaNs, rounding differences). Be conservative if strict IEEE behavior or exceptions must be preserved.

* **Side-effects**
  Do not reorder or eliminate expressions with side effects (function calls, I/O) even if operands are constant unless you can prove safety.

* **Aliasing / pointers / references**
  If a variable can be modified through a pointer alias or via another thread, you cannot treat it as constant unless alias analysis or memory model proves otherwise.

* **Volatile / concurrency**
  Volatile variables or shared memory in concurrent programs shouldn't be treated as compile-time constants.

* **Integer division & overflow**
  In languages with undefined behavior on overflow (e.g., signed integer overflow in C), compile-time evaluation must respect semantics.

* **Precision & representation**
  Replacing an expression by its decimal approximation (e.g., `22/7` → `3.14`) is lossy; better to fold to exact rational or a representation that preserves required semantics.

---

# Interactions with other optimizations

Constant propagation is often applied repeatedly with other passes:

* **Constant folding** — evaluate constant expressions to new constants.
* **Copy propagation** — propagate simple copies (`x = y`) together with constants.
* **Dead code elimination** — remove code that becomes unreachable or unused after propagation.
* **Strength reduction** — simplify operations (e.g., `x*2` → `x<<1`) when operands known.
* **SCCP** — best applied before many other passes because it can simplify control flow.

Order matters: do constant propagation and dead-code elimination iteratively for best results.

---

# Testing & verification

* Unit tests with contrived snippets (boundary cases: joins, phi nodes, floats, divisions-by-zero, volatile).
* Verify compiled code against interpreter/naive implementation for semantic equivalence on many inputs.
* Use regression tests to ensure optimizations don’t change observable behavior.

---

# Quick checklist for an implementation

* [ ] Start with local propagation inside basic blocks.
* [ ] Add global worklist dataflow if you need cross-block propagation.
* [ ] Use SSA or phi-aware logic to simplify global propagation.
* [ ] Be conservative with floats, side effects, and aliasing.
* [ ] Integrate constant folding so that newly formed constants are further propagated.
* [ ] Add SCCP if you want more aggressive, control-flow-aware propagation.
* [ ] Add tests for tricky cases (joins, undefined behavior, volatile, concurrency).

---

# Conclusion

Constant propagation is a low-cost, high-payoff optimization that replaces variables with compile-time constants when provable. Implemented as a simple local pass or as a global dataflow (or SCCP) analysis, it unlocks many other optimizations and often dramatically simplifies generated code. Be conservative around side effects, aliasing, and floating-point rules, and iterate with folding and dead-code elimination for the best results.

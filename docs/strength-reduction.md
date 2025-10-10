# Induction Variables & Strength Reduction — a practical guide for compiler writers

Induction variables and strength reduction are two tightly related loop optimizations that give big wins on inner loops. Induction-variable analysis identifies variables that change predictably each iteration; strength reduction replaces expensive operations on those variables (multiplication, division, costly computations) with cheaper updates (adds, shifts, pointer increments, or small constant work). Together they eliminate repeated expensive computation and expose further optimizations (LICM, GVN, CSE, DCE).

---

# Short summary

* **Induction variable:** a variable that changes by a fixed amount each loop iteration (e.g., `i = i + 1`).
* **Strength reduction:** replace expensive computations that depend on induction variables with cheaper recurrence updates (e.g., replace `k * i` inside loop with an accumulator that is incremented by `k` each iteration).
* Usual benefits: fewer multiplications/divisions, fewer address computations, lower instruction count, faster runtime.

---

# Common patterns

1. **Basic induction variable (BIV)**
   A variable `i` with `i_{t+1} = i_t + c` (or `i_t - c`) every iteration. Example:

   ```c
   for (int i = 0; i < n; ++i) { ... }
   ```

2. **Derived induction variable (DIV)**
   A variable computed as an affine function of a BIV: `j = a * i + b`. Often appears as loop index scaling:

   ```c
   for (int i = 0; i < n; ++i)
       A[a*i + b] = ...;
   ```

   Here `a*i + b` is a DIV.

3. **Induction-based address computation**
   Array indexing `base + i*elem_size` — can be converted into pointer increment or accumulator.

4. **Polynomial induction variables**
   Some variables follow quadratic/cubic recurrences but can be represented as linear combinations of simpler induction variables (less common; more advanced analysis required).

---

# Typical strength-reduction transformations

## 1 — Replace `k * i` with accumulator

Before:

```c
for (int i = 0; i < n; ++i) {
  y = k * i;
  // use y
}
```

After (strength reduction):

```c
int y = 0;            // y = k * 0
for (int i = 0; i < n; ++i) {
  // use y
  y += k;             // update for next i
}
```

Proof: initially y = k*0. After `t` iterations, y = k*t.

## 2 — Replace scaled index with pointer increment

Before:

```c
for (int i = 0; i < n; ++i)
  out[i] = in[i] * 2;         // uses in + i
```

After:

```c
int *p = in, *q = out;
for (int i = 0; i < n; ++i) {
  *q++ = (*p++) * 2;
}
```

Benefit: removes `i*elem_size` multiplications inside the loop.

## 3 — Replace division by multiplication when safe

If `i / d` appears but `d` is power-of-two, replace with right shift; if not power-of-two, consider multiplicative inverse trick (careful about signed/unsigned and overflow semantics).

## 4 — Hoist invariant factors and update accumulator

For `j = a*i + b`, hoist `b` or `a` if invariant and maintain `j` via `j += a` each iteration.

---

# Algorithm sketch for a compiler pass

1. **Detect BIVs**

   * Build CFG and identify loops (natural loops via backedges).
   * For each loop, scan definitions to find variables `v` updated by a simple recurrence:

     * `v_new = v_old + c` or `v_new = v_old - c` where `c` is loop-invariant.
   * These are basic induction variables (BIVs).

2. **Find derived induction variables (DIVs)**

   * Look for variables defined as `u = a * v + b` where `v` is a BIV and `a,b` are invariant constants (or invariants computed prior).
   * A more general pattern: linear combination of BIVs (e.g., `u = a1*v1 + a2*v2 + b`).

3. **Check safety**

   * Ensure moving computation or replacing it with accumulator preserves semantics:

     * No aliasing that changes values (if `v` or `a` can be changed via pointers inside loop, be conservative).
     * No observable side-effects (no volatile/IO behavior).
     * For signed integers, consider overflow semantics: if language requires wraparound vs UB, ensure transformation preserves semantics.
     * Floating-point: be careful due to rounding and NaN/Inf semantics.

4. **Create accumulators / transform**

   * For each DIV `u = a*i + b`:

     * Precompute `u0 = a * i0 + b` before loop (where `i0` is initial BIV value).
     * Replace uses of `u` inside loop with `u_acc`.
     * Update `u_acc += a*c` at the end/beginning of each iteration (where `c` is BIV step).
   * For address calculations, generate pointer `p = base + i0*scale` and `p += scale` per iteration.

5. **Run cleanup passes**

   * LICM (hoist invariants created), GVN/CSE, DCE, register allocation tuning.

6. **Repeat / nested loops**

   * Process inner loops first (bottom-up on loop nesting). This avoids hoisting something that depends on an inner variable later.

---

# Example (step-by-step)

Input:

```c
int k = 5;
for (int i = 0; i < n; ++i) {
  int j = 3 * i + 7;
  A[j] = ...;
}
```

Transform:

1. `i` is a BIV with step `1`.
2. `j` is DIV = `3*i + 7`.
3. Precompute `j0 = 3*0 + 7 = 7` (or compute `j = 3*i0 + 7` if `i` starts elsewhere).
4. Inside loop: use `j`, then `j += 3`.

Result:

```c
int j = 7;
for (int i = 0; i < n; ++i) {
  A[j] = ...;
  j += 3;
}
```

---

# Pseudocode for a simple pass

```
for loops in program (innermost-first):
  find basic induction variables B = {}
  for each instruction def v = op(...) in loop:
    if op is v_old + c (c invariant) or v_old - c:
      mark v as BIV with step = c

  find derived induction vars:
  for each def u = expr in loop:
    if expr is a * v + b  and v ∈ B and a,b invariant:
      mark u as DIV with coeff a and base b

  for each DIV u:
    let v be its BIV with initial value v0
    compute u0 = a*v0 + b  // move this to preheader
    replace uses of u in loop with temp u_acc
    insert u_acc = u0 in preheader
    insert u_acc += a*step at loop end (or start) 
```

Notes: real compilers must handle more patterns and do SSA-aware renaming.

---

# Important correctness caveats

* **Overflow and undefined behavior**
  In languages like C, signed integer overflow is UB. Changing the evaluation order or lifetime of overflow-producing expressions can break correctness. Be conservative: avoid transforming when overflow semantics might differ.

* **Floating-point arithmetic**
  Rounding and exceptions (NaN, -0.0, Inf) make rearrangements observable. Some compilers only allow transformations when flags (like `-ffast-math`) permit relaxed semantics.

* **Alias / memory side effects**
  If a variable used in the DIV can be modified via pointers inside the loop, you cannot assume it is invariant. Use alias analysis to prove safety.

* **Volatile and I/O**
  Do not move/transform volatile memory accesses or operations with I/O side effects.

* **Concurrent code**
  Shared-memory races make invariance assumptions invalid; transformations require synchronization guarantees.

---

# Interactions with other optimizations

* **LICM:** often used together. Strength reduction can create invariants hoistable by LICM (or vice versa).
* **GVN/CSE:** after strength reduction, identical accumulators or hoisted values may be merged by GVN.
* **Register allocation:** strength reduction increases live ranges (an accumulator lives across the loop) and may introduce register pressure; weigh cost/benefit.
* **Induction variable elimination (IVE):** more advanced pass can remove redundant induction variables by rewriting all uses to one canonical induction and removing others.

---

# Heuristics / cost model

* Prefer strength reduction on expensive ops (multiplication/division) rather than trivial adds.
* Prefer pointer-increment conversion for array accesses (saves index multiply).
* If accumulator would live across the loop and likely spill to memory, the transformation might slow code — add spill-aware heuristics or use a register-pressure model from the register allocator.
* Use profiling info: prioritize hot loops.

---

# Test cases & verification

* **Correctness tests**: loops with BIVs/DIVs, nested loops, loops where variables are modified via pointers (should not transform), overflow edge-cases.
* **Performance tests**: microbenchmarks measuring multiply/divide heavy loops with and without transformation.
* **Randomized IR fuzzing**: compare outputs between original and transformed code using a reference interpreter.
* **Interaction tests**: run pipeline LICM → strength reduction → GVN → DCE and ensure semantics preserved.

---

# Practical implementation tips

* Work on SSA IR — it simplifies aliasing of named definitions and phi handling.
* Process loops from innermost to outermost.
* Build an induction-variable graph: BIV nodes and DIV nodes pointing to their base BIVs and coefficients — simplifies reasoning and transformations.
* Provide a way to revert or guard transformations based on register-pressure estimates.
* Add diagnostics to show when an induction variable was transformed — helpful for debugging.

---

# Quick reference examples

C (before):

```c
for (int i = 0; i < n; ++i) {
  int idx = i * 8;
  out[idx] = in[idx] + 1;
}
```

After (pointer increment):

```c
char *p = out, *q = in;
for (int i = 0; i < n; ++i) {
  *(int*)p = *(int*)q + 1;
  p += 8; q += 8;
}
```

C (before):

```c
for (int i = 0; i < n; ++i)
  sum += 7 * i;
```

After:

```c
int t = 0;
for (int i = 0; i < n; ++i) {
  sum += t;
  t += 7;
}
```

---

# Final notes

Induction-variable analysis plus strength reduction are classic, high-payoff compiler optimizations. They are conceptually simple but need careful handling of language semantics (overflow, FP), aliasing, and register pressure. When implemented on SSA IR and combined with LICM, GVN, and DCE, they significantly reduce inner-loop cost — especially in numeric, array-processing, and systems code.

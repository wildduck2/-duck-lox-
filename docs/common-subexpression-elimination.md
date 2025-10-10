# Common Subexpression Elimination — a practical guide for compiler writers

Common Subexpression Elimination (CSE) is a classic compiler optimization that finds expressions which are computed more than once and eliminates the redundant recomputation by reusing the previously computed value. It’s a high-payoff, conceptually simple optimization that reduces instruction count, memory/register traffic, and often enables further optimizations (folding, strength reduction, dead-code elimination).

---

# What CSE does (intuitively)

If the compiler can prove that an expression `E` was previously computed and the operands of `E` haven’t changed since that computation, then all subsequent occurrences of `E` can be replaced with the previously computed value (usually stored in a temporary). Example:

Before:

```
a = 10
b = a + 1 * 2
c = a + 1 * 2
d = c + a
```

After local CSE:

```
a = 10
b = a + 1 * 2
c = b
d = c + a   // or d = b + a
```

You avoid recomputing `a + 1 * 2`.

---

# Why it matters

* Reduces CPU work (less arithmetic).
* Saves memory / register pressure when combined with register allocation.
* Exposes further optimizations (if an expression becomes a constant after folding, other uses can fold too).
* Particularly effective for expensive operations (division, function calls that are pure, complex pointer arithmetic).

---

# Two flavors: local vs global

**Local CSE**

* Works inside a single basic block (no control-flow joins).
* Simpler and cheap: scan the block left-to-right, keep a table mapping expressions → temporary/value, replace later occurrences.

**Global CSE**

* Across basic blocks and control-flow joins.
* More complex: must prove that *every* path reaching the second occurrence preserves the operands’ values (i.e., no redefinition or aliasing).
* Techniques: Global Value Numbering (GVN), SSA-based optimizations, and Sparse Conditional CSE/SCCP-like approaches.

---

# Key challenges & correctness conditions

You may only reuse a previously computed value if:

1. **Operands haven't changed**: no intervening assignment to any operand (or to aliases).
2. **No interfering side-effects**: expressions with side effects (function calls, volatile loads, I/O) must be treated specially.
3. **Memory/aliasing safety**: loads from memory can only be reused if you can prove memory wasn’t written to a location that could alias the load address.
4. **Language semantics**: floating-point semantics, exceptions, and undefined behavior constraints can make naïve re-use incorrect.

Be conservative where uncertainty exists.

---

# Common detection techniques

## 1 — Local scanning (hashing expressions)

* Walk a basic block left-to-right.
* Maintain a hash table mapping an expression key (operator + operand identities) to a temp/value.
* On seeing expression `E`:

  * If found in table → replace with temp.
  * Else compute and insert mapping.
* Works well for local CSE, especially with simple expressions.

## 2 — Directed Acyclic Graph (DAG) construction (basic-block)

* Build an expression DAG for the block; common subexpressions map to the same node.
* Generate code from DAG ensuring each node computed only once.
* Natural way to also do local algebraic simplifications.

## 3 — Value numbering (local and global)

* Assign a "value number" to each computed expression and operand.
* Two expressions with the same value number are equivalent and one can be replaced by the other.
* Global Value Numbering (GVN) extends the idea across basic blocks, often with iterative refinement.

## 4 — SSA-based CSE

* In SSA, each variable is defined once — easier to reason about equivalence.
* Replace an expression whose SSA operands are constant or identical SSA names.
* Phi nodes must be considered: `x = phi(1,1)` is constant, `x = phi(1,2)` is not.
* SSA + GVN is a powerful combo widely used in modern compilers.

## 5 — Sparse Conditional (or Semantic) CSE

* Uses control-flow knowledge (like SCCP) to discover equivalences that are only true on some paths and propagate them.
* More powerful but more complex.

---

# Example: value numbering (simple)

Block:

```
t1 = a + b
t2 = c + d
t3 = a + b
t4 = t2 + t3
```

Value numbers:

* `VN(a)=1`, `VN(b)=2` → `VN(a+b)=3` (t1)
* `VN(c)=4`, `VN(d)=5` → `VN(c+d)=6` (t2)
* `a+b` again → lookup → VN = 3 (t3 can be replaced by t1)
* Replace `t3` uses with `t1`.

---

# Pseudocode — simple local CSE (basic block)

```
expr_table = {}   // map: expression key -> temp
new_block = []
for instr in block:
  if instr is computation of expression E with operands op1..opn:
    key = make_key(E.op, op1.id, ..., opn.id)
    if key in expr_table:
      replace instr's dest uses with expr_table[key]
      // optional: remove instr if dest unused
    else:
      expr_table[key] = instr.dest
      append instr to new_block
  else if instr writes to variable v:
    // conservative invalidation: removes any expr whose key mentions v
    invalidate expr_table entries that use v
    append instr to new_block
  else:
    append instr to new_block
```

Key points:

* `make_key` must encode operation and operand identities (SSA name or canonical id).
* Invalidate entries on writes to any operand.

---

# Global Value Numbering (GVN) — brief sketch

* Initialize value numbers for constants and arguments.
* Iterate over basic blocks in a worklist:

  * For each instruction, compute VN of its operands and operator.
  * If a previous VN matches operator+operand VNs → replace current with earlier value.
  * Update mappings at block entries/exits; iterate to fixed point.
* Handle φ-nodes carefully: their VN depends on incoming VN values and can require merging.

GVN variants trade precision for speed; some use congruence closure, others hashed value numbers with iterative refinement.

---

# Memory loads/stores and aliasing

* Loads are expressions too, e.g. `r = load [p]`. You can reuse a load only if:

  * You can prove neither `p` nor any memory location that may alias `p` has been written since the original load.
* Use alias analysis (flow-sensitive or -insensitive) to determine safety.
* If unsure, do not eliminate.

---

# Side effects, exceptions, and floating point

* Do not eliminate expressions that can raise exceptions (e.g., divide-by-zero) unless you can prove the exception is impossible or preserved.
* Floating-point reordering/constant folding can change semantics due to rounding; be conservative if language or flags forbid transformations.
* Function calls: only reuse a call result if the call is pure (no side effects and deterministic) and its arguments haven’t changed.

---

# Interaction with other passes

* **Constant folding**: after replacing subexpressions, folding may turn expressions into constants → more opportunities.
* **Copy propagation**: helps by replacing temporaries with originals, making expression keys match.
* **Dead-code elimination**: removes now-unused duplicated computations or temporary assignments.
* **Register allocation**: fewer computations can reduce register pressure, but introducing temporaries to hold common results may increase pressure — the code generator/optimizer must balance this.

Cost model: CSE should be guided by estimated cost/benefit (is recomputation cheaper than storing a temp and loading it later?). Some compilers only apply CSE for operations above a cost threshold.

---

# Practical heuristics

* Prefer CSE for expensive ops (division, memory loads, calls to pure functions).
* Avoid creating long-lived temporaries that increase register pressure unless payoff is clear.
* Use profile information if available (hot paths deserve more aggressive CSE).
* For global CSE, require a proof that all reaching definitions agree or use SSA/GVN to reason about equivalence.

---

# Limitations and pitfalls

* **Aliasing / pointers**: conservative treatment reduces opportunities.
* **Register pressure**: storing common values may hurt if they spill.
* **Floating point & undefined behavior**: must respect language semantics.
* **Code bloat**: naive duplication of temporaries across code paths may bloat code size; use heuristics.

---

# Testing & verification

* Unit tests for local cases and joins (phi nodes).
* Tests for aliasing: loads/stores that should or shouldn't be reused.
* Edge cases: exceptions, division by zero, NaNs, infinities.
* Use randomized IR fuzzing + reference interpreter to check semantic equivalence.
* Regression tests for performance: ensure CSE improves measured hotspots.

---

# Implementation checklist

* [ ] Implement local basic-block CSE (DAG or hash table).
* [ ] Add value numbering for increased matching power.
* [ ] Integrate with SSA and implement GVN for global equivalence.
* [ ] Add alias analysis for safe load/store reuse.
* [ ] Respect side-effects, volatiles, and language-specific semantics.
* [ ] Add cost model (avoid increasing register pressure unnecessarily).
* [ ] Add unit and regression tests, including tricky corner cases.
* [ ] Measure performance and code size; tune heuristics.

---

# Conclusion

Common Subexpression Elimination is a practical optimization with straightforward local forms and powerful global variants (GVN, SSA-based). When implemented carefully — respecting aliasing, side effects, and cost trade-offs — it can reduce work dramatically and enable downstream optimizations. Modern compilers often implement several complementary techniques (value numbering, SCCP, and SSA-aware transformations) to capture the broadest set of opportunities safely.

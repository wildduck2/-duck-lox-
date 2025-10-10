# Global Value Numbering (GVN) — a practical guide

Global Value Numbering is a powerful optimization that detects computations that produce the same *value* (not just the same syntactic expression) across a whole procedure and reuses the first computed value instead of recomputing it. When combined with SSA, copy propagation, and constant folding, GVN can eliminate large classes of redundant work and expose many more downstream optimizations.

Below is a compact, practical guide you can use to understand, implement, and test GVN.

---

# Quick intuition

Programmers name temporaries however they like. Value numbering ignores names and asks: *which computations are equivalent?*
If two computations always produce the same runtime value along all possible execution paths that reach a use, they share a value number and one can be replaced by the other.

Example (before):

```
u0 = a + b
...
u1 = a + b
...
u2 = phi u0 u1
u3 = a + b    // redundant
```

After GVN (plus DCE/phi simplification) you'll keep only one `a + b` and reuse its value number.

---

# Why SSA matters

SSA (Static Single Assignment) gives each definition a unique name and makes it trivial to refer to a specific computation as a value: value numbers can initially be SSA names. Without SSA, you must reason about redefinitions of variables across multiple blocks — which becomes complicated and error-prone.

Typical flow:

1. Convert to SSA (insert φ nodes where necessary).
2. Run GVN on SSA-form IR.
3. Optionally convert out of SSA and run DCE.

---

# Two broad families of GVN algorithms

1. **Hash-based techniques** (dominators + hashing)

   * Hash expressions by opcode and the *value numbers* of operands, look up in a table, assign/value-number or reuse existing.
   * Can be fast and easily extended to do copy propagation and constant folding.
2. **Partitioning / congruence closure algorithms**

   * Maintain equivalence classes of expressions and refine them iteratively (more global reasoning; can be more precise but more complex).

Both are usually run on SSA input. The dominator-based (hash) approach is a practical and common choice.

---

# Dominator-based (DVNT) GVN — core idea

Traverse the dominator tree and maintain a hash table mapping normalized expressions (op + operand value numbers) to a canonical value number (often the SSA name where it first appeared). When you reach a block:

* Process φ nodes first: canonicalize trivial or redundant φs and assign value numbers.
* For each instruction, compute operand value numbers, normalize/simplify, then:

  * If hash contains the same normalized expression → assign the existing value number to this instruction (replace with an id / copy).
  * Else create a new entry mapping expression → this instruction’s SSA name (its value number).
* Propagate value numbers into phi operands of dominated children where appropriate.
* Recurse into dominator children.
* When returning, undo hash entries added by this block (so the hash reflects the current path in dominator traversal).

This dominator-locality ensures correctness when working on SSA.

---

# Pseudocode (DVNT-like)

```
DVNT_GVN(block b):
  enter scope for b (push list-of-added-keys)
  // 1. phi nodes
  for phi in b.phis:
    if phi is trivial (same argument/value number everywhere):
      replace phi with that value number (and mark phi removable)
      continue
    assign value_number(phi.name) = phi.name
    add to hash: key(phi) -> phi.name

  // 2. normal instructions
  for instr in b.instructions:
    get operand VNs: v1 = valnum(op1), v2 = valnum(op2), ...
    simplified = try_constant_fold_or_simplify(instr.op, v1, v2, ...)
    if hash contains simplified:
      existing = hash[simplified]
      set value_number(instr.name) = existing
      replace instr with a copy/id instr referencing existing
    else:
      set value_number(instr.name) = instr.name
      add hash[simplified] = instr.name
      (maybe replace instr with folded const if folded)
  // 3. update phi operands in dominated children
  for child in dominator_children(b):
    for phi in child.phis:
      replace phi operand corresponding to predecessor b with value_number_of_computed_value

  // 4. recurse
  for child in dominator_children(b):
    DVNT_GVN(child)

  // 5. leave scope: remove hash entries added here
  undo hash entries added in this block
```

Notes:

* `value_number(x)` returns either a constant literal (folded) or an SSA name that canonically identifies the value.
* `try_constant_fold_or_simplify` performs constant folding and small rewrites (commutativity normalization, canonical operand order for commutative ops).
* You must treat id/copy instructions specially to implement copy propagation: an `id y` maps directly to `value_number(y)`.

---

# Handling φ nodes

* A φ is **meaningless** if all its incoming value numbers are identical — then the φ can be removed and replaced by that value.
* Two φ nodes can be redundant copies of each other (same operands/value numbers) — the second can reuse the first’s value number.
* Because φ arguments come from different predecessors, you must ensure you substitute the *value numbers* computed in each predecessor — that's why the algorithm updates phi operands for dominated children before recursing.

---

# Copy propagation & constant folding in GVN

GVN is a natural place to combine:

* **Copy propagation:** handle identity (`id`) instructions by assigning their value number to be the argument’s value number so later uses point to the original value.
* **Constant folding:** if operand VNs are constants, compute the result at compile time, and map the result’s value number to the constant.

Important: be conservative about folding operations that could raise exceptions (division-by-zero) — either avoid folding or ensure semantics preserved.

---

# Correctness considerations

* SSA simplifies correctness guarantees: a single SSA name equals a single definition.
* Must be careful with memory operations, volatile, or operations with side effects. Only pure computations are eligible for GVN replacement.
* Floating-point: equality of results is tricky because of rounding/NaN/device-specific semantics — be conservative unless language flags allow transformations.
* Ensure phi operand replacements use **value numbers computed in the corresponding predecessor**.

---

# Practical details & heuristics

* **Normalization**: canonicalize expressions (e.g., `a+b` and `b+a` map to the same key) to increase matches.
* **Commutativity & algebraic identities**: use canonical ordering; but beware of non-associativity and floating-point issues.
* **Scope of hashing**: the dominator scope approach localizes hashes to the current path; that keeps algorithm memory bounded and correct.
* **Cost model**: do not aggressively replace cheap ops with temporaries that increase register pressure. Prefer eliminating expensive redundancy (loads, divisions, calls to pure functions).
* **Interaction with DCE**: GVN often creates many id/copy or now-unused constants — run DCE after GVN.
* **Iteration**: running GVN after other simplifications (SCCP, LICM, CSE, constant propagation) can yield more opportunities. Many compilers run several optimization passes in a pipeline.

---

# Example (short, concrete)

Input (SSA):

```
B1:
  u0 = add a b
  v0 = add c d
  w0 = add e f
  br cond B2 B3

B2:
  x0 = add c d
  jmp B4

B3:
  u1 = add a b
  x1 = add e f
  jmp B4

B4:
  u2 = phi u0 u1   // args from B2,B3
  x2 = phi x0 x1
  z0 = add u2 x2
  u3 = add a b
```

After GVN + phi simplification:

* `x0` / `v0` / `x1` / `w0` map to existing v0/w0 entries.
* Meaningless/redundant φs eliminated (u2 where both args same).
* Final: `z0` uses `u0` and `x2` (reused), and the repeated `add a b` removed.

This matches the worked example in the paper you posted.

---

# Testing & evaluation

* **Correctness tests**: small hand-crafted cases for φ simplification, cross-block redundancy, commutativity, and copy propagation.
* **Stress tests**: random IR generation and equivalence checking (run interpreter vs optimized program).
* **Benchmarks**: count instructions before/after, measure runtime for hot paths. Use DCE + other passes to measure combined effect.
* **Edge cases**: division by zero, floating-point NaN/Inf, loads/stores aliases, volatile/memory-mapped I/O.

---

# Integration checklist

* [ ] Convert to SSA (insert φs at dominance frontiers, rename).
* [ ] Build dominator tree and traversal order (reverse-postorder on CFG helps).
* [ ] Implement DVNT_GVN hash table with push/pop scoping for blocks.
* [ ] Add normalization rules (commutativity, constant canonicalization).
* [ ] Extend to copy propagation by handling id ops.
* [ ] Add constant folding (with conservative error handling).
* [ ] Post-process: remove redundant φs, run DCE, maybe run register allocation.
* [ ] Add heuristics/cost model and alias checks for loads.

---

# Pitfalls & gotchas

* Failing to order traversal correctly in SSA renaming can generate incorrect phis (as the Bril team noticed).
* Not handling missing definitions (dynamic languages or ill-typed input) — consider pre-inserting defaults or bailouts.
* Over-aggressive folding can change program trapping behavior — handle exceptions carefully.
* Register pressure: replacing recomputation with temporaries isn’t always a win.

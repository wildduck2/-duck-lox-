# Loop unrolling — practical guide for programmers & compiler writers
—

Loop unrolling is a simple, long-standing loop transformation: replace a loop body that runs many times with a version that repeats the body multiple times per iteration (or fully expands it). The goal is to reduce loop-control overhead (branch/test/increment) and create bigger straight-line chunks of code that the CPU and later passes (vectorizers, instruction-schedulers) can optimize better.

Below is a compact, practical reference: what it is, how it’s done, common techniques, examples, benefits and pitfalls, and when to use it.

---

# What it is (quick)

Given:

```c
for (int i = 0; i < N; ++i) {
  work(i);
}
```

Unroll by factor 4:

```c
for (int i = 0; i + 3 < N; i += 4) {
  work(i);
  work(i+1);
  work(i+2);
  work(i+3);
}
for (; i < N; ++i)           // handle remainder
  work(i);
```

Each loop iteration does 4 `work(...)` calls instead of 1, so the number of branch/test/increment operations is reduced ~4×.

---

# Types of unrolling

* **Complete unrolling:** replace the loop entirely with repeated body copies (works when `N` is known and small).
* **Partial unrolling:** keep a loop but repeat the body several times per iteration (most common).
* **Runtime (dynamic) unrolling:** generate code that decides at runtime how to handle remainder, or use prologue/epilogue to align to a multiple.
* **Duff’s device:** a creative C technique interleaving `switch` and `do/while` to unroll with a compact remainder handler (historical/interesting but less readable).
* **Compiler-driven vs hand-crafted:** compilers often do automatic unrolling; programmers can also write unrolled versions.

---

# Key implementation techniques

1. **Choose an unroll factor `k`** — common values: 2, 4, 8. Balance between reduced loop overhead and code-size/register-pressure growth.
2. **Handle remainder** — use a tail loop (`for (; i < N; ++i)`), prologue/epilogue (do some iterations before entering bulk loop), or switch-based schemes.
3. **Loop peeling** — run a few iterations first to make the main loop aligned or to remove boundary checks, then run a simplified unrolled loop.
4. **Maintain semantics** — be careful with loop-carried dependencies, exceptions, and memory fences.
5. **Preserve debugability & readability** — prefer compiler pragmas or macros when possible.

---

# Pseudocode (partial unroll by k)

```
i = 0
limit = N - (N % k)
while i < limit:
  repeat j from 0..k-1:
    work(i + j)
  i += k

while i < N:
  work(i)
  i += 1
```

---

# Example — C unroll by 4

Before:

```c
for (int i = 0; i < N; ++i)
  A[i] += B[i];
```

After:

```c
int i = 0;
int limit = N - (N % 4);
for (; i < limit; i += 4) {
  A[i]   += B[i];
  A[i+1] += B[i+1];
  A[i+2] += B[i+2];
  A[i+3] += B[i+3];
}
for (; i < N; ++i)
  A[i] += B[i];
```

---

# Duff’s device (compact remainder handling) — C

```c
int n = (N + 7) / 8;
switch (N % 8) {
case 0: do { A[i] = ...; ++i;
case 7:      A[i] = ...; ++i;
case 6:      A[i] = ...; ++i;
case 5:      A[i] = ...; ++i;
case 4:      A[i] = ...; ++i;
case 3:      A[i] = ...; ++i;
case 2:      A[i] = ...; ++i;
case 1:      A[i] = ...; ++i;
        } while (--n > 0);
}
```

Useful historically; modern compilers generally generate clearer code via prologue/epilogue or runtime checks.

---

# Benefits

* **Reduced branch/test overhead** — fewer loop control instructions executed.
* **Better instruction-level parallelism** — larger basic blocks let the CPU scheduler exploit ILP.
* **Enables vectorization** — once you have a regular repeated pattern, auto-vectorizers can generate SIMD.
* **Fewer branch mispredictions** in tight loops (amortized).
* **Potentially fewer address computations if you convert multiplications into increments.**

---

# Costs / drawbacks

* **Code size growth** — unrolling multiplies the body size; excessive unrolling causes cache pressure (instruction cache misses).
* **Increased register pressure** — more live temporaries may increase spills, hurting performance.
* **May prevent other optimizations** or hamper instruction scheduling if it increases register pressure or code complexity.
* **Complex semantics** — unrolling loops with side effects, exception-throwing operations, or aliasing requires caution.
* **Diminishing returns** — after a point, extra unrolling gives little or negative benefit.

---

# Interaction with other optimizations

* **Auto-vectorization:** a very common reason to unroll — vectorizers often rely on unrolled or peeled loops to safely apply SIMD.
* **LICM / GVN / CSE:** unrolling can expose redundant computations or invariants, enabling further optimization.
* **Register allocator:** unrolling increases live ranges; consider register-pressure modeling or let the register allocator guide unroll decisions.
* **Profile-guided optimizations (PGO):** use runtime profile data to decide unroll factor for hot loops.

---

# Compiler support & pragmas (practical)

* Many compilers perform **automatic unrolling** using heuristics (`-O2/-O3`, loop hints).
* Common pragmas/attributes: `#pragma unroll` / `#pragma GCC unroll N` or `__attribute__((optimize("unroll-loops")))` (exact syntax varies by compiler).
* Use **profile-guided** builds or target-specific tuning to get best results.
* Modern compilers often combine unrolling with vectorization and will choose factors that balance performance and code size.

---

# When to unroll (rules of thumb)

* Inner, hot loops (measure with a profiler).
* Loops where body is small and loop-control overhead is a measurable fraction of work.
* Loops that are amenable to vectorization after unrolling/peeling.
* Avoid unrolling large loop bodies or loops that will blow instruction cache or register pressure.

---

# Safety & correctness considerations

* Preserve precise semantics for exceptions and side effects — do not unroll across operations that change program behavior if moved.
* For floating-point math, be aware of reordering/associativity differences (may produce slightly different rounding).
* For multithreaded code, volatile/atomic operations and memory ordering constraints limit safe transformations.
* Use alias analysis for memory accesses: if `work(i)` may alias `work(i+1)`, you cannot assume independence.

---

# Measuring success

* Use microbenchmarks but also measure in realistic workloads — sometimes microbenchmarks mislead.
* Measure cycles per iteration, instruction counts, IPC, cache miss rates, and register spill stats.
* Compare performance across multiple unroll factors, and prefer PGO.

---

# Practical advice

* Let the compiler try automatic unrolling first; inspect generated assembly (`-S` or `objdump`) and measure.
* If compiler misses an important opportunity, try `#pragma unroll N` or hand-unroll small, critical loops.
* Keep readability in mind: prefer macros or small helper functions for hand-unrolled code, or annotate with comments explaining why.
* Use PGO and measure on real data; unroll factor that helps in one case can hurt in another.

---

# Quick checklist before unrolling by hand

* Is this loop a hot inner loop? (profile)
* Is the body small and independent across iterations?
* Will unrolling fit comfortably in instruction cache?
* Can the register allocator handle extra live values without excessive spills?
* Are there side effects, exceptions, or volatile/atomic ops? If yes — be conservative.
* Test for numerical/semantic equivalence if reordering arithmetic.

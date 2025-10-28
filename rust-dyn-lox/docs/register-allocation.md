# Register Allocation Algorithms — Explained Clearly

### Purpose

Register allocation is a *core optimization phase* in compiler backend design.
Goal: map a potentially large number of program variables (temporaries) to a limited number of **machine registers**, while minimizing:

* Memory traffic (LOAD/STORE),
* Spills (when registers run out),
* And maintaining correctness.

Registers are **fast**, but **limited** — hence efficient allocation is critical for performance.

---

## 1️⃣ Naïve Register Allocation

### Idea

Assume **no long-term register allocation**: every operation loads its operands from memory into registers, performs the operation, then stores the result back.

```c
a = b + c
d = a
c = a + d
```

#### Example — Machine Code

```
LOAD R1, b
LOAD R2, c
ADD  R1, R2
STORE R1, a

LOAD R1, a
STORE R1, d

LOAD R1, a
LOAD R2, d
ADD  R1, R2
STORE R1, c
```

### Advantages

* **Simple to implement** — no data-flow or live-range analysis needed.
* **Requires few registers** — only 1–2 at a time.
* **Predictable** and easy to debug.

### Disadvantages

* **Too many memory operations** (LOAD/STORE after every operation).
* **Poor performance** — memory accesses dominate.
* Not used in modern compilers except as a **fallback** or **educational baseline**.

---

## 2️⃣ Linear Scan Register Allocation

### Idea

Scan variables **linearly** through their *live ranges* (from where they are defined until last used).
Allocate registers to active variables.
If registers run out → **spill** the variable with least future use.

* Used in **JIT compilers** (like LLVM’s fast path, HotSpot, etc.) because it’s **fast and simple**.
* Works well for **SSA form** (Static Single Assignment).

#### Steps

1. Compute **live intervals** (from liveness analysis).
2. Sort variables by start position.
3. Iterate linearly through code:

   * When a variable’s interval starts → allocate a register (if available).
   * When it ends → free the register.
   * If no register available → **spill** one variable (usually least recently used or furthest next use).

#### Example

```c
a = b + c
d = e + f
d = d + e
if (a == 0) goto L0
b = a + d
goto L1
L0: b = a - d
L1: i = b
```

At most **4 variables are live simultaneously**, so **4 registers** suffice.

### Advantages

* **Linear time** — very fast compared to graph-coloring.
* **Simple implementation**.
* **Good for JITs** and dynamic compilers.

### Disadvantages

* Doesn’t account for **lifetime holes** (periods when a variable is not live).
* May **overestimate** needed registers.
* Spilling may not always be optimal.

---

## 3️⃣ Graph Coloring Register Allocation (Chaitin’s Algorithm)

### Idea

Treat register allocation as a **graph coloring problem**.

* Each **variable** = a **node**.
* An **edge** = two variables live at the same time (interfere).
* Assign each node a **color** (register) such that adjacent nodes have different colors.
* The minimum number of colors needed = minimum number of registers required.

#### Steps

1. Build **interference graph** from live ranges.
2. Simplify:

   * Remove nodes with degree `< k` (fewer neighbors than registers).
   * Push them onto a stack.
3. Spill:

   * If no such node exists, mark one for **spilling** (store in memory).
4. Select:

   * Pop nodes from the stack and assign colors ensuring no adjacent node shares the same color.
5. If spilling occurs, re-run with modified live ranges.

#### Example

| Step        | Action                               | Result                                         |
| ----------- | ------------------------------------ | ---------------------------------------------- |
| Build graph | Nodes = {a, b, c, d, e, f, i}        | Edges connect live-at-same-time vars           |
| Simplify    | Remove nodes of degree < k (say k=4) | Stack = [i, b, d, a, ...]                      |
| Color       | Assign 4 colors (registers)          | Each node gets unique register, unless spilled |

### Advantages

* **Produces optimal allocation** (for small graphs).
* Considers **lifetime overlaps** precisely.
* Minimizes **spills** intelligently.
* Widely used in **static compilers** (e.g., GCC, LLVM, Rustc backends).

### Disadvantages

* **Expensive** (graph building + coloring ≈ O(n²)).
* Requires **accurate liveness analysis**.
* **Spill decisions** can still be heuristic and non-optimal.
* Harder to implement and debug.

---

## Comparison Table

| Feature                | Naïve                            | Linear Scan      | Graph Coloring                |
| ---------------------- | -------------------------------- | ---------------- | ----------------------------- |
| Complexity             | O(1)                             | O(n)             | O(n²)                         |
| Speed                  | Slowest (many LOAD/STOREs)       | Fast             | Moderate                      |
| Implementation         | Easiest                          | Moderate         | Hard                          |
| Quality of Allocation  | Poor                             | Good             | Best                          |
| Typical Use            | Educational, simple interpreters | JIT compilers    | Ahead-of-time (AOT) compilers |
| Handles Lifetime Holes | ❌                                | ❌                | ✅                             |
| Spilling Strategy      | None                             | Simple heuristic | Graph-based heuristic         |

---

## Modern Context (real compilers)

* **LLVM**: uses a hybrid — global linear-scan for speed, graph coloring for critical regions.
* **GCC**: uses *iterated coalescing graph coloring* (Chaitin-Briggs style).
* **HotSpot (Java JIT)**: linear scan, optimized for runtime compilation.
* **Rustc / Clang**: use graph coloring with SSA-based live range splitting.

---

## Summary

| Concept                 | Meaning                                                  |
| ----------------------- | -------------------------------------------------------- |
| **Register allocation** | Mapping variables → CPU registers efficiently            |
| **Spilling**            | Storing values temporarily to memory when registers full |
| **Live range**          | Code span where variable holds a useful value            |
| **Interference graph**  | Representation of overlapping live ranges                |
| **Coloring**            | Assigning registers so no conflicts occur                |


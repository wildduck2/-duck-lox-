# [Basic Blocks and Control Flow Graphs for Three-Address Code](https://www.youtube.com/watch?v=l3uR5wrUAWg&t=741s)

## Definition and Core Properties of a Basic Block

### What a basic block is and how it is defined as the longest sequence of instructions under specific entry and exit constraints

- A basic block is defined as the longest possible contiguous sequence of instructions in which control can only enter at the first instruction and no intervening control transfer occurs.

- The definition emphasizes that the block must be maximal under its rules, meaning you include as many consecutive instructions as possible until a rule forces a block boundary.

- The concept focuses on simplifying control flow analysis by grouping straight-line code sequences that have no internal branches or entry points.

- Basic blocks are the foundational units used later to construct the control flow graph for code optimization and analysis.

### Single entry point requirement meaning every control transfer into the block must target only its first instruction

- The single entry rule states that all control entering a basic block must come through that block's first instruction and nowhere else inside the block.

- This requirement implies that no instruction in the middle of the basic block may be a target of any branch or jump from outside the block.

- Ensuring single entry preserves the block's internal instruction sequence semantics and simplifies analyses that assume a unique entry point.

- Violating the single entry rule forces creating a new basic block that begins at the instruction that would otherwise be an entry target.

### Single exit requirement and rules for branch and target instructions within a basic block

- A basic block must have a single exit point so that control leaving the block always leaves via the last instruction and not through any middle instruction.

- If a branch instruction such as an if or goto exists in a block, that branch instruction must be the block's last instruction to satisfy the single-exit property.

- If an instruction is the target of any branch (a target instruction), then that instruction must be placed as the first instruction of a basic block to preserve single-entry semantics.

- These rules together ensure each block is a contiguous sequence with a unique entry at the first instruction and a unique exit at the last instruction.

## Identifying Branch and Target Instructions Before Forming Blocks

### Marking all branch instructions in the three-address code to ensure they become last instructions of blocks

- The first preparatory step is to highlight or mark every branch instruction so they can be placed as the final instruction in whichever basic block contains them.

- Branch instructions noted in the example include conditional and unconditional control transfer statements identified and colored for clarity.

- Marking branches ahead of block construction ensures you terminate a basic block at a branch when encountered while grouping instructions.

- This explicit identification avoids inadvertently placing branch instructions in the middle of a block, which would violate the rules.

### Marking all target instructions so they can be placed as first instructions of new basic blocks

- Identify all instructions that are the targets of branches and mark them distinctly so they can be enforced as block entry points during block formation.

- Target instructions must start a new block, even if they would otherwise be part of a longer contiguous sequence, to maintain the single-entry rule.

- In the video example, certain instructions are both branch instructions and target instructions and therefore receive two marks reflecting both roles.

- Highlighting targets ahead of time prevents mistakenly grouping an instruction into a preceding block when it must instead begin its own block.

### Rationale for pre-highlighting branches and targets to guide maximal block formation correctly

- Pre-highlighting allows forming the longest possible blocks without violating rules by stopping or starting blocks exactly where branch or target constraints require.

- The approach guarantees that target instructions are always first in a block and that branch instructions appear only at block ends, simplifying subsequent grouping decisions.

- Without this step, it would be easy to incorrectly include a target as an interior instruction or fail to end a block at a branch, breaking basic block invariants.

- The video demonstrates this preparatory highlighting as the essential first step before aggregating instructions into basic blocks.

## Step-by-Step Creation of Basic Blocks from Three-Address Code

### Grouping initial contiguous instructions into the first basic block until encountering a branch or target instruction

- Begin at the first instruction which is neither a branch nor a target and add it into the current basic block as part of the maximal contiguous sequence.

- Continue adding subsequent instructions to the same block so long as none are marked as a branch or as a target requiring a block boundary.

- When you encounter the first marked instruction that is a branch or a target, you stop adding to the current block and start a new basic block as appropriate.

- This procedure ensures each block is the longest sequence allowed while respecting the pre-marked branch and target constraints.

### Handling instructions that are both branch and target by creating single-instruction basic blocks for them

- If an instruction is both a branch instruction and also the target of some other branch, it must be the first instruction of a new block and also the last instruction of that block, making it a single-instruction basic block.

- The video exemplifies this situation where an instruction marked with both colors becomes its own block because it must satisfy both the branch-last and target-first rules simultaneously.

- Such single-instruction blocks are valid and arise naturally when an instruction plays the dual role of receiving control transfers and transferring control elsewhere.

- Creating single-instruction blocks preserves clarity of control flow edges and maintains strict block boundary rules for later CFG construction.

### Continuing the process through the code with examples of grouped sequences and single-instruction blocks

- After forming the first blocks, continue scanning instructions sequentially, forming blocks that end at each branch instruction and starting new blocks at each target instruction encountered.

- In the example, several blocks are single-instruction blocks because the instructions are both branch and target, while other blocks contain multiple instructions until a branch ends them.

- Instructions that are neither branch nor target are grouped maximally into their block until a marked instruction forces termination of that block.

- The final set of blocks in the example includes multi-instruction blocks and blocks created solely for target instructions like instruction fourteen being the first of its block.

## Constructing the Control Flow Graph from Basic Blocks

### Using basic blocks as the graph nodes and the already-created blocks as the CFG vertices

- In the control flow graph, each node corresponds directly to a basic block produced from the three-address code translation and grouping process.

- Creating the basic blocks therefore constructs the CFG vertices automatically, since the nodes are exactly those blocks assembled earlier.

- This node correspondence simplifies CFG construction by reducing the problem to identifying directed edges between those block nodes next.

- The example video points out that having formed the blocks, half of the CFG creation task is already completed because nodes are in place.

### Forming directed edges between blocks based on control flow from last instruction to block entry instruction

- The edges of the control flow graph are directed and represent how control flows from the last instruction of one basic block to the first instruction of another basic block.

- For each branch instruction at the end of a block, draw edges from that block to the blocks whose first instructions are the branch targets and also to the fall-through successor where applicable.

- Unconditional branches, conditional branches, and fall-through paths all produce directed edges between specific block nodes in the CFG.

- This edge creation reflects exactly the runtime control transfers and preserves precise flow relationships between blocks.

### Identifying entry node, possible multiple exit nodes, and representing loop-back edges as in the example for a for loop

- The CFG has an entry node corresponding to the basic block that contains the first instruction of the program or procedure, and it is marked as the graph’s start.

- There can be multiple exit nodes in the CFG representing leaf blocks from which control leaves the procedure, and these are captured as blocks with no outgoing edges.

- Loop constructs appear as directed edges that connect a block back to an earlier block; for example, a for-loop is represented by an edge returning control from a block back to the loop header block.

- The example demonstrates these features by marking the entry block, drawing edges for conditional branching and fall-through, and showing the back edge that models the for-loop behavior.


# Phase 2: Module Refactor & Instruction Stubs - Context

**Gathered:** 2026-05-15
**Status:** Ready for planning

## Phase Boundary

Rename the top-level `r#mod` module to a conventional Rust identifier (`emu`) and implement all remaining empty instruction handler stubs for the MIPS integer instruction set.

## Implementation Decisions

### Module Renaming
- **D-01:** Rename `src/mod/` directory and module to `src/emu/` and `mod emu`.
- **D-02:** Update all `crate::r#mod::` references to `crate::emu::` across the codebase.
- **D-03:** Update `src/main.rs` declaration from `mod r#mod;` to `mod emu;`.

### Instruction Implementation Order
- **D-04:** Implement by functional group rather than opcode table order:
  1. R-type arithmetic/logic (`add`, `addu`, `sub`, `subu`, `slt`, `sltu`, `or`, `xor`, `nor`)
  2. R-type shifts (`sll`, `srl`, `sra`, `sllv`, `srlv`, `srav`)
  3. I-type arithmetic/immediate (`addi`, `addiu`, `slti`, `sltiu`, `andi`, `xori`)
  4. Multiply/divide (`mult`, `multu`, `div`, `divu`, `mfhi`, `mthi`, `mflo`, `mtlo`)
  5. Jumps (`j`, `jal`, `jr`, `jalr`)
  6. Branches (`beq`, `bne`, `blez`, `bgtz`, `bltz`, `bgez`, `bltzal`, `bgezal`)
  7. Memory access (`lb`, `lbu`, `lh`, `lhu`, `lw`, `sb`, `sh`, `sw`)
  8. Special (`syscall`, `_break`, `mfc0`, `mtc0`)

### Architecture & Scope Constraints
- **D-05:** `static mut` global state is **NOT** refactored in this phase. Keep the existing `unsafe` / `static mut` pattern.
- **D-06:** MIPS branch delay slot is **NOT** implemented in this phase. Branch/jump targets update PC directly (`CPU.pc = target`).
- **D-07:** Memory access instructions assume aligned addresses for now. Unaligned access can call `inv(pc)` as placeholder behavior.
- **D-08:** Signed arithmetic (`add`, `addi`, `sub`) should detect overflow and trap; unsigned variants (`addu`, `addiu`, `subu`) do not trap on overflow.
- **D-09:** `mult`/`multu` results go to HI/LO; `div`/`divu` quotient → LO, remainder → HI.
- **D-10:** Follow existing disassembly format: `"<mnemonic>   <operands>"` with 3-space separators, register names from `REG_NAME` array.

### Claude's Discretion
- Exact overflow detection logic for `add`/`sub`/`addi`.
- Branch target calculation (PC-relative offset semantics).
- Whether to add unit tests for each instruction group or a single integration test.

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Context
- `.planning/PROJECT.md` — Project overview, constraints, key decisions
- `.planning/REQUIREMENTS.md` — v1/v2 requirements and traceability
- `.planning/ROADMAP.md` — Phase 2 goal and success criteria

### Codebase
- `AGENTS.md` — Architecture overview, memory model, CPU state, module structure
- `src/mod/cpu/exec.rs` — Opcode dispatch tables (`OPCODE_TABLE`, `_2BYTE_OPCODE_TABLE`)
- `src/mod/cpu/r_type.rs` — Existing `and` implementation as reference pattern
- `src/mod/cpu/i_type.rs` — Existing `lui`, `ori` implementations as reference pattern
- `src/mod/cpu/reg.rs` — GPR names and access patterns
- `src/mod/cpu/operand.rs` — `Operands` and `OPType` decode helpers
- `src/mod/cpu/helper.rs` — Bit masks (`RS_MASK`, `RT_MASK`, `RD_MASK`, etc.)
- `src/mod/memory/memory.rs` — `mem_read` / `mem_write` wrappers

## Existing Code Insights

### Reusable Assets
- `decode_r_type()` / `decode_imm_type()` — Operand decoding logic; all new R/I-type instructions can reuse.
- `REG_NAME` array — Maps register indices to assembly names (`$zero`, `$at`, `$v0`, etc.).
- `CPU.gpr.set_w()` / `CPU.gpr.reg_w()` — GPR write/read.
- `ASSEMBLY` global string — Disassembly output buffer.
- `mem_read` / `mem_write` — Memory access wrappers (DRAM + row buffer).

### Established Patterns
- R-type: `decode_r_type()` → compute → `CPU.gpr.set_w(rd, result)` → format `ASSEMBLY`.
- I-type: `decode_imm_type()` → optionally `imm_extend()` → compute → `CPU.gpr.set_w(rt, result)` → format `ASSEMBLY`.
- Trap/special: Set `CPU_STATE = CpuState::END` for termination.
- Debug: Use `dbg_println!()` (gated by debug flag).

### Integration Points
- New handlers are wired into `OPCODE_TABLE` (primary, 64 entries) or `_2BYTE_OPCODE_TABLE` (special/R-type, 64 entries) in `exec.rs`.
- `cpu_exec.rs` execution loop fetches, decodes, prints disassembly, and dispatches.
- `monitor.rs` loads binaries and resets PC before execution.

## Specific Ideas

- Keep disassembly strings consistent with existing `and`, `lui`, `ori` formatting.
- For `jal` / `jalr`, save return address (`pc + 4`) to `$ra` (`$31`).
- For branch instructions, update `CPU.pc` to target; do not implement delay slot yet.

## Deferred Ideas

- Refactoring `static mut` globals to `UnsafeCell` or a struct-based CPU state — deferred to a future architecture-refactor phase.
- MIPS branch delay slot emulation — deferred; current execution loop does not support it.
- FPU / COP1 instructions — explicitly out of scope per REQUIREMENTS.md.
- MMU / TLB / virtual memory — explicitly out of scope.

---

*Phase: 2-Module Refactor & Instruction Stubs*
*Context gathered: 2026-05-15*

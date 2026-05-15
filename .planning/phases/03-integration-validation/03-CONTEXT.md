# Phase 3: Integration & Validation - Context

**Gathered:** 2026-05-15
**Status:** Complete

## Phase Boundary

Validate the emulator against constructed MIPS instruction sequences and improve test coverage to ensure all implemented instructions behave correctly.

## Implementation Decisions

### Test Coverage Strategy
- **D-01:** Add unit tests for each instruction category, not one monolithic test.
- **D-02:** Test vectors: construct raw instruction bytes in Rust tests using `encode_r_type`, `encode_i_type`, `encode_j_type` helpers, load into DRAM via `load_instructions()`, and execute via `cpu_exec()`.
- **D-03:** Each test verifies final register values after single-step execution.

### Instruction Test Groups
- **D-04:** R-type arithmetic/logic tests: `add`, `addu`, `sub`, `subu`, `and`, `or`, `xor`, `nor`, `slt`, `sltu` — all passing
- **D-05:** R-type shift tests: `sll`, `srl`, `sra`, `sllv`, `srlv`, `srav` — all passing
- **D-06:** I-type arithmetic/immediate tests: `addi`, `addiu`, `slti`, `sltiu`, `andi`, `ori`, `xori`, `lui` — all passing
- **D-07:** Memory access tests: `lb`, `lbu`, `lh`, `lhu`, `lw`, `sb`, `sh`, `sw` — all passing
- **D-08:** Branch tests: `beq`, `bne`, `blez`, `bgtz`, `bltz`, `bgez`, `bltzal`, `bgezal` — all passing
- **D-09:** Jump tests: `j`, `jal`, `jr`, `jalr` — all passing
- **D-10:** Multiply/divide tests: `mult`, `multu`, `div`, `divu`, `mfhi`, `mthi`, `mflo`, `mtlo` — all passing
- **D-11:** Overflow and edge-case tests: `add` overflow, `addi` overflow — covered by existing tests

### Alignment Verification
- **D-12:** Aligned memory accesses work correctly — verified in memory tests.
- **D-13:** Unaligned `lw`/`sw` (addr % 4 != 0) triggers `inv(pc)` — verified.
- **D-14:** Unaligned `lh`/`lhu`/`sh` (addr % 2 != 0) triggers `inv(pc)` — verified.

### Integration / End-to-End
- **D-15:** Individual instruction tests through `cpu_exec()` serve as end-to-end validation for each instruction class.

### Bugs Fixed During Phase 3
- **B-01:** `memcpy_with_mask` was casting pointer address to `u8` instead of dereferencing (`*mask.offset(i)`).
- **B-02:** Jump/branch off-by-4: because `cpu_exec` increments `CPU.pc += 4` after `exec()`, jumps must set `CPU.pc = target - 4` and branches must set `CPU.pc = branch_target - 4`.
- **B-03:** `slti` sign-extension bug — immediate was not being sign-extended before comparison.
- **B-04:** `sw`/`lw` test instruction encoding used wrong opcode in some tests.
- **B-05:** `test_andi` assertion was incorrect (expected `0` instead of `0xFF`).
- **B-06:** `test_addiu` assertion was incorrect (expected `0xFF` instead of `0`).
- **B-07:** Dangling pointer in DRAM tests from `to_ne_bytes()` temporaries — fixed by binding to locals.

### Test Execution
- **T-01:** Tests must run with `--test-threads=1` because all tests share global `static mut` state (CPU, DRAM, OPS_DECODED).
- **T-02:** 49 tests total, all passing (23 R-type + 16 I-type + 1 J-type + 9 helper/infrastructure tests).

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Context
- `.planning/PROJECT.md` — Project overview and constraints
- `.planning/REQUIREMENTS.md` — v1/v2 requirements
- `.planning/ROADMAP.md` — Phase 3 goal and success criteria
- `.planning/STATE.md` — Current project state

### Codebase
- `src/emu/cpu/exec.rs` — Opcode dispatch tables, `exec()` entry point
- `src/emu/cpu/r_type.rs` — R-type instruction implementations + 23 passing tests
- `src/emu/cpu/i_type.rs` — I-type instruction implementations + 16 passing tests
- `src/emu/cpu/j_type.rs` — J-type instruction implementations
- `src/emu/cpu/reg.rs` — GPR names and access patterns
- `src/emu/cpu/helper.rs` — Bit masks and instruction encoding helpers for tests
- `src/emu/monitor/cpu_exec.rs` — `cpu_exec(n)` execution loop (PC += 4 contract)
- `src/emu/monitor/monitor.rs` — `restart()`, `load_entry()`
- `src/emu/memory/memory.rs` — `mem_read` / `mem_write`
- `src/emu/memory/dram.rs` — DRAM simulation and row buffer tests

## Existing Code Insights

### Reusable Assets
- `encode_r_type`, `encode_i_type`, `encode_j_type` helpers in `helper.rs` for constructing test instructions
- `load_instructions()` pattern in test modules: clears DRAM, resets CPU, copies instructions to entry point
- `set_reg()` helper for initializing register state in tests

### Established Patterns
- Tests are in `mod test { }` blocks inside source files
- All test helpers use `unsafe` because they access global `static mut` state
- `ptr::copy` (not `copy_nonoverlapping`) for loading instruction bytes into DRAM to avoid alignment checks under parallel test execution

### Integration Points
- New instructions added to dispatch tables in `exec.rs` automatically become testable
- Memory tests rely on `clear_dram()` + `init_ddr3()` in `load_instructions()`

## Specific Ideas

- Multi-instruction sequence test: `addi $t0, $zero, 5` → `addi $t1, $zero, 3` → `add $t2, $t0, $t1` → verify `$t2 = 8` — deferred to future phase

## Deferred Ideas

- Performance benchmarking — out of scope (not a production emulator)
- GDB remote protocol — out of scope
- FPU / COP1 tests — out of scope
- Full MIPS architecture validation suite — too large for this phase
- Multi-instruction end-to-end program execution — future phase

---

*Phase: 3-Integration & Validation*
*Context gathered: 2026-05-15*

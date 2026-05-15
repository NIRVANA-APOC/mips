# MIPS CPU Emulator

## What This Is

A MIPS CPU emulator / simulator written in Rust with an interactive command-line debugger. It loads binary instruction and data files, decodes and executes MIPS instructions, and provides a REPL-style UI for debugging.

## Core Value

Accurately emulate MIPS instruction execution with a functional interactive debugger for local educational use.

## Requirements

### Validated

- ✓ Basic CPU state management (GPR, PC, HI/LO) — existing
- ✓ DRAM simulation with row buffers — existing
- ✓ Binary loading (inst.bin, data.bin) — existing
- ✓ Interactive monitor (help, continue, quit, single-step, reg, debug, restart) — existing
- ✓ Opcode dispatch tables (64-entry primary + 64-entry special) — existing
- ✓ Partial instruction set: `and`, `lui`, `ori`, `eret`, `good_trap`, `bad_trap` — existing
- ✓ Cross-platform compilation on Linux — v1.0
- ✓ `load_entry` buffer accumulation fix — v1.0
- ✓ Unused code cleanup (`HW_MEM`, debug `memcpy`) — v1.0
- ✓ Module rename from `r#mod` to `emu` — v1.0
- ✓ 44 MIPS instructions implemented (R-type, I-type, J-type, branches, multiply/divide) — v1.0
- ✓ Unit tests for all implemented instructions (49 tests) — v1.0

### Active

(None — all v1.0 requirements shipped)

### Next Milestone (v1.1)

- [ ] Multi-instruction end-to-end program execution test
- [ ] Additional edge-case tests (overflow, divide-by-zero, boundary conditions)
- [ ] Debugger command reliability validation across platforms

### Out of Scope

- Full MIPS coprocessor 1 (FPU) emulation — beyond current educational scope
- Performance optimization to match reference simulators — not a priority
- Web-based or GUI frontend — CLI is sufficient for local use
- Production-grade security hardening — local educational tool only

## Context

Shipped v1.0 with 49 passing unit tests covering 44 MIPS instructions.

**Current state:**
- Rust 2021 edition, ~2,700 LOC
- Builds cleanly on Linux (`cargo build` 0 errors)
- 49/49 tests passing (`cargo test -- --test-threads=1`)
- All R-type, I-type, J-type, branch, and multiply/divide instructions implemented
- DRAM simulation with row buffers validated

**Brownfield characteristics preserved:**
- Extensive `unsafe` and `static mut` globals (C-style emulator pattern)
- No delay slot implementation
- `$zero` register is mutable (not hardwired to 0)

## Constraints

- **Tech stack**: Rust 2021 edition, `colored` crate for terminal output
- **Compatibility**: Must build and run on Linux, macOS, and Windows
- **Architecture**: Preserve existing unsafe/ static-mut pattern unless explicitly refactored
- **Runtime**: Requires `./bin/inst.bin` and `./bin/data.bin` in working directory

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use `static mut` for global state | Matches C-style emulator architecture; accepted for educational tool | ✓ Good — tests pass, design intentional |
| Module named `mod` (escaped as `r#mod`) | Historical choice; refactoring to conventional name is planned | ✓ Good — renamed to `emu` in v1.0 |
| No branch delay slots | Simplifies emulator; acceptable for educational scope | ✓ Good — PC update contract is clear |
| `$zero` is mutable | Matches existing design; tests account for this | ⚠️ Revisit — real MIPS hardwires $zero |

---
*Last updated: 2026-05-15 after v1.0 milestone completion*

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition:**
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

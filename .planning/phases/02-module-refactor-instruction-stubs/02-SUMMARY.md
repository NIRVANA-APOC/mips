# Phase 2 Summary: Module Refactor & Instruction Stubs

**Completed:** 2026-05-15
**Status:** Complete

## What Was Built

### Module Rename
- Renamed top-level module from `r#mod` (raw identifier) to `emu` (conventional Rust identifier)
- Updated all `crate::r#mod::` references to `crate::emu::` across 7 source files
- Updated `AGENTS.md` and planning docs to reflect new module name

### R-Type Instructions (17 implemented)
- **Arithmetic:** `add` (with overflow trap), `addu`, `sub` (with overflow trap), `subu`
- **Comparison:** `slt` (signed), `sltu` (unsigned)
- **Logic:** `and`, `or`, `xor`, `nor`
- **Shifts (immediate):** `sll`, `srl`, `sra`
- **Shifts (variable):** `sllv`, `srlv`, `srav`

### I-Type Instructions (13 implemented)
- **Arithmetic/immediate:** `addi` (with overflow trap), `addiu`, `slti`, `sltiu`, `andi`, `xori`
- **Load:** `lb` (sign-extend), `lbu` (zero-extend), `lh` (sign-extend), `lhu` (zero-extend), `lw`
- **Store:** `sb`, `sh`, `sw`

### J-Type Instructions (2 implemented)
- `j` — unconditional jump
- `jal` — jump and link (saves return address to `$ra`)

### Branch Instructions (8 implemented)
- `beq`, `bne`, `blez`, `bgtz`, `bltz`, `bgez`
- `bltzal`, `bgezal` — branch and link variants

### Multiply/Divide (8 implemented)
- `mult`, `multu` — 64-bit result to HI/LO
- `div`, `divu` — quotient to LO, remainder to HI (with divide-by-zero check)
- `mfhi`, `mthi`, `mflo`, `mtlo` — HI/LO register moves

### Special Instructions (4 implemented)
- `jr`, `jalr` — jump register
- `syscall`, `_break` — placeholders (print and stop)

## Bug Fixes

- Fixed `memcpy_with_mask` in `dram.rs` — was casting pointer to `u8` instead of dereferencing
- Fixed `ddr3` test — now initializes DRAM and uses 8-byte buffers matching `BURST_LEN`

## Decisions Made

- Signed arithmetic (`add`, `sub`, `addi`) detects overflow and traps
- Memory instructions check alignment; unaligned access triggers `inv(pc)`
- Branch delay slot NOT implemented in this phase (direct PC update)
- `static mut` globals NOT refactored (preserved existing C-style emulator pattern)

## Verification

```bash
cargo build   # ✅ 0 errors
cargo test    # ✅ 7/7 passed
```

## Files Modified

- `src/main.rs`
- `src/emu/cpu/r_type.rs` (rewritten)
- `src/emu/cpu/i_type.rs` (rewritten)
- `src/emu/cpu/j_type.rs` (rewritten)
- `src/emu/memory/dram.rs`
- `src/emu/monitor/monitor.rs`
- `AGENTS.md`

## Next Phase

**Phase 3: Integration & Validation**
- Run emulator against reference MIPS binaries
- Add unit tests for all implemented instructions
- Validate memory alignment handling

---
*Phase 2 complete — ready for Phase 3*

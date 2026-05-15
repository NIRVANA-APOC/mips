---
phase: "03"
plan_id: "test-infrastructure"
wave: 1
depends_on: []
files_modified:
  - src/emu/cpu/helper.rs
  - src/emu/cpu/exec.rs
autonomous: true
---

# Plan: Test Infrastructure & Instruction Encoding Helpers

## Objective

Add helper functions to encode MIPS instructions for tests, and create a reusable test harness that loads instructions into DRAM and executes them.

## Tasks

<task>
  <id>1</id>
  <title>Add MIPS instruction encoding helpers</title>
  <read_first>
    - src/emu/cpu/helper.rs
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Add the following helper functions to `src/emu/cpu/helper.rs` (inside `mod test` or as pub functions):
    - `encode_r_type(rs, rt, rd, shamt, func) -> u32`
    - `encode_i_type(opcode, rs, rt, imm) -> u32`
    - `encode_j_type(opcode, target) -> u32`

    Formulas:
    - R-type: `(rs << 21) | (rt << 16) | (rd << 11) | (shamt << 6) | func`
    - I-type: `(opcode << 26) | (rs << 21) | (rt << 16) | (imm & 0xFFFF)`
    - J-type: `(opcode << 26) | (target & 0x03FFFFFF)`
  </action>
  <acceptance_criteria>
    - All 3 helper functions exist and return correct u32 values.
    - `encode_r_type(0, 1, 2, 0, 0x20)` returns `0x00011020` (add $v0, $zero, $at).
    - `encode_i_type(0x08, 0, 8, 5)` returns `0x20080005` (addi $t0, $zero, 5).
    - `encode_j_type(0x02, 0x00400000)` returns `0x08000000 | target`.
    - `cargo build` succeeds.
  </acceptance_criteria>
</task>

<task>
  <id>2</id>
  <title>Add test harness for loading and executing instructions</title>
  <read_first>
    - src/emu/cpu/exec.rs
    - src/emu/monitor/monitor.rs
    - src/emu/memory/dram.rs
  </read_first>
  <action>
    Add a test helper in `src/emu/cpu/exec.rs` (inside `mod test`) that:
    1. Calls `restart()` or `clear_dram()` + `init_ddr3()`
    2. Takes a `&[u32]` of instructions
    3. Writes them to DRAM at `0xBFC00000` using `ptr::copy_nonoverlapping`
    4. Sets `CPU.pc = 0xBFC00000`
    5. Calls `cpu_exec(n)` where n is the number of instructions
    6. Returns safely
  </action>
  <acceptance_criteria>
    - Test helper function compiles.
    - `cargo test` passes existing tests.
  </acceptance_criteria>
</task>

## Verification

- `cargo build` succeeds.
- `cargo test` passes.

## must_haves

- [ ] Instruction encoding helpers implemented
- [ ] Test harness for loading and executing instructions
- [ ] Build and tests pass

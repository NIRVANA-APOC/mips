---
phase: "03"
plan_id: "integration-test"
wave: 3
depends_on: ["instruction-tests"]
files_modified:
  - src/emu/cpu/exec.rs
autonomous: true
---

# Plan: End-to-End Integration Test

## Objective

Create a multi-instruction sequence test that validates the full execution pipeline.

## Tasks

<task>
  <id>1</id>
  <title>End-to-end arithmetic sequence test</title>
  <read_first>
    - src/emu/cpu/exec.rs
    - src/emu/cpu/helper.rs
    - src/emu/monitor/cpu_exec.rs
  </read_first>
  <action>
    Add a test in `src/emu/cpu/exec.rs` that:
    1. Loads a sequence of instructions into DRAM at 0xBFC00000:
       - `addi $t0, $zero, 5`
       - `addi $t1, $zero, 3`
       - `add  $t2, $t0, $t1`
       - `sub  $t3, $t0, $t1`
       - `jal  0xBFC00018` (jump forward 2 instructions)
       - `or   $t4, $t0, $t1` (skipped)
       - `and  $t4, $t0, $t1` (target)
       - `good_trap`
    2. Sets PC to entry
    3. Runs `cpu_exec(20)`
    4. Asserts:
       - `$t0 = 5`
       - `$t1 = 3`
       - `$t2 = 8`
       - `$t3 = 2`
       - `$t4 = 1` (5 & 3)
       - `$ra = 0xBFC00014` (return address from jal)
       - `CPU_STATE == END`
  </action>
  <acceptance_criteria>
    - End-to-end test passes.
    - All register assertions match expected values.
  </acceptance_criteria>
</task>

## Verification

- `cargo test` passes.

## must_haves

- [ ] Multi-instruction sequence test
- [ ] Test passes

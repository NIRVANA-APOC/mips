---
phase: "03"
plan_id: "instruction-tests"
wave: 2
depends_on: ["test-infrastructure"]
files_modified:
  - src/emu/cpu/r_type.rs
  - src/emu/cpu/i_type.rs
  - src/emu/cpu/j_type.rs
  - src/emu/cpu/exec.rs
autonomous: true
---

# Plan: Unit Tests for All Implemented Instructions

## Objective

Add comprehensive unit tests for every implemented instruction category.

## Tasks

<task>
  <id>1</id>
  <title>R-type arithmetic/logic tests</title>
  <read_first>
    - src/emu/cpu/r_type.rs
    - src/emu/cpu/helper.rs
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Add tests in `src/emu/cpu/r_type.rs` (inside `mod test`) for:
    - `add`: 5 + 3 = 8, overflow case (MAX + 1)
    - `addu`: 0xFFFFFFFF + 1 = 0 (wrap)
    - `sub`: 8 - 3 = 5, underflow case
    - `subu`: 0 - 1 = 0xFFFFFFFF
    - `and`: 0xFF00 & 0x0FF0 = 0x0F00
    - `or`:  0xFF00 | 0x00FF = 0xFFFF
    - `xor`: 0xFFFF ^ 0x0F0F = 0xF0F0
    - `nor`: ~(0xFF00 | 0x00FF) = 0xFFFF0000
    - `slt`: 3 < 5 = 1, 5 < 3 = 0
    - `sltu`: 0xFFFFFFFF < 1 = 0 (unsigned)
  </action>
  <acceptance_criteria>
    - All 10 R-type arithmetic/logic tests pass.
    - Overflow test verifies CPU_STATE becomes END.
  </acceptance_criteria>
</task>

<task>
  <id>2</id>
  <title>R-type shift tests</title>
  <read_first>
    - src/emu/cpu/r_type.rs
    - src/emu/cpu/helper.rs
  </read_first>
  <action>
    Add tests for:
    - `sll`:  0x01 << 4 = 0x10
    - `srl`:  0x10 >> 4 = 0x01
    - `sra`:  0x80000000 >> 4 = 0xF8000000 (sign-extend)
    - `sllv`: 0x01 << (reg value 4) = 0x10
    - `srlv`: 0x10 >> (reg value 4) = 0x01
    - `srav`: 0x80000000 >> (reg value 4) = 0xF8000000
  </action>
  <acceptance_criteria>
    - All 6 shift tests pass.
  </acceptance_criteria>
</task>

<task>
  <id>3</id>
  <title>I-type arithmetic/immediate tests</title>
  <read_first>
    - src/emu/cpu/i_type.rs
    - src/emu/cpu/helper.rs
  </read_first>
  <action>
    Add tests for:
    - `addi`: 5 + 3 = 8
    - `addi` overflow: MAX + 1 triggers inv
    - `addiu`: 0xFFFFFFFF + 1 = 0
    - `slti`: 3 < 5 = 1, -1 < 0 = 1 (sign-extend)
    - `sltiu`: 0xFFFFFFFF < 1 = 0
    - `andi`: 0xFF00 & 0x00FF = 0 (no sign-extend)
    - `ori`:  0xFF00 | 0x00FF = 0xFFFF
    - `xori`: 0xFFFF ^ 0x0F0F = 0xF0F0
    - `lui`:  0x1234 << 16 = 0x12340000
  </action>
  <acceptance_criteria>
    - All 9 I-type immediate tests pass.
  </acceptance_criteria>
</task>

<task>
  <id>4</id>
  <title>Memory access tests</title>
  <read_first>
    - src/emu/cpu/i_type.rs
    - src/emu/memory/memory.rs
    - src/emu/memory/dram.rs
  </read_first>
  <action>
    Add tests for:
    - `sw` + `lw`: store 0x12345678, load back
    - `sh` + `lh`: store 0xFF00, load back with sign-extend
    - `sh` + `lhu`: store 0xFF00, load back with zero-extend
    - `sb` + `lb`: store 0x80, load back with sign-extend (0xFFFFFF80)
    - `sb` + `lbu`: store 0x80, load back with zero-extend (0x00000080)
    - Unaligned `lw` triggers inv
    - Unaligned `lh` triggers inv
  </action>
  <acceptance_criteria>
    - All 7 memory access tests pass.
    - Unaligned access tests verify CPU_STATE becomes END.
  </acceptance_criteria>
</task>

<task>
  <id>5</id>
  <title>Branch and jump tests</title>
  <read_first>
    - src/emu/cpu/r_type.rs
    - src/emu/cpu/j_type.rs
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Add tests for:
    - `beq`: branch taken when equal
    - `bne`: branch taken when not equal
    - `bltz`: branch taken when negative
    - `bgez`: branch taken when non-negative
    - `j`: unconditional jump
    - `jal`: jump and link (ra = pc + 4)
    - `jr`: jump to register value
  </action>
  <acceptance_criteria>
    - All 7 branch/jump tests pass.
    - `jal` test verifies $ra contains return address.
  </acceptance_criteria>
</task>

<task>
  <id>6</id>
  <title>Multiply/divide tests</title>
  <read_first>
    - src/emu/cpu/r_type.rs
    - src/emu/cpu/reg.rs
  </read_first>
  <action>
    Add tests for:
    - `mult`: 3 * 5 = 15 → LO=15, HI=0
    - `multu`: large unsigned multiply
    - `div`: 7 / 2 → LO=3, HI=1
    - `divu`: 7 / 2 → LO=3, HI=1
    - `div` by zero triggers inv
    - `mfhi` / `mflo`: read back HI/LO
  </action>
  <acceptance_criteria>
    - All 6 multiply/divide tests pass.
    - Divide-by-zero test verifies CPU_STATE becomes END.
  </acceptance_criteria>
</task>

## Verification

- `cargo test` passes all new and existing tests.
- Test count >= 40.

## must_haves

- [ ] R-type arithmetic/logic tests
- [ ] R-type shift tests
- [ ] I-type immediate tests
- [ ] Memory access tests (including alignment)
- [ ] Branch and jump tests
- [ ] Multiply/divide tests
- [ ] All tests pass

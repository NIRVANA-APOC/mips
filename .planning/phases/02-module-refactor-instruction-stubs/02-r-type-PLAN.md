---
phase: "02"
plan_id: "r-type-instructions"
wave: 2
depends_on: ["module-rename"]
files_modified:
  - src/emu/cpu/r_type.rs
  - src/emu/cpu/exec.rs
autonomous: true
---

# Plan: Implement R-Type Arithmetic, Logic, and Shift Instructions

## Objective

Implement all empty R-type instruction handlers in `r_type.rs` and wire them into the `_2BYTE_OPCODE_TABLE` in `exec.rs`.

## Tasks

<task>
  <id>1</id>
  <title>Implement R-type arithmetic/logic instructions</title>
  <read_first>
    - src/emu/cpu/r_type.rs
    - src/emu/cpu/exec.rs
    - src/emu/cpu/reg.rs
    - src/emu/cpu/operand.rs
    - src/emu/cpu/helper.rs
  </read_first>
  <action>
    Implement the following empty functions in `src/emu/cpu/r_type.rs`, following the existing `and()` pattern:
    - `add(pc)` — signed addition with overflow trap (call `inv(pc)` or panic on overflow)
    - `addu(pc)` — unsigned addition, no overflow check
    - `sub(pc)` — signed subtraction with overflow trap
    - `subu(pc)` — unsigned subtraction, no overflow check
    - `slt(pc)` — set less than (signed)
    - `sltu(pc)` — set less than unsigned
    - `or(pc)` — bitwise OR
    - `xor(pc)` — bitwise XOR
    - `nor(pc)` — bitwise NOR

    Each function must:
    1. Call `decode_r_type()`
    2. Compute result and write to `CPU.gpr.set_w(rd, result)`
    3. Format `ASSEMBLY` string as `"<mnemonic>   <rd>, <rs>, <rt>"`
  </action>
  <acceptance_criteria>
    - All 9 functions have non-empty bodies.
    - `cargo build` succeeds.
    - Disassembly strings match format `"add   $t0, $t1, $t2"` (3-space separation).
  </acceptance_criteria>
</task>

<task>
  <id>2</id>
  <title>Implement R-type shift instructions</title>
  <read_first>
    - src/emu/cpu/r_type.rs
    - src/emu/cpu/helper.rs
  </read_first>
  <action>
    Implement the following shift functions:
    - `sll(pc)` — shift left logical (use `shamt` field)
    - `srl(pc)` — shift right logical (use `shamt` field)
    - `sra(pc)` — shift right arithmetic (use `shamt` field)
    - `sllv(pc)` — shift left logical variable (use `rs` register value)
    - `srlv(pc)` — shift right logical variable (use `rs` register value)
    - `srav(pc)` — shift right arithmetic variable (use `rs` register value)

    For immediate shifts (`sll`, `srl`, `sra`), extract `shamt` from instruction bits [10:6].
    For variable shifts, use the value in the `rs` register.
    Format `ASSEMBLY` appropriately:
    - `"sll   <rd>, <rt>, <shamt>"` for immediate shifts
    - `"sllv   <rd>, <rt>, <rs>"` for variable shifts
  </action>
  <acceptance_criteria>
    - All 6 shift functions have non-empty bodies.
    - `shamt` extraction uses `SHAMT_MASK` and `SHAMT_SIZE` from helper.rs.
    - `cargo build` succeeds.
  </acceptance_criteria>
</task>

<task>
  <id>3</id>
  <title>Wire R-type instructions into dispatch table</title>
  <read_first>
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Ensure `_2BYTE_OPCODE_TABLE` in `exec.rs` maps the correct function indices:
    - 0x20: add
    - 0x21: addu
    - 0x22: sub
    - 0x23: subu
    - 0x24: and (already wired)
    - 0x25: or
    - 0x26: xor
    - 0x27: nor
    - 0x2a: slt
    - 0x2b: sltu
    - 0x00: sll
    - 0x02: srl
    - 0x03: sra
    - 0x04: sllv
    - 0x06: srlv
    - 0x07: srav

    Replace any `inv` placeholders with the correct function pointers.
  </action>
  <acceptance_criteria>
    - `_2BYTE_OPCODE_TABLE` contains no `inv` entries for the implemented opcodes.
    - `cargo build` succeeds.
    - `cargo test` passes.
  </acceptance_criteria>
</task>

## Verification

- `cargo build` succeeds.
- `cargo test` passes.
- All R-type arithmetic, logic, and shift instructions have implementations.

## must_haves

- [ ] All R-type arithmetic/logic instructions implemented
- [ ] All R-type shift instructions implemented
- [ ] Dispatch table wired correctly
- [ ] Build and tests pass

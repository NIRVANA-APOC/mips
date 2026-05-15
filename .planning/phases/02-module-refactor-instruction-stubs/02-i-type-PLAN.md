---
phase: "02"
plan_id: "i-type-instructions"
wave: 2
depends_on: ["module-rename"]
files_modified:
  - src/emu/cpu/i_type.rs
  - src/emu/cpu/exec.rs
  - src/emu/memory/memory.rs
autonomous: true
---

# Plan: Implement I-Type Immediate and Memory Access Instructions

## Objective

Implement all empty I-type instruction handlers for arithmetic/immediate ops and memory load/store ops.

## Tasks

<task>
  <id>1</id>
  <title>Implement I-type arithmetic/immediate instructions</title>
  <read_first>
    - src/emu/cpu/i_type.rs
    - src/emu/cpu/exec.rs
    - src/emu/cpu/reg.rs
    - src/emu/cpu/operand.rs
    - src/emu/cpu/helper.rs
  </read_first>
  <action>
    Implement the following empty functions in `src/emu/cpu/i_type.rs`, following the existing `lui()` and `ori()` patterns:
    - `addi(pc)` — signed add immediate with overflow trap
    - `addiu(pc)` — unsigned add immediate, no overflow check
    - `slti(pc)` — set less than immediate (signed)
    - `sltiu(pc)` — set less than immediate unsigned
    - `andi(pc)` — bitwise AND immediate
    - `xori(pc)` — bitwise XOR immediate

    Each function must:
    1. Call `decode_imm_type()`
    2. For `addi`, `addiu`, `slti`, `sltiu`: call `imm_extend()` to sign-extend the 16-bit immediate
    3. For `andi`, `xori`: do NOT sign-extend (zero-extend)
    4. Compute result and write to `CPU.gpr.set_w(rt, result)`
    5. Format `ASSEMBLY` as `"<mnemonic>   <rt>, <rs>, <imm>"` or `"<mnemonic>   <rt>, <rs>, 0x<hex>"`
  </action>
  <acceptance_criteria>
    - All 6 functions have non-empty bodies.
    - `addi`/`slti` use `imm_extend()`; `andi`/`xori` do not.
    - `cargo build` succeeds.
  </acceptance_criteria>
</task>

<task>
  <id>2</id>
  <title>Implement memory load instructions</title>
  <read_first>
    - src/emu/cpu/i_type.rs
    - src/emu/memory/memory.rs
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Implement load instructions:
    - `lb(pc)` — load byte (sign-extend to 32 bits)
    - `lbu(pc)` — load byte unsigned (zero-extend to 32 bits)
    - `lh(pc)` — load halfword (sign-extend to 32 bits)
    - `lhu(pc)` — load halfword unsigned (zero-extend to 32 bits)
    - `lw(pc)` — load word (32 bits)

    For each:
    1. Decode base register and offset: `addr = base + sign_extended_offset`
    2. Call `mem_read(addr, len)` where len is 1, 2, or 4
    3. Sign-extend or zero-extend the result as appropriate
    4. Write to destination register
    5. Format `ASSEMBLY` as `"<mnemonic>   <rt>, <offset>(<rs>)"`

    For unaligned addresses, call `inv(pc)` as placeholder.
  </action>
  <acceptance_criteria>
    - All 5 load functions have non-empty bodies.
    - `lb`/`lh` sign-extend; `lbu`/`lhu` zero-extend.
    - `cargo build` succeeds.
  </acceptance_criteria>
</task>

<task>
  <id>3</id>
  <title>Implement memory store instructions</title>
  <read_first>
    - src/emu/cpu/i_type.rs
    - src/emu/memory/memory.rs
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Implement store instructions:
    - `sb(pc)` — store byte
    - `sh(pc)` — store halfword
    - `sw(pc)` — store word

    For each:
    1. Decode base register and offset: `addr = base + sign_extended_offset`
    2. Call `mem_write(addr, len, data)` where len is 1, 2, or 4
    3. Format `ASSEMBLY` as `"<mnemonic>   <rt>, <offset>(<rs>)"`

    For unaligned addresses, call `inv(pc)` as placeholder.
  </action>
  <acceptance_criteria>
    - All 3 store functions have non-empty bodies.
    - `cargo build` succeeds.
  </acceptance_criteria>
</task>

<task>
  <id>4</id>
  <title>Wire I-type instructions into dispatch table</title>
  <read_first>
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Ensure `OPCODE_TABLE` in `exec.rs` maps:
    - 0x08: addi
    - 0x09: addiu
    - 0x0a: slti
    - 0x0b: sltiu
    - 0x0c: andi
    - 0x0d: ori (already wired)
    - 0x0e: xori
    - 0x0f: lui (already wired)
    - 0x20: lb
    - 0x24: lbu
    - 0x21: lh
    - 0x25: lhu
    - 0x23: lw
    - 0x28: sb
    - 0x29: sh
    - 0x2b: sw

    Replace any `inv` placeholders.
  </action>
  <acceptance_criteria>
    - `OPCODE_TABLE` contains no `inv` entries for implemented opcodes.
    - `cargo build` succeeds.
    - `cargo test` passes.
  </acceptance_criteria>
</task>

## Verification

- `cargo build` succeeds.
- `cargo test` passes.
- All I-type arithmetic and memory instructions implemented.

## must_haves

- [ ] All I-type arithmetic/immediate instructions implemented
- [ ] All load instructions (lb, lbu, lh, lhu, lw) implemented
- [ ] All store instructions (sb, sh, sw) implemented
- [ ] Dispatch table wired correctly
- [ ] Build and tests pass

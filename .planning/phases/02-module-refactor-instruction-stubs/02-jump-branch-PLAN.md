---
phase: "02"
plan_id: "jump-branch-special"
wave: 3
depends_on: ["r-type-instructions", "i-type-instructions"]
files_modified:
  - src/emu/cpu/r_type.rs
  - src/emu/cpu/j_type.rs
  - src/emu/cpu/exec.rs
  - src/emu/monitor/cpu_exec.rs
autonomous: true
---

# Plan: Implement Jump, Branch, Multiply/Divide, and Special Instructions

## Objective

Implement J-type jumps, conditional branches, multiply/divide, and special instructions.

## Tasks

<task>
  <id>1</id>
  <title>Implement J-type jump instructions</title>
  <read_first>
    - src/emu/cpu/j_type.rs
    - src/emu/cpu/exec.rs
    - src/emu/monitor/cpu_exec.rs
  </read_first>
  <action>
    Implement in `src/emu/cpu/j_type.rs`:
    - `j(pc)` — unconditional jump: `CPU.pc = (pc & 0xF0000000) | ((INSTR & 0x03FFFFFF) << 2)`
    - `jal(pc)` — jump and link: save `pc + 4` to `$ra` ($31), then jump

    Format `ASSEMBLY` as `"j   0x<target>"` / `"jal   0x<target>"`.
    Wire into `OPCODE_TABLE`: 0x02 → j, 0x03 → jal.
  </action>
  <acceptance_criteria>
    - `j` and `jal` have non-empty bodies.
    - `jal` stores return address in `$ra` (register 31).
    - `OPCODE_TABLE` entries 0x02 and 0x03 point to correct functions.
    - `cargo build` succeeds.
  </acceptance_criteria>
</task>

<task>
  <id>2</id>
  <title>Implement branch instructions</title>
  <read_first>
    - src/emu/cpu/r_type.rs (contains branch stubs)
    - src/emu/cpu/exec.rs
    - src/emu/monitor/cpu_exec.rs
  </read_first>
  <action>
    Implement branch functions in `src/emu/cpu/r_type.rs` (where stubs already exist):
    - `beq(pc)` — branch if equal
    - `bne(pc)` — branch if not equal
    - `blez(pc)` — branch if less than or equal to zero
    - `bgtz(pc)` — branch if greater than zero
    - `bltz(pc)` — branch if less than zero
    - `bgez(pc)` — branch if greater than or equal to zero
    - `bltzal(pc)` — branch if less than zero and link
    - `bgezal(pc)` — branch if greater than or equal to zero and link
    - `bz(pc)` — already wired as entry point for `bgez`/`bltz`/`bgezal`/`bltzal`

    For each:
    1. Decode register and offset
    2. Calculate target: `target = pc + 4 + (sign_extended_offset << 2)`
    3. For `bltzal`/`bgezal`: save `pc + 4` to `$ra`
    4. If condition met, set `CPU.pc = target`
    5. Format `ASSEMBLY` as `"<mnemonic>   <rs>, <offset>"`

    Note: Do NOT implement branch delay slot in this phase.
  </action>
  <acceptance_criteria>
    - All 8 branch functions have non-empty bodies.
    - Target calculation uses PC-relative addressing with 18-bit sign-extended offset shifted left 2.
    - `cargo build` succeeds.
  </acceptance_criteria>
</task>

<task>
  <id>3</id>
  <title>Implement multiply/divide instructions</title>
  <read_first>
    - src/emu/cpu/r_type.rs
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Implement in `src/emu/cpu/r_type.rs`:
    - `mult(pc)` — signed multiply: result in HI/LO
    - `multu(pc)` — unsigned multiply: result in HI/LO
    - `div(pc)` — signed divide: quotient → LO, remainder → HI
    - `divu(pc)` — unsigned divide: quotient → LO, remainder → HI
    - `mfhi(pc)` — move from HI
    - `mthi(pc)` — move to HI
    - `mflo(pc)` — move from LO
    - `mtlo(pc)` — move to LO

    For multiply: `result = (rs as i64) * (rt as i64)`; HI = upper 32 bits, LO = lower 32 bits.
    For divide: check divide-by-zero (call `inv(pc)` if zero).
    Format `ASSEMBLY` appropriately:
    - `"mult   <rs>, <rt>"`
    - `"mfhi   <rd>"`
    - `"mthi   <rs>"`
  </action>
  <acceptance_criteria>
    - All 8 multiply/divide/move functions have non-empty bodies.
    - `div`/`divu` handle divide-by-zero by calling `inv(pc)`.
    - `cargo build` succeeds.
  </acceptance_criteria>
</task>

<task>
  <id>4</id>
  <title>Implement special instructions</title>
  <read_first>
    - src/emu/cpu/r_type.rs
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Implement in `src/emu/cpu/r_type.rs`:
    - `jr(pc)` — jump register: `CPU.pc = rs`
    - `jalr(pc)` — jump and link register: save `pc + 4` to rd, then `CPU.pc = rs`
    - `syscall(pc)` — system call placeholder (call `inv(pc)` or print and stop)
    - `_break(pc)` — breakpoint placeholder (call `inv(pc)` or print and stop)

    Wire `jr` (0x08) and `jalr` (0x09) into `_2BYTE_OPCODE_TABLE`.
  </action>
  <acceptance_criteria>
    - `jr` and `jalr` have non-empty bodies.
    - `syscall` and `_break` have placeholder behavior (e.g., print message and set `CPU_STATE = CpuState::END`).
    - `cargo build` succeeds.
  </acceptance_criteria>
</task>

<task>
  <id>5</id>
  <title>Wire all remaining instructions into dispatch tables</title>
  <read_first>
    - src/emu/cpu/exec.rs
  </read_first>
  <action>
    Ensure all implemented functions are wired:
    - `OPCODE_TABLE`: bz (0x01), beq (0x04), bne (0x05), blez (0x06), bgtz (0x07)
    - `_2BYTE_OPCODE_TABLE`: jr (0x08), jalr (0x09), mult (0x18), multu (0x19), div (0x1a), divu (0x1b)

    Replace remaining `inv` placeholders with correct functions.
  </action>
  <acceptance_criteria>
    - No `inv` placeholders remain for implemented opcodes in either table.
    - `cargo build` succeeds.
    - `cargo test` passes.
  </acceptance_criteria>
</task>

## Verification

- `cargo build` succeeds.
- `cargo test` passes.
- All J-type, branch, multiply/divide, and special instructions implemented.

## must_haves

- [ ] J-type jumps (`j`, `jal`) implemented
- [ ] All branch instructions implemented
- [ ] Multiply/divide and HI/LO move instructions implemented
- [ ] `jr`, `jalr`, `syscall`, `_break` implemented
- [ ] Dispatch tables fully wired
- [ ] Build and tests pass

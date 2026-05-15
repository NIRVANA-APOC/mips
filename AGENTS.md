# AGENTS.md — MIPS CPU Emulator

> This file is intended for AI coding agents. It describes the project structure, build process, and conventions.

## Project Overview

This is a **MIPS CPU emulator / simulator** written in Rust. It emulates a MIPS processor with an interactive command-line debugger (referred to internally as the "monitor"). The emulator loads binary instruction and data files, decodes and executes MIPS instructions, and provides a REPL-style UI for single-stepping, continuing execution, and inspecting registers.

The project is currently a work in progress. Only a subset of the MIPS instruction set is implemented; many instruction handlers are stubs.

## Technology Stack

- **Language**: Rust (Edition 2021)
- **Build Tool**: Cargo
- **External Dependency**: `colored` (v1.7.1 / resolves to v1.9.3) — used for ANSI-colored terminal output
- **Target**: Native binary (`bin` crate)

## Project Structure

```
.
├── Cargo.toml           # Rust package manifest
├── Cargo.lock           # Dependency lock file
├── .gitignore           # Ignores /target
├── bin/
│   ├── inst.bin         # MIPS instruction binary (loaded at 0xBFC0_0000)
│   └── data.bin         # Data binary (loaded at base of DRAM)
└── src/
    ├── main.rs          # Entry point: restarts monitor, runs UI main loop
    └── mod/
        ├── mod.rs       # Re-exports cpu, memory, monitor
        ├── cpu/         # CPU core: decode & execute
        │   ├── mod.rs
        │   ├── exec.rs       # Opcode dispatch tables, instr_fetch, exec loop
        │   ├── reg.rs        # GPR, CPU state (pc, hi, lo), register names
        │   ├── operand.rs    # Operand types and decoded operand struct
        │   ├── helper.rs     # Bit masks for instruction fields
        │   ├── r_type.rs     # R-type instruction implementations
        │   ├── i_type.rs     # I-type instruction implementations
        │   ├── j_type.rs     # J-type instruction implementations
        │   └── ...
        ├── memory/      # Memory subsystem
        │   ├── mod.rs
        │   ├── dram.rs       # DDR3-like DRAM simulation with row buffers
        │   └── memory.rs     # mem_read / mem_write wrappers
        └── monitor/     # Interactive debugger / monitor
            ├── mod.rs
            ├── monitor.rs    # init, binary loading, restart
            ├── ui.rs         # Command REPL (help, c, q, si, r, dbg, restart)
            └── cpu_exec.rs   # Execution loop: cpu_exec(n_steps)
```

## Build Commands

```bash
# Build the project
cargo build

# Build release
cargo build --release

# Run the emulator
cargo run

# Run tests
cargo test
```

### Known Build Issue

The project previously imported `std::os::windows::prelude::FileExt` in `src/emu/monitor/monitor.rs`, which prevented compilation on non-Windows platforms. This import has been removed.

## Runtime Architecture

1. **Startup**: `main()` calls `restart()`, which:
   - Prints "Hello World." (monitor init)
   - Clears DRAM and row buffers
   - Loads `./bin/inst.bin` to address `0xBFC0_0000` (masked to `0x1F_FF_FF_FF` offset in DRAM)
   - Loads `./bin/data.bin` to DRAM base address
   - Resets `CPU.pc` to the entry point `0xBF_C0_00_00`
   - Initializes DDR3 row buffers

2. **UI Main Loop**: `ui_mainloop()` presents a prompt `(Azathoth)>>>` and accepts commands:
   - `help` / `h` — list commands
   - `continue` / `c` — run until halt or breakpoint
   - `quit` / `q` — exit
   - `single` / `si` — single step
   - `reg` / `r [$name]` — print all registers or a specific register
   - `debug` / `dbg` — toggle debug trace output
   - `restart` / `re` — restart the emulator

3. **Execution**: `cpu_exec(n)` fetches, decodes, and executes up to `n` instructions.
   - `instr_fetch(addr, len)` reads memory via `mem_read`.
   - `exec(pc)` uses a 64-entry opcode table (`OPCODE_TABLE`) indexed by the top 6 bits.
   - Special opcode `0x00` dispatches to a secondary 64-entry table (`_2BYTE_OPCODE_TABLE`) indexed by the 6-bit `func` field.
   - After execution, the emulator prints the PC, raw bytes, and disassembly of the executed instruction.

## Memory Model

- **DRAM**: Simulated as a 4-D array `DRAM[NR_RANK][NR_BANK][NR_ROW][NR_COL]`.
- **Address breakdown**: rank (6 bits) | bank (3 bits) | row (10 bits) | column (10 bits).
- **Burst length**: 8 bytes.
- **Row buffers**: The DRAM simulator includes row-buffer caching (`ROW_BUFS`). Accessing a new row triggers a row-buffer load; writes update the row buffer and write back to DRAM.
- **Physical memory size**: `1 << 29` bytes (512 MB addressable via the DDR3 model).

## CPU State

- **GPR**: 32 general-purpose registers (`$zero`, `$at`, `$v0`–`$v1`, `$a0`–`$a3`, `$t0`–`$t7`, `$s0`–`$s7`, `$t8`–`$t9`, `$k0`–`$k1`, `$gp`, `$sp`, `$fp`, `$ra`).
- **PC**: Program counter.
- **HI / LO**: Multiply/divide result registers.

Global mutable state is used extensively (`static mut`) for `CPU`, `CPU_STATE`, `DRAM`, `ROW_BUFS`, `INSTR`, `OPS_DECODED`, etc.

## Implemented Instructions

The following instructions have partial or full implementations:
- **R-type**: `and`
- **I-type**: `lui`, `ori`
- **Special**: `eret`, `good_trap` (opcode `0x13`), `bad_trap` (opcode `0x14`)

Most other instructions are declared as empty stubs (`pub fn xxx(pc: u32) {}`).

## Code Style Guidelines

- **Unsafe code**: The codebase makes heavy use of `unsafe` blocks and `static mut` for performance and to emulate a C-style emulator architecture. When modifying memory or CPU state, expect to work inside `unsafe` blocks.
- **Constants**: Bit masks and field sizes are defined in `src/emu/cpu/helper.rs` using `u32` hex literals.
- **Color output**: `colored::Colorize` traits are used for terminal coloring. Errors are typically red, success/good traps are green, debug output is yellow, and prompt text is green.
- **Module naming**: The top-level module is named `emu` (formerly `mod`, escaped as `r#mod`).
- **Assembly formatting**: Implemented instructions construct disassembly strings in a `ASSEMLY` global buffer, formatted as `"<mnemonic>   <rd>, <rs>, <rt>"` (R-type) or `"<mnemonic>   <rt>, <imm>"` (I-type).

## Testing Strategy

- Unit tests are embedded in `mod test { }` blocks inside individual source files (`dram.rs`, `reg.rs`, `exec.rs`, `monitor.rs`).
- Run all tests with `cargo test`.
- Tests cover:
  - GPR initialization and read/write width variants (`reg.rs`)
  - DRAM row-buffer read/write and unaligned access (`dram.rs`)
  - Binary loading and trap formatting (`exec.rs`, `monitor.rs`)

There is no dedicated integration test suite or automated emulator validation against reference MIPS binaries yet.

## Security Considerations

- The emulator uses extensive `unsafe` code, raw pointers (`ptr::copy_nonoverlapping`), and `static mut` globals. Bounds checking is minimal; out-of-bounds DRAM accesses use `assert!`, but many CPU/memory paths lack thorough validation.
- Register index bounds are soft-checked (prints a red warning and returns index 0 instead of panicking).
- Because this is an emulator intended for local, educational use, untrusted binary input could cause memory safety issues due to the reliance on `unsafe` blocks.

## Notes for Agents

- When adding new instructions, implement the handler in the appropriate `r_type.rs`, `i_type.rs`, or `j_type.rs`, then wire it into the corresponding opcode table in `exec.rs`.
- To add a new monitor command, define a handler function returning `UiState`, register it in `CMD_TABLE` inside `ui.rs`, and update `cmd_help` if needed.
- The `bin/` directory is required at runtime; the executable loads `./bin/inst.bin` and `./bin/data.bin` with relative paths from the working directory.

---
phase: "02"
plan_id: "module-rename"
wave: 1
depends_on: []
files_modified:
  - src/main.rs
  - src/mod/mod.rs
  - src/mod/cpu/mod.rs
  - src/mod/cpu/exec.rs
  - src/mod/cpu/r_type.rs
  - src/mod/cpu/i_type.rs
  - src/mod/memory/dram.rs
  - src/mod/monitor/monitor.rs
  - src/mod/monitor/ui.rs
  - Cargo.toml
autonomous: true
---

# Plan: Rename `r#mod` module to `emu`

## Objective

Rename the top-level module from `r#mod` (raw identifier) to `emu` (conventional Rust identifier), updating all references across the codebase.

## Tasks

<task>
  <id>1</id>
  <title>Rename directory and module declaration</title>
  <read_first>
    - src/main.rs
    - src/mod/mod.rs
    - Cargo.toml
  </read_first>
  <action>
    1. Rename `src/mod/` directory to `src/emu/`.
    2. In `src/main.rs`, change `mod r#mod;` to `mod emu;` and update `use r#mod::monitor::...` to `use emu::monitor::...`.
    3. In `src/emu/mod.rs` (formerly `src/mod/mod.rs`), ensure the module declaration uses `pub mod emu` or adjust as needed.
  </action>
  <acceptance_criteria>
    - `src/emu/` directory exists and contains all former `src/mod/` files.
    - `src/mod/` no longer exists.
    - `src/main.rs` declares `mod emu;` and imports from `emu::`.
    - `cargo build` produces no "module not found" errors.
  </acceptance_criteria>
</task>

<task>
  <id>2</id>
  <title>Update crate-internal references</title>
  <read_first>
    - src/mod/cpu/r_type.rs
    - src/mod/cpu/i_type.rs
    - src/mod/cpu/exec.rs
    - src/mod/memory/dram.rs
    - src/mod/monitor/monitor.rs
    - src/mod/monitor/ui.rs
  </read_first>
  <action>
    Replace every occurrence of `crate::r#mod::` with `crate::emu::` in:
    - src/emu/cpu/r_type.rs
    - src/emu/cpu/i_type.rs
    - src/emu/cpu/exec.rs
    - src/emu/memory/dram.rs
    - src/emu/monitor/monitor.rs
    - src/emu/monitor/ui.rs
  </action>
  <acceptance_criteria>
    - `grep -r "r#mod" src/` returns zero matches.
    - `grep -r "crate::emu::" src/emu/` finds all expected import lines.
    - `cargo build` succeeds with zero errors.
    - `cargo test` passes all 7 tests.
  </acceptance_criteria>
</task>

<task>
  <id>3</id>
  <title>Update AGENTS.md and planning docs</title>
  <read_first>
    - AGENTS.md
    - .planning/PROJECT.md
    - .planning/ROADMAP.md
  </read_first>
  <action>
    Update AGENTS.md, PROJECT.md, and ROADMAP.md to reference `emu` instead of `r#mod` or `mod` where describing the module structure.
  </action>
  <acceptance_criteria>
    - AGENTS.md §Module naming references `emu` or notes the rename.
    - Planning docs are consistent with the new module name.
  </acceptance_criteria>
</task>

## Verification

- `cargo build` succeeds.
- `cargo test` passes.
- No `r#mod` references remain in `src/` or `.planning/`.

## must_haves

- [ ] Module renamed from `r#mod` to `emu`
- [ ] All crate-internal imports updated
- [ ] Build and tests pass

# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v1.0 — MVP

**Shipped:** 2026-05-15
**Phases:** 3 | **Plans:** 7 | **Sessions:** 1

### What Was Built
- Cross-platform compilation fix (removed Windows-only import)
- Module rename from `r#mod` to `emu`
- 44 MIPS instructions implemented (R-type, I-type, J-type, branches, multiply/divide)
- 49 unit tests covering all implemented instructions
- Test infrastructure with `encode_r_type`, `encode_i_type`, `encode_j_type` helpers

### What Worked
- Incremental bug fixing during test writing caught issues early
- Using `load_instructions()` helper standardized test setup
- The `cpu_exec(1)` single-step pattern made tests deterministic

### What Was Inefficient
- Test assertions had copy-paste errors (`test_andi`, `test_addiu`)
- Parallel test execution fails due to shared `static mut` globals
- GSD metadata (SUMMARY.md, checkboxes) lagged behind actual completion

### Patterns Established
- Tests in `mod test {}` blocks inside source files
- Instruction encoding helpers for constructing test vectors
- `clear_dram()` + `init_ddr3()` + `CPU::new()` in test setup

### Key Lessons
1. Test the tests — incorrect assertions silently pass wrong behavior
2. Global mutable state makes parallel testing impossible
3. Off-by-4 PC compensation is easy to miss when `cpu_exec` auto-increments

### Cost Observations
- Model mix: 100% balanced
- Sessions: 1 continuous session
- Notable: High efficiency — all phases completed in single session

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v1.0 | 1 | 3 | Initial GSD adoption — learning workflow patterns |

### Cumulative Quality

| Milestone | Tests | Coverage | Zero-Dep Additions |
|-----------|-------|----------|-------------------|
| v1.0 | 49 | Core instructions | 0 |

### Top Lessons (Verified Across Milestones)

1. (Pending next milestone for cross-validation)

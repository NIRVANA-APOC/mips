# Milestones

## v1.0 MVP (Shipped: 2026-05-15)

**Phases completed:** 3 phases, 7 plans, 49 tests

**Key accomplishments:**

- Fixed cross-platform compilation on Linux (removed Windows-only import)
- Renamed module from `r#mod` to `emu` across entire codebase
- Implemented 44 MIPS instructions (R-type, I-type, J-type, branches, multiply/divide)
- Added 49 unit tests covering all implemented instructions
- Fixed critical bugs: `memcpy_with_mask`, jump off-by-4, `slti` sign-extension
- Validated memory alignment handling and DRAM row buffer simulation

**Files:**
- [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- [v1.0-REQUIREMENTS.md](milestones/v1.0-REQUIREMENTS.md)

---

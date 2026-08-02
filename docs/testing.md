# Testing Guide

This document describes the test strategy and the latest documented verification
status for this repository. It is the authoritative project document for test
layers, documented test counts, verified test status, test coverage boundaries,
and test environment limitations. For general development commands and
environment setup, see [`development.md`](./development.md). For agent working
rules, see [`../AGENTS.md`](../AGENTS.md).

## Purpose

The test suite is intended to protect calculation behavior, validation behavior,
frontend form behavior, Tauri request/response mapping, database lookup behavior,
and data import behavior.

Calculation software needs both automated behavioral tests and domain review.
Automated tests can confirm that the implementation still matches expected test
cases and invariants. They cannot, by themselves, prove that CNC formulas,
standards interpretation, machining assumptions, or industrial usage are correct.

## Test Layers

| Layer                                       | Tooling                                                 | Main locations                                                       |
| ------------------------------------------- | ------------------------------------------------------- | -------------------------------------------------------------------- |
| Frontend unit and component tests           | Vitest, React Testing Library                           | `src/**/*.test.ts`, `src/**/*.test.tsx`                              |
| Rust domain tests                           | Cargo test, Rust test framework, Proptest where present | `src-tauri/tests/domain/`, inline tests in `src-tauri/src/domain/`   |
| Rust application tests                      | Cargo test                                              | `src-tauri/tests/application/`                                       |
| Rust interface tests                        | Cargo test                                              | `src-tauri/tests/interface/tauri/`                                   |
| SQLite/database tests                       | Cargo test                                              | `src-tauri/tests/iso286_database.rs`, `src-tauri/tests/tolerance.rs` |
| Python import tests                         | Python `unittest`                                       | `scripts/test_import_iso286.py`                                      |
| Formatting, linting, static analysis, build | Prettier, ESLint, TypeScript, Vite, cargo fmt, clippy   | project configuration files and `package.json` scripts               |

## Current Verified Status

The latest documented verification reported the following results. These numbers
represent that verification run and may change as tests are added, removed, or
reorganized.

| Check                                       | Latest documented result |
| ------------------------------------------- | ------------------------ |
| Frontend test files                         | 18 files                 |
| Frontend tests                              | 73 passing tests         |
| Rust tests                                  | 275 passing tests        |
| ISO 286 import tests                        | 4 passing tests          |
| Prettier                                    | passes                   |
| ESLint                                      | passes                   |
| TypeScript and Vite build                   | passes                   |
| `cargo fmt --check`                         | passes                   |
| `cargo clippy --all-targets -- -D warnings` | passes                   |

These results verify agreement with the implemented expectations. They do not
prove CNC correctness, safety, or standards compliance beyond the tested cases.

## Test Locations

Frontend tests are colocated under `src/` using `.test.ts` and `.test.tsx` file
names. Representative files include:

- [`../src/shared/parsing/decimalParser.test.ts`](../src/shared/parsing/decimalParser.test.ts)
- [`../src/shared/form/engine/formEngine.test.ts`](../src/shared/form/engine/formEngine.test.ts)
- [`../src/features/calculatorForms.test.tsx`](../src/features/calculatorForms.test.tsx)
- [`../src/features/finishing/finishingRtl.test.tsx`](../src/features/finishing/finishingRtl.test.tsx)
- [`../src/shared/ui/primitives/input/NumberInput/NumberInput.test.tsx`](../src/shared/ui/primitives/input/NumberInput/NumberInput.test.tsx)

Rust integration-style tests are organized under
[`../src-tauri/tests`](../src-tauri/tests). Representative areas include:

- domain behavior in
  [`../src-tauri/tests/domain/`](../src-tauri/tests/domain)
- application workflows in
  [`../src-tauri/tests/application/`](../src-tauri/tests/application)
- Tauri command-facing mapping and validation in
  [`../src-tauri/tests/interface/tauri/`](../src-tauri/tests/interface/tauri)
- ISO 286 database checks in
  [`../src-tauri/tests/iso286_database.rs`](../src-tauri/tests/iso286_database.rs)
- tolerance tests in
  [`../src-tauri/tests/tolerance.rs`](../src-tauri/tests/tolerance.rs)

Inline Rust tests also exist in domain and utility modules, including property
tests where `proptest!` is used. Representative files include:

- [`../src-tauri/src/domain/geometry/circle/circle.rs`](../src-tauri/src/domain/geometry/circle/circle.rs)
- [`../src-tauri/src/domain/geometry/helix/helix.rs`](../src-tauri/src/domain/geometry/helix/helix.rs)
- [`../src-tauri/src/domain/units/length/length.rs`](../src-tauri/src/domain/units/length/length.rs)
- [`../src-tauri/src/domain/units/motion/rpm.rs`](../src-tauri/src/domain/units/motion/rpm.rs)

ISO 286 import behavior is tested by
[`../scripts/test_import_iso286.py`](../scripts/test_import_iso286.py), which
uses Python `unittest`, temporary directories, fixture CSV content, and SQLite
queries.

## What Is Covered

The existing tests cover these areas:

- decimal parsing, comma decimal separators, strict numeric parsing, and
  unsupported decimal formats in
  [`../src/shared/parsing/decimalParser.test.ts`](../src/shared/parsing/decimalParser.test.ts)
- shared form engine behavior, field clearing, solve result application, and
  frontend validation errors in
  [`../src/shared/form/engine/formEngine.test.ts`](../src/shared/form/engine/formEngine.test.ts)
- frontend form interactions for multiple calculator pages in
  [`../src/features/calculatorForms.test.tsx`](../src/features/calculatorForms.test.tsx)
- finishing React Testing Library flows in
  [`../src/features/finishing/finishingRtl.test.tsx`](../src/features/finishing/finishingRtl.test.tsx)
- frontend API mapping for cutting data and tolerances in
  [`../src/features/cuttingData/api/solveCuttingData.test.ts`](../src/features/cuttingData/api/solveCuttingData.test.ts)
  and
  [`../src/features/tolerances/api/solveTolerance.test.ts`](../src/features/tolerances/api/solveTolerance.test.ts)
- shared Tauri error parsing in
  [`../src/shared/api/tauriError.test.ts`](../src/shared/api/tauriError.test.ts)
- cutting data formulas, consistency, partial inputs, and validation in
  [`../src-tauri/tests/domain/machining/cutting_data/`](../src-tauri/tests/domain/machining/cutting_data)
  and
  [`../src-tauri/tests/application/cutting_data/`](../src-tauri/tests/application/cutting_data)
- right triangle domain, workflow, consistency, validation, and Tauri mapping in
  [`../src-tauri/tests/domain/geometry/right_triangle_solver_tests.rs`](../src-tauri/tests/domain/geometry/right_triangle_solver_tests.rs),
  [`../src-tauri/tests/application/right_triangle/`](../src-tauri/tests/application/right_triangle),
  and
  [`../src-tauri/tests/interface/tauri/right_triangle/`](../src-tauri/tests/interface/tauri/right_triangle)
- helix application and Tauri interface behavior in
  [`../src-tauri/tests/application/helix/`](../src-tauri/tests/application/helix)
  and
  [`../src-tauri/tests/interface/tauri/helix/`](../src-tauri/tests/interface/tauri/helix)
- finishing planning, execution, workflow, locking, mapping, and Tauri-facing
  behavior in
  [`../src-tauri/tests/domain/machining/finishing/`](../src-tauri/tests/domain/machining/finishing),
  [`../src-tauri/tests/application/finishing/`](../src-tauri/tests/application/finishing),
  and
  [`../src-tauri/tests/interface/tauri/finishing/`](../src-tauri/tests/interface/tauri/finishing)
- cylinder weight domain, application, repository, UI, and interface behavior in
  [`../src-tauri/tests/domain/machining/cylinder_weight/`](../src-tauri/tests/domain/machining/cylinder_weight),
  [`../src-tauri/tests/application/cylinder_weight/`](../src-tauri/tests/application/cylinder_weight),
  [`../src-tauri/tests/interface/tauri/cylinder_weight/`](../src-tauri/tests/interface/tauri/cylinder_weight),
  and frontend cylinder weight tests under
  [`../src/features/cylinder_weight/`](../src/features/cylinder_weight)
- thread solving and Tauri-facing thread behavior in
  [`../src-tauri/tests/domain/machining/thread/thread_solver_tests.rs`](../src-tauri/tests/domain/machining/thread/thread_solver_tests.rs)
  and
  [`../src-tauri/tests/interface/tauri/thread/`](../src-tauri/tests/interface/tauri/thread)
- ISO 286 database metadata, row counts, interval boundaries, supported classes,
  golden rows, fit calculations, and invalid input behavior in
  [`../src-tauri/tests/iso286_database.rs`](../src-tauri/tests/iso286_database.rs)
- ISO 286 CSV import parsing, incomplete tolerance pairs, required zone
  validation, and SQLite output in
  [`../scripts/test_import_iso286.py`](../scripts/test_import_iso286.py)
- edge cases and property tests where `proptest!` is present in domain unit and
  geometry modules

## What Is Not Yet Covered

The verified commands do not currently prove:

- full Tauri desktop end-to-end user flow
- installed or bundled resource lookup in a packaged app
- packaged application smoke test
- thread database generator workflow in
  [`../scripts/import_threads.py`](../scripts/import_threads.py)
- professional or standards-based validation of CNC formulas
- professional or standards-based validation of imported reference data
- measured code coverage percentages

No coverage percentage is documented because no coverage command has been
measured and recorded.

## Running Tests

Run commands from the repository root unless noted otherwise.

| Purpose                            | Command                                |
| ---------------------------------- | -------------------------------------- |
| Prettier check                     | `npm run format:check`                 |
| ESLint                             | `npm run lint`                         |
| Frontend tests                     | `npm run test:run`                     |
| TypeScript and Vite build          | `npm run build`                        |
| Frontend quality gate              | `npm run check:frontend`               |
| Backend quality gate               | `npm run check:backend`                |
| Combined frontend and backend gate | `npm run check`                        |
| ISO 286 import tests               | `python scripts/test_import_iso286.py` |

`npm run check:frontend` is defined in
[`../package.json`](../package.json) as Prettier check, ESLint, frontend tests,
and frontend build.

`npm run check:backend` is defined in
[`../package.json`](../package.json) as a PowerShell command that changes into
`src-tauri` and runs:

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Environment Notes

Vitest may fail inside the Codex sandbox because esbuild can hit directory
access restrictions. Rust tests may fail inside the Codex sandbox on Windows
because linker access can be restricted.

If the same command passes outside the sandbox, treat the sandbox failure as an
environment issue rather than a confirmed code failure.

## Interpreting Test Results

Passing tests confirm that the current code agrees with the implemented
expectations in the test suite. They do not, by themselves, prove industrial
safety, CNC correctness, or standards compliance.

Failing tests should be separated from environment failures. A failure caused by
sandbox access restrictions has a different meaning from a deterministic
assertion failure, type error, lint error, or build error.

## Future Test Priorities

The most useful next additions are:

1. packaged app smoke test that verifies the built desktop app starts
1. Tauri command-to-runtime integration test for real command invocation paths
1. installed resource lookup test for bundled SQLite files
1. thread database generation verification for
   [`../scripts/import_threads.py`](../scripts/import_threads.py)
1. representative domain reference cases reviewed against trusted CNC or
   standards sources

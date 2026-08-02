# AGENTS.md

Guidance for Codex and other code agents working in this repository.

## Project Overview

This is a Tauri 2 desktop application for CNC calculation and workflow support.
The frontend is built with React, TypeScript, Vite, and Vitest. The backend is
Rust, exposed to the UI through Tauri commands. The app uses local bundled
SQLite reference databases for standards and lookup data.

Core domain behavior covers CNC calculations, machining workflows, tolerances,
threads, cutting data, finishing, geometry, units, and related business rules.

## Before Making Changes

- Inspect the relevant existing implementation before editing.
- Preserve established naming, module boundaries, and architecture patterns.
- Distinguish verified facts from assumptions in explanations and docs.
- Do not combine formatting, refactoring, and functional changes in one task.
- Do not change CNC behavior during documentation-only tasks.
- Do not rewrite unrelated code while fixing a narrow issue.

## Architecture Conventions

- Frontend code is organized by app setup, shared utilities/UI, and feature
  modules under `src/`.
- Frontend-to-backend calls cross the Tauri command boundary through shared API
  helpers.
- Rust backend code is separated into interface, application, and domain layers
  under `src-tauri/src/`.
- CNC calculations and business rules belong in the Rust domain layer.
- UI code must not duplicate domain formulas.
- Cross-module changes must follow existing public interfaces unless the task
  explicitly requires changing the contract.

## Naming and File Conventions

- Use the existing lower camelCase convention for TypeScript utility and
  validation files.
- Use PascalCase for React component files where that matches current modules.
- Preserve case-sensitive import consistency.
- Inspect similar modules before creating new folders, files, or naming
  patterns.

## Testing Requirements

- Add or update tests for behavior changes.
- Calculation changes require representative values and edge cases.
- Do not claim correctness beyond what tests verify.
- Passing tests do not alone prove CNC correctness or standards compliance.
- CNC formulas, standards data, and machining assumptions require manual or
  professional verification when correctness claims are made.

## Documentation Requirements

- Update relevant docs when architecture, commands, data flow, or domain
  behavior changes.
- Describe the current implementation separately from proposed improvements.
- Cite concrete source files for technical claims.
- Clearly mark claims that are not automatically verified or professionally
  reviewed.

## Verification Commands

Use the existing project commands:

```powershell
npm run format:check
npm run lint
npm run test:run
npm run build
npm run check:backend
python scripts/test_import_iso286.py
```

`npm run check:frontend` currently runs Prettier check, ESLint, frontend tests,
and frontend build. `npm run check:backend` currently runs `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, and `cargo test` from `src-tauri`.

## Environment Notes

- Vitest may require execution outside the Codex sandbox in this environment due
  to esbuild access errors.
- Rust tests may require execution outside the sandbox due to Windows linker
  access.
- Do not report these environment failures as code failures if the same commands
  pass outside the sandbox.

## Completion Report

Every completed task should report:

- files changed
- reason for each change
- commands executed
- actual results
- anything not verified
- remaining risks or follow-up work

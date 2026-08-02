# Development Guide

This guide describes the current development workflow for this repository. For a
project overview and feature summary, see [`../README.md`](../README.md). For
test strategy and current verification status, see
[`testing.md`](./testing.md). For agent-specific working rules, see
[`../AGENTS.md`](../AGENTS.md).

## Verified Scope

The commands and paths in this document are based on the current repository
configuration:

- frontend scripts in [`../package.json`](../package.json)
- TypeScript settings in [`../tsconfig.json`](../tsconfig.json)
- Vite and Vitest settings in [`../vite.config.ts`](../vite.config.ts)
- Tauri settings in [`../src-tauri/tauri.conf.json`](../src-tauri/tauri.conf.json)
- Rust crate settings in [`../src-tauri/Cargo.toml`](../src-tauri/Cargo.toml)
- formatting and linting settings in [`../.prettierrc`](../.prettierrc),
  [`../.prettierignore`](../.prettierignore), and
  [`../eslint.config.js`](../eslint.config.js)

Detailed test layers, current documented test counts, verified status, coverage
boundaries, and test environment limitations are documented in
[`testing.md`](./testing.md).

## Project Layout

| Path                         | Purpose                                                                  |
| ---------------------------- | ------------------------------------------------------------------------ |
| `src/`                       | React and TypeScript frontend                                            |
| `src/app/`                   | Application shell, routes, and providers                                 |
| `src/features/`              | Feature-specific UI, controllers, API adapters, and local domain helpers |
| `src/shared/`                | Shared frontend API, form, parsing, UI, and saved-result utilities       |
| `src-tauri/`                 | Tauri 2 application and Rust backend crate                               |
| `src-tauri/src/interface/`   | Tauri command-facing layer                                               |
| `src-tauri/src/application/` | Application use cases and repositories                                   |
| `src-tauri/src/domain/`      | Domain rules, CNC calculations, units, and invariants                    |
| `src-tauri/data/`            | Bundled SQLite reference databases and source data                       |
| `scripts/`                   | Data import and import-test scripts                                      |
| `docs/`                      | Project documentation                                                    |

## Development Commands

Run commands from the repository root unless noted otherwise.

| Task                   | Command                  | Source         |
| ---------------------- | ------------------------ | -------------- |
| Start Vite dev server  | `npm run dev`            | `package.json` |
| Build frontend         | `npm run build`          | `package.json` |
| Run Tauri CLI          | `npm run tauri`          | `package.json` |
| Preview built frontend | `npm run preview`        | `package.json` |
| Run full frontend gate | `npm run check:frontend` | `package.json` |
| Run full backend gate  | `npm run check:backend`  | `package.json` |
| Run combined gate      | `npm run check`          | `package.json` |

Tauri development uses `npm run dev` as `beforeDevCommand` and expects Vite at
`http://localhost:1420`, as configured in
[`../src-tauri/tauri.conf.json`](../src-tauri/tauri.conf.json). The Vite server
uses port `1420` with `strictPort: true` in
[`../vite.config.ts`](../vite.config.ts).

## Quality Commands

Use these commands when validating changes. See [`testing.md`](./testing.md) for
what each command verifies and for the latest documented results.

```powershell
npm run format:check
npm run lint
npm run test:run
npm run build
npm run check:backend
python scripts/test_import_iso286.py
```

## Frontend Notes

The frontend uses React 19, TypeScript, Vite, Vitest, Testing Library, React
Router, and the Tauri JavaScript API. These dependencies are declared in
[`../package.json`](../package.json).

TypeScript uses `strict: true`, `noUnusedLocals: true`, and
`noUnusedParameters: true` in [`../tsconfig.json`](../tsconfig.json). Path
aliases are configured in both [`../tsconfig.json`](../tsconfig.json) and
[`../vite.config.ts`](../vite.config.ts).

Vitest uses [`../src/test/setupTests.ts`](../src/test/setupTests.ts) as its setup
file through [`../vite.config.ts`](../vite.config.ts).

## Backend Notes

The Rust backend is the Tauri crate under `src-tauri/`. Its library crate is
configured in [`../src-tauri/Cargo.toml`](../src-tauri/Cargo.toml) as
`cnc_machining_system_lib`.

Tauri commands are registered from [`../src-tauri/src/lib.rs`](../src-tauri/src/lib.rs).
The backend follows the existing interface, application, and domain layering.
CNC calculations and business rules should remain in the Rust domain layer under
`src-tauri/src/domain/`.

## Data and Resources

The packaged Tauri app declares these bundled resources in
[`../src-tauri/tauri.conf.json`](../src-tauri/tauri.conf.json):

- `data/iso286.sqlite`
- `data/threads.sqlite`

Generated SQLite files and build output are excluded from Prettier and ESLint by
[`../.prettierignore`](../.prettierignore) and
[`../eslint.config.js`](../eslint.config.js).

## Formatting and Linting

Prettier is configured with `printWidth: 88` and `trailingComma: "all"` in
[`../.prettierrc`](../.prettierrc).

ESLint uses the flat config in [`../eslint.config.js`](../eslint.config.js),
including JavaScript recommended rules, TypeScript ESLint recommended rules,
React Hooks rules, and project-specific ignore patterns.

Keep formatting-only changes separate from functional changes, as required by
[`../AGENTS.md`](../AGENTS.md). Test environment limitations and manually
unverified behavior are tracked in [`testing.md`](./testing.md).

## Change Discipline

Before making changes, inspect nearby implementations and preserve existing
patterns. In particular:

- use lower camelCase for TypeScript utility and validation files where matching
  modules already use that convention
- use PascalCase for React component files where matching modules already use
  that convention
- keep case-sensitive imports consistent with file names
- update or add tests for behavior changes
- keep documentation changes separate from code refactoring

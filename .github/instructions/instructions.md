---
applyTo: '**'
---

# egui_kittest Demo – Contributor Guide

This repository demonstrates best practices for testing egui UIs with egui_kittest. The goal is to build responsive, accessible, and testable UIs and to let the agent “see” its work via image snapshots.

## Project overview

- `src/lib.rs`: DemoApp (eframe::App) with responsive layout (menus, side panel, adaptive form, responsive grid)
- `src/main.rs`: Native entry point
- `tests/`: Integration tests, app tests, and snapshot tests
  - Snapshot images live under `tests/snapshots/`

## Key crates

- Runtime: `egui` 0.32, `eframe` 0.32
- Testing: `egui_kittest` 0.32 with `wgpu`, `snapshot`, `eframe` features

## Goals

- Responsive layouts that adapt to narrow/medium/wide widths
- Accessibility via AccessKit roles (queryable in tests)
- Deterministic tests that assert behavior and visuals

## Tools you should use

- rust-mcp-server (Cargo wrapper)
  - Format: cargo-fmt
  - Lints: cargo-clippy
  - Check/Build: cargo-check, cargo-build
  - Test: cargo-test (supports feature flags and env vars)
- Memory tool
  - Store conventions, decisions, and context for future sessions

## Common workflows

1. Add/edit UI
   - Keep labels and roles stable; avoid dynamic text in selectors when possible.
   - Prefer `ui.horizontal_wrapped`, `ui.columns`, and width-based branching for responsiveness.

2. Write tests
   - Basic interactions: use `Harness::new`/`new_ui`, then `get_by_label` / `get_by_role` and `.click()`, `.type_text()`.
   - Resizing: use `Harness::builder().with_size(vec2)` and `harness.set_size(vec2)`; call `harness.run()` after changing size.
   - Snapshots: call `harness.snapshot("name")` to emit `tests/snapshots/name.png`.

3. Update snapshots
   - First run will fail if a reference image is missing. Update by running:
     - `UPDATE_SNAPSHOTS=true cargo test` (macOS/Linux)
   - Commit only reference `.png` files; `.diff.png`/`.new.png` files are ignored.

## Standards

- Formatting: run cargo-fmt before commits.
- Linting: run cargo-clippy; fix or explicitly justify warnings.
- Tests: all features must have tests; don’t regress accessibility queries.
- Snapshot scope: keep images small and focused; prefer logic/assert tests when possible.

### UI accessibility & overflow

- Always ensure overflow content is scrollable: wrap potentially tall content in `egui::ScrollArea::vertical()`.
- Test small sizes to guarantee content remains reachable (e.g., assert bottom items are findable by label/role).

## Quick commands (through rust-mcp-server)

- cargo-fmt: format all crates
- cargo-check: fast type-check
- cargo-clippy: lints (optionally set warnings as errors)
- cargo-test: run unit, integration, and snapshot tests

## Notes

- To minimize flakiness, we disable cursor blink and scroll animations in the harness.
- If multiple sizes are needed, take snapshots in sequence: narrow → medium → wide.

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
- In-editor Git tool
  - Stage, commit in small batches with descriptive messages
  - Keep commits focused (tests, docs, fixes) for easy review
- Memory tool
  - Store conventions, decisions, and context for future sessions

### rust-mcp-server workflows

- Format all crates: cargo-fmt
- Lint with clippy (treat warnings as errors in CI): cargo-clippy
- Fast type-check: cargo-check
- Build binaries: cargo-build
- Run tests (workspace): cargo-test
- Run a specific test target: cargo-test with --test `file`
- Update snapshots (macOS): set UPDATE_SNAPSHOTS=true and run cargo-test

Notes:

- Always run tests after resizing the harness window (call harness.run()).
- If your test calls DemoApp::update directly, import the trait: use eframe::App.

### Git in-editor tasks

- Stage only the files you intend to change (use the source control panel).
- Commit message guidelines:
  - Prefix by scope: tests:, docs:, fix:, ci:, refactor:
  - Imperative mood, short subject; details in body if needed
  - Example: tests: add a11y keyboard/geometry and scroll_to_me coverage
- Commit frequently in small, reviewable chunks:
  - Update snapshots and tests separately from code changes
  - Keep CI/config edits separate from app code

## Common workflows

1. Add/edit UI

- Keep labels and roles stable; avoid dynamic text in selectors when possible.
- Prefer `ui.horizontal_wrapped`, `ui.columns`, and width-based branching for responsiveness.

1. Write tests

- Basic interactions: use `Harness::new`/`new_ui`, then `get_by_label` / `get_by_role` and `.click()`, `.type_text()`.
- Resizing: use `Harness::builder().with_size(vec2)` and `harness.set_size(vec2)`; call `harness.run()` after changing size.
- Snapshots: call `harness.snapshot("name")` to emit `tests/snapshots/name.png`.
- Keyboard & focus: call `.focus()` on the node, then `harness.key_press(..)` or `.type_text(..)`
- Geometry checks: use `.rect()` on nodes to assert order/positions
- Scrolling: use `.scroll_to_me()` to bring off-screen nodes into view

1. Update snapshots

- First run will fail if a reference image is missing. Update by running:
- `UPDATE_SNAPSHOTS=true cargo test` (macOS/Linux)
- Commit only reference `.png` files; `.diff.png`/`.new.png` files are ignored.

1. Quality gates (before commit/PR)

- cargo-fmt (formatting clean)
- cargo-clippy (no warnings; CI denies warnings)
- cargo-test (workspace green; snapshots updated on macOS)

## Standards

- Formatting: run cargo-fmt before commits.
- Linting: run cargo-clippy; fix or explicitly justify warnings.
- Tests: all features must have tests; don’t regress accessibility queries.
- Snapshot scope: keep images small and focused; prefer logic/assert tests when possible.

### Testing patterns & best practices

- Prefer semantic/geometry assertions over large snapshots
- Use stable labels/roles; add explicit labels like `Layout: …` and `Columns: …`
- Test responsive behavior at representative widths:
  - 360 px: stacked + 1 column
  - 820 px: side+central + 2 columns (accounts for SidePanel width)
  - 1280 px: 3 columns
- Wrap tall content in `egui::ScrollArea::vertical()` to keep items reachable at small sizes
- Use `scroll_to_me()` to bring target nodes into view in tests
- For DragValue/SpinButton, `.focus()` + `.type_text(..)` can set the value reliably

### UI accessibility & overflow

- Always ensure overflow content is scrollable: wrap potentially tall content in `egui::ScrollArea::vertical()`.
- Test small sizes to guarantee content remains reachable (e.g., assert bottom items are findable by label/role).

## Quick commands (through rust-mcp-server)

- cargo-fmt: format all crates
- cargo-check: fast type-check
- cargo-clippy: lints (optionally set warnings as errors)
- cargo-test: run unit, integration, and snapshot tests

Examples:

```bash
# format, lint, test (workspace)
cargo-fmt
cargo-clippy
cargo-test

# run a specific test file
cargo-test --test a11y_keyboard_tests

# update snapshots (macOS)
UPDATE_SNAPSHOTS=true cargo-test
```

## Notes

- To minimize flakiness, we disable cursor blink and scroll animations in the harness.
- If multiple sizes are needed, take snapshots in sequence: narrow → medium → wide.

### Troubleshooting

- "method `update` not found": add `use eframe::App` in the test to import the trait
- Snapshot diffs on Linux: skip snapshot assertions on Linux in CI; update snapshots on macOS
- After `set_size(..)` or input events, call `harness.run()` to settle frames
- If a node isn’t visible at small sizes, call `.scroll_to_me()` before asserting

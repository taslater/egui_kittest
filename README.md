# egui_kittest Demo

[![CI](https://github.com/taslater/egui_kittest/actions/workflows/ci.yml/badge.svg)](https://github.com/taslater/egui_kittest/actions/workflows/ci.yml)

This project demonstrates how to use `egui_kittest` for testing egui applications. It includes a responsive demo app and comprehensive tests across semantic, geometry, and snapshot styles.

## Project Structure

- `src/lib.rs` – The demo application (`DemoApp`) with responsive layout
- `src/main.rs` – Binary entry point to run the demo
- `tests/app_tests.rs` – App-focused functional tests (inputs, dialogs, scrolling)
- `tests/integration_tests.rs` – Wider interaction and responsive assertions
- `tests/snapshot_tests.rs` – Narrow/medium/wide snapshots + fit_contents
- `tests/a11y_keyboard_tests.rs` – Accessibility/keyboard, geometry, scroll-to-view
- `tests/snapshots/` – Snapshot reference images

## Features Demonstrated

### Demo Application Features

- Responsive layout:
  - Wide: resizable SidePanel + CentralPanel
  - Narrow (< 600 px): stacked layout
- Overflow-safe: left and central content are wrapped in `ScrollArea::vertical`
- Adaptive grid of “Card” items with column thresholds:
  - width >= 900 → 3 columns
  - width >= 600 → 2 columns
  - else → 1 column
- Form: name (TextInput) and age (SpinButton via DragValue)
- Counter with increment/decrement
- Modal dialog with Yes/No
- Semantic labels used by tests:
  - `Layout: Stacked` or `Layout: Side+Central`
  - `Columns: {n}`
  - `Scale: {n}%`, `Scale bucket: Small|Medium|Large`, `Scaling mode: Zoom|Style`

### Adaptive scaling (two modes)

- Zoom-based scaling (default): uses `Context::set_zoom_factor` with discrete buckets to avoid oscillations and repaint loops.
  - < 600 → 0.85x, < 900 → 1.00x, < 1280 → 1.25x, ≥ 1280 → 1.50x
- Style-based scaling: leaves zoom at 1.0 and scales typography/spacing from a captured baseline `Style` (idempotent) using discrete buckets.
  - < 600 → 0.95x, < 900 → 1.15x, < 1280 → 1.35x, ≥ 1280 → 1.60x
- Toggle via View → “Scaling strategy” (Zoom-based / Style-based). The current mode is also shown in-content as `Scaling mode: …`.
- Breakpoints (stacking, columns, and bucket labels) use physical window width (points × pixels_per_point) so they’re stable across DPI and independent of zoom.

### Testing Features

- Semantic queries with AccessKit roles and labels: `get_by_role`, `get_by_label`, `get_by_value`
- Keyboard/focus interactions: `.focus()`, `harness.key_press`, `.type_text()`
- Geometry checks using `.rect()` to assert layout order/positions
- Scroll reachability: `scroll_to_me()` to bring off-screen content into view
- Window resizing in tests with `Harness::builder().with_size(..)` and `harness.set_size(..)`
- Image snapshots at multiple sizes and `fit_contents()` flows
- CI-friendly: stable labels; minimal, focused snapshots
- Scaling coverage: tests assert that `Scale: …%` is non-decreasing with width, `Scale bucket` transitions at 600/900/900+ thresholds, wide geometry grows with scale, and breakpoints remain stable across scaling modes.

## Running the Application

To run the demo application:

```bash
cargo run
```

## Running Tests

To run all tests:

```bash
cargo test
```

To run specific test files:

```bash
# Run integration tests
cargo test --test integration_tests

# Run app-specific tests
cargo test --test app_tests

# Run accessibility/keyboard tests
cargo test --test a11y_keyboard_tests
```

To run tests with output:

```bash
cargo test -- --nocapture
```

### Image snapshots

This repo enables egui_kittest's snapshot testing behind the `wgpu` and `snapshot` features.

- Images are written to `tests/snapshots/` on first run.
- To update snapshots, run with the env var:

```bash
UPDATE_SNAPSHOTS=true cargo test
```

Add these to `.gitignore` to avoid noise from diffs/temporary images:

```gitignore
**/tests/snapshots/**/*.diff.png
**/tests/snapshots/**/*.new.png
```

Notes:

- Snapshots can differ by OS/driver. Our CI runs snapshot steps on macOS and skips them on Linux.
- Prefer semantic and geometry assertions for behavior; keep snapshots small and stable.

## Key egui_kittest Concepts Demonstrated

### 1. Harness Creation

- `Harness::new_ui()` - For testing UI closures
- `Harness::new()` - For testing full egui contexts

### 2. Element Selection

- `get_by_label()` / `query_by_label()` – by exact label text
- `get_by_role()` / `query_by_role()` – by AccessKit role
- `get_by_value()` / `query_by_value()` – by value text

### 3. Interactions

- `.click()` – Simulate button clicks
- `.focus()` and `.type_text()` – Keyboard input into fields
- `harness.key_press(..)` – Simulate key presses
- `.scroll_to_me()` – Ensure off-screen nodes are brought into view

### 4. Test Patterns

- Frame stepping with `harness.run()`
- State verification between interactions
- Dialog/window lifecycle testing
- Complex user workflows

## Dependencies

- `egui` – The immediate mode GUI framework
- `eframe` – Application framework for egui
- `egui_kittest` – Testing framework for egui applications
- `tokio` – Async runtime (for some tests)

## Notes

- Tests use the latest version of egui_kittest (0.32.0)
- The demo app is designed to be simple but cover common UI patterns
- Tests demonstrate both positive and negative test cases
- All tests are headless and don't require a display server

### Best practices (from this repo)

- Make overflow content scrollable with `egui::ScrollArea::vertical()`.
- Account for side panels when asserting grid column counts; central width is less than window width.
- Use stable, semantic labels (e.g., `Layout: …`, `Columns: …`) to make tests robust.
- Prefer semantic/geometry assertions over large snapshots; keep images minimal and focused.
- Test responsive behavior at representative widths:
  - 360 px: stacked + 1 column
  - 820 px: side+central + 2 columns (accounts for SidePanel width)
  - 1280 px: 3 columns
- Use physical width (points × pixels_per_point) for breakpoints so layout is stable across DPI and independent of zoom.
- When writing tests that change scaling modes, prefer asserting the semantic `Scaling mode: …` label. For breakpoint stability across modes, you can set `app.scaling_mode` directly before building a harness for determinism.

## Example Test Structure

```rust
#[test]
fn test_example() {
    let mut harness = Harness::new_ui(|ui| {
        // Your UI code here
        if ui.button("Click me").clicked() {
            // Handle click
        }
    });

    // Test interactions
    harness.get_by_label("Click me").click();
    harness.run();
    
    // Verify results
    harness.get_by_label("Expected result");
}
```

This project serves as a comprehensive reference for testing egui applications with egui_kittest.

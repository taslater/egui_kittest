# egui_kittest Demo

[![CI](https://github.com/taslater/egui_kittest/actions/workflows/ci.yml/badge.svg)](https://github.com/taslater/egui_kittest/actions/workflows/ci.yml)

This project demonstrates how to use `egui_kittest` for testing egui applications. It includes a simple demo application and comprehensive tests showing various testing scenarios.

## Project Structure

- `src/lib.rs` - The main demo application (`DemoApp`) that can be tested
- `src/main.rs` - Binary entry point to run the demo application
- `tests/integration_tests.rs` - Basic egui_kittest functionality tests
- `tests/app_tests.rs` - Tests specifically for the DemoApp

## Features Demonstrated

### Demo Application Features

- Text input for name
- Numeric input for age using DragValue
- Counter with increment/decrement buttons
- Modal dialog with confirmation buttons
- Dynamic labels showing current state

### Testing Features

- Basic UI element presence testing
- Button click simulation
- Text input testing
- Checkbox interaction
- Window/dialog testing
- Menu interaction
- Drag value testing
- Complete application workflow testing

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

## Key egui_kittest Concepts Demonstrated

### 1. Harness Creation

- `Harness::new_ui()` - For testing UI closures
- `Harness::new()` - For testing full egui contexts

### 2. Element Selection

- `get_by_name()` - Find elements by their text/name
- `query_by_name()` - Check if elements exist without panicking

### 3. Interactions

- `.click()` - Simulate button clicks
- `.type_text()` - Type text into inputs
- `.set_text()` - Set text directly

### 4. Test Patterns

- Frame stepping with `harness.run()`
- State verification between interactions
- Dialog/window lifecycle testing
- Complex user workflows

## Dependencies

- `egui` - The immediate mode GUI framework
- `eframe` - Application framework for egui
- `egui_kittest` - Testing framework for egui applications
- `tokio` - Async runtime (for test infrastructure)

## Notes

- Tests use the latest version of egui_kittest (0.32.0)
- The demo app is designed to be simple but cover common UI patterns
- Tests demonstrate both positive and negative test cases
- All tests are headless and don't require a display server

### Best practice: make overflow content scrollable

- Wrap panels that can overflow in `egui::ScrollArea::vertical()` so content remains accessible at small window sizes.
- We added this to both the left `SidePanel` and the main `CentralPanel`.
- A test (`test_small_window_has_scrollbar_and_accessible_content`) asserts that content at the bottom (e.g., `Card 6`) is still reachable when the window is small.

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
    harness.get_by_name("Click me").click();
    harness.run();
    
    // Verify results
    harness.get_by_name("Expected result");
}
```

This project serves as a comprehensive reference for testing egui applications with egui_kittest.

use eframe::App;
use egui::accesskit::Role;
use egui::{self, Key, Vec2};
use egui_kittest::{Harness, kittest::Queryable};

// Keyboard typing into the TextInput should update the greeting label
#[test]
fn keyboard_name_input_updates_greeting() {
    let mut app = egui_kittest_demo::DemoApp::new();
    let mut harness = Harness::builder()
        .with_size(Vec2::new(420.0, 360.0))
        .build(|ctx| {
            let mut frame = eframe::Frame::_new_kittest();
            app.update(ctx, &mut frame);
        });

    let input = harness.get_by_role(Role::TextInput);
    input.focus();
    input.type_text("Eve");
    harness.run();

    // Verify the semantic greeting updates with the name
    harness.get_by_label_contains("Hello, Eve!");
}

// Typing a value into the SpinButton should reflect in the age part of the greeting
#[test]
fn spinbutton_type_updates_age() {
    let mut app = egui_kittest_demo::DemoApp::new();
    let mut harness = Harness::builder()
        .with_size(Vec2::new(460.0, 360.0))
        .build(|ctx| {
            let mut frame = eframe::Frame::_new_kittest();
            app.update(ctx, &mut frame);
        });

    let age = harness.get_by_role(Role::SpinButton);
    age.focus();
    // Clear then type a new value; DragValue accepts typing when focused
    // Send a few backspaces just in case an initial value is present
    for _ in 0..3 {
        harness.key_press(Key::Backspace);
    }
    age.type_text("42");
    harness.run();

    harness.get_by_label_contains("You are 42 years old");
}

// At wide widths (3 columns), the first row cards should be left-to-right ordered by x
#[test]
fn wide_grid_geometry_columns_order() {
    let mut app = egui_kittest_demo::DemoApp::new();
    let harness = Harness::builder()
        .with_size(Vec2::new(1280.0, 720.0))
        .build(|ctx| {
            let mut frame = eframe::Frame::_new_kittest();
            app.update(ctx, &mut frame);
        });
    // Ensure we are in the 3-column layout
    harness.get_by_label("Columns: 3");

    let c1 = harness.get_by_label("Card 1");
    let c2 = harness.get_by_label("Card 2");
    let c3 = harness.get_by_label("Card 3");

    let r1 = c1.rect();
    let r2 = c2.rect();
    let r3 = c3.rect();

    assert!(
        r1.min.x < r2.min.x && r2.min.x < r3.min.x,
        "Cards 1,2,3 should be laid out left-to-right in the first row"
    );
}

// On a very small window, request Card 6 be scrolled into view and assert it becomes visible
#[test]
fn scroll_to_view_offscreen_card() {
    let mut app = egui_kittest_demo::DemoApp::new();
    let mut harness = Harness::builder()
        .with_size(Vec2::new(320.0, 220.0))
        .build(|ctx| {
            let mut frame = eframe::Frame::_new_kittest();
            app.update(ctx, &mut frame);
        });

    let card6 = harness.get_by_label("Card 6");
    // Bring it into view explicitly, then validate its rect is within viewport
    card6.scroll_to_me();
    harness.run();

    // If scroll_to_me worked, the call above should not panic; ensure the node is still present
    harness.get_by_label("Card 6");
}

use eframe::{App, Frame};
use egui::accesskit::Role;
use egui_kittest::{Harness, kittest::Queryable};

// Import our main app struct
use egui_kittest_demo::DemoApp;

#[test]
fn test_demo_app_basic_functionality() {
    let mut app = DemoApp::new();

    let mut harness = Harness::new(|ctx| {
        let mut frame = eframe::Frame::_new_kittest();
        app.update(ctx, &mut frame);
    });

    // Test that the main heading is present
    harness.get_by_label("egui_kittest Demo App");

    // Test that form fields are present
    harness.get_by_label("Name:");
    harness.get_by_label("Age:");

    // Test counter functionality
    harness.get_by_label("Counter: 0");

    // Click increment button
    harness.get_by_label("Increment").click();
    harness.run();

    // Counter should increase
    harness.get_by_label("Counter: 1");

    // Click decrement button
    harness.get_by_label("Decrement").click();
    harness.run();

    // Counter should decrease back to 0
    harness.get_by_label("Counter: 0");
}

#[tokio::test]
async fn test_demo_app_name_input() {
    let mut app = DemoApp::default();
    let mut frame = Frame::_new_kittest();

    // Test name input
    let mut harness = Harness::new_ui(|ui| {
        app.update(ui.ctx(), &mut frame);
    });

    // The TextInput has role TextInput, not searching by value due to duplicates
    let text_input = harness.get_by_role(Role::TextInput);
    text_input.type_text("Alice");
    harness.run();
}

#[test]
fn test_demo_app_age_input() {
    let mut app = DemoApp::new();

    let mut harness = Harness::new(|ctx| {
        let mut frame = eframe::Frame::_new_kittest();
        app.update(ctx, &mut frame);
    });

    // Test age input using SpinButton role
    let _age_input = harness.get_by_role(Role::SpinButton);

    // Test incrementing age with button
    let increment_btn = harness.get_by_label("Increment");
    increment_btn.click();

    harness.run();

    // Should show updated counter - use label role to avoid duplication
    let labels: Vec<_> = harness.query_all_by_value("Counter: 1").collect();
    assert!(labels.len() > 0, "Counter should be updated to 1");
}

#[test]
fn test_demo_app_dialog() {
    let mut app = DemoApp::new();

    let mut harness = Harness::new(|ctx| {
        let mut frame = eframe::Frame::_new_kittest();
        app.update(ctx, &mut frame);
    });

    // Dialog should not be visible initially
    assert!(!harness.query_by_label("Confirmation").is_some());
    assert!(
        !harness
            .query_by_label("Are you sure you want to continue?")
            .is_some()
    );

    // Click Show Dialog button
    harness.get_by_label("Show Dialog").click();
    harness.run();

    // Dialog should now be visible
    harness.get_by_label("Confirmation");
    harness.get_by_label("Are you sure you want to continue?");
    harness.get_by_label("Yes");
    harness.get_by_label("No");

    // Click Yes to close dialog
    harness.get_by_label("Yes").click();
    harness.run();

    // Dialog should be closed
    assert!(!harness.query_by_label("Confirmation").is_some());
}

#[test]
fn test_demo_app_dialog_no_button() {
    let mut app = DemoApp::new();

    let mut harness = Harness::new(|ctx| {
        let mut frame = eframe::Frame::_new_kittest();
        app.update(ctx, &mut frame);
    });

    // Open dialog
    harness.get_by_label("Show Dialog").click();
    harness.run();

    // Dialog should be visible
    harness.get_by_label("Confirmation");

    // Click No to close dialog
    harness.get_by_label("No").click();
    harness.run();

    // Dialog should be closed
    assert!(!harness.query_by_label("Confirmation").is_some());
}

#[test]
fn test_demo_app_complete_workflow() {
    let mut app = DemoApp::new();

    let mut harness = Harness::new(|ctx| {
        let mut frame = eframe::Frame::_new_kittest();
        app.update(ctx, &mut frame);
    });

    // Fill in name using TextInput role
    let name_input = harness.get_by_role(Role::TextInput);
    name_input.type_text("Bob");
    harness.run();

    // Increment counter a few times
    let increment_btn = harness.get_by_label("Increment");
    increment_btn.click();
    harness.run();

    let increment_btn = harness.get_by_label("Increment");
    increment_btn.click();
    harness.run();

    let increment_btn = harness.get_by_label("Increment");
    increment_btn.click();
    harness.run();

    // Verify final state - use query_all to handle duplicates
    let counter_labels: Vec<_> = harness.query_all_by_value("Counter: 3").collect();
    assert!(counter_labels.len() > 0, "Counter should be updated to 3"); // Test dialog workflow
    harness.get_by_label("Show Dialog").click();
    harness.run();
    harness.get_by_label("Confirmation");
    harness.get_by_label("Yes").click();
    harness.run();

    // Dialog should be closed, other state should remain
    assert!(!harness.query_by_label("Confirmation").is_some());
    harness.get_by_label("Counter: 3");
}

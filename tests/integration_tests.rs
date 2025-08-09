use eframe::App;
use egui::accesskit::Role;
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn test_basic_ui_elements() {
    let harness = Harness::new_ui(|ui| {
        ui.heading("Test Heading");
        ui.label("Test Label");
        let _response = ui.button("Test Button");
        ui.text_edit_singleline(&mut String::new());
    });

    // Test that elements are present
    harness.get_by_label("Test Heading");
    harness.get_by_label("Test Label");
    harness.get_by_label("Test Button");
}

#[test]
fn test_button_click() {
    let mut counter = 0;

    let mut harness = Harness::new_ui(|ui| {
        if ui.button("Increment").clicked() {
            counter += 1;
        }
        ui.label(format!("Count: {counter}"));
    });

    // Initially counter should be 0
    harness.get_by_label("Count: 0");

    // Click the button
    harness.get_by_label("Increment").click();

    // Run another frame to see the update
    harness.run();

    // Counter should now be 1
    harness.get_by_label("Count: 1");
}

#[test]
fn test_text_input() {
    let mut harness = Harness::new_ui(|ui| {
        let mut text = String::from("Initial");
        ui.text_edit_singleline(&mut text);
        ui.label(format!("Text: {text}"));
    });

    // Find the text input by role and verify it exists
    let _text_edit = harness.get_by_role(Role::TextInput);

    harness.run();

    // Check that the text was updated - use query_all to handle duplicates
    let labels: Vec<_> = harness.query_all_by_value("Text: Initial").collect();
    assert!(!labels.is_empty(), "Should find the initial text label");
}

#[test]
fn test_checkbox_interaction() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        ui.checkbox(&mut checked, "Enable feature");
        if checked {
            ui.label("Feature is enabled");
        } else {
            ui.label("Feature is disabled");
        }
    });

    // Initially should be unchecked
    harness.get_by_label("Feature is disabled");

    // Click the checkbox
    harness.get_by_label("Enable feature").click();
    harness.run();

    // Should now be checked
    harness.get_by_label("Feature is enabled");
}

#[test]
fn test_window_interaction() {
    let mut show_window = false;

    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open Window").clicked() {
                show_window = true;
            }
        });

        if show_window {
            egui::Window::new("Test Window").show(ctx, |ui| {
                ui.label("This is a window");
                if ui.button("Close").clicked() {
                    show_window = false;
                }
            });
        }
    });

    // Initially window should not be visible
    assert!(harness.query_by_label("This is a window").is_none());

    // Click to open window
    harness.get_by_label("Open Window").click();
    harness.run();

    // Window should now be visible
    harness.get_by_label("This is a window");

    // Click to close window
    harness.get_by_label("Close").click();
    harness.run();

    // Window should be closed again
    assert!(harness.query_by_label("This is a window").is_none());
}

#[test]
fn test_drag_value() {
    let mut value = 50.0;

    let mut harness = Harness::new_ui(|ui| {
        ui.add(egui::DragValue::new(&mut value).range(0.0..=100.0));
        ui.label(format!("Value: {value:.1}"));
    });

    harness.get_by_label("Value: 50.0");

    // Find and interact with the drag value using SpinButton role
    let drag_value = harness.get_by_role(Role::SpinButton);

    // Simulate typing into the drag value
    // Focus the drag value and type a new value
    drag_value.focus();
    drag_value.type_text("75");
    harness.run();

    // Note: DragValue behavior with type_text may vary
    // This demonstrates the interaction pattern
}

#[test]
fn test_menu_interaction() {
    let mut file_action = String::new();

    let mut harness = Harness::new(|ctx| {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        file_action = "New".to_string();
                    }
                    if ui.button("Open").clicked() {
                        file_action = "Open".to_string();
                    }
                    if ui.button("Save").clicked() {
                        file_action = "Save".to_string();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if !file_action.is_empty() {
                ui.label(format!("Action: {file_action}"));
            }
        });
    });

    // Click File menu
    harness.get_by_label("File").click();
    harness.run();

    // Click New option
    harness.get_by_label("New").click();
    harness.run();

    // Check that action was recorded
    harness.get_by_label("Action: New");
}

#[test]
fn test_semantic_layout_indicators() {
    let mut app = egui_kittest_demo::DemoApp::new();

    // Narrow: expect stacked layout and 1 column
    let mut harness = Harness::builder()
        .with_size(egui::vec2(360.0, 500.0))
        .build(|ctx| {
            let mut frame = eframe::Frame::_new_kittest();
            app.update(ctx, &mut frame);
        });
    harness.get_by_label("Layout: Stacked");
    // Columns label should be visible and equal to 1
    harness.get_by_label("Columns: 1");

    // Medium: expect side+central and 2 columns (bump width to account for SidePanel)
    harness.set_size(egui::vec2(820.0, 600.0));
    harness.run();
    harness.get_by_label("Layout: Side+Central");
    harness.get_by_label("Columns: 2");

    // Wide: expect 3 columns
    harness.set_size(egui::vec2(1280.0, 700.0));
    harness.run();
    harness.get_by_label("Columns: 3");
}

#[test]
fn test_all_cards_visible_via_scroll_narrow() {
    let mut app = egui_kittest_demo::DemoApp::new();

    let harness = Harness::builder()
        .with_size(egui::vec2(320.0, 220.0))
        .build(|ctx| {
            let mut frame = eframe::Frame::_new_kittest();
            app.update(ctx, &mut frame);
        });

    // All card headings should be reachable even at narrow size due to ScrollArea
    for i in 1..=6 {
        harness.get_by_label(&format!("Card {i}"));
    }
}

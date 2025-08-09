use eframe::App; // bring trait for DemoApp::update into scope
use egui_kittest::{Harness, kittest::Queryable};

// Verify a simple UI renders and snapshot is saved at a small size
#[test]
fn snapshot_small_layout() {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(320.0, 240.0))
        .build_ui(|ui| {
            ui.heading("Snapshot Small");
            ui.label("A compact layout for small windows");
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                let _ = ui.button("One").clicked();
                let _ = ui.button("Two").clicked();
                let _ = ui.button("Three").clicked();
            });
        });

    harness.get_by_label("Snapshot Small");
    harness.snapshot("snapshot_small_layout");
}

// Verify our DemoApp adapts across sizes and create snapshots for each size
#[test]
fn demo_app_responsive_snapshots() {
    let mut app = egui_kittest_demo::DemoApp::default();
    let mut harness = Harness::builder()
        .with_size(egui::vec2(360.0, 280.0))
        .build(|ctx| {
            let mut frame = eframe::Frame::_new_kittest();
            app.update(ctx, &mut frame);
        });

    // Narrow view
    harness.get_by_label("egui_kittest Demo App");
    harness.snapshot("demo_narrow");

    // Medium view
    harness.set_size(egui::vec2(600.0, 500.0));
    harness.run();
    harness.snapshot("demo_medium");

    // Wide view
    harness.set_size(egui::vec2(960.0, 700.0));
    harness.run();
    harness.snapshot("demo_wide");
}

// Verify fit_contents works on simple UI and then snapshot
#[test]
fn fit_contents_and_snapshot() {
    let mut harness = Harness::new_ui(|ui| {
        ui.heading("Auto Size");
        ui.label("The window will fit these contents");
        for i in 0..3 {
            ui.label(format!("Row {i}"));
        }
    });
    harness.fit_contents();
    harness.snapshot("fit_contents_auto");
}

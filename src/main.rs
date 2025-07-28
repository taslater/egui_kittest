use eframe::egui;
use egui_kittest_demo::DemoApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "egui_kittest Demo",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
}

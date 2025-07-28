use eframe::egui;

#[derive(Default)]
pub struct DemoApp {
    pub name: String,
    pub age: u32,
    pub counter: i32,
    pub show_confirmation_dialog: bool,
}

impl DemoApp {
    pub fn new() -> Self {
        Self::default()
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui_kittest Demo App");

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.horizontal(|ui| {
                ui.label("Age:");
                ui.add(egui::DragValue::new(&mut self.age).range(0..=120));
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Increment").clicked() {
                    self.counter += 1;
                }
                if ui.button("Decrement").clicked() {
                    self.counter -= 1;
                }
                ui.label(format!("Counter: {}", self.counter));
            });

            ui.separator();

            if ui.button("Show Dialog").clicked() {
                self.show_confirmation_dialog = true;
            }

            if self.show_confirmation_dialog {
                egui::Window::new("Confirmation")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Are you sure you want to continue?");
                        ui.horizontal(|ui| {
                            if ui.button("Yes").clicked() {
                                self.show_confirmation_dialog = false;
                            }
                            if ui.button("No").clicked() {
                                self.show_confirmation_dialog = false;
                            }
                        });
                    });
            }

            ui.separator();

            ui.label(format!(
                "Hello, {}! You are {} years old.",
                self.name, self.age
            ));
        });
    }
}

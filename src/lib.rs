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
        // Top menu bar for navigation and accessibility
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        // no-op in demo
                    }
                    if ui.button("Open").clicked() {
                        // no-op in demo
                    }
                    if ui.button("Save").clicked() {
                        // no-op in demo
                    }
                });
                ui.menu_button("View", |ui| {
                    // In a larger app this could toggle panels, themes, etc.
                    ui.label("Layout is responsive to window width");
                });
                ui.menu_button("Help", |ui| {
                    ui.label("Demo showing responsive layouts");
                });
            });
        });

        // Left side panel with filters/settings – collapses nicely on narrow widths
        let mut dummy_filter_a = false;
        let mut dummy_filter_b = false;
        egui::SidePanel::left("left_filters")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Filters");
                ui.separator();
                ui.checkbox(&mut dummy_filter_a, "Placeholder filter A");
                ui.checkbox(&mut dummy_filter_b, "Placeholder filter B");
                ui.separator();
                ui.label("Use the central area to interact with the app");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui_kittest Demo App");

            // Form area – stays readable by stacking on small widths
            let available_width = ui.available_width();
            let is_narrow = available_width < 500.0;

            if is_narrow {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Age:");
                        ui.add(egui::DragValue::new(&mut self.age).range(0..=120));
                    });
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.name);
                    ui.separator();
                    ui.label("Age:");
                    ui.add(egui::DragValue::new(&mut self.age).range(0..=120));
                });
            }

            ui.separator();

            // Counter controls – buttons first for easy keyboard/screen reader focus
            ui.horizontal_wrapped(|ui| {
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

            ui.separator();

            // Responsive card grid – adapts number of columns to width
            let width = ui.available_width();
            let cols = if width >= 900.0 {
                3
            } else if width >= 600.0 {
                2
            } else {
                1
            };
            let mut columns = vec![Vec::<usize>::new(); cols];
            let cards = 6usize; // a few demo cards
            for i in 0..cards {
                columns[i % cols].push(i);
            }
            ui.columns(cols, |uis| {
                for (col_idx, col_ui) in uis.iter_mut().enumerate() {
                    for card_idx in &columns[col_idx] {
                        egui::Frame::group(col_ui.style())
                            .stroke(col_ui.visuals().widgets.noninteractive.bg_stroke)
                            .show(col_ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.heading(format!("Card {}", card_idx + 1));
                                    ui.label(
                                        "This card wraps text and scales with the layout width.",
                                    );
                                    ui.horizontal_wrapped(|ui| {
                                        let _ = ui.small_button("Action").clicked();
                                        let _ = ui.small_button("More").clicked();
                                        let _ = ui.small_button("Details").clicked();
                                    });
                                });
                            });
                        col_ui.add_space(4.0);
                    }
                }
            });
        });
    }
}

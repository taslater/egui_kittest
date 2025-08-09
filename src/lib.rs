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
                    let _ = ui.button("New");
                    let _ = ui.button("Open");
                    let _ = ui.button("Save");
                });
                ui.menu_button("View", |ui| {
                    ui.label("Layout is responsive to window width");
                });
                ui.menu_button("Help", |ui| {
                    ui.label("Demo showing responsive layouts");
                });
            });
        });

        // Determine if we should stack content for very narrow windows
        let total_width = ctx.available_rect().width();
        let stack_breakpoint = 600.0_f32; // below this, stack filters above content
        let is_stacked = total_width < stack_breakpoint;

        // Shared closures to render filters and main content to avoid duplication
        let render_filters = |ui: &mut egui::Ui| {
            ui.heading("Filters");
            ui.separator();
            ui.checkbox(&mut false, "Placeholder filter A");
            ui.checkbox(&mut false, "Placeholder filter B");
            ui.separator();
            ui.label("Use the central area to interact with the app");
        };

        let render_main_content = |ui: &mut egui::Ui, this: &mut DemoApp, ctx: &egui::Context| {
            ui.heading("egui_kittest Demo App");

            // Form area – stacks on small widths
            let available_width = ui.available_width();
            let is_narrow = available_width < 500.0;
            if is_narrow {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut this.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Age:");
                        ui.add(egui::DragValue::new(&mut this.age).range(0..=120));
                    });
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut this.name);
                    ui.separator();
                    ui.label("Age:");
                    ui.add(egui::DragValue::new(&mut this.age).range(0..=120));
                });
            }

            ui.separator();

            // Counter controls – buttons first for easy keyboard/screen reader focus
            ui.horizontal_wrapped(|ui| {
                if ui.button("Increment").clicked() {
                    this.counter += 1;
                }
                if ui.button("Decrement").clicked() {
                    this.counter -= 1;
                }
                ui.label(format!("Counter: {}", this.counter));
            });

            ui.separator();
            ui.label(format!(
                "Hello, {}! You are {} years old.",
                this.name, this.age
            ));

            ui.separator();

            // Dialog demo
            if ui.button("Show Dialog").clicked() {
                this.show_confirmation_dialog = true;
            }
            if this.show_confirmation_dialog {
                egui::Window::new("Confirmation")
                    .collapsible(false)
                    .show(ctx, |ui| {
                        ui.label("Are you sure you want to continue?");
                        ui.horizontal(|ui| {
                            if ui.button("Yes").clicked() {
                                this.show_confirmation_dialog = false;
                            }
                            if ui.button("No").clicked() {
                                this.show_confirmation_dialog = false;
                            }
                        });
                    });
            }

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
        };

        if is_stacked {
            // Narrow: stack Filters above Main inside a scrollable CentralPanel
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        egui::Frame::group(ui.style()).show(ui, |ui| render_filters(ui));
                        ui.add_space(6.0);
                        egui::Frame::group(ui.style())
                            .show(ui, |ui| render_main_content(ui, self, ctx));
                    });
            });
        } else {
            // Wide: show Filters in a resizable side panel and Main in the central area
            egui::SidePanel::left("left_filters")
                .resizable(true)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| render_filters(ui));
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| render_main_content(ui, self, ctx));
            });
        }
    }
}

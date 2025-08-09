use eframe::egui;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScalingMode {
    Zoom,
    Style,
}

impl Default for ScalingMode {
    fn default() -> Self {
        ScalingMode::Zoom
    }
}

#[derive(Default)]
pub struct DemoApp {
    pub name: String,
    pub age: u32,
    pub counter: i32,
    pub show_confirmation_dialog: bool,
    pub zoom_factor: f32,
    pub scaling_mode: ScalingMode,
    pub base_style: Option<egui::Style>,
}

impl DemoApp {
    pub fn new() -> Self {
    Self { zoom_factor: 1.0, scaling_mode: ScalingMode::default(), base_style: None, ..Default::default() }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Capture baseline style once for style-based scaling
        if self.base_style.is_none() {
            self.base_style = Some((*ctx.style()).clone());
        }

    // Measure unscaled window width (in points) and base pixels-per-point once per frame
    // Use physical width (points * ppp) for stable, DPI-independent breakpoints
    let unscaled_points = ctx.available_rect().width();
    let base_ppp = ctx.pixels_per_point();
    let window_px = unscaled_points * base_ppp;
    // Adaptive scaling based directly on current window width
    let win_width = window_px;
    match self.scaling_mode {
            ScalingMode::Zoom => {
        // Discrete, stronger zoom mapping to avoid oscillations
        let desired = if win_width < 600.0 { 0.85 } else if win_width < 900.0 { 1.0 } else if win_width < 1280.0 { 1.25 } else { 1.50 };
                let eps = 0.01;
                if (self.zoom_factor - desired).abs() > eps {
                    self.zoom_factor = desired;
                    ctx.set_zoom_factor(self.zoom_factor);
                }
            }
            ScalingMode::Style => {
        // Style-driven discrete scaling (typography + spacing), leave zoom at 1.0
        let style_scale = if win_width < 600.0 { 0.95 } else if win_width < 900.0 { 1.15 } else if win_width < 1280.0 { 1.35 } else { 1.60 };
                // Apply only if meaningfully changed
                let eps = 0.01;
                // Track zoom_factor as the effective visual scale for labeling
                if (self.zoom_factor - style_scale).abs() > eps {
                    self.zoom_factor = style_scale;
                    // Build from baseline style so scaling is idempotent
                    let mut style = self.base_style.clone().unwrap_or_else(|| (*ctx.style()).clone());
                    let base = style.clone();
                    // Scale common spacings from base
                    style.spacing.item_spacing = base.spacing.item_spacing * style_scale;
                    style.spacing.button_padding = base.spacing.button_padding * style_scale;
                    style.spacing.indent = base.spacing.indent * style_scale;
                    style.spacing.interact_size = base.spacing.interact_size * style_scale;
                    // Scale text styles from base
                    for (ts, font_id) in style.text_styles.iter_mut() {
                        if let Some(base_font) = base.text_styles.get(ts) {
                            font_id.size = base_font.size * style_scale;
                        } else {
                            font_id.size *= style_scale;
                        }
                    }
                    ctx.set_style(style);
                    // Ensure base zoom (pixels_per_point multiplier) is neutral in this mode
                    ctx.set_zoom_factor(1.0);
                }
            }
        }

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
                    ui.separator();
                    ui.label("Scaling strategy");
                    let mut mode = self.scaling_mode;
                    if ui.radio(mode == ScalingMode::Zoom, "Zoom-based").clicked() {
                        mode = ScalingMode::Zoom;
                    }
                    if ui.radio(mode == ScalingMode::Style, "Style-based").clicked() {
                        mode = ScalingMode::Style;
                    }
                    if mode != self.scaling_mode {
                        self.scaling_mode = mode;
                        // Reset to force re-application next frame
                        self.zoom_factor = 0.0;
                    }
                    ui.label(format!("Scaling mode: {}",
                        match self.scaling_mode { ScalingMode::Zoom => "Zoom", ScalingMode::Style => "Style" }
                    ));
                });
                ui.menu_button("Help", |ui| {
                    ui.label("Demo showing responsive layouts");
                });
            });
        });

    // Determine if we should stack content for very narrow windows
    // Base this on the physical window width so breakpoints are stable across DPI & zoom
    let stack_breakpoint = 600.0_f32; // below this, stack filters above content
    let logical_width = window_px;
    let is_stacked = logical_width < stack_breakpoint;

        // Shared closures to render filters and main content to avoid duplication
        let render_filters = |ui: &mut egui::Ui| {
            ui.heading("Filters");
            ui.separator();
            ui.checkbox(&mut false, "Placeholder filter A");
            ui.checkbox(&mut false, "Placeholder filter B");
            ui.separator();
            ui.label("Use the central area to interact with the app");
        };

        let render_main_content =
            |ui: &mut egui::Ui, this: &mut DemoApp, ctx: &egui::Context, stacked: bool| {
                ui.heading("egui_kittest Demo App");
                ui.label(format!(
                    "Layout: {}",
                    if stacked { "Stacked" } else { "Side+Central" }
                ));
                // Expose semantic scale indicators for tests and a11y
                let scale_pct = (this.zoom_factor * 100.0).round() as i32;
                ui.label(format!("Scale: {scale_pct}%"));
                let bucket = if logical_width >= 900.0 {
                    "Large"
                } else if logical_width >= 600.0 {
                    "Medium"
                } else {
                    "Small"
                };
                ui.label(format!("Scale bucket: {bucket}"));

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

                // Responsive card grid – adapts number of columns to central width (stable vs zoom)
                let width = logical_width;
                let cols = if width >= 900.0 {
                    3
                } else if width >= 600.0 {
                    2
                } else {
                    1
                };
                ui.label(format!("Columns: {cols}"));
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
                        egui::Frame::group(ui.style()).show(ui, render_filters);
                        ui.add_space(6.0);
                        egui::Frame::group(ui.style())
                            .show(ui, |ui| render_main_content(ui, self, ctx, true));
                    });
            });
        } else {
            // Wide: show Filters in a resizable side panel and Main in the central area
            egui::SidePanel::left("left_filters")
                .resizable(true)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, render_filters);
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| render_main_content(ui, self, ctx, false));
            });
        }
    }
}

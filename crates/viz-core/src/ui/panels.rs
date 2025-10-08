use super::PerformanceMetrics;

/// Draw performance metrics panel
pub fn performance_panel(ctx: &egui::Context, metrics: &PerformanceMetrics) {
    egui::Window::new("📊 Performance")
        .default_pos([10.0, 10.0])
        .default_width(250.0)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Frame Stats");

            ui.separator();

            // FPS display
            ui.horizontal(|ui| {
                ui.label("FPS:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.monospace(format!("{:.1}", metrics.average_fps()));
                });
            });

            ui.horizontal(|ui| {
                ui.label("Frame Time:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.monospace(format!("{:.2} ms", metrics.average_frame_time()));
                });
            });

            ui.separator();

            // Min/Max
            ui.horizontal(|ui| {
                ui.label("Min:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.monospace(format!("{:.2} ms", metrics.min_frame_time()));
                });
            });

            ui.horizontal(|ui| {
                ui.label("Max:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.monospace(format!("{:.2} ms", metrics.max_frame_time()));
                });
            });

            ui.separator();

            // Total stats
            ui.horizontal(|ui| {
                ui.label("Total Frames:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.monospace(format!("{}", metrics.total_frames()));
                });
            });

            ui.horizontal(|ui| {
                ui.label("Elapsed:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let secs = metrics.elapsed_time().as_secs();
                    ui.monospace(format!("{}:{:02}", secs / 60, secs % 60));
                });
            });

            // Frame time graph
            if metrics.frame_times().len() > 1 {
                ui.separator();
                ui.label("Frame Times:");

                let points: Vec<f64> = metrics
                    .frame_times()
                    .iter()
                    .map(|&t| t as f64)
                    .collect();

                use egui_plot::{Line, Plot, PlotPoints};
                let line = Line::new(PlotPoints::from_ys_f64(&points));

                Plot::new("frame_times_plot")
                    .height(80.0)
                    .show_axes([false, true])
                    .show_grid([false, true])
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            }
        });
}

/// Control panel for visualization settings
pub struct ControlPanel {
    pub point_size: f32,
    pub show_grid: bool,
    pub dataset_index: usize,
    pub background_color: [f32; 3],
    pub colormap_index: usize,
    pub metadata_field: String,
    pub use_log_scale: bool,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self {
            point_size: 5.0,
            show_grid: false,
            dataset_index: 0,
            background_color: [0.05, 0.05, 0.08],
            colormap_index: 0,
            metadata_field: String::new(),
            use_log_scale: false,
        }
    }
}

impl ControlPanel {
    /// Draw the control panel UI
    pub fn show(&mut self, ctx: &egui::Context, dataset_names: &[&str]) -> bool {
        let mut changed = false;

        egui::Window::new("🎛️ Controls")
            .default_pos([10.0, 350.0])
            .default_width(250.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Visualization");

                ui.separator();

                // Point size slider
                ui.label("Point Size:");
                if ui
                    .add(egui::Slider::new(&mut self.point_size, 1.0..=20.0).suffix(" px"))
                    .changed()
                {
                    changed = true;
                }

                ui.separator();

                // Dataset selector
                if !dataset_names.is_empty() {
                    ui.label("Dataset:");
                    let old_index = self.dataset_index;
                    egui::ComboBox::from_label("")
                        .selected_text(dataset_names[self.dataset_index.min(dataset_names.len() - 1)])
                        .show_ui(ui, |ui| {
                            for (i, &name) in dataset_names.iter().enumerate() {
                                ui.selectable_value(&mut self.dataset_index, i, name);
                            }
                        });

                    if self.dataset_index != old_index {
                        changed = true;
                    }
                }

                ui.separator();

                // Colormap selector
                ui.label("Colormap:");
                let colormap_names = ["Viridis", "Plasma", "Inferno", "Turbo"];
                let old_colormap = self.colormap_index;
                egui::ComboBox::from_label("colormap_select")
                    .selected_text(colormap_names[self.colormap_index])
                    .show_ui(ui, |ui| {
                        for (i, &name) in colormap_names.iter().enumerate() {
                            ui.selectable_value(&mut self.colormap_index, i, name);
                        }
                    });

                if self.colormap_index != old_colormap {
                    changed = true;
                }

                // Colormap preview
                self.draw_colormap_preview(ui, self.colormap_index);

                ui.separator();

                // Scale type toggle
                if ui.checkbox(&mut self.use_log_scale, "Log Scale").changed() {
                    changed = true;
                }

                ui.separator();

                // Grid toggle
                if ui.checkbox(&mut self.show_grid, "Show Grid").changed() {
                    changed = true;
                }

                ui.separator();

                // Background color
                ui.label("Background:");
                let mut bg_color = egui::Color32::from_rgb(
                    (self.background_color[0] * 255.0) as u8,
                    (self.background_color[1] * 255.0) as u8,
                    (self.background_color[2] * 255.0) as u8,
                );

                if ui.color_edit_button_srgba(&mut bg_color).changed() {
                    self.background_color = [
                        bg_color.r() as f32 / 255.0,
                        bg_color.g() as f32 / 255.0,
                        bg_color.b() as f32 / 255.0,
                    ];
                    changed = true;
                }
            });

        changed
    }

    /// Draw a colormap preview strip
    fn draw_colormap_preview(&self, ui: &mut egui::Ui, colormap_index: usize) {
        use crate::color::{Colormap, Viridis, Plasma, Inferno, Turbo};

        let colormap: &dyn Colormap = match colormap_index {
            0 => &Viridis,
            1 => &Plasma,
            2 => &Inferno,
            3 => &Turbo,
            _ => &Viridis,
        };

        let height = 20.0;
        let width = ui.available_width();
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(width, height),
            egui::Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let steps = 64;

            for i in 0..steps {
                let t = i as f32 / (steps - 1) as f32;
                let color = colormap.sample(t);

                let x0 = rect.min.x + (width * i as f32 / steps as f32);
                let x1 = rect.min.x + (width * (i + 1) as f32 / steps as f32);

                let segment_rect = egui::Rect::from_min_max(
                    egui::pos2(x0, rect.min.y),
                    egui::pos2(x1, rect.max.y),
                );

                painter.rect_filled(
                    segment_rect,
                    0.0,
                    egui::Color32::from_rgb(
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                    ),
                );
            }

            // Draw border
            painter.rect_stroke(rect, 0.0, (1.0, egui::Color32::from_gray(100)));
        }
    }

    /// Get background color as wgpu Color
    pub fn background_wgpu_color(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.background_color[0] as f64,
            g: self.background_color[1] as f64,
            b: self.background_color[2] as f64,
            a: 1.0,
        }
    }
}

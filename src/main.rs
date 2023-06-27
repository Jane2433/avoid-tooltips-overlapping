mod marker;

use std::f32::consts::PI;

use eframe::egui;
use marker::{Marker, Tooltip};

const WINDOWS_WIDTH: f32 = 800.0;
const WINDOWS_HEIGHT: f32 = 600.0;
const TOOLTIP_WIDTH: f32 = 60.0;
const TOOLTIP_HEIGHT: f32 = 40.0;
const MARKER_RADIUS: f32 = 6.0;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WINDOWS_WIDTH, WINDOWS_HEIGHT)),
        ..Default::default()
    };
    eframe::run_native("Power", options, Box::new(|_cc| Box::<MyApp>::default()))
}

/// My Application State
struct MyApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    add_point: bool,
    markers: Vec<Marker>,
    colors: Vec<egui::Color32>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            allowed_to_close: false,
            show_confirmation_dialog: false,
            add_point: false,
            markers: Vec::new(),
            colors: vec![
                egui::Color32::from_rgb(0, 254, 223),
                egui::Color32::from_rgb(255, 244, 92),
                egui::Color32::from_rgb(254, 144, 44),
                egui::Color32::from_rgb(254, 4, 4),
                egui::Color32::from_rgb(254, 58, 163),
                egui::Color32::from_rgb(174, 0, 217),
            ],
        }
    }
}

impl eframe::App for MyApp {
    // click stop window
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            set_tooltip_position(&mut self.markers);

            for marker in &self.markers {
                ui.painter()
                    .circle_filled(marker.pos, marker.radius, marker.color);
                if let Some(tooltip) = &marker.tooltip {
                    ui.painter().rect_filled(
                        tooltip.rect,
                        egui::Rounding::same(1.0),
                        egui::Color32::LIGHT_BLUE,
                    );
                    ui.painter().line_segment(
                        [marker.pos, tooltip.rect.min],
                        egui::Stroke::new(1.0, egui::Color32::GRAY),
                    )
                }
            }

            let being_btn = ui.button("stop take points");
            let end_btn = ui.button("begin take points");
            if being_btn.clicked() {
                self.add_point = false;
            }
            if self.add_point && ctx.input(|i| i.pointer.any_click()) {
                if let Some(pos) = ctx.pointer_latest_pos() {
                    self.markers.push(Marker {
                        pos: pos,
                        radius: MARKER_RADIUS,
                        color: *self.colors.get(pos.x as usize % self.colors.len()).unwrap(),
                        tooltip: None,
                    });
                }
            }
            if end_btn.clicked() {
                self.add_point = true;
            }
        });

        // handle close window
        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}

fn set_tooltip_position(markers: &mut Vec<Marker>) {
    for (i, marker) in markers.iter_mut().enumerate() {
        let angle = ((2.0 * PI) / 36.0) * (i as f32);

        let min_pos = egui::pos2(
            marker.pos.x + 50.0 * angle.sin(),
            marker.pos.y + 50.0 * angle.cos(),
        );
        let max_pos = egui::pos2(min_pos.x + TOOLTIP_WIDTH, min_pos.y + TOOLTIP_HEIGHT);

        let tooltip = Some(Tooltip {
            rect: egui::Rect {
                min: min_pos,
                max: max_pos,
            },
            content: String::from("zzl"),
        });

        marker.tooltip = tooltip;
    }
}

// fn layout_by_force(my_app: &mut MyApp) {
//     let start = WINDOWS_WIDTH / 10.0;
//     let times = 50;

//     let mut t;
//     for i in 0..times {
//         t = start * (1 - i / (times - 1));
//         compute_position_step(t);
//     }

//     for dest_point in &my_app.dest_pointes {
//         dest_point.x = dest_point.x - TOOLTIP_WIDTH / 2.0;
//         dest_point.y = dest_point.y - TOOLTIP_HEIGHT / 2.0;
//     }

// }

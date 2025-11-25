use bevy::prelude::*;
use bevy_egui::egui;

pub fn show_debug_window(
    mut contexts:bevy_egui::EguiContexts,
) {
    egui::Window::new("Debug Menu")
        .default_width(200.0)
        .default_height(100.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Controls");

            if ui.add(egui::Button::new("Reset bot simulation")).clicked() {
                println!("Reseting bot simulation");
            }

            if ui.add(egui::Button::new("Reset bot position")).clicked() {
                println!("Reseting the bot position");
            }

            if ui.add(egui::Button::new("Reset bot position & simulation")).clicked() {
                println!("Reseting both simulation and position of the bot");
            }
        });
}
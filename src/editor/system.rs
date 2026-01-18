use bevy::prelude::*;
use bevy_egui::egui;
use machine::prelude::{VirtualMachine, VirtualMachineMetaData};

use crate::player::components::IsSelected;

// TODO: Re-enable when editor is fixed
// use crate::editor::highlight::highlight_asmfg_syntax;
// use super::highlight::highlight_afg_syntax;

// Example usage in your Bevy system
pub fn afg_code_editor_system(
    current_bot: Query<(&IsSelected, &VirtualMachineMetaData, &VirtualMachine)>,
    mut contexts: bevy_egui::EguiContexts,
    mut code: ResMut<AfgSourceCode>,
) {
    egui::Window::new("AFG Code Editor")
        .default_width(800.0)
        .default_height(600.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Bot Editor");

            let _ = current_bot.single().map_err(|e| {
                error!("Can't get single bot: {e}");
            });

            // TODO: Fix text editor integration with VirtualMachineMetaData
            if current_bot.single().is_ok() {
                ui.label("Bot selected - editor WIP");
            } else {
                ui.label("No bot selected");
            }

            ui.horizontal(|ui| {
                if ui.button("Compile").clicked() {
                    // Trigger AFG compilation
                    println!("Compiling AFG code...");
                }

                if ui.button("Run Bot").clicked() {
                    // Execute the compiled bot
                    println!("Running bot...");
                }

                if ui.button("Clear").clicked() {
                    code.source.clear();
                }
            });
        });
}

// Resource to hold the AFG source code
#[derive(bevy::prelude::Resource)]
pub struct AfgSourceCode {
    pub source: String,
    pub compiled: Option<String>,
}

impl Default for AfgSourceCode {
    fn default() -> Self {
        Self {
            source: String::from(
                r#"fn main() {
    // Simple bot that moves forward and avoids obstacles
    set $Velocity[0] = 200;

    loop {
        if $RayType[0] != 0 {
            if $RayDist[0] < 150 {
                call avoid_obstacle();
            }
        }
    }
}

fn avoid_obstacle() {
    set $Velocity[0] = 50;   // Slow down
    set $Moment = -20;       // Turn right

    while $RayType[0] != 0 {
        // Keep turning while obstacle detected
    }

    set $Moment = 0;         // Stop turning
    set $Velocity[0] = 200;  // Resume speed
}"#,
            ),
            compiled: None,
        }
    }
}

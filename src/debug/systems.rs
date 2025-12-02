use bevy::prelude::*;
use bevy_egui::egui;

use machine::prelude::VirtualMachine;

use crate::player::components::IsSelected;
use super::events::*;

pub fn show_debug_window(
    mut contexts: bevy_egui::EguiContexts,
    mut debug_bot_events: EventWriter<DebugBotUpdate>
) {
    egui::Window::new("Debug Menu")
        .default_width(200.0)
        .default_height(100.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Controls");

            if ui.add(egui::Button::new("Reset bot simulation")).clicked() {
                println!("Reseting bot simulation");
                debug_bot_events.write(DebugBotUpdate(BotUpdateType::ResetSimulation));
            }

            if ui.add(egui::Button::new("Reset bot position")).clicked() {
                println!("Reseting the bot position");
                debug_bot_events.write(DebugBotUpdate(BotUpdateType::ResetPosition));
            }

            if ui.add(egui::Button::new("Reset bot position & simulation")).clicked() {
                println!("Reseting both simulation and position of the bot");
                debug_bot_events.write(DebugBotUpdate(BotUpdateType::ResetPositionAndSimulation));
            }
        });
}

pub fn bot_react_to_reset_event(
    mut bot: Query<(&mut VirtualMachine, &mut Transform), With<IsSelected>>,
    mut debug_bot_events: EventReader<DebugBotUpdate>
) {
    if let Ok((mut vm, mut _transform)) = bot.single_mut() {
        for event in debug_bot_events.read() {
            match event.0 {
                BotUpdateType::ResetSimulation => {
                    vm.reset()
                }
                BotUpdateType::ResetPosition => {
                    // Do something with the transform here
                }
                BotUpdateType::ResetPositionAndSimulation => {
                    vm.reset()
                    // Do something with the transform here
                }
            }
        }
    }
}
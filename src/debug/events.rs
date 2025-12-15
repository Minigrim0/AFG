use bevy::prelude::*;

pub enum BotUpdateType {
    ResetPosition,
    ResetSimulation,
    ResetPositionAndSimulation,
}

#[derive(Event)]
pub struct DebugBotUpdate(pub BotUpdateType);

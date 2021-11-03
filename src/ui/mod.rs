pub mod ui_element;
pub mod ui_manager;

use super::{WIN_WIDTH, WIN_HEIGHT, TIME_STEP, utils::vector2::Vector2F};

pub enum SimulationCommands {
    Pause,
    Unpaused,
    SpeedChange,
}

pub fn relative_width(pos: f32) -> f32 {
    WIN_WIDTH * pos
}

pub fn relative_height(pos: f32) -> f32 {
    WIN_HEIGHT * pos
}
pub mod ui_element;
pub mod ui_manager;

use super::{utils::vector2::Vector2F, TIME_STEP, WIN_HEIGHT, WIN_WIDTH};

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

pub mod ui_element;

use ggez::graphics::Text;
use ggez::Context;

use self::ui_element::UIElement;

use super::utils::vector2;
use crate::{TIME_STEP, WIN_HEIGHT, WIN_WIDTH};

pub fn relative_width(pos: f32) -> f32 {
    WIN_WIDTH * pos
}

pub fn relative_height(pos: f32) -> f32 {
    WIN_HEIGHT * pos
}

pub enum SimulationCommands {
    Pause,
    Unpaused,
    SpeedChange,
}

pub struct UIManager {
    ui_elements: Vec<UIElement>
}

impl UIManager {
    pub fn new(ui_elements: Vec<UIElement>) -> Self {
        Self {
            ui_elements,
        }
    }

    pub fn draw_elements(&self, ctx: &mut Context) {
        for element in self.ui_elements.iter() {
            element.draw(ctx);
        }
    }

    pub fn find_by_id(&self, id: &str) -> Result<&UIElement, ()> {
        for element in self.ui_elements.iter() {
            if *element.id() == id.to_string() {
                return Ok(element)
            }
        }
        Err(())
    }
    
    pub fn find_by_id_mut(&mut self, id: &str) -> Result<&mut UIElement, ()> {
        for element in self.ui_elements.iter_mut() {
            if *element.id() == id.to_string() {
                return Ok(element)
            }
        }
        Err(println!("Invalid ID: {0}", id))
    }

    pub fn update_simulation_info(&mut self, current_time_step: f32, command: SimulationCommands) {
        match command {
            SimulationCommands::Pause => {
                self.find_by_id_mut("Pause")
                    .unwrap()
                    .update_text_static("Paused");
            },
            SimulationCommands::Unpaused => {
                self.find_by_id_mut("Pause")
                    .unwrap()
                    .update_text_static("Unpaused");
            },
            SimulationCommands::SpeedChange => {
                self.find_by_id_mut("Pause")
                    .unwrap()
                    .update_text_static(if format!("{:.1}", current_time_step) == "0.0" {"Paused"} else {"Unpaused"});

                self.find_by_id_mut("Speed")
                    .unwrap()
                    .update_text_string(format!("{}{:.1}{}", "Speed: ", (current_time_step / TIME_STEP), "x"));
            },
        }
    }
}
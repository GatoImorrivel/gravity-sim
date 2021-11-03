use ggez::Context;
use ggez::graphics::{self, DrawParam, PxScale, TextFragment, Text};

use super::Vector2F;

const ANTI_ALIASING: f32 = 4.0;

pub struct UIElement {
    id: String,
    text: Text,
    font_size: f32,
    position: Vector2F,
}

impl UIElement {
    pub fn new(id: &str, str: &str, font_size: f32, position: Vector2F) -> Self {
        Self {
            id: id.to_string(),
            font_size,
            text: Text::new(
                TextFragment::new(str.to_string()).scale(PxScale { x: (font_size * ANTI_ALIASING), y: (font_size * ANTI_ALIASING) })
            ),
            position
        }
    }

    pub fn update_text_static(&mut self, str: &str) {
        self.text = Text::new(
            TextFragment::new(str.to_string()).scale(PxScale { x: (self.font_size * ANTI_ALIASING), y: (self.font_size * ANTI_ALIASING) })
        );
    }

    pub fn update_text_string(&mut self, str: String) {
        self.text = Text::new(
            TextFragment::new(str.to_string()).scale(PxScale { x: (self.font_size * ANTI_ALIASING), y: (self.font_size * ANTI_ALIASING) })
        );
    }

    pub fn draw(&self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.text,
            DrawParam::default()
            .dest(
                graphics::mint::Point2 {
                    x: self.position.x,
                    y: self.position.y,
                }
            )
            .scale(
                graphics::mint::Point2 {
                    x: 1.0 / ANTI_ALIASING,
                    y: 1.0 / ANTI_ALIASING,
                }
            ),
        )
        .expect("Couldnt Draw Text");
    }

    pub fn id(&self) -> &String {
        &self.id
    } 
}

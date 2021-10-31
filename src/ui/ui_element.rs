use ggez::graphics::{self, DrawParam, PxScale, TextFragment};


use super::Context;

use super::Text;
use super::vector2::Vector2F;

pub struct UIElement {
    id: String,
    text: Text,
    position: Vector2F,
}

impl UIElement {
    pub fn new(id: &str, str: &str, font_size: f32, position: Vector2F) -> Self {
        Self {
            id: id.to_string(),
            text: Text::new(
                TextFragment::new(str.to_string()).scale(PxScale { x: (font_size * 4.0), y: (font_size * 4.0) })
            ),
            position
        }
    }

    pub fn update_text_static(&mut self, str: &str) {
        self.text = Text::new(str);
    }

    pub fn update_text_string(&mut self, str: String) {
        self.text = Text::new(str);
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
                    x: 0.25,
                    y: 0.25,
                }
            ),
        )
        .expect("Couldnt Draw Text");
    }

    pub fn id(&self) -> &String {
        &self.id
    } 
}

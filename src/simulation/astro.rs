use ggez::{Context, graphics::{self, Mesh}};

use super::Vector2F;

#[derive(Clone, PartialEq, Debug)]
pub struct Astro {
    mesh: graphics::Mesh,
    mass: f32,
    position: Vector2F,
    velocity: Vector2F,
}

impl Astro {
    pub fn new(ctx: &mut Context, mass: f32, position: Vector2F, velocity: Vector2F) -> Self {
        Self {
            mesh: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::mint::Point2::<f32> {
                    x: position.x,
                    y: position.y,
                },
                mass * 0.5,
                0.2,
                graphics::Color::CYAN,
            )
            .unwrap(),
            mass,
            position,
            velocity,
        }
    }

    pub fn set_velocity(&mut self, velocity: Vector2F) {
        self.velocity = velocity;
    }

    pub fn set_position(&mut self, position: Vector2F) {
        self.position = position;
    }

    pub fn position(&self) -> Vector2F {
        self.position
    }

    pub fn velocity(&self) -> Vector2F {
        self.velocity
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }
}

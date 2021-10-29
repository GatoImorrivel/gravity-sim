pub mod astro;

use super::utils::vector2::Vector2F;
use ggez::{Context, GameResult, event::EventHandler, graphics::{self, Color, DrawParam}};

use astro::Astro;
use nalgebra::Point;

pub struct Simulation {
    g_const: f32,
    softening: f32,
    time_step: f32,
    astros: Vec<Astro>,
}

impl Simulation {
    pub fn new(g_const: f32, softening: f32, time_step: f32) -> Self {
        Simulation {
            g_const,
            softening,
            time_step,
            astros: Vec::<Astro>::new(),
        }
    }

    pub fn calculate_velocity(&mut self, astro_idx: usize, other_astro_idx: usize) -> Vector2F {
        let mut velocity = Vector2F { x: 0.0, y: 0.0 };

        let astro = &self.astros[astro_idx];
        let other_astro = &self.astros[other_astro_idx];

        if astro != other_astro {
            //skips over itself
            let dist = (astro.position() - other_astro.position()).magnituded();
            let force_dir = (other_astro.position() - astro.position())
                .normalized()
                .expect("Divided by zero");

            let acceleration = force_dir.multiplied_by(self.g_const * other_astro.mass() / dist);
            velocity += acceleration.multiplied_by(self.time_step);
        }
        return velocity
    }

    pub fn update_sim(&mut self) {
        let mut velocity;

        for astro in 0..self.astros.len() {
            velocity = Vector2F { x: 0.0, y: 0.0 };
            for other_astro in 0..self.astros.len() {
                velocity += self.calculate_velocity(astro, other_astro);
            }
            self.astros[astro].set_velocity(velocity);
        }

        for astro in self.astros.iter_mut() {
            astro.set_position((astro.position() + astro.velocity()).multiplied_by(self.time_step));
        }
    }

    pub fn add_astro(&mut self, astro: Astro) {
        self.astros.push(astro);
    }
}

impl EventHandler<ggez::GameError> for Simulation {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.update_sim();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::RED);
        for astro in self.astros.iter() {
            graphics::draw(ctx, astro.mesh(), (
                    graphics::mint::Point2::<f32> {
                        x: astro.position().x,
                        y: astro.position().y,
                    },
                )
            )?;
        }   
        graphics::present(ctx)
    }
}

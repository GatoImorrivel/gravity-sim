use ggez::{graphics, Context};

use super::{
    Vector2F,
    Astro,
};

pub struct Simulation {
    g_const: f32,
    astros: Vec<Astro>,
}

impl Simulation {
    pub fn new(g_const: f32) -> Self {
        Simulation {
            g_const,
            astros: Vec::<Astro>::new(),
        }
    }

    pub fn calculate_velocity(&mut self, astro_idx: usize, other_astro_idx: usize, time_step: f32) -> Vector2F {
        let mut velocity = Vector2F::new(0f32,0f32);
        let astro = &self.astros[astro_idx];
        let other_astro = &self.astros[other_astro_idx];

        if astro != other_astro {
            let dist = (astro.position() - other_astro.position()).magnituded();
            let force_dir = (other_astro.position() - astro.position())
                .normalized()
                .unwrap();

            let acceleration = force_dir.multiplied_by(self.g_const * other_astro.mass() / dist);
            velocity += acceleration.multiplied_by(time_step);
        }
        velocity
    }

    pub fn update_sim(&mut self, time_step: f32) {
        let mut velocity = Vector2F::new(0f32,0f32);

        for astro in 0..self.astros.len() {
            for other_astro in 0..self.astros.len() {
                velocity += self.calculate_velocity(astro, other_astro, time_step);
            }
            self.astros[astro].add_velocity(velocity);
            velocity = Vector2F::new(0f32,0f32);
        }

        for astro in self.astros.iter_mut() {
            astro.set_position(
                astro.position() + astro.velocity().multiplied_by(time_step),
            );
        }
    }

    pub fn draw_astros(&self, ctx: &mut Context) {
        for astro in self.astros.iter() {
            graphics::draw(
                ctx,
                astro.mesh(),
                (graphics::mint::Point2::<f32> {
                    x: astro.position().x,
                    y: astro.position().y,
                },),
            ).expect("Couldnt draw astro");
        }
    }

    pub fn add_astro(&mut self, astro: Astro) {
        self.astros.push(astro);
    }
    
    pub fn remove_astro(&mut self, astro_idx: usize) {
        self.astros.remove(astro_idx);
    }

    pub fn reset_astros(&mut self) {
        self.astros = vec![];
    }
}

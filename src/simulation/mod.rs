pub mod astro;

use super::utils::vector2::Vector2F;
use ggez::{Context, GameResult, event::EventHandler, graphics::{self, Color}, input};

use astro::Astro;

pub struct Simulation {
    g_const: f32,
    time_step: f32,
    current_time_step: f32,
    astros: Vec<Astro>,
}

impl Simulation {
    pub fn new(g_const: f32, time_step: f32) -> Self {
        Simulation {
            g_const,
            time_step,
            current_time_step: time_step,
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

            velocity += acceleration.multiplied_by(self.current_time_step);
        }
        return velocity
    }

    pub fn update_sim(&mut self) {
        let mut velocity = Vector2F {x: 0.0, y: 0.0};

        for astro in 0..self.astros.len() {
            for other_astro in 0..self.astros.len() {
                velocity += self.calculate_velocity(astro, other_astro);
            }
            self.astros[astro].add_velocity(velocity);
            velocity = Vector2F {x: 0.0, y: 0.0};
        }

        for astro in self.astros.iter_mut() {
            astro.set_position(astro.position() + astro.velocity().multiplied_by(self.current_time_step));
        }
    }

    pub fn add_astro(&mut self, astro: Astro) {
        self.astros.push(astro);
    }
}

impl EventHandler<ggez::GameError> for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update_sim();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(46, 52, 54));
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

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: ggez::event::KeyCode, _keymods: ggez::event::KeyMods) {
        match _keycode {
            input::keyboard::KeyCode::P => {
                self.current_time_step = if self.current_time_step == self.time_step {0.0} else {self.time_step}
            },
            input::keyboard::KeyCode::R => {
                for _astro in 0..self.astros.len() {
                    self.astros.pop();
                }
            },
            _ => (),
        }

    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: ggez::event::MouseButton, x_pos: f32, y_pos: f32) {
        if button == input::mouse::MouseButton::Left {
            self.add_astro(Astro::new(ctx, 50.0, Vector2F { x: (x_pos / 2.0), y: (y_pos / 2.0) }, Vector2F { x: (0.0), y: (0.0) }));
        }
    }
}

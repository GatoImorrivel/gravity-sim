pub mod astro;

use crate::{TIME_STEP, WIN_HEIGHT, WIN_WIDTH, ui::{UIManager, ui_element::UIElement}};

use super::utils::vector2::Vector2F;
use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    input, Context, GameResult,
};

use super::ui;
use astro::Astro;

pub struct Simulation {
    is_paused: bool,
    ui_manager: UIManager,
    g_const: f32,
    current_time_step: f32,
    astros: Vec<Astro>,
}

impl Simulation {
    pub fn new(g_const: f32, time_step: f32) -> Self {
        Simulation {
            is_paused: false,
            current_time_step: time_step,
            ui_manager: UIManager::new(
                vec!
                [
                    UIElement::new("Pause","Unpaused", Vector2F::new(WIN_WIDTH - WIN_WIDTH * 0.058, WIN_HEIGHT * 0.005)),
                    UIElement::new("Speed", "Speed: 1.0x", Vector2F::new(WIN_WIDTH - WIN_WIDTH * 0.075, WIN_HEIGHT * 0.025)),
                ],
            ),
            g_const,
            astros: Vec::<Astro>::new(),
        }
    }

    pub fn calculate_velocity(&mut self, astro_idx: usize, other_astro_idx: usize) -> Vector2F {
        let mut velocity = Vector2F::new(0f32,0f32);
        let astro = &self.astros[astro_idx];
        let other_astro = &self.astros[other_astro_idx];

        if astro != other_astro {
            let dist = (astro.position() - other_astro.position()).magnituded();
            let force_dir = (other_astro.position() - astro.position())
                .normalized()
                .expect("Divided by zero");

            let acceleration = force_dir.multiplied_by(self.g_const * other_astro.mass() / dist);
            velocity += acceleration.multiplied_by(self.current_time_step);
        }
        velocity
    }

    pub fn update_sim(&mut self) {
        let mut velocity = Vector2F::new(0f32,0f32);

        for astro in 0..self.astros.len() {
            for other_astro in 0..self.astros.len() {
                velocity += self.calculate_velocity(astro, other_astro);
            }
            self.astros[astro].add_velocity(velocity);
            velocity = Vector2F::new(0f32,0f32);
        }

        for astro in self.astros.iter_mut() {
            astro.set_position(
                astro.position() + astro.velocity().multiplied_by(self.current_time_step),
            );
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
            graphics::draw(
                ctx,
                astro.mesh(),
                (graphics::mint::Point2::<f32> {
                    x: astro.position().x,
                    y: astro.position().y,
                },),
            )?;
        }
        self.ui_manager.draw_elements(ctx);
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: ggez::event::KeyCode,
        _keymods: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            input::keyboard::KeyCode::P => {
                if _repeat != false {
                    return;
                }
                self.is_paused = !self.is_paused;
                self.ui_manager.update_simulation_info(
                    self.current_time_step,
                    if self.is_paused == true {
                        ui::SimulationCommands::Pause
                    } else {
                        ui::SimulationCommands::Unpaused
                    },
                );
            }
            input::keyboard::KeyCode::R => {
                if _repeat != false {
                    return;
                }
                for _astro in 0..self.astros.len() {
                    self.astros.pop();
                }
            }
            input::keyboard::KeyCode::Down => {
                if format!("{:.1}", self.current_time_step) != "0.0" {
                    self.current_time_step -= TIME_STEP / (TIME_STEP * 10.0);
                    self.ui_manager.update_simulation_info(
                        self.current_time_step,
                        ui::SimulationCommands::SpeedChange,
                    );
                }
            }
            input::keyboard::KeyCode::Up => {
                self.current_time_step += TIME_STEP / (TIME_STEP * 10.0);
                self.ui_manager.update_simulation_info(
                    self.current_time_step,
                    ui::SimulationCommands::SpeedChange,
                );
            }
            _ => (),
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if _button == input::mouse::MouseButton::Left {
            self.add_astro(Astro::new(
                _ctx,
                20.0,
                Vector2F {
                    x: (_x / 2.0),
                    y: (_y / 2.0),
                },
                Vector2F { x: (0.0), y: (0.0) },
            ));
        }
    }
}

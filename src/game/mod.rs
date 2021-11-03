use ggez::{graphics, input, Context, event::EventHandler, GameResult};
use ggez::graphics::Color;

use super::{
    TIME_STEP,
    simulation::{astro::Astro, Simulation},
    ui::{self, ui_manager::UIManager},
    utils::vector2::Vector2F,
};

pub struct GameState {
    is_paused: bool,
    time_step: f32,
    simulation: Simulation,
    ui_manager: UIManager,
}

impl GameState {
    pub fn new(time_step: f32, g_const: f32, ui_manager: UIManager) -> Self {
        Self {
            is_paused: false,
            time_step: time_step,
            simulation: Simulation::new(g_const),
            ui_manager: ui_manager,
        }
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.simulation.update_sim(self.time_step);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(46, 52, 54));
        self.simulation.draw_astros(ctx);
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
                self.time_step = if self.is_paused == true {0.0} else {1.0};
                self.ui_manager.update_simulation_info(
                    self.time_step,
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
                self.simulation.reset_astros();
            }
            input::keyboard::KeyCode::Down => {
                if format!("{:.1}", self.time_step) != "0.0" {
                    self.time_step -= TIME_STEP / (TIME_STEP * 10.0);
                    self.ui_manager.update_simulation_info(
                        self.time_step,
                        ui::SimulationCommands::SpeedChange,
                    );
                }
            }
            input::keyboard::KeyCode::Up => {
                self.time_step += TIME_STEP / (TIME_STEP * 10.0);
                self.ui_manager.update_simulation_info(
                    self.time_step,
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
            self.simulation.add_astro(Astro::new(
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
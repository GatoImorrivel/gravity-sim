pub mod game;
pub mod simulation;
pub mod ui;
pub mod utils;

use ggez::{ContextBuilder, event};

use game::GameState;
use ui::{ui_manager::UIManager, ui_element::UIElement};
use utils::vector2::Vector2F;

const G: f32 = 2.0;
const TIME_STEP: f32 = 0.5;
const WIN_WIDTH: f32 = 1280.0;
const WIN_HEIGHT: f32 = 848.0;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("NBody Simulation", "Gato")
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(WIN_WIDTH, WIN_HEIGHT)
                .resizable(false),
        )
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Gravity Simulator")
                .samples(ggez::conf::NumSamples::Sixteen),
        )
        .build()
        .expect("Error");

    let game_state = GameState::new(
        TIME_STEP,
        G,
        UIManager::new(vec![
            UIElement::new(
                "Pause", 
                "Unpaused",
                16.0, 
                Vector2F::new(0.0, ui::relative_height(0.005))),
            UIElement::new(
                "Speed",
                "Speed: 1.0x",
                16.0, 
                Vector2F::new(0.0, ui::relative_height(0.025)),
            ),
        ]),
    );
    event::run(ctx, event_loop, game_state);
}

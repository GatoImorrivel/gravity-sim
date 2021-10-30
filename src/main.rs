pub mod utils;
pub mod ui;
pub mod simulation;
pub mod game;

use ggez::{ContextBuilder, event};

use game::GameState;
use utils::vector2::Vector2F;
use ui::{UIManager, ui_element::UIElement};

const G: f32 = 1.0;
const TIME_STEP: f32 = 0.5;
const WIN_WIDTH: f32 = 1280.0;
const WIN_HEIGHT: f32 = 848.0;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("NBody Simulation", "Gato")
        .window_mode(ggez::conf::WindowMode::default()
            .dimensions(WIN_WIDTH, WIN_HEIGHT)
            .resizable(false)
        )
        .window_setup(ggez::conf::WindowSetup::default().title("Gravity Simulator")
            .samples(ggez::conf::NumSamples::Sixteen)
        )
        .build()
        .expect("error creating window");

    let game_state = GameState::new(
        TIME_STEP, 
        G, 
        UIManager::new(
            vec!
            [
                UIElement::new("Pause","Unpaused", Vector2F::new(WIN_WIDTH - WIN_WIDTH * 0.058, WIN_HEIGHT * 0.005)),
                UIElement::new("Speed", "Speed: 1.0x", Vector2F::new(WIN_WIDTH - WIN_WIDTH * 0.075, WIN_HEIGHT * 0.025)),
            ],
        ),
    );
    event::run(ctx, event_loop, game_state);
}



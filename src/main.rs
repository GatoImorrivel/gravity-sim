pub mod utils;
pub mod ui;
pub mod simulation;

use ggez::{ContextBuilder, event};

use simulation::Simulation;

const G: f32 = 1.0;
const TIME_STEP: f32 = 0.5;
const WIN_WIDTH: f32 = 1280.0;
const WIN_HEIGHT: f32 = 848.0;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("NBody Simulation", "Gato")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT))
        .window_setup(ggez::conf::WindowSetup::default().title("Gravity Simulator")
            .samples(ggez::conf::NumSamples::Sixteen)
        )
        .build()
        .expect("error creating window");

    let simulation = Simulation::new(G, TIME_STEP);
    event::run(ctx, event_loop, simulation);
}



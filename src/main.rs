pub mod utils;

mod simulation;

use ggez::{ContextBuilder, event};

use simulation::Simulation;

const G: f32 = 1.0;
const TIME_STEP: f32 = 0.5;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("NBody Simulation", "Gato")
        .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 848.0))
        .window_setup(ggez::conf::WindowSetup::default().title("Gravity Simulator").samples(ggez::conf::NumSamples::Sixteen))
        .build()
        .expect("error creating window");

    let simulation = Simulation::new(G, TIME_STEP);
    event::run(ctx, event_loop, simulation);
}



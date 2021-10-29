mod simulation;
pub mod utils;

use ggez::{ContextBuilder, event, graphics};
use simulation::Simulation;
use simulation::astro::Astro;
use utils::vector2::Vector2F;

const G: f32 = 1.0;
const SOFTENING: f32 = 0.1;
const TIME_STEP: f32 = 1.0;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("NBody Simulation", "Gato")
        .build()
        .expect("error creating window");
    graphics::set_resizable(&mut ctx, false).expect("error");
    graphics::set_window_title(&mut ctx, "Gravity Simulator");
    graphics::set_drawable_size(&mut ctx, 1280.0, 848.0).expect("error");

    let mut simulation = Simulation::new(G, SOFTENING, TIME_STEP);
    simulation.add_astro(Astro::new(&mut ctx, 10.0, Vector2F { x: (20.0), y: (20.0) }, Vector2F { x: (20.0), y: (20.0) }));
    simulation.add_astro(Astro::new(&mut ctx, 20.0, Vector2F { x: (200.0), y: (200.0) }, Vector2F { x: (0.0), y: (0.0) }));
    simulation.add_astro(Astro::new(&mut ctx, 15.0, Vector2F { x: (400.0), y: (100.0) }, Vector2F { x: (0.0), y: (0.0) }));
    event::run(ctx, event_loop, simulation);
}



use std::f64::consts;

use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, ContextBuilder, GameResult};

use std::time::Instant;

pub mod simulation;
use crate::simulation::Simulation;

fn main() {
    if cfg!(debug_assertions) {
        println!("Running in debug build will be alot slower, especially in precision mode!");
    }

    let (ctx, event_loop) = ContextBuilder::new("Double Pendulum", "Peanutt42")
        .window_mode(WindowMode {
            width: 800.0,
            height: 400.0,
            resizable: true,
            ..Default::default()
        })
        .build()
        .expect("aieee, could not create ggez context!");

    let mut visualization = Visualization::new();
    visualization.set_default_sim();

    event::run(ctx, event_loop, visualization);
}

struct Visualization {
    simulations: Vec<Simulation>,
    precision_mode_enabled: bool,
    fixed_time_step: f64,
    accumulator: f64,
    last_update_time: Instant,
    trails: bool,
    show_pendulum: bool,
}

impl Visualization {
    pub fn new() -> Self {
        Visualization {
            simulations: vec![],
            precision_mode_enabled: false,
            fixed_time_step: 0.0002, // FPS: 5000
            accumulator: 0.0,
            last_update_time: Instant::now(),
            trails: true,
            show_pendulum: true,
        }
    }

    pub fn set_default_sim(&mut self) {
        println!("Default simulation");
        self.simulations.clear();
        self.simulations
            .push(Simulation::new(120.0 * consts::PI / 180.0, Color::WHITE));
        self.trails = true;
        self.show_pendulum = true;
    }
    pub fn set_chaos_sim(&mut self) {
        const COUNT: i32 = 1000;
        println!("Chaos simulation with {COUNT} double-pendulums");
        self.simulations.clear();
        for i in 0..COUNT {
            let color = rainbow_color(i as f32 / COUNT as f32);
            self.simulations.push(Simulation::new(
                (120.0 + 0.0001 * i as f64) * consts::PI / 180.0,
                color,
            ));
        }
        self.trails = false;
        self.show_pendulum = false;
    }
}

impl EventHandler for Visualization {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::Key1) {
            self.set_default_sim();
        } else if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::Key2) {
            self.set_chaos_sim();
        } else if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::Space) {
            self.precision_mode_enabled = !self.precision_mode_enabled;
            println!(
                "Precision Mode: {}",
                if self.precision_mode_enabled {
                    "Enabled"
                } else {
                    "Disabled"
                }
            );
        }

        let now = Instant::now();
        let delta_time = (now - self.last_update_time).as_secs_f64();
        self.last_update_time = Instant::now();

        if self.precision_mode_enabled {
            self.accumulator += delta_time;
            while self.accumulator >= self.fixed_time_step {
                for sim in self.simulations.iter_mut() {
                    sim.update(self.fixed_time_step);
                }
                self.accumulator -= self.fixed_time_step;
            }
        } else {
            for sim in self.simulations.iter_mut() {
                sim.update(delta_time);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        for sim in self.simulations.iter_mut() {
            sim.draw(ctx, &mut canvas, self.trails, self.show_pendulum);
        }

        canvas.finish(ctx)
    }
}

fn rainbow_color(mut scalar: f32) -> Color {
    scalar = scalar.clamp(0.0, 1.0);

    let hue = scalar * 360.0;
    let saturation = 1.0;
    let value = 1.0;

    // Convert HSV to RGB manually
    let hi = (f32::floor(hue / 60.0) as i32) % 6;
    let f = hue / 60.0 - f32::floor(hue / 60.0);
    let p = value * (1.0 - saturation);
    let q = value * (1.0 - f * saturation);
    let t = value * (1.0 - (1.0 - f) * saturation);

    match hi {
        0 => Color::new(value, t, p, 1.0),
        1 => Color::new(q, value, p, 1.0),
        2 => Color::new(p, value, t, 1.0),
        3 => Color::new(p, q, value, 1.0),
        4 => Color::new(t, p, value, 1.0),
        5 => Color::new(value, p, q, 1.0),
        _ => Color::new(value, p, q, 1.0),
    }
}

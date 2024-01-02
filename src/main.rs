use std::f64::consts;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::winit::event::VirtualKeyCode;

pub mod simulation;
use crate::simulation::Simulation;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Double Pendulum", "Peanutt42")
        .build()
        .expect("aieee, could not create ggez context!");

    let mut visualization = Visualization::new(&mut ctx);
    visualization.set_default_sim();

    event::run(ctx, event_loop, visualization);
}

struct Visualization {
    simulations: Vec<Simulation>,
    trails: bool,
    show_pendulum: bool,
}

impl Visualization {
    pub fn new(_ctx: &mut Context) -> Self {
        Visualization {
            simulations: vec![],
            trails: true,
            show_pendulum: true,
        }
    }

    fn rainbow_color(mut scalar: f32) -> Color {
        scalar = f32::min(1.0, f32::max(0.0, scalar));

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

    pub fn set_default_sim(&mut self) {
        self.simulations.clear();
        self.simulations.push(Simulation::new(120.0 * consts::PI / 180.0, Color::WHITE));
        self.trails = true;
        self.show_pendulum = true;
    }
    pub fn set_chaos_sim(&mut self) {
        self.simulations.clear();
        const COUNT: i32 = 1000;
        for i in 0..COUNT {
            let color = Self::rainbow_color(i as f32 / COUNT as f32);
            self.simulations.push(Simulation::new((120.0 + 0.0001 * i as f64) * consts::PI / 180.0, color));
        }
        self.trails = false;
        self.show_pendulum = false;
    }
}

impl EventHandler for Visualization {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if _ctx.keyboard.is_key_just_pressed(VirtualKeyCode::Key1) {
            self.set_default_sim();
        }
        else if _ctx.keyboard.is_key_just_pressed(VirtualKeyCode::Key2) {
            self.set_chaos_sim();
        }

        for sim in self.simulations.iter_mut() {
            sim.update();
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
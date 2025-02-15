use ggez::glam::*;
use ggez::graphics::{self, Canvas, Color};
use ggez::mint::Point2;
use ggez::Context;

use glm::DVec2;
#[inline(always)]
fn dvec2_to_vec2(v: &DVec2) -> Vec2 {
    Vec2 {
        x: v.x as f32,
        y: v.y as f32,
    }
}

#[inline(always)]
fn vec2_to_point2(v: &Vec2) -> Point2<f32> {
    Point2 { x: v.x, y: v.y }
}

struct Pendulum {
    position: DVec2,
    velocity: f64,
    acceleration: f64,
    length: f64,
    mass: f64,
    angle: f64,
}

impl Pendulum {
    fn new(angle: f64, mass: f64, length: f64) -> Self {
        Self {
            position: DVec2::new(length * f64::sin(angle), length * f64::cos(angle)),
            velocity: 0.0,
            acceleration: 0.0,
            length,
            mass,
            angle,
        }
    }

    fn get_debug_position(&self, center: &Vec2, debug_length: f64) -> Vec2 {
        dvec2_to_vec2(&(self.position * debug_length)) + *center
    }

    fn draw_debug(&self, ctx: &mut Context, canvas: &mut Canvas, color: Color, center: &Vec2) {
        let debug_length = ctx.gfx.window().inner_size().width as f64 / 7.5;
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.mass as f32,
            0.1,
            color,
        )
        .unwrap();
        canvas.draw(&circle, self.get_debug_position(center, debug_length));
    }
}

pub struct Simulation {
    top_pendulum: Pendulum,
    bottom_pendulum: Pendulum,
    gravity: f64,
    debug_color: Color,
    debug_trail_points: Vec<Point2<f32>>,
}

impl Simulation {
    pub fn new(angle: f64, color: Color) -> Self {
        Self {
            top_pendulum: Pendulum::new(angle, 10.0, 2.0),
            bottom_pendulum: Pendulum::new(angle, 20.0, 1.0),
            gravity: 9.81,
            debug_color: color,
            debug_trail_points: vec![],
        }
    }

    pub fn update(&mut self, time_step: f64) {
        // See https://www.myphysicslab.com/pendulum/double-pendulum-en.html under 'Numerical Solution' at the bottom of the page
        let g = self.gravity;
        let l1 = self.top_pendulum.length;
        let l2 = self.bottom_pendulum.length;
        let m1 = self.top_pendulum.mass;
        let m2 = self.bottom_pendulum.mass;
        let angle1 = self.top_pendulum.angle;
        let angle2 = self.bottom_pendulum.angle;
        let velo1 = self.top_pendulum.velocity;
        let velo2 = self.bottom_pendulum.velocity;

        let n11 = -g * (2.0 * m1 + m2) * f64::sin(angle1);
        let n12 = -m2 * g * f64::sin(angle1 - 2.0 * angle2);
        let n13 = -2.0 * f64::sin(angle1 - angle2) * m2;
        let n14 = velo2 * velo2 * l2 + velo1 * velo1 * l1 * f64::cos(angle1 - angle2);
        let den = 2.0 * m1 + m2 - m2 * f64::cos(2.0 * angle1 - 2.0 * angle2);
        let n21 = 2.0 * f64::sin(angle1 - angle2);
        let n22 = velo1 * velo1 * l1 * (m1 + m2);
        let n23 = g * (m1 + m2) * f64::cos(angle1);
        let n24 = velo2 * velo2 * l2 * m2 * f64::cos(angle1 - angle2);

        self.top_pendulum.acceleration = (n11 + n12 + n13 * n14) / (l1 * den);
        self.bottom_pendulum.acceleration = (n21 * (n22 + n23 + n24)) / (l2 * den);

        self.top_pendulum.velocity += self.top_pendulum.acceleration * time_step;
        self.bottom_pendulum.velocity += self.bottom_pendulum.acceleration * time_step;

        self.top_pendulum.angle += self.top_pendulum.velocity * time_step;
        self.bottom_pendulum.angle += self.bottom_pendulum.velocity * time_step;

        self.top_pendulum.position.x = self.top_pendulum.length * f64::sin(self.top_pendulum.angle);
        self.top_pendulum.position.y = self.top_pendulum.length * f64::cos(self.top_pendulum.angle);

        self.bottom_pendulum.position.x = self.bottom_pendulum.length
            * f64::sin(self.bottom_pendulum.angle)
            + self.top_pendulum.position.x;
        self.bottom_pendulum.position.y = self.bottom_pendulum.length
            * f64::cos(self.bottom_pendulum.angle)
            + self.top_pendulum.position.y;
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        trails: bool,
        show_pendulum: bool,
    ) {
        let window_size = ctx.gfx.window().inner_size();
        let debug_length = window_size.width as f64 / 7.5;
        let center: Vec2 = Vec2::new(
            window_size.width as f32 / 2.0,
            window_size.height as f32 / 2.0,
        );

        if show_pendulum {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                10.0,
                0.1,
                self.debug_color,
            );
            canvas.draw(&circle.unwrap(), center);

            self.top_pendulum
                .draw_debug(ctx, canvas, self.debug_color, &center);
            self.bottom_pendulum
                .draw_debug(ctx, canvas, self.debug_color, &center);
        }

        let bottom_debug_position = self
            .bottom_pendulum
            .get_debug_position(&center, debug_length);

        if trails {
            self.debug_trail_points
                .push(vec2_to_point2(&bottom_debug_position));
            let lines = graphics::Mesh::new_line(ctx, &self.debug_trail_points, 1.0, Color::GREEN);
            if let Ok(lines) = &lines {
                canvas.draw(lines, Vec2::new(0.0, 0.0));
            }
        }

        let points: [Point2<f32>; 3] = [
            vec2_to_point2(&center),
            vec2_to_point2(&self.top_pendulum.get_debug_position(&center, debug_length)),
            vec2_to_point2(&bottom_debug_position),
        ];
        let lines = graphics::Mesh::new_line(ctx, &points, 1.0, self.debug_color);

        canvas.draw(&lines.unwrap(), Vec2::new(0.0, 0.0));
    }
}

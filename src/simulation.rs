use std::time::Instant;

use ggez::Context;
use ggez::graphics::{ Canvas, Color, self };
use ggez::glam::*;
use ggez::mint::Point2;

use glm::DVec2;
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

	fn draw_debug(&mut self, ctx: &mut Context, canvas: &mut Canvas, center: &Vec2, trails: bool, show_pendulum: bool) {
		if show_pendulum {
			let circle = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), Vec2::new(0.0, 0.0), self.mass as f32, 2.0, Color::WHITE);
			canvas.draw(&circle.unwrap(), Vec2::new((self.position.x * 100.0) as f32 + center.x, (self.position.y * 100.0) as f32 + center.y));
		}

		if trails {
			// todo
		}
	}
}

pub struct Simulation {
	top_pendulum: Pendulum,
	bottom_pendulum: Pendulum,
	last_update_time: Instant,
	gravity: f64,
}

impl Simulation {
	pub fn new(angle: f64, color: Color) -> Self {
		Self {
			top_pendulum: Pendulum::new(angle, 10.0, 2.0),
			bottom_pendulum: Pendulum::new(angle, 20.0, 1.0),
			last_update_time: Instant::now(),
			gravity: 9.81,
		}
	}

	pub fn update(&mut self) {
		let now = Instant::now();
		let delta_time = (now - self.last_update_time).as_secs_f64();
		self.last_update_time = now;

		// See https://www.myphysicslab.com/pendulum/double-pendulum-en.html under 'Numerical Solution' at the bottom of the page
		let n11 = -self.gravity*(2.0*self.top_pendulum.mass+self.bottom_pendulum.mass)*f64::sin(self.top_pendulum.angle);
		let n12 = -self.bottom_pendulum.mass*self.gravity*f64::sin(self.top_pendulum.angle-2.0*self.bottom_pendulum.angle);
		let n13 = -2.0*f64::sin(self.top_pendulum.angle-self.bottom_pendulum.angle) * self.bottom_pendulum.mass;
		let n14 = (self.bottom_pendulum.velocity*self.bottom_pendulum.velocity*self.bottom_pendulum.length + self.top_pendulum.velocity*self.top_pendulum.velocity*self.top_pendulum.length*f64::cos(self.top_pendulum.angle-self.bottom_pendulum.angle));
		let den = 2.0*self.top_pendulum.mass+self.bottom_pendulum.mass-self.bottom_pendulum.mass*f64::cos(2.0*self.top_pendulum.angle-2.0*self.bottom_pendulum.angle);
		let n21 = 2.0*f64::sin(self.top_pendulum.angle-self.bottom_pendulum.angle);
		let n22 = self.top_pendulum.velocity*self.top_pendulum.velocity*self.top_pendulum.length*(self.top_pendulum.mass+self.bottom_pendulum.mass);
		let n23 = self.gravity*(self.top_pendulum.mass+self.bottom_pendulum.mass)*f64::cos(self.top_pendulum.angle);
		let n24 = self.bottom_pendulum.velocity*self.bottom_pendulum.velocity*self.bottom_pendulum.length*self.bottom_pendulum.mass*f64::cos(self.top_pendulum.angle-self.bottom_pendulum.angle);

		self.top_pendulum.acceleration = (n11+n12+n13*n14) /(self.top_pendulum.length*den);
		self.bottom_pendulum.acceleration = (n21*(n22+n23+n24)) /(self.bottom_pendulum.length*den);

		self.top_pendulum.velocity += self.top_pendulum.acceleration * delta_time;
		self.bottom_pendulum.velocity += self.bottom_pendulum.acceleration * delta_time;

		self.top_pendulum.angle += self.top_pendulum.velocity * delta_time;
		self.bottom_pendulum.angle += self.bottom_pendulum.velocity * delta_time;
		println!("delta: {},top: {}, bottom: {}", delta_time, self.top_pendulum.angle, self.bottom_pendulum.angle);

		self.top_pendulum.position.x = self.top_pendulum.length * f64::sin(self.top_pendulum.angle);
		self.top_pendulum.position.y = self.top_pendulum.length * f64::cos(self.top_pendulum.angle);

		self.bottom_pendulum.position.x = self.bottom_pendulum.length * f64::sin(self.bottom_pendulum.angle) + self.top_pendulum.position.x;
		self.bottom_pendulum.position.y = self.bottom_pendulum.length * f64::cos(self.bottom_pendulum.angle) + self.top_pendulum.position.y;
	}

	pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, trails: bool, show_pendulum: bool) {
		let center: Vec2 = Vec2::new(400.0, 200.0);
		
		if show_pendulum {
			let circle = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), Vec2::new(0.0, 0.0), 10.0, 2.0, Color::WHITE);
			canvas.draw(&circle.unwrap(), center);
		}

		self.top_pendulum.draw_debug(ctx, canvas, &center, trails, show_pendulum);
		self.bottom_pendulum.draw_debug(ctx, canvas, &center, trails, show_pendulum);
		
		let points: [Point2<f32>; 3] = [
			Point2{ x: center.x, y: center.y },
			Point2{ x: self.top_pendulum.position.x as f32 + center.x, y: self.top_pendulum.position.y as f32 + center.y },
			Point2{ x: self.bottom_pendulum.position.x as f32 + center.x, y: self.bottom_pendulum.position.y as f32 + center.y }
		];
		graphics::Mesh::new_line(ctx, &points, 1.0, Color::WHITE);
	}
}
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    let visualization = Visualization::new(&mut ctx);

    event::run(ctx, event_loop, visualization);
}

struct Visualization {
    
}

impl Visualization {
    pub fn new(_ctx: &mut Context) -> Self {
        Visualization {
        
        }
    }
}

impl EventHandler for Visualization {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        

        
        canvas.finish(ctx)
    }
}
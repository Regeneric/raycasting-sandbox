use eyre::Result;

use ggez::event::{EventHandler};
use ggez::graphics::{self, Color, TextFragment, DrawParam, Text};
use ggez::{Context, GameResult};


#[derive(Default)]
pub struct MainState {
    game_name: String,
}

impl MainState {
    pub fn new(game_name: &str) -> Self {
        Self {
            game_name: game_name.to_owned(),
        }
    }

    pub fn setup(&mut self, _context: &mut Context) -> Result<()> {
        Ok(())
    } 
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let game_name = TextFragment::new(self.game_name.clone()).color(Color::WHITE);
        let text = Text::new(game_name);
        canvas.draw(&text, DrawParam::new());

        canvas.finish(ctx)?;
        Ok(())
    }
}
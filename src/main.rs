use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings, State, Window},
    Result,
};

struct Game;

impl State for Game {
    /// Load the asset and initialise the game
    fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

    /// Draw stuff on the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }
}

fn main() {
    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("Quicksilver Roguelike", Vector::new(800, 600), settings);
}


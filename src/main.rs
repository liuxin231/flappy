mod obstacle;
mod player;
mod state;

use crate::player::Player;
use crate::state::State;
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 60.;

#[derive(Debug)]
enum GameMode {
    Menu,
    Playing,
    End,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}

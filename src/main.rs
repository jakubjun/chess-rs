use std::{collections::HashMap, fmt::Display, io::stdin};

use colored::{Colorize, CustomColor};
use game::Game;

mod figure;
mod game;

fn main() -> Result<(), ()> {
    let mut g = Game::new();
    g.game_loop();
    Ok(())
}

use game::Game;

mod figure;
mod game;

fn main() -> Result<(), ()> {
    let mut g = Game::new();
    g.game_loop();
    Ok(())
}

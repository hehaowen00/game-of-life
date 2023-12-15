use bevy::prelude::*;
use game_of_life::Game;

fn main() {
    let cells = (50, 50);

    let mut game = Game::new(cells.0, cells.1);
    let grid = game.grid();

    grid.set_index(6, 6);
    grid.set_index(6, 7);
    grid.set_index(6, 8);
    grid.set_index(5, 8);
    grid.set_index(4, 7);

    App::new().add_plugins(game).run();
}

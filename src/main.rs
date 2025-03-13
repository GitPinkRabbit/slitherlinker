mod game;

fn main() {
    let mut game = game::Game::new(5, 5, "b2b2a1d222a202b333a");
    game.print_cells();
    game.print_cells_and_links();
    game.print_cells();
    game.print_cells_and_links();
    game.print_cells();
}

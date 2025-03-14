mod game;
use std::fs;

fn main() {
    let mut game = game::Game::new(5, 5, "b2b2a1d222a202b333a");
    game.print_cells();
    game.print_cells_and_links();
    game.full_print();
    game.print_cells_and_links();
    game.print_cells();
    let rules_texts = fs::read_to_string("./assets/rules.txt")
        .unwrap()
        .trim()
        .split("[NAME]")
        .filter_map(|s| {
            let strs = s.split("[BEGIN]").collect::<Vec<_>>();
            if strs.len() != 2 {
                return None;
            }
            let name = strs[0].trim().to_owned();
            let content = strs[1].split("[END]").collect::<Vec<_>>()[0]
                .trim()
                .to_owned();
            Some((name, content))
        })
        .collect::<Vec<_>>();
    let rules = rules_texts
        .iter()
        .map(|(name, content)| game::Rule::new(name, content))
        .collect::<Vec<_>>();
    for rule in rules {
        rule.print();
    }
}

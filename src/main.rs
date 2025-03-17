mod game;
use std::fs;

fn main() {
    let mut game = game::Game::new(5, 5, "b2b2a1d222a202b333a");
    game.print_cells();
    game.print_cells_and_links();
    game.full_print();
    game.print_cells_and_links();
    game.print_cells();
    let read_rules_texts_with_priority =
        |path: &str, default_priority: Option<i32>| -> Vec<(String, String, i32)> {
            fs::read_to_string(path)
                .unwrap()
                .trim()
                .split("[NAME]")
                .filter_map(|s| {
                    let strs = s.split("[BEGIN]").collect::<Vec<_>>();
                    if strs.len() != 2 {
                        return None;
                    }
                    let (name, priority) =
                        match *strs[0].split("[PRIORITY]").collect::<Vec<_>>().as_slice() {
                            [name] => (name.trim().to_owned(), default_priority.unwrap()),
                            [name, priority] => (
                                name.trim().to_owned(),
                                priority.trim().parse::<i32>().unwrap(),
                            ),
                            _ => unreachable!(),
                        };
                    let content = strs[1].split("[END]").collect::<Vec<_>>()[0]
                        .trim()
                        .to_owned();
                    Some((name, content, priority))
                })
                .collect::<Vec<_>>()
        };
    let unconditional_rules_texts =
        read_rules_texts_with_priority("./assets/unconditional_rules.txt", Some(100));
    let basic_rules_texts = read_rules_texts_with_priority("./assets/basic_rules.txt", Some(200));
    let mut rules = unconditional_rules_texts
        .iter()
        .chain(basic_rules_texts.iter())
        .map(|(name, content, priority)| {
            let rule = game::Rule::new(name, content);
            let mut v = rule.symmetries();
            v.insert(0, rule);
            (*priority, v)
        })
        .collect::<Vec<_>>();
    rules.sort_by_key(|(priority, _)| *priority);
    let rules = rules;
    for (_, rule_sym) in &rules {
        rule_sym[0].print();
        println!("Number of symmetries: {}", rule_sym.len());
    }
    let res = game.try_apply_rule(&rules[3].1[0], 4, 3);
    assert_eq!(res, Some(true));
    game.print_cells_and_links();
    let res = game.try_apply_rule(&rules[3].1[0], 4, 3);
    assert_eq!(res, Some(false));
    let res = game.try_apply_rule(&rules[3].1[0], 4, 4);
    assert_eq!(res, None);
}

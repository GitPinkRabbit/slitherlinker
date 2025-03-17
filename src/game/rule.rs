use super::element::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub(super) struct HalfRule {
    pub(super) height: usize,
    pub(super) width: usize,
    pub(super) cells: Vec<Vec<CellType>>,
    pub(super) hlinks: Vec<Vec<LinkType>>,
    pub(super) vlinks: Vec<Vec<LinkType>>,
    pub(super) corners: Vec<Vec<CornerType>>,
}

impl HalfRule {
    fn new(rule_str: &str) -> HalfRule {
        let lines = rule_str
            .trim()
            .lines()
            .map(str::as_bytes)
            .collect::<Vec<_>>();
        assert!(lines.len() % 4 == 1);
        let height = (lines.len() - 1) / 4;
        assert!(lines[0].len() % 8 == 1);
        let width = (lines[0].len() - 1) / 8;
        assert!(lines.iter().all(|line| line.len() == width * 8 + 1));
        let mut cells = vec![vec![Empty; width]; height];
        for row in 0..height {
            for col in 0..width {
                cells[row][col] = match lines[row * 4 + 2][col * 8 + 4] as char {
                    ' ' => Empty,
                    '0' => Zero,
                    '1' => One,
                    '2' => Two,
                    '3' => Three,
                    _ => panic!(),
                }
            }
        }
        let mut hlinks = vec![vec![LMaybe; width]; height + 1];
        let mut vlinks = vec![vec![LMaybe; width + 1]; height];
        let mut corners = vec![vec![CMaybe; 2 * width]; 2 * height];
        for row in 0..=height {
            for col in 0..width {
                hlinks[row][col] = match lines[row * 4][col * 8 + 4] as char {
                    '.' => LMaybe,
                    '=' => Link,
                    ' ' => Unlink,
                    _ => panic!(),
                };
            }
        }
        for row in 0..height {
            for col in 0..=width {
                vlinks[row][col] = match lines[row * 4 + 2][col * 8] as char {
                    '.' => LMaybe,
                    '$' => Link,
                    ' ' => Unlink,
                    _ => panic!(),
                };
            }
        }
        for row in 0..2 * height {
            for col in 0..2 * width {
                corners[row][col] = match lines[row * 2 + 1][col * 4 + 2] as char {
                    ' ' => CMaybe,
                    '0' => CZero,
                    '1' => COne,
                    '2' => CTwo,
                    'E' => Even,
                    'L' => Less,
                    'G' => Greater,
                    _ => panic!(),
                };
            }
        }
        HalfRule {
            height,
            width,
            cells,
            hlinks,
            vlinks,
            corners,
        }
    }

    fn full_print_row(&self, row: usize) {
        full_print_row(
            self.height,
            self.width,
            &self.cells,
            &self.hlinks,
            &self.vlinks,
            &self.corners,
            row,
            false,
        );
    }

    fn rotated_90(&self) -> HalfRule {
        let height = self.width;
        let width = self.height;
        let mut cells = vec![vec![Empty; width]; height];
        let mut hlinks = vec![vec![LMaybe; width]; height + 1];
        let mut vlinks = vec![vec![LMaybe; width + 1]; height];
        let mut corners = vec![vec![CMaybe; 2 * width]; 2 * height];
        #[expect(clippy::needless_range_loop)]
        for row in 0..height {
            for col in 0..width {
                cells[row][col] = self.cells[col][height - 1 - row];
            }
        }
        #[expect(clippy::needless_range_loop)]
        for row in 0..=height {
            for col in 0..width {
                hlinks[row][col] = self.vlinks[col][height - row];
            }
        }
        #[expect(clippy::needless_range_loop)]
        for row in 0..height {
            for col in 0..=width {
                vlinks[row][col] = self.hlinks[col][height - 1 - row];
            }
        }
        #[expect(clippy::needless_range_loop)]
        for row in 0..2 * height {
            for col in 0..2 * width {
                corners[row][col] = self.corners[col][2 * height - 1 - row];
            }
        }
        HalfRule {
            height,
            width,
            cells,
            hlinks,
            vlinks,
            corners,
        }
    }

    fn reversed_lr(&self) -> HalfRule {
        let cells = self
            .cells
            .iter()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
        let hlinks = self
            .hlinks
            .iter()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
        let vlinks = self
            .vlinks
            .iter()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
        let corners = self
            .corners
            .iter()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
        HalfRule {
            height: self.height,
            width: self.width,
            cells,
            hlinks,
            vlinks,
            corners,
        }
    }

    fn reversed_ud(&self) -> HalfRule {
        let cells = self.cells.iter().rev().cloned().collect();
        let hlinks = self.hlinks.iter().rev().cloned().collect();
        let vlinks = self.vlinks.iter().rev().cloned().collect();
        let corners = self.corners.iter().rev().cloned().collect();
        HalfRule {
            height: self.height,
            width: self.width,
            cells,
            hlinks,
            vlinks,
            corners,
        }
    }

    fn rotated_180(&self) -> HalfRule {
        let cells = self
            .cells
            .iter()
            .rev()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
        let hlinks = self
            .hlinks
            .iter()
            .rev()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
        let vlinks = self
            .vlinks
            .iter()
            .rev()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
        let corners = self
            .corners
            .iter()
            .rev()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
        HalfRule {
            height: self.height,
            width: self.width,
            cells,
            hlinks,
            vlinks,
            corners,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rule {
    name: String,
    rule_in: HalfRule,
    rule_out: HalfRule,
}

impl Rule {
    pub fn new(name: &str, rule_str: &str) -> Rule {
        let mut parts = rule_str.trim().split("=>");
        let rule_in = HalfRule::new(parts.next().unwrap());
        let rule_out = HalfRule::new(parts.next().unwrap());
        assert!(parts.next().is_none());
        assert_eq!(rule_in.height, rule_out.height);
        assert_eq!(rule_in.width, rule_out.width);
        Rule {
            name: name.to_owned(),
            rule_in,
            rule_out,
        }
    }

    pub(super) fn rule_in(&self) -> &HalfRule {
        &self.rule_in
    }

    pub(super) fn rule_out(&self) -> &HalfRule {
        &self.rule_out
    }

    pub fn print(&self) {
        println!("Rule \"{}\":", self.name);
        let rule_in = &self.rule_in;
        let rule_out = &self.rule_out;
        for row in 0..(4 * rule_in.height + 1) {
            rule_in.full_print_row(row);
            print!(
                "  {}  ",
                if row == 2 * rule_in.height {
                    "=>"
                } else {
                    "  "
                }
            );
            rule_out.full_print_row(row);
            println!();
        }
    }

    fn rotated_90(&self) -> Rule {
        Rule {
            name: self.name.clone(),
            rule_in: self.rule_in.rotated_90(),
            rule_out: self.rule_out.rotated_90(),
        }
    }

    #[allow(dead_code)]
    fn reversed_lr(&self) -> Rule {
        Rule {
            name: self.name.clone(),
            rule_in: self.rule_in.reversed_lr(),
            rule_out: self.rule_out.reversed_lr(),
        }
    }

    fn reversed_ud(&self) -> Rule {
        Rule {
            name: self.name.clone(),
            rule_in: self.rule_in.reversed_ud(),
            rule_out: self.rule_out.reversed_ud(),
        }
    }

    fn rotated_180(&self) -> Rule {
        Rule {
            name: self.name.clone(),
            rule_in: self.rule_in.rotated_180(),
            rule_out: self.rule_out.rotated_180(),
        }
    }

    pub fn symmetries(&self) -> Vec<Rule> {
        let r180 = self.rotated_180();
        if *self != r180 {
            let r90 = self.rotated_90();
            let r270 = r90.rotated_180();
            let refl = self.reversed_ud();
            if refl == *self || refl == r90 || refl == r180 || refl == r270 {
                return vec![r180, r90, r270];
            }
            let refl2 = r90.reversed_ud();
            let refl3 = r180.reversed_ud();
            let refl4 = r270.reversed_ud();
            return vec![r90, r180, r270, refl, refl2, refl3, refl4];
        }
        let r90 = self.rotated_90();
        if *self != r90 {
            let refl = self.reversed_ud();
            if *self != refl {
                let refl2 = r90.reversed_ud();
                if *self != refl2 {
                    return vec![r90, refl, refl2];
                }
                return vec![r90];
            }
            return vec![r90];
        }
        let refl = self.reversed_ud();
        if *self == refl { vec![] } else { vec![refl] }
    }
}

use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum CellType {
    Empty,
    Zero,
    One,
    Two,
    Three,
}
use CellType::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum LinkType {
    LMaybe,
    Link,
    Unlink,
}
use LinkType::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum CornerType {
    CMaybe,
    CZero,
    COne,
    CTwo,
    Even,
    Less,
    Greater,
}
use CornerType::*;

fn full_print_row(
    height: usize,
    width: usize,
    cells: &[Vec<CellType>],
    hlinks: &[Vec<LinkType>],
    vlinks: &[Vec<LinkType>],
    corners: &[Vec<CornerType>],
    row: usize,
    trim_left_and_right: bool,
) {
    assert!(row <= 4 * height);
    let col_l = if trim_left_and_right { 1 } else { 0 };
    let col_r = width - if trim_left_and_right { 1 } else { 0 };
    if row % 4 == 0 {
        let row = row / 4;
        print!("+");
        for col in col_l..col_r {
            print!(
                "{}+",
                match hlinks[row][col] {
                    LMaybe => ".......",
                    Link => "=======",
                    Unlink => "       ",
                }
            );
        }
        return;
    }
    if row % 4 == 2 {
        let row = row / 4;
        let print_vlink = |col: usize| {
            print!(
                "{}",
                match vlinks[row][col] {
                    LMaybe => ".",
                    Link => "$",
                    Unlink => " ",
                }
            );
        };
        let mut first = true;
        for col in col_l..col_r {
            if first {
                print_vlink(col);
                first = false;
            }
            print!(
                "   {}   ",
                match cells[row][col] {
                    Empty => ' ',
                    Zero => '0',
                    One => '1',
                    Two => '2',
                    Three => '3',
                }
            );
            print_vlink(col + 1);
        }
        return;
    }
    let row = row / 2;
    let parity = row % 2;
    let row = row / 2;
    let print_vlink = |col: usize| {
        print!(
            "{}",
            match vlinks[row][col] {
                LMaybe => ".",
                Link => "$",
                Unlink => " ",
            }
        );
    };
    let mut first = true;
    for col in col_l..col_r {
        if first {
            print_vlink(col);
            first = false;
        }
        let to_char = |corner: CornerType| match corner {
            CMaybe => ' ',
            CZero => '0',
            COne => '1',
            CTwo => '2',
            Even => 'E',
            Less => 'L',
            Greater => 'G',
        };
        print!(
            " {}   {} ",
            to_char(corners[2 * row + parity][2 * col]),
            to_char(corners[2 * row + parity][2 * col + 1])
        );
        print_vlink(col + 1);
    }
}

pub struct Game {
    height: usize,
    width: usize,
    cells: Vec<Vec<CellType>>,
    hlinks: Vec<Vec<LinkType>>,
    vlinks: Vec<Vec<LinkType>>,
    corners: Vec<Vec<CornerType>>,
}

impl Game {
    pub fn new(mut width: usize, mut height: usize, task: &str) -> Game {
        assert!(width >= 5 && height >= 5);
        let mut cells = vec![vec![Empty; width]; height];
        {
            let mut row = 0;
            let mut col = 0;
            let next = |row: &mut usize, col: &mut usize| {
                *col += 1;
                if *col == width {
                    *col = 0;
                    *row += 1;
                }
            };
            for c in task.chars() {
                match c {
                    '0' => cells[row][col] = Zero,
                    '1' => cells[row][col] = One,
                    '2' => cells[row][col] = Two,
                    '3' => cells[row][col] = Three,
                    c => {
                        assert!(c.is_ascii_lowercase());
                        for _ in 0..(c as u8 - b'a') {
                            next(&mut row, &mut col);
                        }
                    }
                }
                next(&mut row, &mut col);
            }
            assert_eq!(row, height);
            assert_eq!(col, 0);
        }
        for crow in &mut cells {
            crow.insert(0, Empty);
            crow.push(Empty);
        }
        cells.insert(0, vec![Empty; width + 2]);
        cells.push(vec![Empty; width + 2]);
        width += 2;
        height += 2;
        let mut hlinks = vec![vec![LMaybe; width]; height + 1];
        let mut vlinks = vec![vec![LMaybe; width + 1]; height];
        let mut corners = vec![vec![CMaybe; 2 * width]; 2 * height];
        for row in 0..=height {
            hlinks[row][0] = Unlink;
            hlinks[row][width - 1] = Unlink;
            if row < height {
                vlinks[row][0] = Unlink;
                vlinks[row][width] = Unlink;
            }
        }
        for col in 0..=width {
            vlinks[0][col] = Unlink;
            vlinks[height - 1][col] = Unlink;
            if col < width {
                hlinks[0][col] = Unlink;
                hlinks[height][col] = Unlink;
            }
        }
        for row in 0..height {
            corners[2 * row][0] = CZero;
            corners[2 * row][2 * width - 1] = CZero;
            corners[2 * row + 1][0] = CZero;
            corners[2 * row + 1][2 * width - 1] = CZero;
        }
        for col in 0..width {
            corners[0][2 * col] = CZero;
            corners[2 * height - 1][2 * col] = CZero;
            corners[0][2 * col + 1] = CZero;
            corners[2 * height - 1][2 * col + 1] = CZero;
        }
        Game {
            width,
            height,
            cells,
            hlinks,
            vlinks,
            corners,
        }
    }

    pub fn print_cells(&self) {
        println!();
        print!("+");
        for _ in 1..(self.width - 1) {
            print!("--");
        }
        println!("-+");
        for row in 1..(self.height - 1) {
            print!("|");
            for col in 1..(self.width - 1) {
                print!(
                    " {}",
                    match self.cells[row][col] {
                        Empty => ' ',
                        Zero => '0',
                        One => '1',
                        Two => '2',
                        Three => '3',
                    }
                );
            }
            println!(" |");
        }
        print!("+");
        for _ in 1..(self.width - 1) {
            print!("--");
        }
        println!("-+");
        println!();
    }

    pub fn print_cells_and_links(&self) {
        let print_row_hlinks = |row: usize| {
            print!(" ");
            for col in 1..(self.width - 1) {
                print!(
                    "+{}",
                    match self.hlinks[row][col] {
                        LMaybe => "...",
                        Link => "===",
                        Unlink => "   ",
                    }
                );
            }
            println!("+");
        };
        println!();
        for row in 1..(self.height - 1) {
            if row == 1 {
                print_row_hlinks(1);
            }
            let print_vlink = |col: usize| {
                print!(
                    "{}",
                    match self.vlinks[row][col] {
                        LMaybe => ".",
                        Link => "|",
                        Unlink => " ",
                    }
                );
            };
            print!(" ");
            for col in 1..(self.width - 1) {
                if col == 1 {
                    print_vlink(1);
                }
                print!(
                    " {} ",
                    match self.cells[row][col] {
                        Empty => ' ',
                        Zero => '0',
                        One => '1',
                        Two => '2',
                        Three => '3',
                    }
                );
                print_vlink(col + 1);
            }
            println!();
            print_row_hlinks(row + 1);
        }
        println!();
    }

    pub fn full_print(&self) {
        for row in 4..(4 * self.height - 3) {
            full_print_row(
                self.height,
                self.width,
                &self.cells,
                &self.hlinks,
                &self.vlinks,
                &self.corners,
                row,
                true,
            );
            println!();
        }
    }
}

trait Mask {
    fn to_mask(&self) -> u8;
    fn from_mask(mask: u8) -> Self;
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mask = self.to_mask();
        let other_mask = other.to_mask();
        if mask == other_mask {
            return Some(Ordering::Equal);
        }
        if mask & other_mask == mask {
            return Some(Ordering::Less);
        }
        if mask & other_mask == other_mask {
            return Some(Ordering::Greater);
        }
        None
    }
}

impl Mask for CellType {
    fn to_mask(&self) -> u8 {
        match self {
            Empty => 15,
            Zero => 1,
            One => 2,
            Two => 4,
            Three => 8,
        }
    }

    fn from_mask(mask: u8) -> Self {
        match mask {
            15 => Empty,
            1 => Zero,
            2 => One,
            4 => Two,
            8 => Three,
            _ => unreachable!(),
        }
    }
}

impl Mask for LinkType {
    fn to_mask(&self) -> u8 {
        match self {
            LMaybe => 3,
            Link => 1,
            Unlink => 2,
        }
    }

    fn from_mask(mask: u8) -> Self {
        match mask {
            3 => LMaybe,
            1 => Link,
            2 => Unlink,
            _ => unreachable!(),
        }
    }
}

impl Mask for CornerType {
    fn to_mask(&self) -> u8 {
        match self {
            CMaybe => 7,
            CZero => 1,
            COne => 2,
            CTwo => 4,
            Even => 5,
            Less => 3,
            Greater => 6,
        }
    }

    fn from_mask(mask: u8) -> Self {
        match mask {
            7 => CMaybe,
            1 => CZero,
            2 => COne,
            4 => CTwo,
            5 => Even,
            3 => Less,
            6 => Greater,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for CellType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Mask::partial_cmp(self, other)
    }
}

impl PartialOrd for LinkType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Mask::partial_cmp(self, other)
    }
}

impl PartialOrd for CornerType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Mask::partial_cmp(self, other)
    }
}

trait Gcd {
    fn gcd(&self, other: &Self) -> Self;
}

impl<T: Mask> Gcd for T {
    fn gcd(&self, other: &Self) -> Self {
        T::from_mask(self.to_mask() & other.to_mask())
    }
}

struct HalfRule {
    height: usize,
    width: usize,
    cells: Vec<Vec<CellType>>,
    hlinks: Vec<Vec<LinkType>>,
    vlinks: Vec<Vec<LinkType>>,
    corners: Vec<Vec<CornerType>>,
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
                    _ => unreachable!(),
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
                    _ => unreachable!(),
                };
            }
        }
        for row in 0..height {
            for col in 0..=width {
                vlinks[row][col] = match lines[row * 4 + 2][col * 8] as char {
                    '.' => LMaybe,
                    '$' => Link,
                    ' ' => Unlink,
                    _ => unreachable!(),
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
                    _ => unreachable!(),
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
}

pub struct Rule {
    name: String,
    rule_in: HalfRule,
    rule_out: HalfRule,
}

impl Rule {
    pub fn new(name: &str, rule_str: &str) -> Rule {
        let mut parts = rule_str.trim().split("=>");
        let rule = Rule {
            name: name.to_owned(),
            rule_in: HalfRule::new(parts.next().unwrap()),
            rule_out: HalfRule::new(parts.next().unwrap()),
        };
        assert!(parts.next().is_none());
        assert_eq!(rule.rule_in.height, rule.rule_out.height);
        assert_eq!(rule.rule_in.width, rule.rule_out.width);
        rule
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
}

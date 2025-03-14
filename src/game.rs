mod element;
mod rule;
use element::*;
pub use rule::*;

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

use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum CellType {
    Empty,
    Zero,
    One,
    Two,
    Three,
}
pub use CellType::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum LinkType {
    LMaybe,
    Link,
    Unlink,
}
pub use LinkType::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum CornerType {
    CMaybe,
    CZero,
    COne,
    CTwo,
    Even,
    Less,
    Greater,
}
pub use CornerType::*;

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

pub trait Gcd {
    fn gcd(&self, other: &Self) -> Self;
}

impl<T: Mask> Gcd for T {
    fn gcd(&self, other: &Self) -> Self {
        T::from_mask(self.to_mask() & other.to_mask())
    }
}

pub fn full_print_row(
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

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

use std::fmt::{self, Display};
use num::{CheckedAdd, CheckedSub};

use super::directions::*;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position<T> (pub T, pub T);

impl<T> Position<T> {
    pub fn new(i: T, j: T) -> Self {
        Position(i, j)
    }
}

impl<T: Display> fmt::Display for Position<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub trait StepMove<Direction> where Self: Sized {
    fn step(&self, d: Direction) -> Option<Self>;
}

impl<T: Clone + CheckedAdd + CheckedSub + TryFrom<u32>>
StepMove<Direction4> for Position<T> {
    fn step(&self, d: Direction4) -> Option<Position<T>> {
        let one = if let Ok(one) = T::try_from(1) { one } else { return None };    
        let d = match d {
            Direction4::North =>
                Position(self.0.checked_sub(&one)?, self.1.clone()),
            Direction4::East =>
                Position(self.0.clone(), self.1.checked_add(&one)?),
            Direction4::South =>
                Position(self.0.checked_add(&one)?, self.1.clone()),
            Direction4::West =>
                Position(self.0.clone(), self.1.checked_sub(&one)?)
        };
        Some (d)
    }
}

impl<T: Clone + CheckedAdd + CheckedSub + TryFrom<u32>>
StepMove<Direction8> for Position<T> {
    fn step(&self, d: Direction8) -> Option<Position<T>> {
        let one = if let Ok(one) = T::try_from(1) { one } else { return None };
        let d = match d {
            Direction8::North =>
                Position(self.0.checked_sub(&one)?, self.1.clone()),
            Direction8::NorthEast =>
                Position(self.0.checked_sub(&one)?, self.1.checked_add(&one)?),
            Direction8::East =>
                Position(self.0.clone(), self.1.checked_add(&one)?),
            Direction8::SouthEast =>
                Position(self.0.checked_add(&one)?, self.1.checked_add(&one)?),
            Direction8::South =>
                Position(self.0.checked_add(&one)?, self.1.clone()),
            Direction8::SouthWest =>
                Position(self.0.checked_add(&one)?, self.1.checked_sub(&one)?),
            Direction8::West =>
                Position(self.0.clone(), self.1.checked_sub(&one)?),
            Direction8::NorthWest =>
                Position(self.0.checked_sub(&one)?, self.1.checked_sub(&one)?)
        };
        Some (d)
    }
}

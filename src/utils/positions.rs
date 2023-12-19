use std::fmt::{self, Display};
use super::directions::*;

#[derive (Debug, Clone, PartialEq, Eq)]
pub struct Position<T> (pub T, pub T);

impl<T: Display> fmt::Display for Position<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub trait StepMove<Direction> where Self: Sized {
    fn step(&self, d: Direction) -> Option<Self>;
}

impl<T> StepMove<Direction4> for Position<T> {
    fn step(&self, d: Direction4) -> Option<Position<T>> {
        let d = match d {
            Direction4::North => Position(self.0.checked_sub(1)?, self.1),
            Direction4::East => Position(self.0, self.1.checked_add(1)?),
            Direction4::South => Position(self.0.checked_add(1)?, self.1),
            Direction4::West => Position(self.0, self.1.checked_sub(1)?)
        };
        Some (d)
    }
}

impl<T> StepMove<Direction8> for Position<T> {
    fn step(&self, d: Direction8) -> Option<Position<T>> {
        let d = match d {
            Direction8::North => Position(self.0.checked_sub(1)?, self.1),
            Direction8::NorthEast => Position(self.0.checked_sub(1)?, self.1.checked_add(1)?),
            Direction8::East => Position(self.0, self.1.checked_add(1)?),
            Direction8::SouthEast => Position(self.0.checked_add(1)?, self.1.checked_add(1)?),
            Direction8::South => Position(self.0.checked_add(1)?, self.1),
            Direction8::SouthWest => Position(self.0.checked_add(1)?, self.1.checked_sub(1)?),
            Direction8::West => Position(self.0, self.1.checked_sub(1)?),
            Direction8::NorthWest => Position(self.0.checked_sub(1)?, self.1.checked_sub(1)?)
        };
        Some (d)
    }
}

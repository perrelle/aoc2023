use std::{fmt, ops::{Index, IndexMut}};

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction4 {
    North, East, South, West
}

impl fmt::Display for Direction4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }    
}

impl Direction4 {
    pub const ALL : [Direction4 ; 4] = [
        Direction4::North,
        Direction4::East,
        Direction4::South,
        Direction4::West,
    ];

    pub fn invert(self) -> Self {
        match self {
            Direction4::North => Direction4::South,
            Direction4::East => Direction4::West,
            Direction4::South => Direction4::North,
            Direction4::West => Direction4::East
        }
    }
}

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction8 {
    North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest
}

impl fmt::Display for Direction8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }    
}

impl Direction8 {
    pub const ALL : [Direction8 ; 8] = [
        Direction8::North,
        Direction8::NorthEast,
        Direction8::East,
        Direction8::SouthEast,
        Direction8::South,
        Direction8::SouthWest,
        Direction8::West,
        Direction8::NorthWest
    ];

    pub fn invert(self) -> Self {
        match self {
            Direction8::North => Direction8::South,
            Direction8::NorthEast => Direction8::SouthWest,
            Direction8::East => Direction8::West,
            Direction8::SouthEast => Direction8::NorthWest,
            Direction8::South => Direction8::North,
            Direction8::SouthWest => Direction8::NorthEast,
            Direction8::West => Direction8::East,
            Direction8::NorthWest => Direction8::SouthEast
        }
    }

    pub fn next(&self) -> Direction8 {
        match self {
            Direction8::North => Direction8::NorthEast,
            Direction8::NorthEast => Direction8::East,
            Direction8::East => Direction8::SouthEast,
            Direction8::SouthEast => Direction8::South,
            Direction8::South => Direction8::SouthWest,
            Direction8::SouthWest => Direction8::West,
            Direction8::West => Direction8::NorthWest,
            Direction8::NorthWest => Direction8::North
        }
    }
}

impl From<Direction4> for Direction8 {
    fn from(d: Direction4) -> Self {
        match d {
            Direction4::North => Direction8::North,
            Direction4::East => Direction8::East,
            Direction4::South => Direction8::South,
            Direction4::West => Direction8::West,
        }        
    }
}

pub struct Direction4Map<T> ([T ; 4]);

impl Direction4Map<bool> {
    pub fn count(&self) -> usize {
        usize::from(self.0[0]) +
        usize::from(self.0[1]) +
        usize::from(self.0[2]) +
        usize::from(self.0[3])
    }
}

impl<T: Default> Default for Direction4Map<T> {
    fn default() -> Self {
        Direction4Map(std::array::from_fn(|_| T::default()))
    }
}

impl<T> Index<Direction4> for Direction4Map<T> {
    type Output = T;

    fn index(&self, index: Direction4) -> &T {
        match index {
            Direction4::North => &self.0[0],
            Direction4::South => &self.0[1],
            Direction4::West  => &self.0[2],
            Direction4::East  => &self.0[3]
        }
    }
}

impl<T> IndexMut<Direction4> for Direction4Map<T> {
    fn index_mut(&mut self, index: Direction4) -> &mut T {
        match index {
            Direction4::North => &mut self.0[0],
            Direction4::South => &mut self.0[1],
            Direction4::West  => &mut self.0[2],
            Direction4::East  => &mut self.0[3]
        }
    }
}

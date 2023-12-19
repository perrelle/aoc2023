use std::fmt;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction4 {
    North, East, South, West
}

impl fmt::Display for Direction4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
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
        write!(f, "{}", self.to_string())
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

use std::{fmt::{self, Display}, ops::{Index, IndexMut}};
use super::positions::*;
use array2d::Array2D;

#[derive (Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> (pub Array2D<T>);

impl<T,I> Grid<T> {
    pub fn is_index_valid(&self, p: &Position<I>) -> bool {
        p.0 >= 0 &&
        p.1 >= 0 &&
        p.0 < self.0.num_rows() &&
        p.1 < self.0.num_columns()
    }
}

impl<T,I> Index<&Position<I>> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Position<I>) -> &Self::Output {
        &self.0[(index.0, index.1)]
    }
}

impl<T,I> IndexMut<&Position<I>> for Grid<T> {
    fn index_mut(&mut self, index: &Position<I>) -> &mut Self::Output {
        &mut self.0[(index.0, index.1)]
    }
}

pub trait ConvertibleToChar {
    fn to_char(&self) -> char;
}

impl<T: ConvertibleToChar> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_it in self.0.rows_iter() {
            for cell in row_it {
                let c = cell.to_char();
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok (())
    }
}

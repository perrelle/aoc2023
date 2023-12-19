use std::{fmt::{self, Display}, ops::{Index, IndexMut}};
use super::positions::*;
use array2d::Array2D;

#[derive (Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> (pub Array2D<T>);

impl<T> Grid<T> {
    pub fn is_index_valid<I>(&self, p: &Position<I>) -> bool 
            where I: Copy + TryInto<usize> {
        let i: Result<usize,_> = p.0.try_into();
        let j: Result<usize,_> = p.1.try_into();
        if let (Ok(i), Ok(j)) = (i,j) {
            i < self.0.num_rows() && j < self.0.num_columns()
        }
        else {
            false
        }
    }

    pub fn filled_default(num_rows: usize, num_columns: usize) -> Self
            where T: Default {
        Grid (Array2D::filled_by_row_major(T::default, num_rows, num_columns))
    }
}

impl<T,I:Copy + TryInto<usize> + fmt::Debug> Index<Position<I>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position<I>) -> &Self::Output {
        let i = index.0.try_into();
        let j = index.1.try_into();
        if let (Ok(i), Ok(j)) = (i,j) {
            &self.0[(i, j)]
        }
        else {
            panic!()
        }
    }
}

impl<T,I:Copy + TryInto<usize> + fmt::Debug> IndexMut<Position<I>> for Grid<T> {
    fn index_mut(&mut self, index: Position<I>) -> &mut Self::Output {
        let i = index.0.try_into();
        let j = index.1.try_into();
        if let (Ok(i), Ok(j)) = (i,j) {
            &mut self.0[(i, j)]
        }
        else {
            panic!()
        }
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

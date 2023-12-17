use std::fmt;

use array2d::Array2D;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell { Ash, Rocks }

#[derive (Debug, Clone)]
pub struct Grid(Array2D<Cell>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_it in self.0.rows_iter() {
            for cell in row_it {
                let c = match cell {
                    Cell::Ash => '.',
                    Cell::Rocks => '#',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok (())
    }
}

type Input = Vec<Grid>;

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    };

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = one_of(".#")(input)?;
        let p = match c {
            '.' => Cell::Ash,
            '#' => Cell::Rocks,
            _ => panic!()
        };

        Ok ((input, p))
    }

    fn grid(input: &str) -> IResult<&str, Grid> {
        map(separated_list1(line_ending, many1(cell)), |v|
            Grid(Array2D::from_rows(&v).unwrap()))
            (input)
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        all_consuming(terminated(
            separated_list1(multispace1, grid),
            multispace0))(input)
    }
}

fn is_horizontaly_symetric(grid: &Grid, k: usize, mut smudges: u32) -> bool {
    let l = if 2*k >= grid.0.num_rows() {
        2*k - grid.0.num_rows() 
    }
    else {
        0
    };

    for i in l..k {
        for j in 0..(grid.0.num_columns()) {
            if grid.0[(i,j)] != grid.0[(2*k-i-1,j)] {
                if smudges > 0 {
                    smudges -= 1;
                }
                else {
                    return false;
                }
            }
        }
    }

    smudges == 0
}

fn is_vertically_symetric(grid: &Grid, k: usize, mut smudges: u32) -> bool {
    let l = if 2*k >= grid.0.num_columns() {
        2*k - grid.0.num_columns() 
    }
    else {
        0
    };

    for i in 0..(grid.0.num_rows()) {
        for j in l..k {
            if grid.0[(i,j)] != grid.0[(i,2*k-j-1)] {
                if smudges > 0 {
                    smudges -= 1;
                }
                else {
                    return false;
                }
            }
        }
    }

    smudges == 0
}

fn score(grid: &Grid, smudges: u32) -> u32 {
    for k in 1..(grid.0.num_rows()) {
        if is_horizontaly_symetric(grid, k, smudges) {
            return 100 * k as u32;
        }
    }

    for k in 1..(grid.0.num_columns()) {
        if is_vertically_symetric(grid, k, smudges) {
            return k as u32;
        }
    }

    panic!();
}

pub fn solve(input: &str) -> (u32, u32) {
    let (_,input) = parser::parse(input).unwrap();

    let mut result1 = 0;
    let mut result2 = 0;

    for grid in input {
        result1 += score(&grid, 0);
        result2 += score(&grid, 1);
    }
    
    (result1, result2)
}

#[test]
fn day13_example() {
    let solution = solve(include_str!("../inputs/day13-example"));
    assert_eq!(solution, (405, 400));
}

#[test]
fn day13_input() {
    let solution = solve(include_str!("../inputs/day13-input"));
    assert_eq!(solution, (29130, 33438));
}

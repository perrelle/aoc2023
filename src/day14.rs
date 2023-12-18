use std::{fmt, collections::HashMap};

use array2d::Array2D;

#[derive (Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell { Empty, RoundedRock, CubeRock }

#[derive (Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid(Array2D<Cell>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_it in self.0.rows_iter() {
            for cell in row_it {
                let c = match cell {
                    Cell::Empty => '.',
                    Cell::RoundedRock => 'O',
                    Cell::CubeRock => '#',
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
        let (input, c) = one_of(".O#")(input)?;
        let p = match c {
            '.' => Cell::Empty,
            'O' => Cell::RoundedRock,
            '#' => Cell::CubeRock,
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

#[allow(clippy::needless_range_loop)]
fn tilt_north(grid: &mut Grid) {
    let mut last_free_space = vec![0 ; grid.0.num_columns()];

    for i in 0..(grid.0.num_rows()) {
        for j in 0..(grid.0.num_columns()) {
            match grid.0[(i,j)] {
                Cell::Empty => (),
                Cell::CubeRock => {
                    last_free_space[j] = i + 1;
                }
                Cell::RoundedRock => {
                    let k = last_free_space[j];
                    if k < i {
                        grid.0[(i,j)] = Cell::Empty;
                        grid.0[(k,j)] = Cell::RoundedRock;
                    }
                    last_free_space[j] = k + 1;
                }
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn tilt_south(grid: &mut Grid) {
    let mut last_free_space = vec![grid.0.num_rows() - 1 ; grid.0.num_columns()];

    for i in (0..(grid.0.num_rows())).rev() {
        for j in 0..(grid.0.num_columns()) {
            match grid.0[(i,j)] {
                Cell::Empty => (),
                Cell::CubeRock => {
                    last_free_space[j] = i.saturating_sub(1);
                }
                Cell::RoundedRock => {
                    let k = last_free_space[j];
                    if k > i {
                        grid.0[(i,j)] = Cell::Empty;
                        grid.0[(k,j)] = Cell::RoundedRock;
                    }
                    last_free_space[j] = k.saturating_sub(1);
                }
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn tilt_west(grid: &mut Grid) {
    let mut last_free_space = vec![0 ; grid.0.num_rows()];

    for j in 0..(grid.0.num_columns()) {
        for i in 0..(grid.0.num_rows()) {
            match grid.0[(i,j)] {
                Cell::Empty => (),
                Cell::CubeRock => {
                    last_free_space[i] = j + 1;
                }
                Cell::RoundedRock => {
                    let k = last_free_space[i];
                    if k < j {
                        grid.0[(i,j)] = Cell::Empty;
                        grid.0[(i,k)] = Cell::RoundedRock;
                    }
                    last_free_space[i] = k + 1;
                }
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn tilt_east(grid: &mut Grid) {
    let mut last_free_space = vec![grid.0.num_columns() - 1 ; grid.0.num_rows()];

    for j in (0..(grid.0.num_columns())).rev() {
        for i in 0..(grid.0.num_rows()) {
            match grid.0[(i,j)] {
                Cell::Empty => (),
                Cell::CubeRock => {
                    last_free_space[i] = j.saturating_sub(1);
                }
                Cell::RoundedRock => {
                    let k = last_free_space[i];
                    if k > j {
                        grid.0[(i,j)] = Cell::Empty;
                        grid.0[(i,k)] = Cell::RoundedRock;
                    }
                    last_free_space[i] = k.saturating_sub(1);
                }
            }
        }
    }
}

fn score(grid: &Grid) -> usize {
    let height = grid.0.num_columns();
    let mut result = 0;
    for (i,row_iter) in grid.0.rows_iter().enumerate() {
        for &cell in row_iter {
            if cell == Cell::RoundedRock {
                result += height - i;
            }
        }
    }

    result
}

fn solve_grid1(mut grid: Grid) -> usize {
    tilt_north(&mut grid);
    score(&grid)
}

fn solve_grid2(mut grid: Grid) -> usize {
    let mut cache = HashMap::new();
    let end_cycle = 1000000000;

    let mut i = 0;
    while i < end_cycle {
        cache.insert(grid.clone(), i);
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
        i += 1;

        if let Some(k) = cache.get(&grid) {
            let period = k - i;
            i += ((end_cycle - i) / period) * period;
        }
        
    }

    score(&grid)
}

pub fn solve(input: &str) -> (usize, usize) {
    let (_,input) = parser::parse(input).unwrap();
    let mut result1 = 0;
    let mut result2 = 0;

    for grid in input {
        result1 += solve_grid1(grid.clone());
        result2 += solve_grid2(grid);
    }
    
    (result1, result2)
}

#[test]
fn day14_example() {
    let solution = solve(include_str!("../inputs/day14-example"));
    assert_eq!(solution, (136, 64));
}

#[test]
fn day14_input() {
    let solution = solve(include_str!("../inputs/day14-input"));
    assert_eq!(solution, (107430, 96317));
}

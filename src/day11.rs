use std::{fmt, ops};

use array2d::Array2D;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Star,
    Space
}

impl Cell {
    pub fn is_space(&self) -> bool {
        *self == Self::Space
    }
}

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(usize, usize);

impl Position {
    fn distance(d1: &Self, d2: &Self) -> u32 {
        let dr = if d1.0 > d2.0 { d1.0 - d2.0 } else { d2.0 - d1.0 };
        let dc = if d1.1 > d2.1 { d1.1 - d2.1 } else { d2.1 - d1.1 };
        dr as u32 + dc as u32
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

pub struct Grid (Array2D<Cell>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_it in self.0.rows_iter() {
            for cell in row_it {
                let c = match cell {
                    Cell::Space => '.',
                    Cell::Star => '#',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok (())
    }
}

impl ops::Index<Position> for Grid {
    type Output = Cell;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[(index.0,index.1)]
    }
}

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
            '.' => Cell::Space,
            '#' => Cell::Star,
            _ => panic!()
        };

        Ok ((input, p))
    }

    fn grid(input: &str) -> IResult<&str, Grid> {
        map(
                separated_list1(line_ending, many1(cell)),
                |v| Grid(Array2D::from_rows(&v).unwrap()))
            (input)
    }

    pub fn parse(input: &str) -> IResult<&str, Grid> {
        all_consuming(terminated(grid, multispace0))(input)
    }
}

fn empty_rows(grid: &Grid) -> Vec<usize> {
    let mut result = Vec::new();
    for (i,mut row_it) in grid.0.rows_iter().enumerate() {
        if row_it.all(Cell::is_space) {
            result.push(i);
        }
    }
    result
}

fn empty_cols(grid: &Grid) -> Vec<usize> {
    let mut result = Vec::new();
    for (i,mut column_it) in grid.0.columns_iter().enumerate() {
        if column_it.all(Cell::is_space) {
            result.push(i);
        }
    }
    result
}

fn expand(grid: &Grid) -> Grid {
    let empty_cols = empty_cols(grid);
    let empty_rows = empty_rows(grid);
    let num_rows = grid.0.num_rows() + empty_rows.len();
    let num_columns = grid.0.num_columns() + empty_cols.len();
    let mut result = Array2D::filled_with(Cell::Space, num_rows, num_columns);
    let mut skipped_rows = 0;

    for (i,row_it) in grid.0.rows_iter().enumerate() {
        if empty_rows.contains(&i) {
            skipped_rows += 1;
            continue;
        }

        let mut skipped_colums = 0;
        for (j, cell) in row_it.enumerate() {
            if empty_cols.contains(&j) {
                skipped_colums += 1;
                continue;
            }

            if *cell == Cell::Star {
                result[(i+skipped_rows, j+skipped_colums)] = Cell::Star
            }
        }
    }

    Grid(result)
}

fn stars(grid: &Grid) -> Vec<Position> {
    let mut result = Vec::new();

    for (i,row_it) in grid.0.rows_iter().enumerate() {
        for (j, cell) in row_it.enumerate() {
            if *cell == Cell::Star {
                result.push(Position(i,j));
            }
        }
    }

    result
}

fn add_star_distances(stars: Vec<Position>) -> u64 {
    let mut result = 0;

    let mut it1 = stars.iter();
    while let Some(s1) = it1.next() {
        let it2 = it1.clone();
        for s2 in it2 {
            let d = Position::distance(s1, s2);
            result += d as u64;
        }
    }

    result
}

fn solve_part1(grid: &Grid) -> u64 {
    let grid = expand(grid);
    let stars = stars(&grid);
    add_star_distances(stars)
}

fn translate_position(
    p: &Position,
    empty_cols: &[usize],
    empty_rows: &[usize],
    expansion: usize) -> Position {
    let mut r = *p;

    for i in 0..p.0 {
        if empty_rows.contains(&i) {
            r.0 += expansion - 1;
        }
    }

    for j in 0..p.1 {
        if empty_cols.contains(&j) {
            r.1 += expansion - 1;
        }
    }

    r
}

fn solve_part2(grid: &Grid, expansion: usize) -> u64 {
    let mut stars = stars(grid);
    let empty_cols = empty_cols(grid);
    let empty_rows = empty_rows(grid);
    stars.iter_mut().for_each(|p| {
        *p = translate_position(p, &empty_cols, &empty_rows, expansion)
    });
    add_star_distances(stars)
}

pub fn solve(input: &str, expansion: usize) -> (u64, u64) {
    let (_,grid) = parser::parse(input).unwrap();
    (solve_part1(&grid), solve_part2(&grid, expansion))
}

#[test]
fn day11_example() {
  let input =include_str!("../inputs/day11-example");
  let solution = solve(&input, 10);
  assert_eq!(solution, (374, 1030));
  let solution = solve(&input, 100);
  assert_eq!(solution, (374, 8410));
}

#[test]
fn day11_input() {
  let solution = solve(&include_str!("../inputs/day11-input"), 1000000);
  assert_eq!(solution, (9403026, 543018317006));
}

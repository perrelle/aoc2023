use std::{fmt, ops::{Index, IndexMut}};

use array2d::Array2D;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
enum Direction { Up, Down, Left, Right }

#[derive (Debug, Clone, PartialEq, Eq)]
struct Position (usize, usize);

impl Position {
    fn step(&self, d: Direction) -> Option<Self> {
        let p = match d {
            Direction::Up   => Position (self.0.checked_sub(1)?, self.1),
            Direction::Down  => Position (self.0.checked_add(1)?, self.1),
            Direction::Left  => Position (self.0, self.1.checked_sub(1)?),
            Direction::Right => Position (self.0, self.1.checked_add(1)?)
        };
        Some (p)
    }
}

#[derive (Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell { Empty, Mirror, AntiMirror, HSplitter, VSplitter }

#[derive (Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid(Array2D<Cell>);

impl Grid {
    fn is_index_valid(&self, p: &Position) -> bool {
        p.0 < self.0.num_rows() &&
        p.1 < self.0.num_columns()
    }
}

impl Index<&Position> for Grid {
    type Output = Cell;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.0[(index.0, index.1)]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_it in self.0.rows_iter() {
            for cell in row_it {
                let c = match cell {
                    Cell::Empty => '.',
                    Cell::Mirror => '/',
                    Cell::AntiMirror => '\\',
                    Cell::HSplitter => '—',
                    Cell::VSplitter => '|',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok (())
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
        let (input, c) = one_of("./\\-|")(input)?;
        let p = match c {
            '.' => Cell::Empty,
            '/' => Cell::Mirror,
            '\\' => Cell::AntiMirror,
            '-' => Cell::HSplitter,
            '|' => Cell::VSplitter,
            _ => panic!()
        };

        Ok ((input, p))
    }

    fn grid(input: &str) -> IResult<&str, Grid> {
        map(separated_list1(line_ending, many1(cell)), |v|
            Grid(Array2D::from_rows(&v).unwrap()))
            (input)
    }

    pub fn parse(input: &str) -> IResult<&str, Grid> {
        all_consuming(terminated(grid, multispace0))(input)
    }
}

struct DirectionMap<T> ([T ; 4]);

impl DirectionMap<bool> {
    fn count(&self) -> usize {
        usize::from(self.0[0]) +
        usize::from(self.0[1]) +
        usize::from(self.0[2]) +
        usize::from(self.0[3])
    }
}

impl<T: Default> Default for DirectionMap<T> {
    fn default() -> Self {
        DirectionMap(std::array::from_fn(|_| T::default()))
    }
}

impl<T> Index<Direction> for DirectionMap<T> {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Up   => &self.0[0],
            Direction::Down  => &self.0[1],
            Direction::Left  => &self.0[2],
            Direction::Right => &self.0[3]
        }
    }
}

impl<T> IndexMut<Direction> for DirectionMap<T> {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::Up   => &mut self.0[0],
            Direction::Down  => &mut self.0[1],
            Direction::Left  => &mut self.0[2],
            Direction::Right => &mut self.0[3]
        }        
    }
}

struct LightMap (Array2D<DirectionMap<bool>>);

impl Index<&Position> for LightMap {
    type Output = DirectionMap<bool>;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.0[(index.0, index.1)]
    }
}

impl IndexMut<&Position> for LightMap {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.0[(index.0, index.1)]
    }
}

impl LightMap {
    fn new(rows: usize, columns: usize) -> Self {
        LightMap (Array2D::filled_by_row_major(
            DirectionMap::default,
            rows,
            columns))
    }
}

impl fmt::Display for LightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_it in self.0.rows_iter() {
            for cell in row_it {
                let count = cell.count();
                let c = if count > 0 { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok (())
    }
}

fn mirror(d: Direction, anti: bool) -> Direction {
    if anti {
        match d {
            Direction::Up   => Direction::Left,
            Direction::Down  => Direction::Right,
            Direction::Left  => Direction::Up,
            Direction::Right => Direction::Down
        }
    }
    else {
        match d {
            Direction::Up   => Direction::Right,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Down,
            Direction::Right => Direction::Up
        }
    }
}

fn fill_light(grid: &Grid, light: &mut LightMap, p: Position, d: Direction) {
    if !grid.is_index_valid(&p) || light[&p][d] {
        return;
    }

    light[&p][d] = true;
    let cell = grid[&p];

    match cell {
        Cell::Empty => {
            if let Some(p) = p.step(d) {
                fill_light(grid, light, p, d)
            }
        },
        Cell::Mirror | Cell::AntiMirror => {
            let d = mirror(d, cell == Cell::AntiMirror);
            if let Some(p) = p.step(d) {
                fill_light(grid, light, p, d)
            }
        },
        Cell::HSplitter => {
            if d == Direction::Left || d == Direction::Right {
                if let Some(p) = p.step(d) {
                    fill_light(grid, light, p, d)
                }    
            }
            else {
                let d1 = Direction::Left;
                let d2 = Direction::Right;
                if let Some(p) = p.step(d1) {
                    fill_light(grid, light, p, d1)
                }    
                if let Some(p) = p.step(d2) {
                    fill_light(grid, light, p, d2)
                }    
            }
        },
        Cell::VSplitter => {
            if d == Direction::Up || d == Direction::Down {
                if let Some(p) = p.step(d) {
                    fill_light(grid, light, p, d)
                }    
            }
            else {
                let d1 = Direction::Up;
                let d2 = Direction::Down;
                if let Some(p) = p.step(d1) {
                    fill_light(grid, light, p, d1)
                }    
                if let Some(p) = p.step(d2) {
                    fill_light(grid, light, p, d2)
                }    
            }
        }
    }
}

fn score(light: LightMap) -> u32 {
    let mut result = 0;
    for l in light.0.elements_row_major_iter() {
        if l.count() > 0 {
            result += 1;
        }
    }
    result
}

pub fn solve1(grid: &Grid) -> u32 {
    let mut light = LightMap::new(grid.0.num_rows(), grid.0.num_columns());
    fill_light(grid, &mut light, Position(0,0), Direction::Right);
    score(light)
}

pub fn solve2(grid: &Grid) -> u32 {
    let mut best_score = 0;
    let mut one_fill = |p: Position, d: Direction| {
        let mut light = LightMap::new(grid.0.num_rows(), grid.0.num_columns());
        fill_light(grid, &mut light, p, d);
        let s = score(light);
        if s > best_score {
            best_score = s;
        }
    };

    let height = grid.0.num_rows();
    let width = grid.0.num_columns();

    for i in 0..height {
        one_fill(Position(i,0), Direction::Left);
        one_fill(Position(i,width-1), Direction::Right);
    }

    for j in 0..width {
        one_fill(Position(0,j), Direction::Down);
        one_fill(Position(height-1,j), Direction::Up);
    }
    

    best_score
}

pub fn solve(input: &str) -> (u32, u32) {
    let (_,grid) = parser::parse(input).unwrap();
    (solve1(&grid), solve2(&grid))
}

#[test]
fn day16_example() {
    let solution = solve(include_str!("../inputs/day16-example"));
    assert_eq!(solution, (46, 51));
}

#[test]
fn day16_input() {
    let solution = solve(include_str!("../inputs/day16-input"));
    assert_eq!(solution, (7307, 7635));
}

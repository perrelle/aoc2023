use crate::utils::{*, directions::*, positions::*, grids::*};
use array2d::Array2D;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell { Empty, Mirror, AntiMirror, HSplitter, VSplitter }

impl ConvertibleToChar for Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Mirror => '/',
            Cell::AntiMirror => '\\',
            Cell::HSplitter => '—',
            Cell::VSplitter => '|',
        }
    }
}

type Grid = grids::Grid<Cell>;
type Position = positions::Position<usize>;
type Direction = directions::Direction4;

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


type LightMap = grids::Grid<Direction4Map<bool>>;

impl ConvertibleToChar for Direction4Map<bool> {
    fn to_char(&self) -> char {
        let count = self.count();
        if count > 0 { '#' } else { '.' }
    }
}


fn mirror(d: Direction, anti: bool) -> Direction {
    if anti {
        match d {
            Direction4::North   => Direction4::West,
            Direction4::South  => Direction4::East,
            Direction4::West  => Direction4::North,
            Direction4::East => Direction4::South
        }
    }
    else {
        match d {
            Direction4::North   => Direction4::East,
            Direction4::South  => Direction4::West,
            Direction4::West  => Direction4::South,
            Direction4::East => Direction4::North
        }
    }
}

fn fill_light(grid: &Grid, light: &mut LightMap, p: Position, d: Direction) {
    if !grid.is_index_valid(&p) || light[p][d] {
        return;
    }

    light[p][d] = true;
    let cell = grid[p];

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
            if d == Direction::West || d == Direction::East {
                if let Some(p) = p.step(d) {
                    fill_light(grid, light, p, d)
                }    
            }
            else {
                let d1 = Direction::West;
                let d2 = Direction::East;
                if let Some(p) = p.step(d1) {
                    fill_light(grid, light, p, d1)
                }    
                if let Some(p) = p.step(d2) {
                    fill_light(grid, light, p, d2)
                }    
            }
        },
        Cell::VSplitter => {
            if d == Direction::North || d == Direction::South {
                if let Some(p) = p.step(d) {
                    fill_light(grid, light, p, d)
                }    
            }
            else {
                let d1 = Direction::North;
                let d2 = Direction::South;
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
    let mut light =
        LightMap::filled_default(grid.0.num_rows(), grid.0.num_columns());
    fill_light(grid, &mut light, Position(0,0), Direction::East);
    score(light)
}

pub fn solve2(grid: &Grid) -> u32 {
    let mut best_score = 0;
    let mut one_fill = |p: Position, d: Direction| {
        let mut light =
            LightMap::filled_default(grid.0.num_rows(), grid.0.num_columns());
        fill_light(grid, &mut light, p, d);
        let s = score(light);
        if s > best_score {
            best_score = s;
        }
    };

    let height = grid.0.num_rows();
    let width = grid.0.num_columns();

    for i in 0..height {
        one_fill(Position(i,0), Direction::East);
        one_fill(Position(i,width-1), Direction::West);
    }

    for j in 0..width {
        one_fill(Position(0,j), Direction::South);
        one_fill(Position(height-1,j), Direction::North);
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

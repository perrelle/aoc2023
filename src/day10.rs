use crate::utils::{*,positions::*,directions::*,grids::*};
use array2d::Array2D;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    NorthEast,
    NorthSouth,
    NorthWest,
    EastSouth,
    EastWest,
    SouthWest,
    Start,
    Empty
}

impl Pipe {
    fn ends(self) -> Option<[Direction4 ; 2]> {
        match self {
            Pipe::NorthEast  => Some ([Direction4::North, Direction4::East]),
            Pipe::NorthSouth => Some ([Direction4::North, Direction4::South]),
            Pipe::NorthWest  => Some ([Direction4::North, Direction4::West]),
            Pipe::EastSouth  => Some ([Direction4::East,  Direction4::South]),
            Pipe::EastWest   => Some ([Direction4::East,  Direction4::West]),
            Pipe::SouthWest  => Some ([Direction4::South, Direction4::West]),
            Pipe::Empty | Pipe::Start => None
        }
    }

    fn other_end(self, d: Direction4) -> Option<Direction4> {
        if let Some(ends) = self.ends() {
            if ends[0] == d {
                Some(ends[1])
            }
            else if ends[1] == d {
                Some(ends[0])
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}

impl ConvertibleToChar for Pipe {
    fn to_char(&self) -> char {
        match self {
            Pipe::NorthEast => '└',
            Pipe::NorthSouth => '│',
            Pipe::NorthWest => '┘',
            Pipe::EastSouth => '┌',
            Pipe::EastWest => '─',
            Pipe::SouthWest => '┐',
            Pipe::Start => 'S',
            Pipe::Empty => ' '
        }
    }
}

type Position = positions::Position<usize>;
type Grid = grids::Grid<Pipe>;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
enum Enclosure { Inside, Outside, Frontier, NotComputed }

impl Enclosure {
    fn invert(&self) -> Option<Enclosure> {
        match self {
            Self::Inside => Some(Self::Outside),
            Self::Outside => Some(Self::Inside),
            Self::Frontier | Self::NotComputed => None
        }
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

    fn pipe(input: &str) -> IResult<&str, Pipe> {
        let (input, c) = one_of("|-LJ7F.S")(input)?;
        let p = match c {
            '|' => Pipe::NorthSouth,
            '-' => Pipe::EastWest,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::EastSouth,
            '.' => Pipe::Empty,
            'S' => Pipe::Start,
            _ => panic!()
        };

        Ok ((input, p))
    }

    fn grid(input: &str) -> IResult<&str, Grid> {
        map(
                separated_list1(line_ending, many1(pipe)),
                |v| Grid(Array2D::from_rows(&v).unwrap()))
            (input)
    }

    pub fn parse(input: &str) -> IResult<&str, Grid> {
        all_consuming(terminated(grid, multispace0))(input)
    }
}

fn find_start(grid: &Grid) -> Position {
    for (i,row_it) in grid.0.rows_iter().enumerate() {
        for (j,&p) in row_it.enumerate() {
            if p == Pipe::Start {
                return Position::new(i,j)
            }
        }
    }
    panic!()
}

fn follow_pipe(grid: &Grid, d: Direction4, p: Position)
        -> Option<(Direction4, Position)> {
    let pipe = grid[p];
    let d = pipe.other_end(d.invert())?;
    let p = p.step(d)?;
    Some ((d, p))
}

fn cycle_length(grid: &Grid, mut d: Direction4, mut p: Position) -> Option<u32> {
    let mut length = 1;
    let starting_position = p;
    p = p.step(d).unwrap();

    loop {
        (d, p) = follow_pipe(grid, d, p)?;
        length += 1;
        if p == starting_position {
            return Some (length);
        }
    }
}

fn mark_cycle(grid: &Grid, mut d: Direction4, mut p: Position)
        -> Option<Array2D<bool>> {
    let mut marks = Array2D::filled_with(
        false,
        grid.0.num_rows(),
        grid.0.num_columns());
    let starting_position = p;
    marks[(p.0, p.1)] = true;
    p = p.step(d)?;

    loop {
        marks[(p.0, p.1)] = true;
        (d, p) = follow_pipe(grid, d, p)?;
        if p == starting_position {
            return Some (marks);
        }
    }
}

fn solve_part1(grid: &Grid) -> u32 {
    let p = find_start(grid);
    for d in Direction4::ALL {
        if let Some(l) = cycle_length(grid, d, p) {
            return l / 2;
        }
    }

    panic!();
}

fn mark_loop(grid: &Grid) -> Array2D<bool> {
    let p = find_start(grid);
    for d in Direction4::ALL {
        if let Some(marks) = mark_cycle(grid, d, p) {
            return marks;
        }
    }

    panic!()
}

fn compute_enclosure(grid: &Grid) -> Array2D<Enclosure> {
    let marks = mark_loop(grid);
    let mut stack = vec![
        (Direction8::North, Position(0, 0), Enclosure::Outside)
    ];
    let mut enclosure = Array2D::filled_with(
        Enclosure::NotComputed,
        grid.0.num_rows(),
        grid.0.num_columns());

    while let Some((d,p,e)) = stack.pop() {
        if let Some(&e) = enclosure.get(p.0, p.1) {
            if e != Enclosure::NotComputed {
                continue
            }
        }
        else {
            continue;
        }

        if marks[(p.0,p.1)] {
            enclosure[(p.0,p.1)] = Enclosure::Frontier;
            let pipe = grid[p];
            if let Some(pipe_ends) = pipe.ends() {
                let pipe_ends = pipe_ends.map(Direction8::from);
                let d_start = d.invert();
                let mut d_current = d_start.next();
                let mut e_current = e;
                while d_current != d_start {
                    if pipe_ends.contains(&d_current) {
                        e_current = e_current.invert().unwrap();
                    }
                    else if let Some(p_current) = p.step(d_current) {
                        stack.push((d_current, p_current, e_current));
                    }
                    d_current = d_current.next();
                }
            }
        }
        else {
            enclosure[(p.0,p.1)] = e;

            for d in Direction4::ALL {
                if let Some(p) = p.step(d) {
                    stack.push((Direction8::from(d), p, e));
                }
            }
        }
    }

    enclosure
}

fn _print_enclosure(grid: &Grid, enclosure: &Array2D<Enclosure>) {
    for (i,row_it) in enclosure.rows_iter().enumerate() {
        for (j,&e) in row_it.enumerate() {
            let c = match e {
                Enclosure::Inside => '*',
                Enclosure::Outside => ' ', 
                Enclosure::Frontier => grid[Position(i,j)].to_char(),
                Enclosure::NotComputed => 'x',
            };
            print!("{c}");
        }
        println!();
    }
}

fn count_inner_tiles(enclosure: &Array2D<Enclosure>) -> u32 {
    let mut count = 0;

    for row_it in enclosure.rows_iter() {
        for &e in row_it {
            if e == Enclosure::Inside {
                count += 1;
            }
        }
    }

    count
}

fn solve_part2(grid: &Grid) -> u32 {
    let enclosure = compute_enclosure(grid);
    _print_enclosure(grid, &enclosure);
    count_inner_tiles(&enclosure)
}

pub fn solve(input: &str) -> (u32, u32) {
    let (_,grid) = parser::parse(input).unwrap();
    (solve_part1(&grid), solve_part2(&grid))
}

pub fn solve2(input: &str) -> u32 {
    let (_,grid) = parser::parse(input).unwrap();
    solve_part2(&grid)
}

#[test]
fn day10_example1() {
  let solution = solve(&include_str!("../inputs/day10-example1"));
  assert_eq!(solution, (4, 1));
}

#[test]
fn day10_example2() {
  let solution = solve(&include_str!("../inputs/day10-example2"));
  assert_eq!(solution, (8, 1));
}

#[test]
fn day10_example3() {
  let solution = solve2(&include_str!("../inputs/day10-example3"));
  assert_eq!(solution, 4);
}

#[test]
fn day10_example4() {
  let solution = solve2(&include_str!("../inputs/day10-example4"));
  assert_eq!(solution, 8);
}

#[test]
fn day10_example5() {
  let solution = solve2(&include_str!("../inputs/day10-example5"));
  assert_eq!(solution, 10);
}

#[test]
fn day10_input() {
  let solution = solve(&include_str!("../inputs/day10-input"));
  assert_eq!(solution, (6682, 353));
}

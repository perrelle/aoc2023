use std::{fmt, ops::{Index, IndexMut}, collections::BinaryHeap, cmp::Ordering};

use array2d::Array2D;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    const ALL: [Direction ; 4] =  [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right
    ];

    fn opposite(&self) -> Self {
        match self {
            Direction::Up    => Direction::Down,
            Direction::Down  => Direction::Up,
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

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

type Cell = u8;

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
                let c = cell.to_string();
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
        let (input, c) = one_of("0123456789")(input)?;
        let i = c as u8 - b'0';
        Ok ((input, i))
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

struct Marks (Array2D<DirectionMap<Vec<bool>>>);

impl Marks {
    fn new(num_rows: usize, num_columns: usize) -> Self {
        Marks(Array2D::filled_by_row_major(
            DirectionMap::default,
            num_rows,
            num_columns))
    }
}

impl Index<&ExplorationNode> for Marks {
    type Output = bool;

    fn index(&self, index: &ExplorationNode) -> &Self::Output {
        let v = &self.0
            [(index.position.0, index.position.1)]
            [index.direction];
        let i = index.consecutive_blocks as usize;
        if i < v.len() {
            &v[i]
        }
        else {
            &false
        }
    }
}

impl IndexMut<&ExplorationNode> for Marks {
    fn index_mut(&mut self, index: &ExplorationNode) -> &mut Self::Output {
        let v = &mut self.0
            [(index.position.0, index.position.1)]
            [index.direction];
        let i = index.consecutive_blocks as usize;
        if i < v.len() {
            &mut v[i]
        }
        else {
            v.resize(i + 1, false);
            &mut v[i]
        }
    }
}

#[derive (Debug)]
struct ExplorationNode {
    cost: u32,
    position: Position,
    direction: Direction,
    consecutive_blocks: u8
}

impl PartialEq for ExplorationNode {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for ExplorationNode {}

impl Ord for ExplorationNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for ExplorationNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Crucible {
    min_forward: u8,
    max_forward: u8
}

fn shortest_path(
        grid: &Grid,
        start: &Position,
        end: &Position,
        crucible: &Crucible) -> u32 {
    let mut marks = Marks::new(grid.0.num_rows(), grid.0.num_columns());
    let mut nodes : BinaryHeap<ExplorationNode> = BinaryHeap::new();

    nodes.push(ExplorationNode {
        cost: 0,
        position: start.clone(),
        direction: Direction::Down,
        consecutive_blocks: 0
    });

    while let Some(node) = nodes.pop() {
        if marks[&node] {
            continue;
        }
        marks[&node] = true;

        if node.position == *end {
            return node.cost
        }

        for d in Direction::ALL {
            if d == node.direction.opposite() {
                continue;
            }

            let consecutive_blocks =
                if d == node.direction {
                    node.consecutive_blocks + 1
                } else {
                    if node.consecutive_blocks != 0 && /* start condition */
                        node.consecutive_blocks < crucible.min_forward {
                        continue;
                    }
                    1
                };

            if consecutive_blocks > crucible.max_forward {
                continue;
            }

            if let Some(p) = node.position.step(d) {
                if grid.is_index_valid(&p) {
                    nodes.push(ExplorationNode {
                        cost: node.cost + (grid[&p] as u32),
                        position: p,
                        direction: d,
                        consecutive_blocks
                    });
                }
            }
        }
    }

    panic!();
}


pub fn solve(input: &str) -> (u32, u32) {
    let (_,grid) = parser::parse(input).unwrap();
    println!("{grid}");
    let start = Position(0,0);
    let end = Position(grid.0.num_rows() - 1, grid.0.num_columns() - 1);
    let normal_crucible = Crucible {
        min_forward: 0,
        max_forward: 3
    };
    let ultra_crucible = Crucible {
        min_forward: 4,
        max_forward: 10
    };
    let result1 = shortest_path(&grid, &start, &end, &normal_crucible);
    let result2 = shortest_path(&grid, &start, &end, &ultra_crucible);
    (result1, result2)
}

#[test]
fn day17_example() {
    let solution = solve(include_str!("../inputs/day17-example"));
    assert_eq!(solution, (102, 94));
}

#[test]
fn day17_input() {
    let solution = solve(include_str!("../inputs/day17-input"));
    assert_eq!(solution, (817, 925));
}

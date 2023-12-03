use std::{collections::{HashMap, HashSet}};

use array2d::Array2D;

#[derive (Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Digit (u8),
    Symbol (char)
}

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*,
        branch::*,
    };

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        alt((
            map(char('.'), |_| Cell::Empty),
            map(one_of("+-*/%#$=@&"), Cell::Symbol),
            map(satisfy(
                |c| c.is_ascii_digit()),
                |c| Cell::Digit(c.to_digit(10).unwrap() as u8))
        ))(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
        let data = separated_list1(line_ending, many1(cell));
        all_consuming(terminated(data, multispace0))(input)
    }
}

fn adjacent_symbols(array: &Array2D<Cell>, i: usize, j : usize) ->
        Vec<(char, usize, usize)> {
    let mut symbols = Vec::new();

    for di in [-1, 0, 1] {
        for dj in [-1, 0, 1] {
            let i = usize::try_from(i as i32 + di);
            let j = usize::try_from(j as i32 + dj);
            if let (Ok (i), Ok (j)) = (i,j) {
                let cell = array.get(i, j);
                if let Some (Cell::Symbol (c)) = cell {
                    symbols.push((*c, i, j));
                }
            }
        }
    }

    symbols
}

pub fn solve_part1(array: &Array2D<Cell>) -> u32 {
    let mut result = 0;

    for (i,row) in array.rows_iter().enumerate() {
        let mut current_number : Option<(u32, bool)> = None;

        for (j,cell) in row.enumerate() {
            if let Cell::Digit(d) = cell {
                let (n, b) =
                    if let Some ((n, b)) = current_number {
                        (n * 10 + (*d as u32), b)
                    }
                    else {
                        (*d as u32, false)
                    };
                let b = b || !adjacent_symbols(array, i, j).is_empty() ;
                current_number = Some ((n, b))
            }
            else {
                if let Some ((n, true)) = current_number {
                    result += n;
                }

                current_number = None
            }
        }

        if let Some ((n, true)) = current_number {
            result += n;
        } 
    }

    result
}

pub fn solve_part2(array: &Array2D<Cell>) -> u32 {
    let mut gears : HashMap<(usize,usize), Vec<u32>> = HashMap::new();

    let mut add_number = |n, gear_set: HashSet<(usize, usize)> | {
        for g in gear_set.iter() {
            if let Some (v) = gears.get_mut(g) {
                v.push(n);
            }
            else {
                let v = vec![n];
                gears.insert(*g, v);
            }
        }
    };

    for (i,row) in array.rows_iter().enumerate() {
        let mut current_number : Option<(u32, HashSet<(usize, usize)>)> = None;

        for (j,cell) in row.enumerate() {
            if let Cell::Digit(d) = cell {
                let (n, mut gear_set) =
                    if let Some ((n, gear_set)) = current_number {
                        (n * 10 + (*d as u32), gear_set)
                    }
                    else {
                        (*d as u32, HashSet::new())
                    };
                let adjacent_gears : HashSet<(usize,usize)> =
                    adjacent_symbols(array, i, j)
                        .iter()
                        .filter_map(|(s,i,j)| {
                            if let '*' = s {
                                Some ((*i,*j))
                            }
                            else {
                                None
                            }
                        })
                        .collect();

                gear_set.extend(adjacent_gears);
                current_number = Some ((n, gear_set))
            }
            else {
                if let Some ((n, gear_set)) = current_number {
                    add_number(n, gear_set);
                }

                current_number = None
            }
        }

        if let Some ((n, gear_set)) = current_number {
            add_number(n, gear_set);
        } 
    }

    let mut result2 = 0;

    for (_, v) in gears.iter() {
        if v.len() == 2 {
            result2 += v[0] * v[1];
        }
    }

    result2
}

pub fn solve(input: &str) -> (u32, u32) {
    let (_,data) = parser::parse(input).unwrap();
    let array = Array2D::from_rows(&data).unwrap();
    (solve_part1(&array), solve_part2(&array))
}

#[test]
fn test_day3_example() {
    let solution = solve(&include_str!("../inputs/day3-example"));
    assert_eq!(solution, (4361, 467835));
}

#[test]
fn test_day3_input() {
    let solution = solve(&include_str!("../inputs/day3-input"));
    assert_eq!(solution, (517021, 81296995));
}

use array2d::Array2D;

use crate::utils::{directions::*, positions::*, grid::*};

#[derive (Debug)]
pub struct Order {
    direction: Direction4,
    distance: u32,
    color: u32
}

type Input = Vec<Order>;

mod parser {
    use nom::{
        IResult,
        character::complete::{*, u32},
        combinator::*,
        sequence::*,
        bytes::complete::*,
        multi::*
    }; 

    use super::*;

    fn direction(input: &str) -> IResult<&str, Direction4> {
        let (input, c) = one_of("UDRL")(input)?;
        let d = match c {
                'U' => Direction4::North,
                'D' => Direction4::South,
                'R' => Direction4::East,
                'L' => Direction4::West,
                _ => panic!()
            };
        Ok ((input, d))
    }

    fn color(input: &str) -> IResult<&str, u32> {
        map(hex_digit1, |s| u32::from_str_radix(s, 16).unwrap())(input)
    }

    fn order(input: &str) -> IResult<&str, Order> {
        let (input, (direction, _, distance, _, _, color, _)) = tuple((
                direction, space1, u32, space1, tag("(#"), color, tag(")")
            ))(input)?;

        Ok((input, Order { direction, distance, color }))
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        all_consuming(terminated(
            separated_list1(multispace1, order),
            multispace0))(input)
    }
}

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
enum Cell { Trench, Untouched }

impl ConvertibleToChar for Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Trench => '#',
            Cell::Untouched => '.'
        }
    }
}

type G = Grid<Cell>;
type Position = crate::utils::positions::Position<usize>;

enum Pass<'a> { Measuring(&'a mut (usize, usize)), Filling(&'a mut G, &'a mut Vec<Position>) }

fn walk(orders: &Vec<Order>, pass: &mut Pass) {
    let mut p = Position(0, 0);

    for order in orders {
        match pass {
            Pass::Measuring(_) => (),
            Pass::Filling(_, v) => {
                v.push(p.clone());
            }
        }

        for _ in 0..order.distance {
            p = p.step(order.direction).unwrap();
            match pass {
                Pass::Measuring(r) => {
                    **r = (r.0.max(p.0), r.1.max(p.1))
                },
                Pass::Filling(grid, _) => {
                    grid[&p] = Cell::Trench;
                }
            }
        }
    }
}

fn area_inside_curve(curve: &Vec<Position>, with_perimeter: bool) -> u32 {
    let mut double_area: i32 = 0;
    let mut perimeter: i32 = 0;

    for i in 0..curve.len() {
        let j = if i + 1 >= curve.len() {
                i + 1 - curve.len()
            } else {
                i + 1
            };
        double_area +=
            (curve[i].0 as i32 - curve[j].0 as i32) *
            (curve[i].1 as i32 + curve[j].1 as i32);
        perimeter +=
            (curve[i].0 as i32 - curve[j].0 as i32).abs() +
            (curve[i].1 as i32 - curve[j].1 as i32).abs();
    }

    let pc = if with_perimeter { 1 } else { -1 };
    let real_area = (double_area.abs() + pc * perimeter) / 2 + 1;
    println!("A = {}, P = {}, R = {}", double_area, perimeter, real_area);
    real_area as u32
}

pub fn solve(input: &str) -> (u32, u32) {
    let (_,orders) = parser::parse(input).unwrap();
    let mut dimensions = (0usize, 0usize);
    let mut pass1 = Pass::Measuring(&mut dimensions);
    walk(&orders, &mut pass1);
    let mut grid = Grid (Array2D::filled_with(
        Cell::Untouched,
        dimensions.0 + 1,
        dimensions.1 + 1));
    let mut curve: Vec<Position> = Vec::new();
    let mut pass2 = Pass::Filling(&mut grid, &mut curve);
    walk(&orders, &mut pass2);
    println!("{grid}");

    (area_inside_curve(&curve, true), 0)
}

#[test]
fn day18_example() {
    let solution = solve(include_str!("../inputs/day18-example"));
    assert_eq!(solution, (62, 0));
}

#[test]
fn day18_input() {
    let solution = solve(include_str!("../inputs/day18-input"));
    assert_eq!(solution, (0, 0));
}

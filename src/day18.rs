use crate::utils::{directions::*, positions::*};

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


#[derive (Debug, Clone)]
struct SimpleOrder {
    direction: Direction4,
    distance: u32
}

impl SimpleOrder {
    fn from(order: &Order) -> Self {
        Self { direction: order.direction, distance: order.distance }
    }

    fn fix(order: &Order) -> Self {
        let direction = match order.color & 0xf {
                0 => Direction4::East,
                1 => Direction4::South,
                2 => Direction4::West,
                3 => Direction4::North,
                _ => panic!()
            };
        let distance = order.color / 16;
        Self { direction, distance }
    }
}


type Position = crate::utils::positions::Position<i32>;

fn walk(orders: &Vec<SimpleOrder>) -> Vec<Position> {
    let mut p = Position(0, 0);
    let mut v = Vec::new();

    for order in orders {
        v.push(p);
        for _ in 0..order.distance {
            p = p.step(order.direction).unwrap();
        }
    }

    v
}

fn area_inside_curve(curve: &Vec<Position>, with_perimeter: bool) -> u64 {
    let mut double_area: i64 = 0;
    let mut perimeter: i64 = 0;

    for i in 0..curve.len() {
        let j = if i + 1 >= curve.len() {
                i + 1 - curve.len()
            } else {
                i + 1
            };
        double_area +=
            (curve[i].0 as i64 - curve[j].0 as i64) *
            (curve[i].1 as i64 + curve[j].1 as i64);
        perimeter +=
            (curve[i].0 as i64 - curve[j].0 as i64).abs() +
            (curve[i].1 as i64 - curve[j].1 as i64).abs();
    }

    let pc = if with_perimeter { 1 } else { -1 };
    let real_area = (double_area.abs() + pc * perimeter) / 2 + 1;
    real_area as u64
}

pub fn solve(input: &str) -> (u64, u64) {
    let (_,orders) = parser::parse(input).unwrap();
    let orders1 = orders.iter().map(SimpleOrder::from).collect();
    let orders2 = orders.iter().map(SimpleOrder::fix).collect();
    let curve1 = walk(&orders1);
    let curve2 = walk(&orders2);

    (area_inside_curve(&curve1, true), area_inside_curve(&curve2, true))
}

#[test]
fn day18_example() {
    let solution = solve(include_str!("../inputs/day18-example"));
    assert_eq!(solution, (62, 952408144115));
}

#[test]
fn day18_input() {
    let solution = solve(include_str!("../inputs/day18-input"));
    assert_eq!(solution, (34329, 42617947302920));
}

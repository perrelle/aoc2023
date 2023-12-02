#![allow(clippy::neg_cmp_op_on_partial_ord)]

use std::{cmp::{Ordering, max}, ops::Add};

#[derive (Debug, Clone, PartialEq, Eq)]
pub struct Handful {
    red: u32,
    green: u32,
    blue: u32,
}

impl Handful {
    fn empty() -> Handful {
        Handful { red: 0, green: 0, blue: 0 }
    }

    fn upper_bound(x: &Self, y: &Self) -> Handful {
        Handful {
            red: max(x.red, y.red),
            green: max(x.green, y.green),
            blue: max(x.blue, y.blue)
        }
    }
}

impl PartialOrd for Handful {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some (Ordering::Equal)
        }
        else if
                self.red <= other.red &&
                self.green <= other.green &&
                self.blue <= other.blue {
            Some (Ordering::Less)
        }
        else if
                self.red >= other.red &&
                self.green >= other.green &&
                self.blue >= other.blue {
            Some (Ordering::Greater)
        }
        else {
            None
        }
    }
}

impl Add for Handful {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

pub struct Game {
    id: u32,
    handfuls: Vec<Handful>
}

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        bytes::complete::*,
        combinator::*,
        sequence::*,
        multi::*,
        branch::*,
    };

    use super::*;

    fn color(input: &str) -> IResult<&str, Handful> {
        let color = alt((tag("red"), tag("green"), tag("blue")));
        let (input, (n, c)) = separated_pair(u32, space1, color)(input)?;
        let h = match c {
            "red" =>   Handful { red : n, green : 0, blue : 0 },
            "green" => Handful { red : 0, green : n, blue : 0 },
            "blue" =>  Handful { red : 0, green : 0, blue : n },
            _ => panic!()
        };
        Ok ((input, h))
    }

    fn handful(input: &str) -> IResult<&str, Handful> {
        let (input, colors) = separated_list1(tag(", "), color)(input)?;
        let h = colors.into_iter().fold(Handful::empty(), |acc, h| acc + h);
        Ok ((input, h))
    } 

    fn game(input: &str) -> IResult<&str, Game> {
        let (input, id) = delimited(tag("Game "), u32, tag(": "))(input)?;
        let (input, handfuls) = separated_list1(tag("; "), handful)(input)?;
        let game = Game { id, handfuls };
        Ok ((input, game))
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
        let data = separated_list1(line_ending, game);
        all_consuming(terminated(data, multispace0))(input)
    }
}


pub fn solve_part1(data: &Vec<Game>) -> u32 {
    let max = Handful { red : 12, green : 13, blue : 14 };
    let mut result = 0;

    for game in data {
        let mut possible = true;
        for h in &game.handfuls {
            if !(h <= &max) {
                possible = false;
                break;
            }
        }
        if possible {
            result += game.id;
        }
    }

    result
}

pub fn solve_part2(data: &Vec<Game>) -> u32 {
    let mut result = 0;

    for game in data {
        let mut max = Handful::empty();

        for h in &game.handfuls {
            max = Handful::upper_bound(&max, h);
        }

        let power = max.red * max.green * max.blue;
        result += power;
    }
    
    result
}

pub fn solve(input: &str) -> (u32, u32) {
    let (_,data) = parser::parse(input).unwrap();
    (solve_part1(&data), solve_part2(&data))
}

#[test]
fn test_day2_example() {
    let solution = solve(&include_str!("../inputs/day2-example"));
    assert_eq!(solution, (8,2286));
}

#[test]
fn test_day2_input() {
    let solution = solve(&include_str!("../inputs/day2-input"));
    assert_eq!(solution, (2278,67953));
}

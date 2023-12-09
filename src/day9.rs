use std::collections::VecDeque;

type Series = VecDeque<i32>;

type Input = Vec<Series>;

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    };

    use super::*;

    pub fn series(input: &str) -> IResult<&str, Series> {
        map(separated_list1(space1, i32), Series::from)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        all_consuming(
            terminated(
                separated_list1(multispace1, series),
                multispace0))(input)
    }
}

fn derive(v: &Series) -> Series {
    let mut result = VecDeque::new();
    let mut it = v.iter();

    if let Some(mut prev) = it.next() {
        for cur in it {
            result.push_back(cur - prev);
            prev = cur;
        }
    }

    result
}

fn extrapolate_right(v1: &mut Series, v2: &Series) {
    let x = *v1.back().unwrap() + v2.back().unwrap();
    v1.push_back(x);
}

fn extrapolate_left(v1: &mut Series, v2: &Series) {
    let x = *v1.front().unwrap() - v2.front().unwrap();
    v1.push_front(x);
}

fn is_zero(v: &Series) -> bool {
    v.iter().all(|&x| x == 0)
}

fn solve_vector_right(v: &Series) -> i32 {
    let mut all: Vec<Series> = vec![v.clone()];
    loop {
        let w = derive(all.last().unwrap());
        if is_zero(&w) {
            break;
        }
        all.push(w);
    }

    let mut it = all.iter_mut().rev();
    if let Some (mut w) = it.next() {
        for v in it {
            extrapolate_right(v, w);
            w = v;
        }
    }

    return *all.first().unwrap_or(&Series::new()).back().unwrap_or(&0)
}

fn solve_vector_left(v: &Series) -> i32 {
    let mut all: Vec<Series> = vec![v.clone()];
    loop {
        let w = derive(all.last().unwrap());
        if is_zero(&w) {
            break;
        }
        all.push(w);
    }

    let mut it = all.iter_mut().rev();
    if let Some (mut w) = it.next() {
        for v in it {
            extrapolate_left(v, w);
            w = v;
        }
    }

    return *all.first().unwrap_or(&Series::new()).front().unwrap_or(&0)
}


fn part1(input: &Input) -> i32 {
    let mut result = 0;

    for v in input {
        result += solve_vector_right(v);
    }

    result
}

fn part2(input: &Input) -> i32 {
    let mut result = 0;

    for v in input {
        result += solve_vector_left(v);
    }

    result
}

pub fn solve(input: &str) -> (i32, i32) {
    let (_,input) = parser::parse(input).unwrap();

    (part1(&input), part2(&input))
}

#[test]
fn day9_example() {
  let solution = solve(&include_str!("../inputs/day9-example"));
  assert_eq!(solution, (114, 2));
}

#[test]
fn day9_input() {
  let solution = solve(&include_str!("../inputs/day9-input"));
  assert_eq!(solution, (1882395907, 1005));
}

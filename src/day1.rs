use nom::InputIter;

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*,
    };

    pub fn parse(input: &str) -> IResult<&str, Vec<&str>> {
        let data = separated_list1(line_ending, alphanumeric1);
        all_consuming(terminated(data, multispace0))(input)
    }
}

pub fn solve_one(input: &str) -> u32 {
    let vec : Vec<u32> =
        input.chars().
        filter(|c| !(c.is_alphabetic())).
        map(|c| c.to_digit(10).unwrap()).
        collect();
    let result = vec.first().unwrap() * 10 + vec.last().unwrap();
    result
}

pub fn solve1(input: &str) -> u32 {
    let (_,data) = parser::parse(input).unwrap();
    let mut result = 0;

    for line in data {
        result += solve_one(line);
    }

    result
}

fn convert_digit(input: &str) -> Option<u32> {
    if input.starts_with("one") {
        Some(1)
    }
    else if input.starts_with("two") {
        Some(2)
    }
    else if input.starts_with("three") {
        Some(3)
    }
    else if input.starts_with("four") {
        Some(4)
    }
    else if input.starts_with("five") {
        Some(5)
    }
    else if input.starts_with("six") {
        Some(6)
    }
    else if input.starts_with("seven") {
        Some(7)
    }
    else if input.starts_with("eight") {
        Some(8)
    }
    else if input.starts_with("nine") {
        Some(9)
    }
    else {
        let c = input.chars().next()?;
        c.to_digit(10)
    }
}

fn find_first(input: &str) -> Option<u32> {
    for (i,_) in input.iter_indices() {
        if let Some(x) = convert_digit(&input[i..]) {
            return Some(x);
        }
    }

    None
}

fn find_last(input: &str) -> Option<u32> {
    for (i,_) in input.iter_indices().rev() {
        if let Some(x) = convert_digit(&input[i..]) {
            return Some(x);
        }
    }

    None
}

pub fn solve2(input: &str) -> u32 {
    let (_,data) = parser::parse(input).unwrap();
    let mut result = 0;

    for line in data {
        let first = find_first(line).unwrap();
        let last = find_last(line).unwrap();
        result += first * 10 + last;
    }

    result
}

pub fn solve(input: &str) -> (u32, u32) {
    (solve1(input), solve2(input))
}


#[test]
fn test1_1a() {
    let solution = solve1(&include_str!("../inputs/day1.1a"));
    assert_eq!(solution, 142);
}

#[test]
fn test1_1b() {
    let solution = solve2(&include_str!("../inputs/day1.1b"));
    assert_eq!(solution, 281);
}

#[test]
fn test1_2() {
    let solution = solve(&include_str!("../inputs/day1.2"));
    assert_eq!(solution, (56397, 55701));
}

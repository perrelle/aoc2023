pub struct Race {
    time: u64,
    distance: u64
}

type Input = Vec<Race>;

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        bytes::complete::*,
        multi::*
    };

    use super::*;

    fn line<'a>(header: &str, input: &'a str) -> IResult<&'a str, Vec<u64>> {
        preceded(pair(tag(header), space1), separated_list1(space1, u64))(input)
    }

    fn races(input: &str) -> IResult<&str, Input> {
        let line = |header| move |input| line(header, input);
        map(
            separated_pair(line("Time:"), multispace1, line("Distance:")),
            |(times, distances)|
                times.iter().zip(distances.iter()).map(|(&t,&d)|
                    Race { time:t, distance:d })
                    .collect()
            )(input)
    }
    
    pub fn parse(input: &str) -> IResult<&str, Vec<Race>> {
        all_consuming(terminated(races, multispace0))(input)
    }
}

fn win_count(race: &Race) -> u64 {
    let mut wins = 0;

    for i in 1..race.time {
        if i * (race.time - i) > race.distance {
            wins += 1;
        }
    }

    wins
}

pub fn solve_part1(races: &[Race]) -> u64 {
    let mut result = 1;

    for race in races {
        result *= win_count(race);
    }

    result
}

fn concat_races(races: &[Race]) -> Race {
    let mut time = String::new();
    let mut distance = String::new();

    for r in races {
        time.push_str(&r.time.to_string());
        distance.push_str(&r.distance.to_string());
    }

    Race { time: time.parse().unwrap(), distance: distance.parse().unwrap() } 
}

pub fn solve_part2(races: &[Race]) -> u64 {
    let race = concat_races(races);
    win_count(&race)
}

pub fn solve(input: &str) -> (u64, u64) {
    let (_,races) = parser::parse(input).unwrap();
    (solve_part1(&races), solve_part2(&races))
}

#[test]
fn test_day6_example() {
    let solution = solve(&include_str!("../inputs/day6-example"));
    assert_eq!(solution, (288, 71503));
}

#[test]
fn test_day6_input() {
    let solution = solve(&include_str!("../inputs/day6-input"));
    assert_eq!(solution, (1084752, 28228952));
}

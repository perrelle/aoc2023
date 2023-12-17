use std::collections::HashMap;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Damaged,
    Operational,
    Unknown
}

#[derive (Debug, Clone)]
pub struct Correction {
    size: usize,
    maximum: Option<usize>
}

#[derive (Debug, Clone)]
pub struct Row {
    states: Vec<State>,
    correction: Vec<Correction>
}

type Input = Vec<Row>;

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

    fn state(input: &str) -> IResult<&str, State> {
        let (input, c) = one_of(".#?")(input)?;
        let p = match c {
            '.' => State::Operational,
            '#' => State::Damaged,
            '?' => State::Unknown,
            _ => panic!()
        };

        Ok ((input, p))
    }

    fn correction(input: &str) -> IResult<&str, Correction> {
        map(u32, |x| Correction {
            size: usize::try_from(x).unwrap(),
            maximum: None
         })(input)
    }

    fn row(input: &str) -> IResult<&str, Row> {
        let (input, states) = many1(state)(input)?;
        let (input, _) = space1(input)?;
        let (input, correction) = separated_list1(tag(","), correction)(input)?;
        Ok ((input, Row { states, correction }))
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        all_consuming(terminated(
            separated_list1(multispace1, row),
            multispace0))(input)
    }
}

fn block_fits(states: &Vec<State>, i: usize, size: usize) -> bool {
    for j in i..(i+size) {
        if j >= states.len() || states[j] == State::Operational {
            return false;
        }
    }

    i + size >= states.len() || states[i+size] != State::Damaged
}

fn find_maximum_index(states: &Vec<State>, corrections: &mut [Correction]) {
    let mut i = (states.len() - 1) as i32;

    for correction in corrections.iter_mut().rev() {
        while !block_fits(states, i as usize + 1 - correction.size, correction.size) {
            i -= 1;
        }

        correction.maximum = Some (i as usize + 1 - correction.size);
        i -= (correction.size + 1) as i32; // May be < 0 if this is the last iteration
    }
}

#[derive (Debug, Clone, PartialEq, Eq, Hash)]
struct CacheIndex {
    corrections_len: usize,
    current_index: usize,
}

impl CacheIndex {
    fn new(corrections: &mut Vec<Correction>, index: usize) -> Self {
        CacheIndex {
            corrections_len: corrections.len(),
            current_index: index
        }
    }
}

type Cache = HashMap<CacheIndex, u64>;

fn count_rec(
        states: &Vec<State>,
        corrections: &mut Vec<Correction>,
        i: usize,
        cache: &mut Cache) -> u64 {
    let cache_index = CacheIndex::new(corrections, i);
    if let Some(r) = cache.get(&cache_index) {
        return *r;
    }

    let result = {
        if i >= states.len() {
            if corrections.is_empty() { 1 } else { 0 }
        }
        else {
            let s = states[i];
            if s == State::Operational {
                count_rec(states, corrections, i+1, cache)
            }
            else if let Some(c) = corrections.pop() {
                let mut count = 0;
                let size = c.size;
                
                let faisible = if let Some(maximum) = c.maximum {
                    i <= maximum
                } else {
                    true
                };

                if faisible && block_fits(states, i, size) {
                    count += count_rec(states, corrections, i + size + 1, cache);
                }

                corrections.push(c);
                if faisible && s == State::Unknown {
                    count += count_rec(states, corrections, i + 1, cache);
                }

                count
            }
            else if s == State::Damaged {
                0
            }
            else {
                count_rec(states, corrections, i+1, cache)
            }
        }
    };
    
    cache.insert(cache_index, result);
    result
}

fn count_arrangements(row: &Row) -> u64 {
    let mut row = row.clone();
    find_maximum_index(&row.states, &mut row.correction);
    let mut correction_stack = row.correction.clone();
    correction_stack.reverse();
    let mut cache = HashMap::new();
    count_rec(&row.states, &mut correction_stack, 0, &mut cache)
}

fn solve_part1(input: &Input) -> u64 {
    let mut result = 0;
    for row in input {
        let arrangements = count_arrangements(row);
        result += arrangements;
    }
    result
}

fn unfold(row: &Row) -> Row {
    let mut states = row.states.clone();
    let mut correction = row.correction.clone();

    for _ in 0..4 {
        states.push(State::Unknown);
        states.extend(row.states.iter());
        correction.extend(row.correction.iter().cloned());
    }

    Row { states, correction }
}

fn solve_part2(input: &Input) -> u64 {
    let mut result = 0;
    for row in input {
        let row = unfold(row);
        let arrangements = count_arrangements(&row);
        result += arrangements;
    }
    result
}

pub fn solve(input: &str) -> (u64, u64) {
    let (_,input) = parser::parse(input).unwrap();
    (solve_part1(&input), solve_part2(&input))
}

#[test]
fn day12_example() {
    let solution = solve(include_str!("../inputs/day12-example"));
    assert_eq!(solution, (21, 525152));
}

#[test]
fn day12_input() {
    let solution = solve(include_str!("../inputs/day12-input"));
    assert_eq!(solution, (6958, 6555315065024));
}

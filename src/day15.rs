use std::fmt;

type Input1 = Vec<String>;

#[derive (Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Remove,
    Add(u8)
}

#[derive (Debug, Clone)]
pub struct Order {
    label: String,
    action: Action
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)?;
        match self.action {
            Action::Remove => write!(f, "-"),
            Action::Add(i) => write!(f, "={i}")
        }
    }
}

type Input2 = Vec<Order>;

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

    fn word(input: &str) -> IResult<&str, String> {
        map(is_not(",\n"), String::from)(input)
    }

    pub fn parse1(input: &str) -> IResult<&str, Input1> {
        all_consuming(terminated(
            separated_list1(char(','), word),
            multispace0))(input)
    }

    fn label(input: &str) -> IResult<&str, String> {
        map(alpha1, String::from)(input)
    }

    fn action(input: &str) -> IResult<&str, Action> {
        let (input, c) = one_of("=-")(input)?;
        let (input, action) = match c {
                '-' => (input, Action::Remove),
                '=' => {
                        let (input, l) = u8(input)?;
                        (input, Action::Add(l))
                    },
                _ => panic!()
            };
        Ok((input, action))
    }

    fn order(input: &str) -> IResult<&str, Order> {
        let (input, (label, action)) = pair(label, action)(input)?;
        Ok((input, Order { label, action }))
    }

    pub fn parse2(input: &str) -> IResult<&str, Input2> {
        all_consuming(terminated(
            separated_list1(char(','), order),
            multispace0))(input)
    }
}

fn hash(s: &str) -> u32 {
    let mut h = 0;
    for c in s.chars() {
        h += c as u32;
        h *= 17;
        h %= 256;
    }
    h
}

pub fn solve1(input: &str) -> u32 {
    let (_,input) = parser::parse1(input).unwrap();
    let mut result1 = 0;

    for s in input {
        let h = hash(&s);
        result1 += h;
    }

    result1
}

#[derive (Debug, Clone)]
struct Len {
    label: String,
    focal: u8,
}

#[derive (Debug, Clone)]
struct Box {
    lenses: Vec<Len>
}

impl Box {
    fn new() -> Self {
        Box { lenses: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.lenses.is_empty()
    }

    fn remove(&mut self, label: &str) {
        if let Some(index) = self.lenses.iter().position(|x| x.label == label) {
            self.lenses.remove(index);
        }
    }

    fn add(&mut self, label: &str, focal: u8) {
        if let Some(index) = self.lenses.iter().position(|x| x.label == label) {
            self.lenses[index].focal = focal
        }
        else {
            self.lenses.push(Len { label: String::from(label), focal });
        }
    }
}

#[derive (Debug, Clone)]
struct Machine ([Box ; 256]);

impl Machine {
    fn new() -> Self {
        Machine (std::array::from_fn(|_| Box::new()))
    }

    fn execute(&mut self, order: &Order) {
        let h = hash(&order.label);
        match order.action {
            Action::Remove => {
                self.0[h as usize].remove(&order.label)
            },
            Action::Add(i) => {
                self.0[h as usize].add(&order.label, i)
            }
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i,b) in self.0.iter().enumerate() {
            if !b.is_empty() {
                write!(f, "Box {i}:")?;
                for len in &b.lenses {
                    write!(f, " [{} {}]", len.label, len.focal)?;
                }
                writeln!(f)?;
            }
        }

        Ok (())
    }
}

fn score2(machine: &Machine) -> u32 {
    let mut score = 0;
    for (i,b) in machine.0.iter().enumerate() {
        for (j,len) in b.lenses.iter().enumerate() {
            score += (i as u32 + 1) * (j as u32 + 1) * (len.focal as u32);
        }
        
    }
    score
}

pub fn solve2(input: &str) -> u32 {
    let (_,input) = parser::parse2(input).unwrap();

    let mut machine = Machine::new();
    for order in input {
        machine.execute(&order);
        //println!("After {order}:\n{machine}\n");
    }
    
    score2(&machine)
}

pub fn solve(input: &str) -> (u32, u32) {
    (solve1(input), solve2(input))
}

#[test]
fn day15_small_example() {
    assert_eq!(hash("HASH"), 52);
}

#[test]
fn day15_example() {
    let solution = solve(include_str!("../inputs/day15-example"));
    assert_eq!(solution, (1320, 145));
}

#[test]
fn day15_input() {
    let solution = solve(include_str!("../inputs/day15-input"));
    assert_eq!(solution, (511498, 284674));
}

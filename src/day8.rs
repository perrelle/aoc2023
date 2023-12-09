use std::collections::HashMap;

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction { Left, Right }

type Node = String;

type Neighbors = (Node, Node);

type Input = (Vec<Direction>, Vec<(Node, Neighbors)>);

type Graph = HashMap<String, (String, String)>;

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*,
        branch::*,
        bytes::complete::tag,
    };

    use super::*;

    fn direction(input: &str) -> IResult<&str, Direction> {
        alt((
                map(char('L'), |_| Direction::Left),
                map(char('R'), |_| Direction::Right))
            )(input)
    }

    fn neighbors(input: &str) -> IResult<&str, Neighbors> {
        delimited(
                tag("("),
                separated_pair(
                    map(alphanumeric1, String::from),
                    tag(", "),
                    map(alphanumeric1, String::from)),
                tag(")")
            )(input)
    }

    fn node(input: &str) -> IResult<&str, (String, Neighbors)> {
        separated_pair(map(alphanumeric1, String::from), tag(" = "), neighbors)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        all_consuming(
            terminated(
                separated_pair(
                    many1(direction),
                    multispace1,
                    separated_list1(line_ending, node)),
                multispace0))(input)
    }
}

fn build_graph(neighbors: Vec<(String, Neighbors)>) -> Graph {
    neighbors.into_iter().collect()
}

fn advance<'a>(graph: &'a Graph, node: &Node, direction: Direction)
        -> &'a Node {
    let neighbors = graph.get(node).unwrap();
    //println!("node {node} -> {neighbors:?}");

    match direction {
        Direction::Left => &neighbors.0,
        Direction::Right => &neighbors.1
    }
}

fn part1(path: &Vec<Direction>, graph: &Graph) -> u32 {
    let mut count = 0;
    let mut current : &Node = &String::from("AAA");

    loop {
        for d in path {
            current = advance(graph, current, *d);
            count += 1;
            if current == "ZZZ" {
                return count;
            }
        }
    }
}

fn starting_nodes(graph: &Graph) -> Vec<&Node> {
    graph.iter()
        .map(|(name,_)| name)
        .filter(|name| name.ends_with('A'))
        .collect()
}

fn _naive_part2(path: &Vec<Direction>, graph: &Graph) -> u32 {
    let mut count = 0;
    let mut current : Vec<&Node> = starting_nodes(graph);

    loop {
        for d in path {
            for n in current.iter_mut() {
                *n = advance(graph, n, *d);
            }

            count += 1;
            if current.iter().all(|n| n.ends_with('Z')) {
                return count;
            }
        }
    }
}

fn part2(path: &Vec<Direction>, graph: &Graph) -> u64 {
    let sources: Vec<&Node> = starting_nodes(graph);
    let mut lengths: Vec<u64> = Vec::new();

    for mut node in sources {
        let mut count = 0;
    
        'outer: loop {
            for d in path {
                node = advance(graph, node, *d);
                count += 1;
                if node.ends_with('Z') {
                    lengths.push(count);
                    break 'outer;
                }
            }
        }
    }

    lengths.into_iter().fold(1u64, num::integer::lcm)
}

pub fn solve_part1(input: &str) -> u32 {
    let (_,(path, neighbors)) = parser::parse(input).unwrap();
    let graph = build_graph(neighbors);

    part1(&path, &graph)
}

pub fn solve_part2(input: &str) -> u64 {
    let (_,(path, neighbors)) = parser::parse(input).unwrap();
    let graph = build_graph(neighbors);

    part2(&path, &graph)
}

pub fn solve(input: &str) -> (u32, u64) {
    let (_,(path, neighbors)) = parser::parse(input).unwrap();
    let graph = build_graph(neighbors);

    (part1(&path, &graph), part2(&path, &graph))
}

#[test]
fn day8_example1() {
    let solution = solve_part1(&include_str!("../inputs/day8-example1"));
    assert_eq!(solution, 2);
}

#[test]
fn day8_example2() {
    let solution = solve_part1(&include_str!("../inputs/day8-example2"));
    assert_eq!(solution, 6);
}

#[test]
fn day8_example3() {
    let solution = solve_part2(&include_str!("../inputs/day8-example3"));
    assert_eq!(solution, 6);
}

#[test]
fn day8_input() {
    let solution = solve(&include_str!("../inputs/day8-input"));
    assert_eq!(solution, (22357, 10371555451871));
}

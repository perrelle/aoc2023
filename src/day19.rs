use std::{ops::{Index,IndexMut}, collections::HashMap};
use crate::utils::intervals::*;

// --- Types definitions ---

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variable {X, M, A, S}
type WorkflowName = String;
type Value = u32;

#[derive (Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Lower(Variable, Value),
    Greater(Variable, Value)
}

#[derive (Debug, Clone, PartialEq, Eq)]
pub enum Action { Accept, Reject, Follow(WorkflowName) }

#[derive (Debug, Clone)]
pub struct Rule {
    condition: Condition,
    action: Action
}

#[derive (Debug, Clone)]
pub struct Workflow {
    name: WorkflowName,
    rules: Vec<Rule>,
    default: Action
}

#[derive (Debug, Clone, PartialEq, Eq)]
pub struct Valuation<T> {
    x: T,
    m: T,
    a: T,
    s: T
}

impl<T: Default> Default for Valuation<T> {
    fn default() -> Self {
        Valuation {
            x: T::default(),
            m: T::default(),
            a: T::default(),
            s: T::default()
        }
    }
}

impl<T> Index<Variable> for Valuation<T> {
    type Output = T;

    fn index(&self, index: Variable) -> &T {
        match index {
            Variable::X => &self.x,
            Variable::M => &self.m,
            Variable::A => &self.a,
            Variable::S => &self.s
        }
    }
}

impl<T> IndexMut<Variable> for Valuation<T> {
    fn index_mut(&mut self, index: Variable) -> &mut T {
        match index {
            Variable::X => &mut self.x,
            Variable::M => &mut self.m,
            Variable::A => &mut self.a,
            Variable::S => &mut self.s
        }
    }
}

type Rating  = Valuation<Value>;
type Input = (Vec<Workflow>, Vec<Rating>);


// --- Parser ---

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    }; 

    use super::*;

    fn value(input: &str) -> IResult<&str, Value> {
        u32(input)
    }

    fn variable(input: &str) -> IResult<&str, Variable> {
        let (input, c) = one_of("xmas")(input)?;
        let d = match c {
                'x' => Variable::X,
                'm' => Variable::M,
                'a' => Variable::A,
                's' => Variable::S,
                _ => panic!()
            };
        Ok ((input, d))
    }

    fn condition(input: &str) -> IResult<&str, Condition> {
        let (input, (v, c, i)) =
            tuple((variable, one_of("<>"), value))(input)?;
        let condition = match c {
                '<' => Condition::Lower(v, i),
                '>' => Condition::Greater(v, i),
                _ => panic!()
            };
        Ok ((input, condition))
    }

    fn action(input: &str) -> IResult<&str, Action> {
        map(alpha1, |s| match s {
            "A" => Action::Accept,
            "R" => Action::Reject,
            s => Action::Follow(String::from(s))
        })(input)
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        let (input, (condition, action)) =
            separated_pair(condition, char(':'), action)(input)?;
        Ok ((input, Rule { condition, action }))
    }

    fn workflow(input: &str) -> IResult<&str, Workflow> {
        let (input, (name, (rules, default))) =
            pair(
                alpha1,
                delimited(
                    char('{'),
                    separated_pair(
                        separated_list0(char(','), rule),
                        char(','),
                        action),
                    char('}')))(input)?;
        Ok((input, Workflow { name: String::from(name), rules, default }))
    }

    fn initialization(input: &str) -> IResult<&str, (Variable, Value)> {
        separated_pair(variable, char('='), value)(input)
    }

    fn rating(input: &str) -> IResult<&str, Rating> {
        let (input, v) = delimited(
            char('{'),
            separated_list1(char(','), initialization),
            char('}'))(input)?;
        let mut rating = Valuation::default();
        for (variable, value) in v {
            rating[variable] = value;
        }
        Ok((input, rating))
    }
    pub fn parse(input: &str) -> IResult<&str, Input> {
        all_consuming(terminated(
            separated_pair(
                separated_list1(multispace1, workflow),
                multispace1,
                separated_list1(multispace1, rating)),
            multispace0))(input)
    }
}


// --- Part 1 ---

fn check_condition(condition: &Condition, part: &Rating) -> bool {
    match *condition {
        Condition::Greater(var, val) => part[var] > val,
        Condition::Lower(var, val) => part[var] < val,
    }
}

type WorkflowMap<'a> = HashMap<&'a WorkflowName, &'a Workflow>;


fn sort_part(workflows: &WorkflowMap, part: &Rating) -> bool {

    let mut current_workflow = &String::from("in");
    loop {
        let workflow = *workflows.get(&current_workflow).unwrap();
        let mut matching_action = &workflow.default;
        for rule in &workflow.rules {
            if check_condition(&rule.condition, part) {
                matching_action = &rule.action;
                break;
            }
        }

        match matching_action {
            Action::Accept => return true,
            Action::Reject => return false,
            Action::Follow(target) => current_workflow = target
        }
    }
}

fn rate_part(part: &Rating) -> Value {
    part.x + part.m + part.a + part.s
}

fn solve1(workflows: &WorkflowMap, parts: &Vec<Rating>) -> u32 {
    let mut score = 0;

    for part in parts {
        if sort_part(workflows, part) {
            score += rate_part(part);
        }
    }

    score
}


// --- Part2 ---

#[derive (Debug, Clone, PartialEq, Eq)]
struct PartSet {
    x: Interval<Value>,
    m: Interval<Value>,
    a: Interval<Value>,
    s: Interval<Value>
}

impl PartSet {
    fn cardinal(&self) -> u64 {
        self.x.cardinal() as u64 *
        self.m.cardinal() as u64 *
        self.a.cardinal() as u64 *
        self.s.cardinal() as u64 
    }
}

impl Index<Variable> for PartSet {
    type Output = Interval<Value>;

    fn index(&self, index: Variable) -> &Interval<Value> {
        match index {
            Variable::X => &self.x,
            Variable::M => &self.m,
            Variable::A => &self.a,
            Variable::S => &self.s
        }
    }
}

impl IndexMut<Variable> for PartSet {
    fn index_mut(&mut self, index: Variable) -> &mut Interval<Value> {
        match index {
            Variable::X => &mut self.x,
            Variable::M => &mut self.m,
            Variable::A => &mut self.a,
            Variable::S => &mut self.s
        }
    }
}

fn do_action(
        workflows: &WorkflowMap,
        parts: PartSet,
        action: &Action) -> u64 {
    match action {
        Action::Accept => parts.cardinal(),
        Action::Reject => 0,
        Action::Follow(target) => {
            let workflow = workflows.get(target).unwrap();
            do_rule(workflows, parts, workflow, 0)
        }
    }
}

fn do_rule(
        workflows: &WorkflowMap,
        parts: PartSet,
        workflow: &Workflow,
        rule_number: usize) -> u64 {
    println!("{}:{rule_number} / {parts:?}", workflow.name);

    if rule_number >= workflow.rules.len() {
        do_action(workflows, parts, &workflow.default)
    }
    else {
        let rule = &workflow.rules[rule_number];
        let (var, intervals, accepted) = match rule.condition {
            Condition::Lower(var, val) =>
                (var, parts[var].split_before(val), SplitResult::Below),
            Condition::Greater(var, val) =>
                (var, parts[var].split_after(val), SplitResult::Above),
        };

        let mut count = 0;

        for (split_result, interval) in intervals {
            let mut new_state = parts.clone();
            new_state[var] = interval;
            if split_result == accepted {
                count += do_action(workflows, new_state, &rule.action)
            }
            else {
                count += do_rule(workflows, new_state, workflow, rule_number + 1)
            }
        }

        count
    }
}

fn solve2(workflows: &WorkflowMap) -> u64 {
    let every_parts = PartSet {
            x: Interval(1, 4000),
            m: Interval(1, 4000),
            a: Interval(1, 4000),
            s: Interval(1, 4000)
        };
    do_action(workflows, every_parts, &Action::Follow(String::from("in")))
}

pub fn solve(input: &str) -> (u32,u64) {
    let (_,(workflows, parts)) = parser::parse(input).unwrap();
    let workflows: WorkflowMap =
        workflows.iter().map(|w| (&w.name, w)).collect();

    (solve1(&workflows, &parts), solve2(&workflows))
}

#[test]
fn day19_example() {
    let solution = solve(include_str!("../inputs/day19-example"));
    assert_eq!(solution, (19114, 167409079868000));
}

#[test]
fn day19_input() {
    let solution = solve(include_str!("../inputs/day19-input"));
    assert_eq!(solution, (418498, 123331556462603));
}

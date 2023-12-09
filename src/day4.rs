use std::collections::{HashSet, HashMap};

pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>
}

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

    fn card(input: &str) -> IResult<&str, Card> {
        let (input, (id, (winning_numbers, owned_numbers))) =
            pair(
                delimited(pair(tag("Card"), space1), u32, tag(":")),
                separated_pair(
                    preceded(multispace1, separated_list1(multispace1, u32)),
                    preceded(multispace0, tag("|")),
                    preceded(multispace1, separated_list1(multispace1, u32)))
                )(input)?;
        Ok((input, Card { id, winning_numbers, owned_numbers }))
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Card>> {
        let data = separated_list1(line_ending, card);
        all_consuming(terminated(data, multispace0))(input)
    }
}

fn card_winning_numbers(card: &Card) -> Vec<u32> {
    let winning_set: HashSet<u32> =
        card.winning_numbers.iter().copied().collect();
    card.owned_numbers.iter()
        .filter(|c| winning_set.contains(c))
        .copied().collect()
}

pub fn solve_part1(cards: &Vec<Card>) -> u32 {
    let mut score = 0;
    
    for card in cards {
        let winning_numbers = card_winning_numbers(card);
        if winning_numbers.is_empty() {
            continue;
        }
        else {
            let mut card_score = 1;
            for _ in 1..winning_numbers.len() {
                card_score <<= 1;
            }
            score += card_score;
        }

    }

    score
}

pub fn solve_part2(cards: &Vec<Card>) -> u32 {
    let mut copies: HashMap<u32, u32> =
        cards.iter().map(|c| (c.id, 1)).collect();

    for card in cards {
        let count = *copies.get(&card.id).unwrap();
        let winning_numbers = card_winning_numbers(card);
        for i in 1..=(winning_numbers.len() as u32) {
            let c = copies.get_mut(&(card.id + i)).unwrap();
            *c += count;
        }
    }

    copies.values().sum()
}

pub fn solve(input: &str) -> (u32, u32) {
    let (_,data) = parser::parse(input).unwrap();
    (solve_part1(&data), solve_part2(&data))
}

#[test]
fn test_day4_example() {
    let solution = solve(&include_str!("../inputs/day4-example"));
    assert_eq!(solution, (13, 30));
}

#[test]
fn test_day4_input() {
    let solution = solve(&include_str!("../inputs/day4-input"));
    assert_eq!(solution, (22193, 5625994));
}

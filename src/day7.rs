use std::{collections::HashMap, cmp::Ordering};

type Card = char;

type Hand = [Card; 5];

#[derive (Debug, Clone)]
pub struct Player {
    hand: Hand,
    bid: u32
}

type Input = Vec<Player>;


mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    };

    use super::*;

    fn card(input: &str) -> IResult<&str, Card> {
        one_of("AKQJT98765432")(input)
    }

    fn hand(input: &str) -> IResult<&str, Hand> {
        map(count(card, 5), |v| v.try_into().unwrap())(input)
    }

    fn player(input: &str) -> IResult<&str, Player> {
        map(separated_pair(hand, space1, u32), |(hand,bid)|
                Player { hand, bid }
            )(input)
    }
    
    pub fn parse(input: &str) -> IResult<&str, Input> {
        all_consuming(terminated(
            separated_list1(multispace1, player),
            multispace0
            ))(input)
    }
}

fn hand_type1(hand: &Hand) -> i32 {
    let mut count = HashMap::new();
    for card in hand {
        if let Some(c) = count.get_mut(card) {
            *c += 1;
        }
        else {
            count.insert(card, 1);
        }
    }

    let mut count_vec: Vec<(&char, i32)> = count.into_iter().collect();
    count_vec.sort_by_key(|(_,n)| -n);

    match count_vec[0] {
        (_,5) => 7, /* Five of a kind */
        (_,4) => 6, /* Four of a kind */
        (_,3) => {
            match count_vec[1] {
                (_,2) => 5, /* Full house */
                _ => 4 /* Three of a kind */
            }
        }
        (_,2) => {
            match count_vec[1] {
                (_,2) => 3, /* Two pairs */
                _ => 2 /* One pair */
            }
        }
        _ => 1 /* High card */
    }
}

fn card_rank1(c: &Card) -> i32 {
    match c {
        'A'=> 13,
        'K'=> 12,
        'Q'=> 11,
        'J'=> 10,
        'T'=> 9,
        '9'=> 8,
        '8'=> 7,
        '7'=> 6,
        '6'=> 5,
        '5'=> 4,
        '4'=> 3,
        '3'=> 2,
        '2'=> 1,
        _ => panic!()
    }
}

fn cmp_hand1(h1: &Hand, h2: &Hand) -> Ordering {
    let c = i32::cmp(&hand_type1(h1), &hand_type1(h2));
    if c != Ordering::Equal {
        return c;
    }

    for (c1,c2) in h1.iter().zip(h2.iter()) {
        let c = i32::cmp(&card_rank1(c1), &card_rank1(c2));
        if c != Ordering::Equal {
            return c;
        }    
    }

    Ordering::Equal
}

fn score(ranked_players: &Input) -> u32 {
    let mut result = 0;

    for (rank, player) in ranked_players.iter().enumerate() {
        println!("rank: {} - {:?}", rank+1, player.hand);
        result += (rank as u32 + 1) * player.bid;
    }

    result
}

pub fn solve_part1(input: &Input) -> u32 {
    let mut ranked_players = input.clone();
    ranked_players.sort_by(|p1, p2| cmp_hand1(&p1.hand, &p2.hand));
    score(&ranked_players)
}

fn hand_type2(hand: &Hand) -> i32 {
    let mut count = HashMap::new();
    for card in hand {
        if let Some(c) = count.get_mut(card) {
            *c += 1;
        }
        else {
            count.insert(card, 1);
        }
    }

    let jokers = *count.get(&'J').unwrap_or(&0);
    count.remove(&'J');

    let mut count_vec: Vec<(&char, i32)> = count.into_iter().collect();
    count_vec.sort_by_key(|(_,n)| -n);

    if jokers == 5 {
        return 7;
    }

    match count_vec[0].1 + jokers {
        5 => 7, /* Five of a kind */
        4 => 6, /* Four of a kind */
        3 => {
            match count_vec[1].1 {
                2 => 5, /* Full house */
                _ => 4 /* Three of a kind */
            }
        }
        2 => {
            match count_vec[1].1 {
                2 => 3, /* Two pairs */
                _ => 2 /* One pair */
            }
        }
        _ => 1 /* High card */
    }
}

fn card_rank2(c: &Card) -> i32 {
    match c {
        'A'=> 13,
        'K'=> 12,
        'Q'=> 11,
        'T'=> 10,
        '9'=> 9,
        '8'=> 8,
        '7'=> 7,
        '6'=> 6,
        '5'=> 5,
        '4'=> 4,
        '3'=> 3,
        '2'=> 2,
        'J'=> 1,
        _ => panic!()
    }
}

fn cmp_hand2(h1: &Hand, h2: &Hand) -> Ordering {
    let c = i32::cmp(&hand_type2(h1), &hand_type2(h2));
    if c != Ordering::Equal {
        return c;
    }

    for (c1,c2) in h1.iter().zip(h2.iter()) {
        let c = i32::cmp(&card_rank2(c1), &card_rank2(c2));
        if c != Ordering::Equal {
            return c;
        }    
    }

    Ordering::Equal
}

pub fn solve_part2(input: &Input) -> u32 {
    let mut ranked_players = input.clone();
    ranked_players.sort_by(|p1, p2| cmp_hand2(&p1.hand, &p2.hand));
    score(&ranked_players)
}

pub fn solve(input: &str) -> (u32, u32) {
    let (_,input) = parser::parse(input).unwrap();
    (solve_part1(&input), solve_part2(&input))
}

#[test]
fn test_day7_example() {
    let solution = solve(&include_str!("../inputs/day7-example"));
    assert_eq!(solution, (6440, 5905));
}

#[test]
fn test_day7_input() {
    let solution = solve(&include_str!("../inputs/day7-input"));
    assert_eq!(solution, (253866470, 254494947));
}

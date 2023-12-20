use regex::Regex;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io::Write;
use std::iter::zip;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use std::{fs, io, vec};

const DAY: i8 = 7;

const DUMMY_INPUT_P1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

fn read_input(s: &str) -> Vec<Hand> {
    return s
        .lines()
        .into_iter()
        .map(|s| {
            let hand_s = s.split_once(" ").unwrap();
            let bid = hand_s.1.parse::<i64>().unwrap();
            return Hand::new(hand_s.0, bid);
        })
        .collect();
}

// Every hand is exactly one type. From strongest to weakest, they are:

//     Five of a kind, where all five cards have the same label: AAAAA
//     Four of a kind, where four cards have the same label and one card has a different label: AA8AA
//     Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
//     Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
//     Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
//     One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
//     High card, where all cards' labels are distinct: 23456

// The labels used are A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, and 2. Note that A is the strongest label, followed by K, Q, and so on down to 2, which is the weakest label.

// write a function to determine the type of hand

#[derive(Debug, Clone, Copy)]
struct Card {
    c: char,
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        return self.c == other.c;
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(self: &Card, other: &Card) -> Option<Ordering> {
        if *self == *other {
            return Some(Ordering::Equal);
        }
        let cards = vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        let self_idx = cards.iter().position(|&c| c == self.c).unwrap();
        let other_idx = cards.iter().position(|&c| c == other.c).unwrap();
        return Some(self_idx.cmp(&other_idx));
    }
}

impl Ord for Card {
    fn cmp(self: &Card, other: &Card) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

struct Hand {
    hand: Vec<Card>,
    card_tuples: HashMap<char, i32>,
    bid: i64,
}

impl Hand {
    fn new(hand: &str, bid: i64) -> Hand {
        let mut card_tuples: HashMap<char, i32> = HashMap::new();
        for c in hand.chars() {
            card_tuples
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        return Hand {
            hand: hand.chars().map(|c| Card { c }).collect(),
            card_tuples,
            bid,
        };
    }
    fn high_card(&self) -> Card {
        return self.hand.iter().max().unwrap().clone();
    }
    fn pair(&self) -> Option<Card> {
        self.card_tuples.iter().find_map(|(card, count)| {
            if *count == 2 {
                return Some(Card { c: *card });
            }
            return None;
        })
    }
    fn two_pair(&self) -> Option<(Card, Card)> {
        let res: Vec<(&char, &i32)> = self
            .card_tuples
            .iter()
            .filter(|(_, &count)| count == 2)
            .collect();
        if res.len() == 2 {
            return Some((Card { c: *res[0].0 }, Card { c: *res[1].0 }));
        }
        return None;
    }
    fn three_of_a_kind(&self) -> Option<Card> {
        self.card_tuples.iter().find_map(|(card, count)| {
            if *count == 3 {
                return Some(Card { c: *card });
            }
            return None;
        })
    }
    fn four_of_a_kind(&self) -> Option<Card> {
        self.card_tuples.iter().find_map(|(card, count)| {
            if *count == 4 {
                return Some(Card { c: *card });
            }
            return None;
        })
    }
    fn five_of_a_kind(&self) -> Option<Card> {
        self.card_tuples.iter().find_map(|(card, count)| {
            if *count == 5 {
                return Some(Card { c: *card });
            }
            return None;
        })
    }
    fn get_rank(&self) -> i32 {
        if self.five_of_a_kind().is_some() {
            return 7;
        } else if self.four_of_a_kind().is_some() {
            return 6;
        } else if self.three_of_a_kind().is_some() && self.pair().is_some() {
            return 5;
        } else if self.three_of_a_kind().is_some() {
            return 4;
        } else if self.two_pair().is_some() {
            return 3;
        } else if self.pair().is_some() {
            return 2;
        } else {
            return 1;
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        return self.hand == other.hand;
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        if self.get_rank() > other.get_rank() {
            return Some(Ordering::Greater);
        } else if self.get_rank() < other.get_rank() {
            return Some(Ordering::Less);
        } else if self.hand > other.hand {
            return Some(Ordering::Greater);
        } else if self.hand < other.hand {
            return Some(Ordering::Less);
        }
        return Some(Ordering::Equal);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

fn solve_part_1(s: &str) -> i64 {
    let mut v: Vec<Hand> = read_input(s);
    v.sort();

    return v.iter().enumerate().fold(0, |acc, (i, hand)| {
        return acc + hand.bid * (i as i64 + 1);
    });
}
fn solve_part_2(s: &str) -> i64 {
    return 0;
}

fn gather_input_and_solve_p1() {
    let res = solve_part_1(DUMMY_INPUT_P1);
    println!("Result: {}", res);

    let path = Path::new("resources")
        .join(format!("d{}", DAY))
        .join("input.txt");
    let input = fs::read_to_string(path).expect("Something went wrong reading the file");

    let res = solve_part_1(input.as_str());
    println!("Result: {}", res);
}

fn gather_input_and_solve_p2() {
    let res = solve_part_2(DUMMY_INPUT_P1);

    println!("Result: {}", res);
    let path = Path::new("resources")
        .join(format!("d{}", DAY))
        .join("input.txt");
    let input = fs::read_to_string(path).expect("Something went wrong reading the file");

    let res = solve_part_2(input.as_str());
    println!("Result: {}", res);
}

// Define a function to solve the code advent problem
pub fn solve() {
    gather_input_and_solve_p1();
    gather_input_and_solve_p2();
}

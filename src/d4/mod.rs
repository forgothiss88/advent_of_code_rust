

use std::collections::{HashSet};


use std::path::Path;
use std::{fs};

const DAY: i8 = 4;

const DUMMY_INPUT_P1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

// create a type alias for the matrix

fn read_input(s: &str) -> Vec<(HashSet<i32>, HashSet<i32>)> {
    return s
        .lines()
        .into_iter()
        .map(|line| line.split_once("|").unwrap())
        .map(|(winning_numbers_s, my_numbers_s)| {
            (
                winning_numbers_s
                    .split(" ")
                    .filter_map(|x| x.parse().ok())
                    .collect(),
                my_numbers_s
                    .split(" ")
                    .filter_map(|x| x.parse().ok())
                    .collect(),
            )
        })
        .collect();
}

fn solve_part_1(input: &str) -> i32 {
    let deck_extractions = read_input(input);
    let my_winning_numbers: Vec<HashSet<i32>> = deck_extractions
        .iter()
        .map(|(winning_cards, my_cards)| winning_cards.intersection(my_cards).cloned().collect())
        .collect();

    return my_winning_numbers
        .iter()
        .map(|nums| match nums.len() {
            0 => 0,
            _ => 2i32.pow((nums.len() - 1).try_into().unwrap()),
        })
        .sum();
}

fn solve_part_2(input: &str) -> i32 {
    let deck_extractions = read_input(input);
    let my_winning_numbers_count: Vec<i32> = deck_extractions
        .iter()
        .map(|(winning_cards, my_cards)| winning_cards.intersection(my_cards).count() as i32)
        .collect();

    let num_cards = my_winning_numbers_count.len();

    let mut my_card_copies: Vec<i32> = (0..num_cards).map(|_| 1).collect();

    (0..num_cards).for_each(|i| {
        let winning_numbers_count = my_winning_numbers_count[i];
        let card_copies = my_card_copies[i];

        my_card_copies[i + 1..i + 1 + (winning_numbers_count as usize)]
            .iter_mut()
            .for_each(|x| *x += card_copies);
    });

    return my_card_copies.iter().sum();
}

fn gather_input_and_solve_p1() {
    let res = solve_part_1(DUMMY_INPUT_P1);
    println!("Result: {}", res);
    // read a string from file input.txt
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
    // read a string from file input.txt
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

// Define the main function to run the code advent solution
fn main() {
    solve();
}

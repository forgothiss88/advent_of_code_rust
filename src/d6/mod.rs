use regex::Regex;

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io::Write;
use std::iter::zip;
use std::ops::Deref;
use std::path::Path;
use std::{fs, io, vec};

const DAY: i8 = 6;

const DUMMY_INPUT_P1: &str = "Time:      7  15   30
Distance:  9  40  200";
// write a regex to parse the input

fn read_numbers_whitespace(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn read_input(s: &str) -> (Vec<i64>, Vec<i64>) {
    let (time_s, distance_s) = s.split_once("\n").unwrap();
    let times = read_numbers_whitespace(time_s.split_once(":").unwrap().1);
    let distances = read_numbers_whitespace(distance_s.split_once(":").unwrap().1);
    return (times, distances);
}

fn read_input_p2(s: &str) -> (i64, i64) {
    let (time_s, distance_s) = s.split_once("\n").unwrap();
    let time_s = time_s
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();
    let time: i64 = time_s.parse().unwrap();

    let distance_s = distance_s
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();
    let distance: i64 = distance_s.parse().unwrap();
    return (time, distance);
}

struct RaceSimulation {
    charge_time: i64,
    final_distance: i64,
}

fn compute_results(available_time: i64, min_distance: i64) -> Vec<RaceSimulation> {
    let acceleration = 1; //
    return (1..available_time)
        .into_iter()
        .map(|charge_time| {
            let cur_speed = acceleration * charge_time;
            let final_distance = cur_speed * (available_time - charge_time);
            return RaceSimulation {
                charge_time: charge_time,
                final_distance,
            };
        })
        .filter(|race_simulation| race_simulation.final_distance > min_distance)
        .collect();
}

fn solve_part_1(s: &str) -> i64 {
    let (times, distances) = read_input(s);
    let available_combinations: Vec<Vec<RaceSimulation>> = zip(times.clone(), distances.clone())
        .into_iter()
        .map(|(t, d)| compute_results(t, d))
        .collect();

    return available_combinations
        .iter()
        .fold(1, |acc: i64, race_simulations: &Vec<RaceSimulation>| {
            (race_simulations.len() as i64) * acc
        });
}

fn solve_part_2(s: &str) -> i64 {
    let (available_time, min_distance) = read_input_p2(s);
    let available_combinations: Vec<RaceSimulation> = compute_results(available_time, min_distance);

    return available_combinations.iter().len() as i64;
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

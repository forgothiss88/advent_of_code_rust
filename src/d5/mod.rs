use core::num;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::zip;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::{fs, iter, vec};

const DAY: i8 = 5;

const DUMMY_INPUT_P1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
// write a regex to parse the input

fn read_input(s: &str) -> (Vec<i32>, Vec<(&str, Vec<(i32, i32, i32)>)>) {
    let seeds_re: Regex = Regex::new(r"(\d+)").unwrap();
    let map_re: Regex = Regex::new(
        r"(?:(?<map_name>\w+-\w+-\w+) map:)|(?:(?<dest_start>\d+) (?<source_start>\d+) (?<len>\d+))",
    )
    .unwrap();
    let (seeds_s, maps_s) = s.split_once(r"\n").unwrap();
    // let MAP_NAME_RE: Regex = Regex::new(r"^(\w+-\w+) map:\n").unwrap();
    let seeds: Vec<i32> = seeds_re
        .captures_iter(seeds_s)
        .map(|cap| cap[0].parse::<i32>().unwrap())
        .collect();
    let mut maps: Vec<(&str, Vec<(i32, i32, i32)>)> = vec![];
    map_re
        .captures_iter(maps_s)
        .into_iter()
        .fold(None, |cur_map_name: Option<&str>, cap| {
            let map_name = cap.name("map_name");
            if map_name.is_some() {
                maps.push((map_name.unwrap().as_str(), vec![]));
                return Some(map_name.unwrap().as_str());
            }
            maps.last_mut().unwrap().1.push((
                cap.name("source_start")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
                cap.name("dest_start")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
                cap.name("len").unwrap().as_str().parse::<i32>().unwrap(),
            ));
            return cur_map_name;
        });
    return (seeds, maps);
}

fn find_seed_position(seed: i32, maps: &Vec<(&str, Vec<(i32, i32, i32)>)>) -> i32 {
    return maps.iter().fold(seed, |seed_position: i32, map| {
        map.1
            .iter()
            .find_map(|(source_start, dest_start, len)| {
                if *source_start <= seed_position && seed_position < *source_start + *len {
                    return Some(*dest_start + (seed_position - *source_start));
                }
                return None;
            })
            .unwrap_or(seed_position)
    });
}

fn solve_part_1(input: &str) -> i32 {
    let (seeds, maps) = read_input(input);
    return seeds
        .iter()
        .map(|seed| find_seed_position(*seed, &maps))
        .min()
        .unwrap();
}

fn solve_part_2(input: &str) -> i32 {
    0
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

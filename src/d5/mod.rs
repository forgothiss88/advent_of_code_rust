use regex::Regex;

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io::Write;
use std::iter::zip;
use std::ops::Deref;
use std::path::Path;
use std::{fs, io, vec};

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
56 93 4";
// write a regex to parse the input

fn read_input(s: &str) -> (Vec<i64>, Vec<(&str, Vec<(i64, i64, i64)>)>) {
    let seeds_re: Regex = Regex::new(r"(\d+)").unwrap();
    let map_re: Regex = Regex::new(
        r"(?:(?<map_name>\w+-\w+-\w+) map:)|(?:(?<dest_start>\d+) (?<source_start>\d+) (?<len>\d+))",
    )
    .unwrap();
    let (seeds_s, maps_s) = s.split_once("\n").unwrap();
    // let MAP_NAME_RE: Regex = Regex::new(r"^(\w+-\w+) map:").unwrap();
    let seeds: Vec<i64> = seeds_re
        .captures_iter(seeds_s)
        .map(|cap| cap[0].parse::<i64>().unwrap())
        .collect();
    let mut maps: Vec<(&str, Vec<(i64, i64, i64)>)> = vec![];
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
                cap.name("dest_start")
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap(),
                cap.name("source_start")
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap(),
                cap.name("len").unwrap().as_str().parse::<i64>().unwrap(),
            ));
            return cur_map_name;
        });
    return (seeds, maps);
}

fn find_seed_position(seed: i64, maps: &Vec<(&str, Vec<(i64, i64, i64)>)>) -> i64 {
    return maps.iter().fold(seed, |seed_position: i64, map| {
        map.1
            .iter()
            .find_map(|(dest_start, source_start, len)| {
                if *source_start <= seed_position && seed_position < *source_start + *len {
                    return Some(*dest_start + (seed_position - *source_start));
                }
                return None;
            })
            .unwrap_or(seed_position)
    });
}

#[derive(Debug, Clone, Copy)]
struct SeedRange {
    start: i64,
    end: i64,
}

#[derive(Debug, Clone, Copy)]
struct SeedRangeNextPositions {
    seeds_to_elaborate: Option<SeedRange>,
    next_positions: Option<SeedRange>,
}

#[derive(Debug, Clone)]
struct MapFilter {
    source_range: SeedRange,
    dest_range: SeedRange,
}

#[derive(Debug, Clone)]
struct MapState {
    seeds_to_elaborate: Vec<SeedRange>,
    next_positions: Vec<SeedRange>,
}

fn elaborate_next_positions(
    seed_range: SeedRange,
    source_range: SeedRange, // equal length
    dest_range: SeedRange,   // equal length
) -> SeedRangeNextPositions {
    let delta = dest_range.start - source_range.start;
    // check if seed_range is not contained in source_range
    if source_range.end <= seed_range.start || seed_range.end <= source_range.start {
        return SeedRangeNextPositions {
            seeds_to_elaborate: Some(seed_range),
            next_positions: None,
        };
    }
    // check if seed_range is contained in source_range
    else if source_range.start <= seed_range.start && seed_range.end <= source_range.end {
        return SeedRangeNextPositions {
            seeds_to_elaborate: None,
            next_positions: Some(SeedRange {
                start: seed_range.start + delta,
                end: seed_range.end + delta,
            }),
        };
    }
    // check if seed_range overlaps with left side of source_range
    else if source_range.start <= seed_range.start && seed_range.start < source_range.end {
        return SeedRangeNextPositions {
            seeds_to_elaborate: Some(SeedRange {
                // wont translate
                start: seed_range.start,
                end: source_range.start,
            }),
            next_positions: Some(SeedRange {
                // will translate
                start: dest_range.start,
                end: dest_range.start + (seed_range.end - source_range.end),
            }),
        };
    }
    // check if seed_range overlaps with right side of next_position_range
    else if {
        source_range.start < seed_range.end
            && seed_range.end <= source_range.end
            && seed_range.start < source_range.end
    } {
        return SeedRangeNextPositions {
            seeds_to_elaborate: Some(SeedRange {
                // wont translate
                start: source_range.end,
                end: seed_range.end,
            }),
            next_positions: Some(SeedRange {
                // will translate
                start: dest_range.end - (source_range.end - seed_range.end),
                end: dest_range.end,
            }),
        };
    }
    // check if seed_range overlaps with both sides of source_range
    else if source_range.start < seed_range.start && seed_range.end < source_range.end {
        return SeedRangeNextPositions {
            seeds_to_elaborate: Some(SeedRange {
                // wont translate
                start: source_range.end,
                end: seed_range.end,
            }),
            next_positions: Some(SeedRange {
                // will translate
                start: dest_range.end - (source_range.end - seed_range.end),
                end: dest_range.end,
            }),
        };
    }
    panic!("unhandled case");
}

fn translate_seed_ranges_with_filter(state: MapState, cur_filter: MapFilter) -> MapState {
    let seeds_to_elaborate = state.seeds_to_elaborate;
    let mut new_state = MapState {
        seeds_to_elaborate: vec![],
        next_positions: state.next_positions,
    };
    for seed_range in seeds_to_elaborate {
        let _state = elaborate_next_positions(
            seed_range.clone(),
            cur_filter.source_range,
            cur_filter.dest_range,
        );
        if _state.next_positions.is_some() {
            new_state
                .next_positions
                .push(_state.next_positions.unwrap());
        }
        if _state.seeds_to_elaborate.is_some() {
            new_state
                .seeds_to_elaborate
                .push(_state.seeds_to_elaborate.unwrap());
        }
    }
    return new_state;
}

fn find_seed_range_positions(
    seed_range: SeedRange,
    maps: &Vec<(&str, Vec<MapFilter>)>,
) -> Vec<SeedRange> {
    println!("##############################################");
    println!("Start execution for seed_range={:?}", seed_range.clone());

    let res = maps.iter().fold(
        MapState {
            seeds_to_elaborate: vec![seed_range],
            next_positions: vec![],
        }, // init with seed range
        |map_state, cur_map| {
            println!("|--map_state={:?}", map_state.clone());
            println!("|--map={:?}", cur_map.clone());
            let map_res = cur_map
                .1
                .iter()
                .fold(map_state.clone(), |filter_state, cur_filter| {
                    println!("|----filter={:?}", cur_filter.clone());
                    let new_filter_state =
                        translate_seed_ranges_with_filter(filter_state, cur_filter.clone());
                    println!("|----new_filter_state={:?}", new_filter_state.clone());
                    return new_filter_state;
                });
            println!("|--new_map_state={:?}", map_res.clone());
            let mut new_seed_to_elaborate = map_res.seeds_to_elaborate.clone();
            new_seed_to_elaborate.extend(map_res.next_positions.clone());
            MapState {
                seeds_to_elaborate: new_seed_to_elaborate,
                next_positions: vec![],
            }
        },
    );
    println!("res={:?}", res.clone());
    println!("End execution for seed_range={:?}", seed_range.clone());
    println!("##############################################");
    io::stdout().flush().unwrap();
    res.seeds_to_elaborate
}

fn solve_part_1(_input: &str) -> i64 {
    let (seeds, maps) = read_input(_input);
    return seeds
        .iter()
        .map(|seed| find_seed_position(*seed, &maps))
        .min()
        .unwrap();
}

fn solve_part_2(_input: &str) -> i64 {
    let (seeds, maps) = read_input(_input);
    // iter over pair of seeds
    let seed_ranges: Vec<SeedRange> = zip(
        seeds[0..seeds.len() - 1].iter().step_by(2),
        seeds[1..seeds.len()].iter().step_by(2),
    )
    .map(|(seed_pos, num_seeds)| SeedRange {
        start: *seed_pos,
        end: *seed_pos + *num_seeds,
    })
    .collect();

    let maps: Vec<(&str, Vec<MapFilter>)> = maps
        .iter()
        .map(|(map_name, map)| {
            (
                *map_name,
                map.iter()
                    .map(|(dest_start, source_start, len)| MapFilter {
                        source_range: SeedRange {
                            start: *source_start,
                            end: *source_start + *len,
                        },
                        dest_range: SeedRange {
                            start: *dest_start,
                            end: *dest_start + *len,
                        },
                    })
                    .collect(),
            )
        })
        .collect(); // add source range to map
                    // iter 1 seed range over many maps

    return seed_ranges
        .iter()
        .map(|seed_range| {
            find_seed_range_positions(seed_range.clone(), &maps)
                .iter()
                .map(|seed_range| seed_range.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
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
    // let path = Path::new("resources")
    //     .join(format!("d{}", DAY))
    //     .join("input.txt");
    // let input = fs::read_to_string(path).expect("Something went wrong reading the file");

    // let res = solve_part_2(input.as_str());
    // println!("Result: {}", res);
}

// Define a function to solve the code advent problem
pub fn solve() {
    // gather_input_and_solve_p1();
    gather_input_and_solve_p2();
}


use regex::Regex;
use std::collections::{HashMap};
use std::path::Path;
use std::{fs, vec};

const DAY: i8 = 3;

const DUMMY_INPUT_P1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

// create a type alias for the matrix

fn get_numbers_and_matrix_from_str(s: &str) -> (HashMap<(usize, usize), String>, Vec<String>) {
    let lines: Vec<&str> = s.lines().collect();

    let mut numbers: HashMap<(usize, usize), String> = HashMap::new();

    for (row_num, line) in lines.iter().enumerate() {
        let re = Regex::new(r"(\d+)").unwrap();
        numbers.extend(
            re.captures_iter(line)
                .map(|cap| ((cap.get(0).unwrap().start(), row_num), cap[0].to_string())),
        );
    }
    return (numbers, lines.iter().map(|x| x.to_string()).collect());
}

struct NumberInMatrix {
    id: (usize, usize),
    number: i32,
}

struct Gear {
    x: usize,
    y: usize,
}

fn get_gears_numbers_and_matrix_from_str(s: &str) -> (Vec<Gear>, Vec<Vec<Option<NumberInMatrix>>>) {
    let lines: Vec<&str> = s.lines().collect();

    let mut numbers: Vec<Vec<Option<NumberInMatrix>>> = vec![];
    let mut gears = vec![];

    for (row_num, line) in lines.iter().enumerate() {
        let num_re = Regex::new(r"(\d+)").unwrap();
        numbers.push((0..line.len()).map(|_x| None).collect()); // initialize the row with no values
        num_re.captures_iter(line).for_each(|cap| {
            let x = cap.get(0).unwrap().start();
            let curr_number = cap[0].parse::<i32>().unwrap();
            let number_len = cap[0].len();
            for i in x..(x + number_len) {
                numbers[row_num][i] = Some(NumberInMatrix {
                    id: (row_num, x),
                    number: curr_number,
                });
            }
        });
        gears.extend(line.chars().enumerate().filter_map(|(i, c)| {
            if c != '*' {
                return None;
            }
            return Some(Gear { x: i, y: row_num });
        }))
    }
    return (gears, numbers);
}

fn get_adjacent_indexes(
    x: i32,
    y: i32,
    length: i32,
    max_x: i32,
    max_y: i32,
) -> Vec<(usize, usize)> {
    let mut indexes: Vec<(i32, i32)> = vec![];
    let mut walked_indexes: Vec<(i32, i32)> = vec![];
    let y_range = (y - 1).max(0)..=(y + 1).min(max_y);
    for j in y_range {
        let x_range = (x - 1).max(0)..=(x + length).min(max_x);
        for i in x_range {
            walked_indexes.push((i, j));
            if (y == j) && (x <= i) && (i < x + length) {
                let _b = 1;
                continue;
            }
            indexes.push((i, j));
        }
    }
    return indexes
        .into_iter()
        .map(|(x, y)| (x as usize, y as usize))
        .collect();
}

fn is_part_number(mx: &Vec<String>, x: usize, y: usize, digits: &str) -> bool {
    // filter m to get adiacent elements of x,y
    // slice matrix to a get a matrix arout x,y knowing that x,y is the leftmost element of string digits
    let x = x as i32;
    let y = y as i32;
    let max_x_idx = (mx[0].len() - 1) as i32;
    let max_y_idx = (mx.len() - 1) as i32;
    let adjacent_indexes = get_adjacent_indexes(x, y, digits.len() as i32, max_x_idx, max_y_idx);
    let close_chars: Vec<char> = adjacent_indexes
        .into_iter()
        .map(|(x, y)| mx[y].chars().nth(x).unwrap())
        .collect();

    let first_symbol = close_chars
        .into_iter()
        .find(|c| !c.is_digit(10) && (c != &'.'));

    return first_symbol.is_some();
}

fn get_gear_ratio(gear: &Gear, numbers_matrix: &Vec<Vec<Option<NumberInMatrix>>>) -> i32 {
    // list of (x,y)
    let adj_idxs: Vec<(usize, usize)> = get_adjacent_indexes(
        gear.x as i32,
        gear.y as i32,
        1,
        (numbers_matrix.len() - 1) as i32,
        (numbers_matrix[0].len() - 1) as i32,
    );

    let _nums: HashMap<(usize, usize), i32> = adj_idxs
        .iter()
        .filter_map(|(x, y)| {
            // filter only numbers adjacent numbers in matrix
            let n: &NumberInMatrix = numbers_matrix[*y][*x].as_ref()?;
            return Some((n.id, n.number));
        })
        .collect();

    if _nums.len() == 2 {
        return _nums.values().into_iter().fold(1, |a, b| a * b);
    }
    return 0;
}

fn solve_part_2(input: &str) -> i32 {
    let (gears, numbers_matrix) = get_gears_numbers_and_matrix_from_str(input);
    let ratios = gears
        .iter()
        .map(|gear| get_gear_ratio(gear, &numbers_matrix));
    return ratios.into_iter().sum();
}

fn solve_part_1(input: &str) -> i32 {
    let (numbers, m) = get_numbers_and_matrix_from_str(input);
    let mut sum_of_parts = 0;
    for ((x, y), digits) in numbers {
        if is_part_number(&m, x, y, digits.as_str()) {
            sum_of_parts += digits.parse::<i32>().unwrap();
        }
    }
    return sum_of_parts;
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

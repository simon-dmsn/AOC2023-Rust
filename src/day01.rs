#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

fn find_digit(line: &str, reverse: bool) -> Option<u64> {
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5",
        "6", "7", "8", "9",
    ];
    let mut min_position = None;
    let mut min_digit = None;

    for digit in &digits {
        if reverse {
            if let Some(position) = line.rfind(digit) {
                if position >= min_position.unwrap_or(position) {
                    min_position = Some(position);
                    min_digit = Some(digit);
                }
            }
        } else if let Some(position) = line.find(digit) {
            if position <= min_position.unwrap_or(position) {
                min_position = Some(position);
                min_digit = Some(digit);
            }
        }
    }

    min_digit.map(|digit| match *digit {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse::<u64>().unwrap(),
    })
}

pub fn join_digits(input: &str) -> u64 {
    input
        .lines()
        .map(|line: &str| {
            let first = u64::from(line.chars().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap());
            let last = u64::from(line.chars().rfind(char::is_ascii_digit).unwrap().to_digit(10).unwrap());
            first * 10 + last
        })
        .sum()
}

pub fn join_speelled_digits(input: &str) -> u64 {
    input
        .lines()
        .map(|line: &str| {
            let first = find_digit(line, false).unwrap();
            let last = find_digit(line, true).unwrap();
            first * 10 + last
        })
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day01.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", join_digits(&input));
    println!("PART 2 = {}", join_speelled_digits(&input));
    println!("Execution time: {:?}", now.elapsed());
}









/*#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::File;
use std::io::{self, BufRead};
use std::u32::MAX;
use std::convert::TryFrom;
use std::path::Path;

fn concat(vec: &[u32]) -> u32 {
    let mut acc = 0;
    for elem in vec {
        acc *= 10;
        acc += elem;
    }
    acc
}

fn get_first_number(l: String) -> u32 {
    let mut best_index = MAX;
    let mut number_used: u32 = MAX;
    let array = ["zero","one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for i in 0..10 {
        if let Some(index) = l.find(array[i]) {
            if (u32::try_from(index).unwrap() < best_index) {
                best_index = u32::try_from(index).unwrap();
                number_used = u32::try_from(i).unwrap();
            }
        }
    }
    let mut position: u32 = 0;
    for char in l.chars() {
        if char.is_numeric() {
            if (position < best_index) {
                best_index = position;
                number_used = char.to_digit(10).unwrap();
            } else {
                break;
            }
        }
        position += 1;
    }
    return number_used;
}

fn get_last_number(l: String) -> u32 {
    let mut best_index = 0;
    let mut number_used: u32 = MAX;
    let array = ["zero","one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for i in 0..10 {
        if let Some(index) = l.rfind(array[i]) {
            if (u32::try_from(index).unwrap() > best_index) {
                best_index = u32::try_from(index).unwrap();
                number_used = u32::try_from(i).unwrap();
            }
        }
    }
    let mut position: u32 = 0;
    for char in l.chars() {
        if char.is_numeric() {
            if (position > best_index) {
                best_index = position;
                number_used = char.to_digit(10).unwrap();
            }
        }
        position += 1;
    }
    return number_used;
}

pub fn read_file_line_by_line(filepath: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    let mut big_sum: u64 = 0;
    for line in reader.lines() {
        let mut vec: Vec<u32> = Vec::new();
        if let Ok(l) = line {
            vec.push(get_first_number(l.clone()));
            vec.push(get_last_number(l.clone()));
        }
        println!("{}",concat(&vec));
        big_sum = big_sum + u64::from(concat(&vec));
    }



    Ok(big_sum)
}

pub fn main() {
    let path = Path::new("src/input/day01.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let response = read_file_line_by_line("src/input/day01.txt");
    match response {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
    //println!("the sum is {}",read_file_line_by_line("day01.txt"));
}
*/
use regex::Regex;

pub fn part_a(input: &str) -> u64 {
    let mut array: Vec<(u32, usize, usize, usize)> = Vec::new();
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut matrix: Vec<Vec<bool>> = Vec::new();
    let mut row_number: usize = 0;
    let mut line_length: usize = 0;
    for line in input.lines() {
        line_length = line.len();
        matrix.push(Vec::new());
        for char in line.chars() {
            if char == '.' {
                matrix[row_number].push(false);
            } else if char.is_digit(10) {
                matrix[row_number].push(false);
            } else {
                matrix[row_number].push(true);
            }
        }
        for ma in re.find_iter(line) {
            array.push((u32::from(ma.as_str().chars().fold(None, |acc, ch| {
                ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
            }).unwrap()), row_number, ma.start(), ma.end()));
        }
        row_number += 1;
    }
    let mut big_sum: u64 = 0;
    line_length = line_length - 1;
    row_number = row_number - 1;
    for (number, row, start, end) in array {
        let mut is_valid = false;
        for pos in start..end {
            if (row > 0) && matrix[row - 1][pos] {
                is_valid = true;
            }
            if (row < row_number) && matrix[row + 1][pos] {
                is_valid = true;
            }
            if (pos > 0) && matrix[row][pos - 1] {
                is_valid = true;
            }
            if (pos < line_length) && matrix[row][pos + 1] {
                is_valid = true;
            }
            if (row > 0) && (pos > 0) && matrix[row - 1][pos - 1] {
                is_valid = true;
            }
            if (row > 0) && (pos < line_length) && matrix[row - 1][pos + 1] {
                is_valid = true;
            }
            if (row < row_number) && (pos > 0) && matrix[row + 1][pos - 1] {
                is_valid = true;
            }
            if (row < row_number) && (pos < line_length) && matrix[row + 1][pos + 1] {
                is_valid = true;
            }
        }
        if is_valid {
            big_sum += number as u64
        }
    }

    return big_sum;
}

pub fn part_b(input: &str) -> u64 {
    let mut array: Vec<(u32, usize, usize, usize)> = Vec::new();
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut matrix: Vec<Vec<bool>> = Vec::new();
    let mut map : Vec<Vec<Option<Vec<u32>>>> = Vec::new();
    let mut row_number: usize = 0;
    let mut line_length: usize = 0;
    for line in input.lines() {
        line_length = line.len();
        matrix.push(Vec::new());
        map.push(Vec::new());
        for char in line.chars() {
            if char == '.' {
                matrix[row_number].push(false);
                map[row_number].push(None);
            } else if char == '*' {
                map[row_number].push(Some(Vec::new()));
                matrix[row_number].push(true);

            } else if char.is_digit(10) {
                matrix[row_number].push(false);
                map[row_number].push(None);
            } else {
                map[row_number].push(None);
                matrix[row_number].push(true);
            }
        }
        for ma in re.find_iter(line) {
            array.push((ma.as_str().parse().unwrap(), row_number, ma.start(), ma.end()));
        }
        row_number += 1;
    }
    let mut big_sum: u64 = 0;
    line_length = line_length - 1;
    row_number = row_number - 1;
    for (number, row, start, end) in array {
        let mut gear_added = false;
        for pos in start..end {
            if (row > 0) && matrix[row - 1][pos] {
                if map[row-1][pos].is_some() && !gear_added {
                    gear_added = true;
                    map[row-1][pos].as_mut().unwrap().push(number);
                }

            }
            if (row < row_number) && matrix[row + 1][pos] {
                if map[row+1][pos].is_some() && !gear_added {
                    gear_added = true;
                    map[row+1][pos].as_mut().unwrap().push(number);
                }
            }
            if (pos > 0) && matrix[row][pos - 1] {
                if map[row][pos-1].is_some() && !gear_added {
                    gear_added = true;
                    map[row][pos-1].as_mut().unwrap().push(number);
                }
            }
            if (pos < line_length) && matrix[row][pos + 1] {
                if map[row][pos+1].is_some() && !gear_added {
                    gear_added = true;
                    map[row][pos+1].as_mut().unwrap().push(number);
                }
            }
            if (row > 0) && (pos > 0) && matrix[row - 1][pos - 1] {
                if map[row-1][pos-1].is_some() && !gear_added {
                    gear_added = true;
                    map[row-1][pos-1].as_mut().unwrap().push(number);
                }
            }
            if (row > 0) && (pos < line_length) && matrix[row - 1][pos + 1] {
                if map[row-1][pos+1].is_some() && !gear_added {
                    gear_added = true;
                    map[row-1][pos+1].as_mut().unwrap().push(number);
                }
            }
            if (row < row_number) && (pos > 0) && matrix[row + 1][pos - 1] {
                if map[row+1][pos-1].is_some() && !gear_added {
                    gear_added = true;
                    map[row+1][pos-1].as_mut().unwrap().push(number);
                }
            }
            if (row < row_number) && (pos < line_length) && matrix[row + 1][pos + 1] {
                if map[row+1][pos+1].is_some() && !gear_added {
                    gear_added = true;
                    map[row+1][pos+1].as_mut().unwrap().push(number);
                }
            }
        }
    }
    for vec in map  {
        for mut opt in vec {
            if opt.is_some() && opt.as_mut().unwrap().len() == 2 {
                big_sum += (opt.as_mut().unwrap()[0]*opt.as_mut().unwrap()[1]) as u64;
            }
        }
    }
    return big_sum;
}


pub fn main() {
    let input = std::fs::read_to_string("input/day03.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part_a(&input));
    println!("PART 2 = {}", part_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
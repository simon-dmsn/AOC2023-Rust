use regex::Regex;

pub fn part_a(input: &str) -> u64 {
    let mut array: Vec<(u32, usize, usize, usize)> = Vec::new();
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut matrix: Vec<Vec<bool>> = Vec::new();
    let mut row_number: usize = 0;
    let mut lineLength: usize = 0;
    for line in input.lines() {
        lineLength = line.len();
        matrix.push(Vec::new());
        for char in line.chars() {
            if (char == '.') {
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
    let mut bigSum: u64 = 0;
    lineLength = lineLength - 1;
    row_number = row_number - 1;
    for (number, mut row, start, end) in array {
        let mut isValid = false;
        for pos in start..end {
            if (row > 0) && matrix[row - 1][pos] {
                isValid = true;
            }
            if (row < row_number) && matrix[row + 1][pos] {
                isValid = true;
            }
            if (pos > 0) && matrix[row][pos - 1] {
                isValid = true;
            }
            if (pos < lineLength) && matrix[row][pos + 1] {
                isValid = true;
            }
            if (row > 0) && (pos > 0) && matrix[row - 1][pos - 1] {
                isValid = true;
            }
            if (row > 0) && (pos < lineLength) && matrix[row - 1][pos + 1] {
                isValid = true;
            }
            if (row < row_number) && (pos > 0) && matrix[row + 1][pos - 1] {
                isValid = true;
            }
            if (row < row_number) && (pos < lineLength) && matrix[row + 1][pos + 1] {
                isValid = true;
            }
        }
        if isValid {
            bigSum += number as u64
        }
    }

    return bigSum;
}

pub fn part_b(input: &str) -> u64 {
    let mut array: Vec<(u32, usize, usize, usize)> = Vec::new();
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut matrix: Vec<Vec<bool>> = Vec::new();
    let mut map : Vec<Vec<Option<Vec<u32>>>> = Vec::new();
    let mut rowNumber: usize = 0;
    let mut lineLength: usize = 0;
    for line in input.lines() {
        lineLength = line.len();
        matrix.push(Vec::new());
        map.push(Vec::new());
        let mut x: usize = 0;
        for char in line.chars() {
            if (char == '.') {
                matrix[rowNumber].push(false);
                map[rowNumber].push(None);
            } else if char == '*' {
                map[rowNumber].push(Some(Vec::new()));
                matrix[rowNumber].push(true);

            } else if char.is_digit(10) {
                matrix[rowNumber].push(false);
                map[rowNumber].push(None);
            } else {
                map[rowNumber].push(None);
                matrix[rowNumber].push(true);
            }
            x+=1;
        }
        for ma in re.find_iter(line) {
            array.push((ma.as_str().parse().unwrap(), rowNumber, ma.start(), ma.end()));
        }
        rowNumber += 1;
    }
    let mut bigSum: u64 = 0;
    lineLength = lineLength - 1;
    rowNumber = rowNumber - 1;
    for (number, row, start, end) in array {
        let mut gearAdded = false;
        for pos in start..end {
            if (row > 0) && matrix[row - 1][pos] {
                if(map[row-1][pos].is_some() && !gearAdded) {
                    gearAdded = true;
                    map[row-1][pos].as_mut().unwrap().push(number);
                }

            }
            if (row < rowNumber) && matrix[row + 1][pos] {
                if(map[row+1][pos].is_some() && !gearAdded) {
                    gearAdded = true;
                    map[row+1][pos].as_mut().unwrap().push(number);
                }
            }
            if (pos > 0) && matrix[row][pos - 1] {
                if(map[row][pos-1].is_some() && !gearAdded) {
                    gearAdded = true;
                    map[row][pos-1].as_mut().unwrap().push(number);
                }
            }
            if (pos < lineLength) && matrix[row][pos + 1] {
                if(map[row][pos+1].is_some() && !gearAdded) {
                    gearAdded = true;
                    map[row][pos+1].as_mut().unwrap().push(number);
                }
            }
            if (row > 0) && (pos > 0) && matrix[row - 1][pos - 1] {
                if(map[row-1][pos-1].is_some() && !gearAdded) {
                    gearAdded = true;
                    map[row-1][pos-1].as_mut().unwrap().push(number);
                }
            }
            if (row > 0) && (pos < lineLength) && matrix[row - 1][pos + 1] {
                if(map[row-1][pos+1].is_some() && !gearAdded) {
                    gearAdded = true;
                    map[row-1][pos+1].as_mut().unwrap().push(number);
                }
            }
            if (row < rowNumber) && (pos > 0) && matrix[row + 1][pos - 1] {
                if(map[row+1][pos-1].is_some() && !gearAdded) {
                    gearAdded = true;
                    map[row+1][pos-1].as_mut().unwrap().push(number);
                }
            }
            if (row < rowNumber) && (pos < lineLength) && matrix[row + 1][pos + 1] {
                if(map[row+1][pos+1].is_some() && !gearAdded) {
                    gearAdded = true;
                    map[row+1][pos+1].as_mut().unwrap().push(number);
                }
            }
        }
    }
    for vec in map  {
        for mut opt in vec {
            if(opt.is_some() && opt.as_mut().unwrap().len() == 2) {
                bigSum += (opt.as_mut().unwrap()[0]*opt.as_mut().unwrap()[1]) as u64;
            }
        }
    }
    return bigSum;
}


pub fn main() {
    let input = std::fs::read_to_string("input/day03.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part_a(&input));
    println!("PART 2 = {}", part_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
use std::cmp::{max, min};
use num::{Zero};

fn parse(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|l| l.chars().collect()).collect();
}

fn expand_lines(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    return lines.into_iter().fold(vec![], |mut acc, l| {
        if l.iter().filter(|c| { (**c) == '#' }).count().is_zero() {
            acc.push(l.clone());
        }
        acc.push(l);
        acc
    });
}

fn expand_lines_b(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let dup_line = vec!['D'; lines[0].len()];
    return lines.into_iter().fold(vec![], |mut acc, l| {
        if l.iter().filter(|c| { (**c) == '#' }).count().is_zero() {
            acc.push(dup_line.clone());
        } else {
            acc.push(l);
        }
        acc
    });
}


fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn count_all_pairs(matrix: Vec<Vec<char>>) -> u64 {
    let mut count_selection = 0usize;
    let mut big_count = 0usize;
    loop {
        let mut selected = false;
        let mut selection = (0usize, 0usize);
        let mut own_count = 0usize;

        for (x, line) in matrix.clone().into_iter().enumerate() {
            for (y, _char) in line.into_iter().enumerate().filter(|(_y, c)| *c == '#') {
                if selected {
                    big_count += max(x, selection.0) - min(x, selection.0) + max(y, selection.1) - min(y, selection.1);
                } else {
                    if own_count == count_selection {
                        selection = (x, y);
                        selected = true;
                        count_selection += 1;
                    } else {
                        own_count += 1;
                    }
                }
            }
        }
        if !selected {
            break;
        }
    }


    return big_count as u64;
}

fn count_steps_between_two_positions_b(matrix: Vec<Vec<char>>, pos: (usize, usize), pos1: (usize, usize)) -> usize {
    let mut number_of_d = 0usize;
    for (y, line) in matrix.into_iter().enumerate() {
        if y == pos1.1 {
            number_of_d += line.into_iter().enumerate().filter(|(index, c)| {
                if  (min(pos.0,pos1.0) < *index) && (*index < max(pos.0, pos1.0)) {
                    *c == 'D'
                }
                else {false}
            }).count()
        } else {
            if (pos.1 < y) && (y< pos1.1) {
                if line[pos.0] == 'D' {
                    number_of_d+=1;
                }
            }
        }
    }

    return number_of_d*999999 + max(pos.0, pos1.0) - min(pos.0, pos1.0) + max(pos.1, pos1.1) - min(pos.1, pos1.1);
}

fn count_all_pairs_b(matrix: Vec<Vec<char>>) -> u64 {
    let mut count_selection = 0usize;
    let mut big_count = 0u64;
    loop {
        let mut selected = false;
        let mut selection = (0usize, 0usize);
        let mut own_count = 0usize;


        for (y, line) in matrix.clone().into_iter().enumerate() {
            for (x, _char) in line.clone().into_iter().enumerate().filter(|(_y, c)| *c == '#') {
                if selected {
                    big_count += count_steps_between_two_positions_b(matrix.clone(), selection, (x, y)) as u64;
                } else {
                    if own_count == count_selection {
                        selection = (x, y);
                        selected = true;
                        count_selection += 1;
                    } else {
                        own_count += 1;
                    }
                }
            }
        }
        if !selected {
            break;
        }
    }


    return big_count;
}

fn solve_a(input: &str) -> u64 {
    let matrix = parse(input);
    let expand_matrix = expand_lines(transpose(expand_lines(matrix)));


    return count_all_pairs(expand_matrix);
}

fn solve_b(input: &str) -> u64 {
    let matrix = parse(input);
    let expand_matrix = expand_lines_b(transpose(expand_lines_b(matrix)));

    //expand_matrix.iter().for_each(|l| println!("{:?}", *l));
    return count_all_pairs_b(expand_matrix);
}


pub fn main() {
    let input = std::fs::read_to_string("input/day11.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
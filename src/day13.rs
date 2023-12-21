use std::ops::Add;
use itertools::Itertools;


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

fn parse(input: &str) -> (Vec<Vec<Vec<char>>>, Vec<Vec<Vec<char>>>) {
    let mut vec: Vec<Vec<Vec<char>>> = vec![vec![]];
    let mut pattern_count = 0usize;
    for l in input.lines() {
        if l.is_empty() {
            pattern_count += 1;
            vec.push(vec![]);
            continue;
        }
        //vec[pattern_count].push(l.parse().unwrap());
        vec[pattern_count].push(vec![]);
        l.chars().for_each(|c| vec[pattern_count].last_mut().unwrap().push(c));
    }

    return (vec.clone(), vec.into_iter().map(|pattern| transpose(pattern)).collect());
}

// return index of the line
fn find_symmetric_line_horiz(pattern: &Vec<Vec<char>>) -> usize {
    let mut index = 0;
    for (l, l1) in pattern.iter().tuple_windows() {
        if l.eq(l1) && helper_verify_symmetry(pattern, index + 1) {
            return index + 1;
        }
        index += 1;
    }


    return 0;
}

fn find_symmetric_line_horiz_with_smudge(pattern: &Vec<Vec<char>>,x : usize) -> usize {
    let mut index = 0;
    for (l, l1) in pattern.iter().tuple_windows() {
        if l.eq(l1) && helper_verify_symmetry_smudge(pattern, index + 1,x) {
            return index + 1;
        }
        index += 1;
    }


    return 0;
}

// true if valid
fn helper_verify_symmetry(pattern: &Vec<Vec<char>>, line_index: usize) -> bool {
    assert!(line_index > 0);
    let mut valid;
    let mut start = line_index - 1;
    let mut second = line_index;

    loop {
        if !pattern[start].eq(&pattern[second]) {
            return false;
        }
        valid = (start > 0) && (second < pattern.len() - 1);
        if !valid { break; }
        start -= 1;
        second += 1;
    }
    return true;
}

fn helper_verify_symmetry_smudge(pattern: &Vec<Vec<char>>, line_index: usize,x:usize) -> bool {
    assert!(line_index > 0);
    let mut valid;
    let mut start = line_index - 1;
    let mut second = line_index;
    let mut can_be_valid = false;

    loop {
        if x == start || x == second {
            can_be_valid = true;
        }
        if !pattern[start].eq(&pattern[second]) {
            return false;
        }
        valid = (start > 0) && (second < pattern.len() - 1);
        if !valid { break; }
        start -= 1;
        second += 1;
    }
    return can_be_valid;
}



fn solve_a(input: &str) -> u64 {
    let (vec_patterns, t_vec_patterns) = parse(input);
    return vec_patterns.iter().fold(0, |acc, pat| {
        acc + 100 * (find_symmetric_line_horiz(pat))
    }).add(t_vec_patterns.iter().fold(0, |acc, pat| {
        acc + find_symmetric_line_horiz(pat)
    })) as u64;
}

fn solve_b(input: &str) -> u64 {
    let (mut vec_patterns, mut t_vec_patterns) = parse(input);
    let len_patterns = vec_patterns.len();
    let mut index_pattern = 0;
    vec_patterns.append(&mut t_vec_patterns);
    let mut pattern_used_list = vec![false;len_patterns+1]; // shift to go from 1 to len instead of 0 to len
    return vec_patterns.iter_mut().fold(0, |acc, mut pat| {
        index_pattern += 1;
        let mut symmetric_line = 0;
        for x in 0..pat.len() {
            for y in 0..pat[0].len() {
                match pat[x][y] {
                    '.' => pat[x][y] = '#',
                    '#' => pat[x][y] = '.',
                    _ => unreachable!()
                }
                symmetric_line = find_symmetric_line_horiz_with_smudge(pat,x);
                if symmetric_line != 0 {
                    if index_pattern > len_patterns && !pattern_used_list[index_pattern%len_patterns] {
                        return acc + symmetric_line
                    } else {
                        pattern_used_list[index_pattern] = true;
                        return acc + 100 * (symmetric_line)
                    }
                } else { // reverse the change
                    match pat[x][y] {
                        '.' => pat[x][y] = '#',
                        '#' => pat[x][y] = '.',
                        _ => unreachable!()
                    }
                }
            }
        }
        acc
    }) as u64;


    return 0;
}

#[allow(dead_code)]
fn fact(p0: usize) -> u64 {
    if p0 == 0 { return 0; } // special case for counting
    return if p0 > 1 {
        (p0 as u64) * fact(p0 - 1)
    } else { 1 };
}


pub fn main() {
    let input = std::fs::read_to_string("input/day13.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
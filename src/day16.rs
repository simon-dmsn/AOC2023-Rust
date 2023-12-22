use std::cmp::max;
use std::thread;

fn parse(input: &str) -> Vec<Vec<char>> {
    return input.lines().fold(vec![], |mut acc, l| {
        acc.push(l.chars().collect());
        acc
    });
}

fn solve_a(input: &str) -> u64 {
    let mut vec = parse(input);
    let mut is_energized = vec![vec![(false, false, false, false, false); vec[0].len()]; vec.len()];
    rec_solve(&vec, &mut is_energized, 0, 0, 0);
    return is_energized.into_iter().map(|l| l.iter().filter(|b| (**b).0).count() as u64).sum();
}

fn helper_b(vec: &Vec<Vec<char>>,x : usize,y :usize,dir:usize) -> u64 {
    let mut is_energized = vec![vec![(false, false, false, false, false); vec[0].len()]; vec.len()];
    rec_solve(&vec, &mut is_energized, x, y, dir);
    return is_energized.into_iter().map(|l| l.iter().filter(|b| (**b).0).count() as u64).sum();
}

fn rec_solve(vec: &Vec<Vec<char>>, is_energized: &mut Vec<Vec<(bool, bool, bool, bool, bool)>>, x: usize, y: usize, direction: usize) { // Left=0, Up=1,Right=2,Down=3
    is_energized[y][x].0 = true;
    match direction {
        0 => if is_energized[y][x].1 { return; } else { is_energized[y][x].1 = true; }
        1 => if is_energized[y][x].2 { return; } else { is_energized[y][x].2 = true; }
        2 => if is_energized[y][x].3 { return; } else { is_energized[y][x].3 = true; }
        3 => if is_energized[y][x].4 { return; } else { is_energized[y][x].4 = true; },
        _ => { unreachable!() }
    }
    match vec[y][x] {
        '.' => {
            match direction {
                0 => { if x < vec[0].len() - 1 { rec_solve(vec, is_energized, x + 1, y, direction) } }
                1 => { if y < vec.len() - 1 { rec_solve(vec, is_energized, x, y + 1, direction) } }
                2 => { if x > 0 { rec_solve(vec, is_energized, x - 1, y, direction) } }
                3 => { if y > 0 { rec_solve(vec, is_energized, x, y - 1, direction) } }
                _ => { unreachable!() }
            }
        }
        '/' => {
            match direction {
                3 => { if x < vec[0].len() - 1 { rec_solve(vec, is_energized, x + 1, y, 0) } }
                2 => { if y < vec.len() - 1 { rec_solve(vec, is_energized, x, y + 1, 1) } }
                1 => { if x > 0 { rec_solve(vec, is_energized, x - 1, y, 2) } }
                0 => { if y > 0 { rec_solve(vec, is_energized, x, y - 1, 3) } }
                _ => { unreachable!() }
            }
        }
        '\\' => {
            match direction {
                1 => { if x < vec[0].len() - 1 { rec_solve(vec, is_energized, x + 1, y, 0) } }
                0 => { if y < vec.len() - 1 { rec_solve(vec, is_energized, x, y + 1, 1) } }
                3 => { if x > 0 { rec_solve(vec, is_energized, x - 1, y, 2) } }
                2 => { if y > 0 { rec_solve(vec, is_energized, x, y - 1, 3) } }
                _ => { unreachable!() }
            }
        }
        '|' => {
            match direction {
                0 | 2 => {
                    if y > 0 { rec_solve(vec, is_energized, x, y - 1, 3) }
                    if y < vec.len() - 1 { rec_solve(vec, is_energized, x, y + 1, 1) }
                } // *2
                1 => { if y < vec.len() - 1 { rec_solve(vec, is_energized, x, y + 1, direction) } }
                3 => { if y > 0 { rec_solve(vec, is_energized, x, y - 1, direction) } }
                _ => { unreachable!() }
            }
        }
        '-' => {
            match direction {
                0 => { if x < vec[0].len() - 1 { rec_solve(vec, is_energized, x + 1, y, direction) } }
                1 | 3 => {
                    if x < vec[0].len() - 1 { rec_solve(vec, is_energized, x + 1, y, 0) }
                    if x > 0 { rec_solve(vec, is_energized, x - 1, y, 2) }
                } // *2
                2 => { if x > 0 { rec_solve(vec, is_energized, x - 1, y, direction) } }
                _ => { unreachable!() }
            }
        }
        _ => { unreachable!() }
    }
}

fn solve_b(input: &str) -> u64 {
    let mut vec = parse(input);
    let mut max_num = 0u64;
    for i in 0..vec.len() {
        max_num = max(max_num, helper_b(&vec,0,i,0));
        max_num = max(max_num, helper_b(&vec,vec[0].len()-1,i,2));
        max_num = max(max_num, helper_b(&vec,i,0,1));
        max_num = max(max_num, helper_b(&vec,i,vec.len()-1,3));
    }
    return max_num;
}

fn huge_reduction() {
    let builder = thread::Builder::new()
        .name("reductor".into())
        .stack_size(32 * 1024 * 1024); // 32MB of stack space
    let handler = builder.spawn(|| {
        answer();
    }).unwrap();

    handler.join().unwrap();
}


pub fn main() {
    huge_reduction();
}

fn answer() {
    let input= std::fs::read_to_string("input/day16.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
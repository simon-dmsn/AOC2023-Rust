use std::collections::HashMap;


fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let moves: HashMap<char, Vec<char>> = HashMap::from([
        ('|', vec!['S', 'N']), // South, West, North, East
        ('-', vec!['W', 'E']),
        ('L', vec!['N', 'E']),
        ('J', vec!['N', 'W']),
        ('7', vec!['W', 'S']),
        ('F', vec!['E', 'S']),
        ('S', vec!['S', 'W', 'N', 'E', 'Z']),
        ('.', vec![]),
    ]);
    let vec: Vec<Vec<Vec<char>>> = input.lines().map(|l| { l.chars().map(|c| moves.get(&c).unwrap().clone()).collect() }).collect();

    return vec;
}

fn solve_a(input: &str) -> u64 {
    let matrix: Vec<Vec<Vec<char>>> = parse_input(input);

    let vec_s = vec!['S', 'W', 'N', 'E', 'Z'];
    let x_s = matrix.iter().position(|line| line.iter().find(|v| (**v).eq(&vec_s)).is_some()).unwrap();
    let y_s = matrix[x_s].iter().position(|char| (*char).eq(&vec_s)).unwrap();

    let mut ptr_tunnel = vec_s.clone();
    let mut count_moves = 0;
    let mut pos = (x_s, y_s);

    let mut last_use = '.';
    loop {
        for char in ptr_tunnel.clone() {
            if char == last_use { continue; }
            if char == 'N' && pos.0 > 0 && matrix[pos.0 - 1][pos.1].contains(&'S') {
                ptr_tunnel = matrix[pos.0 - 1][pos.1].clone();
                pos.0 -= 1;
                last_use = 'S';

                break;
            } else if char == 'S' && pos.0 < (matrix.len() - 1) && matrix[pos.0 + 1][pos.1].contains(&'N') {
                ptr_tunnel = matrix[pos.0 + 1][pos.1].clone();
                pos.0 += 1;
                last_use = 'N';
                break;
            } else if char == 'W' && pos.1 > 0 && matrix[pos.0][pos.1 - 1].contains(&'E') {
                ptr_tunnel = matrix[pos.0][pos.1 - 1].clone();
                pos.1 -= 1;
                last_use = 'E';
                break;
            } else if char == 'E' && pos.1 < (matrix.len() - 1) && matrix[pos.0][pos.1 + 1].contains(&'W') {
                ptr_tunnel = matrix[pos.0][pos.1 + 1].clone();
                pos.1 += 1;
                last_use = 'W';
                break;
            }
        }
        count_moves += 1;
        if ptr_tunnel == vec_s {
            break;
        }
    }


    return count_moves / 2;
}

fn solve_b(input: &str) -> u64 {
    let matrix: Vec<Vec<Vec<char>>> = parse_input(input);
    let mut mat_bool: Vec<Vec<u8>> = vec![vec![0; matrix.len()]; matrix.len()];

    let vec_s = vec!['S', 'W', 'N', 'E', 'Z'];
    let x_s = matrix.iter().position(|line| line.iter().find(|v| (**v).eq(&vec_s)).is_some()).unwrap();
    let y_s = matrix[x_s].iter().position(|char| (*char).eq(&vec_s)).unwrap();

    let mut ptr_tunnel = vec_s.clone();
    let mut pos = (x_s, y_s);
    mat_bool[pos.0][pos.1] = 1;

    let mut last_use = '.';
    loop {
        for char in ptr_tunnel.clone() {
            if char == last_use { continue; }
            if char == 'N' && pos.0 > 0 && matrix[pos.0 - 1][pos.1].contains(&'S') {
                ptr_tunnel = matrix[pos.0 - 1][pos.1].clone();
                pos.0 -= 1;
                last_use = 'S';
                if matrix[pos.0][pos.1].contains(&'S') || matrix[pos.0][pos.1].contains(&'N') {
                    mat_bool[pos.0][pos.1] = 1;
                } else {
                    mat_bool[pos.0][pos.1] = 3;
                }
                break;
            } else if char == 'S' && pos.0 < (matrix.len() - 1) && matrix[pos.0 + 1][pos.1].contains(&'N') {
                ptr_tunnel = matrix[pos.0 + 1][pos.1].clone();
                pos.0 += 1;
                last_use = 'N';
                if matrix[pos.0][pos.1].contains(&'S') || matrix[pos.0][pos.1].contains(&'N') {
                    mat_bool[pos.0][pos.1] = 1;
                } else {
                    mat_bool[pos.0][pos.1] = 3;
                }
                break;
            } else if char == 'W' && pos.1 > 0 && matrix[pos.0][pos.1 - 1].contains(&'E') {
                ptr_tunnel = matrix[pos.0][pos.1 - 1].clone();
                pos.1 -= 1;
                last_use = 'E';
                if matrix[pos.0][pos.1].contains(&'S') || matrix[pos.0][pos.1].contains(&'N') {
                    mat_bool[pos.0][pos.1] = 1;
                } else {
                    mat_bool[pos.0][pos.1] = 3;
                }
                break;
            } else if char == 'E' && pos.1 < (matrix.len() - 1) && matrix[pos.0][pos.1 + 1].contains(&'W') {
                ptr_tunnel = matrix[pos.0][pos.1 + 1].clone();
                pos.1 += 1;
                last_use = 'W';
                if matrix[pos.0][pos.1].contains(&'S') || matrix[pos.0][pos.1].contains(&'N') {
                    mat_bool[pos.0][pos.1] = 1;
                } else {
                    mat_bool[pos.0][pos.1] = 3;
                }
                break;
            }
        }
        if ptr_tunnel == vec_s {
            break;
        }
    }

    // now we have to check for the point inside the loop formed by all the true in the mat_bool
    let mut count = 0;
    mat_bool.iter().for_each(|f|{f.iter().for_each(|b| {print!("{} ",*b) });println!(""); });



    for (_x, line) in mat_bool.into_iter().enumerate() {
        let mut is_inner =false;
        for y in line {
            if y == 1 {
                is_inner = !is_inner;
            } else if is_inner && y == 0 {
                count +=1;
            }

        }
    }


    return count as u64;
}


pub fn main() {
    let input = std::fs::read_to_string("input/day10.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
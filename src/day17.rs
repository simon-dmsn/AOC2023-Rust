use std::cmp::min;



fn parse(input: &str) -> Vec<Vec<u64>> {
    return input.lines().map(|f| { f.chars().map(|c| { c.to_digit(10).unwrap() as u64 }).collect() }).collect();
}

fn solve_a(input: &str) -> u64 {

    return solve_dp(input,3,0);
}

fn solve_b(input: &str) -> u64 {
    return solve_dp(input,10,4);
}

fn solve_dp (input : &str, max_number_of_moves : usize, min_number_of_moves_before_stop : usize) -> u64 {
    let grid = parse(input);
    let mut dp = vec![vec![vec![vec![u64::MAX - 10; 11]; 4]; grid[0].len()]; grid.len()]; // MAX - 10 just for overflow problem in the DP

    dp[1][0][1][1] = grid[1][0];
    dp[0][1][0][1] = grid[0][1];
    // k = Direction : 0=Left,1=Down,2=Right,3=Up | x = Moves : 1,2,3
    let mut is_updating = true; // first run to true
    while is_updating {
        is_updating = false;
        for i in 0..dp.len() {
            for j in 0..dp[0].len() {
                for x in 1..max_number_of_moves+1 {
                    for k in 0..4usize {
                        if (i < dp.len() - 1 && k == 3 && x < max_number_of_moves) || (i < dp.len() - 1 && k != 1 && k != 3 && x >= min_number_of_moves_before_stop) {
                            let dir_added = 3; //UP

                            if k != 3 {
                                let temp = dp[i][j][dir_added][1];
                                dp[i][j][dir_added][1] = min(dp[i][j][dir_added][1], dp[i + 1][j][k][x] + grid[i][j]);
                                is_updating = is_updating || temp!=dp[i][j][dir_added][1];
                            } else {
                                let temp = dp[i][j][dir_added][1];
                                dp[i][j][dir_added][x+1] = min(dp[i][j][dir_added][x+1], dp[i + 1][j][k][x] + grid[i][j]);
                                is_updating = is_updating || temp!=dp[i][j][dir_added][1];                            }
                        }
                        if (i > 0 && k == 1 && x < max_number_of_moves) || (i > 0 && k != 3 && k != 1 && x >= min_number_of_moves_before_stop) {
                            let dir_added = 1; // DOWN

                            if k != 1 {
                                let temp = dp[i][j][dir_added][1];
                                dp[i][j][dir_added][1] = min(dp[i][j][dir_added][1], dp[i - 1][j][k][x] + grid[i][j]);
                                is_updating = is_updating || temp!=dp[i][j][dir_added][1];
                            } else {
                                let temp = dp[i][j][dir_added][x+1];
                                dp[i][j][dir_added][x + 1] = min(dp[i][j][dir_added][x + 1], dp[i - 1][j][k][x] + grid[i][j]);
                                is_updating = is_updating || temp!=dp[i][j][dir_added][x+1];
                            }
                        }
                        if (j > 0 && k == 0 && x < max_number_of_moves) || (j > 0 && k != 2 && k != 0 && x >= min_number_of_moves_before_stop) {
                            let dir_added = 0; // LEFT
                            if k != 0 {
                                let temp = dp[i][j][dir_added][1];
                                dp[i][j][dir_added][1] = min(dp[i][j][dir_added][1], dp[i][j - 1][k][x] + grid[i][j]);
                                is_updating = is_updating || temp!=dp[i][j][dir_added][1];
                            } else {
                                let temp = dp[i][j][dir_added][x+1];
                                dp[i][j][dir_added][x + 1] = min(dp[i][j][dir_added][x + 1], dp[i][j - 1][k][x] + grid[i][j]);
                                is_updating = is_updating || temp!=dp[i][j][dir_added][x+1];
                            }
                        }
                        if (j < dp.len() - 1 && k == 2 && x < max_number_of_moves) || (j < dp.len() - 1 && k != 0 && k != 2 && x >= min_number_of_moves_before_stop) {
                            let dir_added = 2; // RIGHT

                            if k != 2 {
                                let temp = dp[i][j][dir_added][1];
                                dp[i][j][dir_added][1] = min(dp[i][j][dir_added][1], dp[i][j + 1][k][x] + grid[i][j]);
                                is_updating = is_updating || temp!=dp[i][j][dir_added][1];
                            } else {
                                let temp = dp[i][j][dir_added][x+1];
                                dp[i][j][dir_added][x + 1] = min(dp[i][j][dir_added][x + 1], dp[i][j + 1][k][x] + grid[i][j]);
                                is_updating = is_updating || temp!=dp[i][j][dir_added][x+1];
                            }
                        }
                    }
                }
            }
        }
    }

    let mut ret = u64::MAX;
    for x in min_number_of_moves_before_stop..max_number_of_moves+1 {
        for k in 0..4usize {
            ret = min(ret, dp[dp.len() - 1][dp.len() - 1][k][x])
        }
    }


    return ret;
}


pub fn main() {
    let input = std::fs::read_to_string("input/day17.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}

fn solve_a(input: &str) -> u64 {



    return 0;
}

fn solve_b(input: &str) -> u64 {



    return 0;
}



pub fn main() {
    let input = std::fs::read_to_string("input/dayXX.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
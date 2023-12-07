use itertools::Itertools;

fn parse_input_a(input : &str) -> Vec<(u64, u64)> {
    let mut time = vec![];
    let mut distance = vec![];

    let mut first_l = true;
    for line in input.lines() {
        for num in line.split_whitespace().skip(1) {
            if first_l {
                time.push(num.parse::<u64>().unwrap());
            } else {
                distance.push(num.parse::<u64>().unwrap());
            }
        }
        first_l = false;
    }

    return time.into_iter().zip(distance.into_iter()).collect();
}

fn parse_input_b(input : &str) -> (u64,u64) {
    let mut tuple: (u64,u64) = (0,0);

    let mut first_l = true;
    for line in input.lines() {
        let l : u64 = line.split_whitespace().skip(1).join("").parse().unwrap(); {
            if first_l {
                tuple.0 = l;
            } else {
                tuple.1 = l;
            }
        }
        first_l = false;
    }

    return tuple;
}
fn beat_record_verification (time_holding : u64,course_length : u64, record_to_beat : u64) -> bool {

    let time_remaining = course_length - time_holding;
    return record_to_beat < time_remaining*time_holding;

}
pub fn solve_a(input : &str) -> u64 {
    let races = parse_input_a(input);
    let mut mult : u64 = 1;
    for (time,distance) in races {
        let mut ways: u64 = 0;
        for i in 0..(time+1) {
            if beat_record_verification(i,time,distance) {
                ways+=1;
            }
        }
        mult *= ways;
    }
    return mult;

}
pub fn solve_b(input : &str) -> u64 {
    let (time,distance) = parse_input_b(input);

    let mut first : u64 = 0;
    for i in 0..(time+1) {
        if beat_record_verification(i,time,distance) {
            first = i;
            break;
        }
    }
    let mut last : u64 = 0;
    for i in (0..time+1).rev() {
        if beat_record_verification(i,time,distance) {
            last = i;
            break;
        }
    }

    return last - first + 1;
}

pub fn main() {
    let input = std::fs::read_to_string("input/day06.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
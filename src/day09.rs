use num::abs;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut vec = vec![];

    let mut index = 0;
    for line in input.lines() {
        vec.push(vec![]);
        for num in line.split_whitespace() {
            vec[index].push(num.parse::<i64>().unwrap());
        }
        index += 1;
    }
    return vec;
}

fn get_next_number(mut line: Vec<i64>) -> i64 {
    let mut full_of_zeros = false;
    let mut sum = line.last().unwrap().clone();

    while !full_of_zeros {
        let mut temp_vec = vec![];
        let mut count_of_zeros = 0;

        temp_vec = line.windows(2).map(|wind| {
            let t = wind[1] - wind[0];
            if(t==0) { count_of_zeros +=1};
            t
        }).collect();
        line = temp_vec.to_owned();
        sum += line.last().unwrap();
        if count_of_zeros == line.len() {full_of_zeros = true;}
    }
    return sum;
}

fn solve_a(input: &str) -> i64 {
    let mut inp = parse_input(input);

    let mut big_sum = 0;
    for line in inp {
        big_sum += get_next_number(line);
    }


    return big_sum;
}

fn solve_b(input: &str) -> i64 {
    let mut inp = parse_input(input);

    let mut big_sum = 0;
    for mut line in inp {

        line.reverse();
        big_sum += get_next_number(line);
    }


    return big_sum;
}

pub fn main() {
    let input = std::fs::read_to_string("input/day09.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
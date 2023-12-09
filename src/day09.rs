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

fn solve_a(input: &str) -> i64 {
    let mut inp = parse_input(input);

    let mut big_sum = 0;
    for line in inp {
        println!("{:?}",line);
        let mut full_of_zeros = false;
        let mut vec = line;
        let mut sum = vec.last().unwrap().clone();

        while !full_of_zeros {
            let mut temp_vec = vec![];
            let mut count = 0;
            for i in 0..vec.len()-1 {
                let dif = vec[i+1] -vec[i];
                if dif == 0 {count+=1};
                temp_vec.push(dif);

            }
            vec = temp_vec.to_owned();
            sum += vec.last().unwrap();
            if count == vec.len() {full_of_zeros = true;}
        }
        println!("nombre proposé : {}",sum);
        big_sum += sum;
    }


    return big_sum;
}

fn solve_b(input: &str) -> i64 {
    let mut inp = parse_input(input);

    let mut big_sum = 0;
    for line in inp {
        println!("{:?}",line);
        let mut full_of_zeros = false;
        let mut vec = line;
        let mut sum = vec.first().unwrap().clone();

        while !full_of_zeros {
            let mut temp_vec = vec![];
            let mut count = 0;
            for i in 0..vec.len()-1 {
                let dif = vec[i+1] -vec[i];
                if dif == 0 {count+=1};
                temp_vec.push(dif);

            }
            vec = temp_vec.to_owned();
            sum -= vec.first().unwrap();
            if count == vec.len() {full_of_zeros = true;}
        }
        println!("nombre proposé : {}",sum);
        big_sum += sum;
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
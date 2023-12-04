pub fn part_a(input: &str) -> u64 {
    let mut big_sum = 0;
    for line in input.lines() {
        let spliting = line.split_once('|').unwrap();
        let winning_numbers = spliting.0.split_once(':').unwrap().1;
        let my_numbers = spliting.1;
        let mut tab: Vec<u32> = Vec::new();
        for number in winning_numbers.split_whitespace() {
            tab.push(number.parse::<u32>().unwrap());
        }
        let points = my_numbers.split_whitespace().fold::<u32, _>(1, |acc, number| {
            if tab.contains(&number.parse::<u32>().unwrap()) {
                2 * acc
            } else {
                acc
            }
        }) / 2;


        big_sum += points as u64;
    }


    return big_sum;
}

pub fn part_b(input: &str) -> u64 {
    let amount_of_lines = input.lines().count();
    let mut array_of_cards: Vec<u32> = vec![1; amount_of_lines];
    let mut index = 0;
    for line in input.lines() {
        let spliting = line.split_once('|').unwrap();
        let winning_numbers = spliting.0.split_once(':').unwrap().1;
        let my_numbers = spliting.1;
        let mut tab: Vec<u32> = Vec::new();
        for number in winning_numbers.split_whitespace() {
            tab.push(number.parse::<u32>().unwrap());
        }
        let new_cards = my_numbers.split_whitespace().fold::<usize, _>(0, |acc, number| {
            if tab.contains(&number.parse::<u32>().unwrap()) {
                acc + 1
            } else {
                acc
            }
        });

        for i in 1..(new_cards + 1) {
            array_of_cards[index + i] += array_of_cards[index];
        }

        index += 1;
    }

    //let sum : u32 = array_of_cards.iter().sum();

    return array_of_cards.iter().fold(0,|acc,x| acc+x) as u64;
}

pub fn main() {
    let input = std::fs::read_to_string("input/day04.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part_a(&input));
    println!("PART 2 = {}", part_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
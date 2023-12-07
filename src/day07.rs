use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(Vec<u32>, u32)> {
    let mut vec = vec![];

    for line in input.lines() {
        let mut cards = vec![];
        for char in line.split_once(' ').unwrap().0.chars().filter(|c| !c.is_whitespace()) {
            match char {
                'A' => { cards.push(14) }
                'K' => { cards.push(13) }
                'Q' => { cards.push(12) }
                'J' => { cards.push(11) }
                'T' => { cards.push(10); }
                _ => { cards.push(char.to_digit(10).unwrap()); }
            }
        }
        let bet: u32 = line.split_once(' ').unwrap().1.parse().unwrap();
        vec.push((cards, bet));
    }

    return vec;
}

fn parse_input_b(input: &str) -> Vec<(Vec<u32>, u32)> {
    let mut vec = vec![];

    for line in input.lines() {
        let mut cards = vec![];
        for char in line.split_once(' ').unwrap().0.chars().filter(|c| !c.is_whitespace()) {
            match char {
                'A' => { cards.push(14) }
                'K' => { cards.push(13) }
                'Q' => { cards.push(12) }
                'J' => { cards.push(1) }
                'T' => { cards.push(10); }
                _ => { cards.push(char.to_digit(10).unwrap()); }
            }
        }
        let bet: u32 = line.split_once(' ').unwrap().1.parse().unwrap();
        vec.push((cards, bet));
    }

    return vec;
}


fn get_power_cards(vec: Vec<(Vec<u32>, u32)>) -> Vec<(u32, Vec<u32>, u32)> {
    let mut ret_vec = vec![];
    for (mut cards, bet) in vec {
        let copy_cards = cards.clone();

        cards.sort_unstable();
        let count = cards.into_iter().dedup_with_count();
        let sorted = count.sorted_unstable_by_key(|(n, count)| { *n });
        let mut pair = 0;
        let mut brelan = false;
        let mut carre = false;
        let mut cinq = false;
        for (count, n) in sorted {
            match count {
                2 => pair += 1,
                3 => brelan = true,
                4 => carre = true,
                5 => cinq = true,
                _ => {}
            }
        }
        if cinq {
            ret_vec.push((6, copy_cards, bet));
        } else if carre {
            ret_vec.push((5, copy_cards, bet));
        } else if brelan && pair == 1 {
            ret_vec.push((4, copy_cards, bet));
        } else if brelan {
            ret_vec.push((3, copy_cards, bet));
        } else if pair == 2 {
            ret_vec.push((2, copy_cards, bet));
        } else if pair == 1 {
            ret_vec.push((1, copy_cards, bet));
        } else {
            ret_vec.push((0, copy_cards, bet));
        }
    }
    return ret_vec;
}

fn get_power_cards_b(vec: Vec<(Vec<u32>, u32)>) -> Vec<(u32, Vec<u32>, u32)> {
    let mut ret_vec = vec![];
    for (mut cards, bet) in vec {
        let copy_cards = cards.clone();

        cards.sort_unstable();
        let count_1 = cards.into_iter().dedup_with_count();
        let sorted = count_1.sorted_unstable_by_key(|(count, n)| { *count });
        let number_j = sorted.clone().find(|(n,count)| *count == 1 ).or(Some((0,2))).unwrap().0;
        let mut pair = 0;
        let mut brelan = false;
        let mut carre = false;
        let mut cinq = false;
        let mut first= true;
        //println!("{:?}",sorted);
        for (mut count, n) in sorted.rev() {
            if(n!=1) {
                if first {
                    count += number_j;
                    first = false;
                }
                match count {
                    2 => pair += 1,
                    3 => brelan = true,
                    4 => carre = true,
                    5 => cinq = true,
                    _ => {}
                }
            } else if count == 5 {
                cinq = true;
            }
        }
        if cinq {
            ret_vec.push((6, copy_cards, bet));
        } else if carre {
            ret_vec.push((5, copy_cards, bet));
        } else if brelan && pair == 1 {
            ret_vec.push((4, copy_cards, bet));
        } else if brelan {
            ret_vec.push((3, copy_cards, bet));
        } else if pair == 2 {
            ret_vec.push((2, copy_cards, bet));
        } else if pair == 1 {
            ret_vec.push((1, copy_cards, bet));
        } else {
            ret_vec.push((0, copy_cards, bet));
        }
    }
    println!("{:?}",ret_vec);
    return ret_vec;
}

fn compare_same_rank(mut input: Vec<(Vec<u32>, u32)>) -> Vec<(u32, u32)> { // return vec of rank,bet
    //input is just cards and bet
    input.sort_unstable_by(|this, that| {
        let (cards_1, bet_1) = this;
        let (cards_2, bet_2) = that;
        for i in 0..cards_1.len() {
            if cards_1[i] > cards_2[i] {
                return cards_1[i].cmp(&cards_2[i]);
            } else if cards_1[i] < cards_2[i] {
                return cards_1[i].cmp(&cards_2[i]);
            }
        }
        cards_1[0].cmp(&cards_2[0])
    });
    let mut ind: u32 = 0;

    return input.into_iter().map(|(cards, bet)| {
        ind += 1;
        (ind, bet)
    }).collect();
}

fn calcul_result(ranks: Vec<Vec<(u32, u32)>>) -> u64 {
    let mut big_sum : u64 = 0;
    let mut shift: u32 = 0;
    for rank in ranks {
        for (rank, bet) in &rank {
            big_sum += ((*rank+shift)*(*bet)) as u64;
        }
        shift += rank.len() as u32;
    }
    return big_sum;
}

fn solve_a(input: &str) -> u64 {
    let input_vec = get_power_cards(parse_input(input));
    let mut power_ranks: Vec<Vec<(Vec<u32>, u32)>> = vec![vec![]; 7];

    for (rank, cards, bet) in input_vec {
        power_ranks[rank as usize].push((cards, bet));
    }

    let mut ranks: Vec<Vec<(u32, u32)>> = vec![vec![]; 7];

    for i in 0..7 {
        ranks[i] = compare_same_rank(power_ranks[i].clone());
    }


    return calcul_result(ranks);
}


fn solve_b(input: &str) -> u64 {
    let input_vec = get_power_cards_b(parse_input_b(input));
    let mut power_ranks: Vec<Vec<(Vec<u32>, u32)>> = vec![vec![]; 7];

    for (rank, cards, bet) in input_vec {
        power_ranks[rank as usize].push((cards, bet));
    }

    let mut ranks: Vec<Vec<(u32, u32)>> = vec![vec![]; 7];

    for i in 0..7 {
        ranks[i] = compare_same_rank(power_ranks[i].clone());
    }


    return calcul_result(ranks);
}

pub fn main() {
    let input = std::fs::read_to_string("input/day07.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
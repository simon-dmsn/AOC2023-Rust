use std::collections::{HashMap, HashSet};
use num::integer::lcm;

fn parse_input(input: &str) -> (Vec<u32>, HashMap<&str,[&str;2]>) {
    let mut vec = vec![];
    let line1 = input.lines().next().unwrap(); {
        for char in line1.chars() {
            match char {
                'L' => vec.push(0),
                'R' => vec.push(1),
                _ => unreachable!()
            }
        }
    }



    let mut map = HashMap::new();
    for line in input.lines().skip(2) {
        let key = line.split_once(' ').unwrap().0;


        let left = line.split_once('(').unwrap().1.split_once(',').unwrap().0;
        let right = line.split_once('(').unwrap().1.split_once(' ').unwrap().1
            .split_once(')').unwrap().0;

        map.insert(key,[left,right]);
    }

    return (vec,map);
}

fn how_many_steps(from : &str, to : &str, map : HashMap<&str,[&str;2]>,path : Vec<u32>) -> u64 {
    let mut it = from;
    let mut index : usize = 0;
    let mut count : u64 = 0;
    while  it != to {
        index = index % path.len();
        it = (*(map.get(it).unwrap()))[path[index] as usize];
        index+=1;
        count+=1;
    }

    return count;
}

fn how_many_steps_par(start_list : Vec<&str>, end_set : HashSet<&str>, map : HashMap<&str,[&str;2]>,path : Vec<u32>) -> u64 {
    println!("{:?}",start_list);
    println!("{:?}",end_set);
    let mut finished : usize = 0;
    let mut it_list : Vec<&str> = start_list.clone();
    let mut index : usize = 0;
    let mut count : u64 = 0;
    while(finished != start_list.len()) {
        finished = 0;
        index = index % path.len();
        for i in 0..start_list.len()  {
            it_list[i] = (*(map.get(it_list[i]).unwrap()))[path[index] as usize];
            if end_set.contains(&it_list[i]) {
                finished += 1;
            }
        }
        index+=1;
        count+=1;

    }

    return count;
}


fn solve_a(input : &str) -> u64 {
    let (path,map) = parse_input(input);

    return how_many_steps("AAA","ZZZ",map,path);
}

fn solve_b(input : &str) -> u64 {
    let (path,map) = parse_input(input);
    let map_start_end = HashMap::from([
        ("AAA", "ZZZ"),
        ("RPA", "BJZ"),
        ("VCA", "GLZ"),
        ("FRA", "JGV"),
        ("SNA","JPZ"),
        ("HNA","VSZ")
    ]);
    let mut num = vec![];
    for (key,value) in map_start_end  {
        num.push(how_many_steps(key,value,map.clone(),path.clone()));
    }

    return num.into_iter().fold(1,|acc,x| lcm(acc,x));
}

pub fn main() {
    let input = std::fs::read_to_string("input/day08.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
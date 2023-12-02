#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]


const MAX_CUBES: [u32; 3] = [12, 13, 14];

pub fn valid_games(input : &str) -> u32 {
    input.lines().map( |line : &str|{
        let id = line.split_once(':').unwrap().0.chars()
            .skip_while(|ch| !ch.is_digit(10))
            .take_while(|ch| ch.is_digit(10))
            .fold(None, |acc, ch| {
                ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
            }).unwrap();
        //println!("{}",id);
        let cubes = line.split_once(':').unwrap().1;
        let mut is_possible = true;
        for throw in cubes.split(';') {

            for color in throw.split(',') {
                let mut iterator = color.split_whitespace();
                let number = iterator.next().unwrap().parse::<u32>().unwrap();
                let colour = iterator.next().unwrap();
                match colour {
                    "red" => if number > MAX_CUBES[0] { is_possible = false;}
                    "green" => if number > MAX_CUBES[1] { is_possible = false;}
                    "blue" => if number > MAX_CUBES[2] { is_possible = false;},
                    _ => unreachable!()
                }
            }


        }
        if is_possible {
            id
        } else {
            0
        }
    }).sum()

}

pub fn minimum_power(input :&str) -> u32 {
    input.lines().map( |line : &str|{
        let cubes = line.split_once(':').unwrap().1;
        let mut max_r:u32 = 0;
        let mut max_b:u32 = 0;
        let mut max_g:u32 = 0;
        for throw in cubes.split(';') {

            for color in throw.split(',') {
                let mut iterator = color.split_whitespace();
                let number = iterator.next().unwrap().parse::<u32>().unwrap();
                let colour = iterator.next().unwrap();
                match colour {
                    "red" => if number > max_r { max_r = number;}
                    "green" => if number > max_g { max_g = number;}
                    "blue" => if number > max_b { max_b = number;},
                    _ => unreachable!()
                }
            }


        }
        //println!("R: {} G: {} B: {}", max_r, max_g, max_b);
        max_r * max_g * max_b
    }).sum()

}

pub fn main() {
    let input = std::fs::read_to_string("input/day02.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", valid_games(&input));
    println!("PART 2 = {}", minimum_power(&input));
    println!("Execution time: {:?}", now.elapsed());
}
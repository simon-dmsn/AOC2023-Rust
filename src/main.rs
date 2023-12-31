#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

#[allow(clippy::wildcard_imports)]
use adventofcode::*;

fn main() {
    let mains = [
        day01::main,
        day02::main,
        day03::main,
        day04::main,
        day05::main,
        day06::main,
        day07::main,
        day08::main,
        day09::main,
        //day10::main,
        day11::main,
        day12::main,
        // day13::main,
        // day14::main,
        day15::main,
        // day16::main,
        day17::main,
        day18::main,
        // day19::main,
        // day20::main,
        // day21::main,
        // day23::main,
        // day24::main,
        // day25::main,
    ];

    let now = std::time::Instant::now();

    for (day, main) in mains.iter().enumerate() {
        println!(
            "------------------------------------ DAY {} ------------------------------------",
            day + 1
        );
        main();
        println!();
    }

    println!("------------------------------------  ALL   ------------------------------------");
    println!("Execution time: {:?}\n", now.elapsed());
}
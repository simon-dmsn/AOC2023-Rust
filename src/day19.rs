use std::collections::HashMap;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

struct Rules {
    // {'x','m','a','s'}
    vec_rules: Vec<String>,
}

impl Rules {
    pub fn get_direction_with_a_part(&self, part: &Part) -> String {
        //with the String of the all rules (drop the } at the end
        // we can get which condition the part is corresponding
        return  self.vec_rules.iter().fold_while(String::new(), |_acc, rule| {
            if !rule.contains(':') {
                Done(rule.to_owned())
            } else {
                let mut chars = rule.chars();
                let letter = chars.next().unwrap();
                let comparator = chars.next().unwrap();
                let number_to_comp: u64 = chars.fold_while(0, |acc, c| {
                    if c.is_digit(10) {
                        Continue(acc * 10 + c.to_digit(10).unwrap() as u64)
                    } else {
                        Done(acc)
                    }
                }).into_inner();
                let direction = rule.split_once(':').unwrap().1;
                // now let's check if our part is good for this condition
                let is_valid = match comparator {
                    '<' => {
                        match letter {
                            'x' => part.x < number_to_comp,
                            'm' => part.m < number_to_comp,
                            'a' => part.a < number_to_comp,
                            's' => part.s < number_to_comp,
                            _ => { unreachable!() }
                        }
                    }
                    '>' => {
                        match letter {
                            'x' => part.x > number_to_comp,
                            'm' => part.m > number_to_comp,
                            'a' => part.a > number_to_comp,
                            's' => part.s > number_to_comp,
                            _ => { unreachable!() }
                        }
                    }
                    _ => { unreachable!() }
                };
                if is_valid {
                    Done(direction.to_owned())
                } else {
                    Continue(String::new())
                }
            }
        }).into_inner();
    }
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}
impl Part {
    pub fn sum(&self) -> u64 { return self.s + self.x + self.m + self.a; }
}

fn parse(input: &str) -> (HashMap<&str, Rules>, Vec<Part>) {
    let vec_parts = input.lines().skip_while(|l| (*l).len() != 0).skip(1).map(|l| {
        let mut splitter = l.split('=').skip(1);
        Part {
            x: splitter.next().unwrap().split_once(',').unwrap().0.parse().unwrap(),
            m: splitter.next().unwrap().split_once(',').unwrap().0.parse().unwrap(),
            a: splitter.next().unwrap().split_once(',').unwrap().0.parse().unwrap(),
            s: splitter.next().unwrap().split_once('}').unwrap().0.parse().unwrap(),
        }
    }).collect();
    let hash_map: HashMap<&str, Rules> = input.lines().filter(|l| !(*l).is_empty() && (*l).strip_prefix("{").is_none()).map(|l| {
        let (key, rules) = l.split_once('{').unwrap_or(("", ""));
        let rules_struct = Rules {
            vec_rules: rules.split(',').map(|rule| {
                rule.split_once('}').unwrap_or((rule, "")).0.to_string()
            }).collect()
        };
        (key, rules_struct)
    }).collect();


    return (hash_map, vec_parts);
}

fn solve_a(input: &str) -> u64 {
    let (hash, vec) = parse(input);

    return vec.iter().fold(0, |acc, part| {
        let mut result = String::from("in");
        while result != "A" && result != "R" { // let's check if this syntax works
            result = hash.get(result.as_str()).unwrap().get_direction_with_a_part(part); // gérer le moved
        }
        if result == "A" {
            acc + part.sum()
        } else {
            acc
        }
    });
}

fn solve_b(input: &str) -> u64 {
    return 0;
}


pub fn main() {
    let input = std::fs::read_to_string("input/day19.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
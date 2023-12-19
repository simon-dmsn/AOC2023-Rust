 enum Dir {
    LEFT = 0,
    RIGHT = 1,
    UP = 2,
    DOWN = 3,
}

fn parse_a(input: &str) -> Vec<(Dir,i64)> {

    return input.lines().map(|f| {
        let mut l = f.split_whitespace();
        ( match l.next().unwrap() {"L"=> Dir::LEFT, "R" => Dir::RIGHT, "U" => Dir::UP, "D" => Dir::DOWN, _ => unreachable!()},l.next().unwrap().parse().unwrap())
    }).collect()
}

 fn parse_b(input: &str) -> Vec<(Dir,i64)> {

     return input.lines().map(|f| {
         let mut l = f.split_once('#').unwrap().1;
         let split = l.split_at(5);
         ( match split.1.get(0..1).unwrap() {"2"=> Dir::LEFT, "0" => Dir::RIGHT, "3" => Dir::UP, "1" => Dir::DOWN, _ => unreachable!()},i64::from_str_radix(split.0,16).unwrap())
     }).collect()
 }
fn solve(parsed_input : Vec<(Dir,i64)>) -> u64 {
    let mut digged_places = parsed_input;
    let mut perimeter: i64 = 0;
    let mut updating_area = 0;
    let mut actual_pos = (0,0);
    for (dir,length) in digged_places {
        let (x,y) = match dir {
            Dir::DOWN => (0,length),
            Dir::LEFT => (-length,0),
            Dir::RIGHT => (length,0),
            Dir::UP =>  (0,-length)
        };
        actual_pos.0 += x;
        actual_pos.1 += y;
        perimeter+= length;
        updating_area += actual_pos.0 * y;
    }



    return (updating_area + perimeter/2 + 1) as u64;
}

 fn solve_a(input : &str) -> u64 {
     return solve(parse_a(input));
 }

fn solve_b(input: &str) -> u64 {

    return solve(parse_b(input));
}



pub fn main() {
    let input = std::fs::read_to_string("input/day18.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}
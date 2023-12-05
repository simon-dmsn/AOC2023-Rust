use std::ops::Range;
use rangemap::RangeMap;
use lazy_regex::regex;


fn apply_map(number: u64, map: &Vec<(u64, u64, u64)>) -> (u64,u64) {
    for (dest_r, start_r, length) in map{
        if number >= *start_r && number < *start_r + *length {
            return ((number - *start_r) + *dest_r,*length+*start_r-number);
        }
    }

    return (number,u64::MAX);
}

fn parse_seeds(input: &str) -> Vec<u64> {
    input.split_once(':').unwrap().1.split_whitespace().map(|string| string.parse::<u64>().unwrap()).collect()
}

fn parse_maps_seeds(input: &str) -> (Vec<u64>,  Vec<Vec<(u64, u64, u64)>>) {
    let mut ind = 0;
    let mut seeds = vec![];
    let mut ind_maps: usize = 0;
    let mut maps : Vec<Vec<(u64,u64,u64)>> = Vec::new();
    for line in input.lines() {
        if ind == 0 {
            seeds = parse_seeds(line);
        } else {
            if line.is_empty() {}
            else if line.split_whitespace().next().unwrap_or("je").parse::<u64>().is_err() {
                maps.push(Vec::new());
                ind_maps+=1;
            } else {
                let mut l = line.split_whitespace();
                let tuple : (u64,u64,u64) = (l.next().unwrap().parse::<u64>().unwrap(),
                                             l.next().unwrap().parse::<u64>().unwrap(),
                                             l.next().unwrap().parse::<u64>().unwrap());
                maps[ind_maps-1].push(tuple);
            }
        }
        ind += 1;
    }
    return (seeds, maps.clone());
}

fn parse_maps(input: &str) -> Vec<RangeMap<i64,i64>> {
    let mut ind = 0;
    let mut seeds = vec![];
    let mut ind_maps: usize = 0;
    let mut maps : Vec<RangeMap<i64,i64>> = Vec::new();
    for line in input.lines() {
        if ind == 0 {
            seeds = parse_seeds(line);
        } else {
            if line.is_empty() {}
            else if line.split_whitespace().next().unwrap_or("je").parse::<u64>().is_err() {
                maps.push(RangeMap::new());
                ind_maps+=1;
            } else {
                let mut l = line.split_whitespace();
                let tuple : (i64,i64,i64) = (l.next().unwrap().parse::<i64>().unwrap(),
                                             l.next().unwrap().parse::<i64>().unwrap(),
                                             l.next().unwrap().parse::<i64>().unwrap());
                maps[ind_maps-1].insert(tuple.1..tuple.1 + tuple.2,tuple.0 - tuple.1);
            }
        }
        ind += 1;
    }
    return maps;
}

fn map_range(inputs: &mut Vec<Range<i64>>, map: &RangeMap<i64, i64>) -> Vec<Range<i64>> {
    let mut output = Vec::new();
    while let Some(input) = inputs.pop() {
        if map.overlaps(&input) {
            for (range, offset) in map.overlapping(&input) {
                let start = std::cmp::max(input.start, range.start);
                let end = std::cmp::min(input.end, range.end);
                output.push(start + offset..end + offset);
                //if input.start < start { // USELESS BECAUSE map.overlapping iter over all the overlaps.
                //   inputs.push(input.start..start);
                //}
                //if end < input.end {
                //    inputs.push(end..input.end);
                //}
            }
        } else {
            output.push(input);
        }
    }
    output
}




pub fn part_a(input : &str) -> u64 {
    let parser = parse_maps_seeds(input);
    let seeds = parser.0;
    let maps_r = parser.1;
    let mut locations = vec![];
    for seed in seeds {

        let mut loc: u64 = seed;
        for i in 0..(*maps_r).len() {
            loc = apply_map(loc, maps_r.get(i).unwrap()).0;
        }
        locations.push(loc);
    }
    return *locations.iter().min().unwrap();



}


pub fn part_b(input : &str) -> u64 {
    //let parser = parse_maps_seeds(input);
    //let seeds = parser.0;
    //let maps_r = parser.1;
    ////let mut locations = vec![];
    //let mut min_loc : u64 = u64::MAX;
    //for i in 0..(seeds.len()-1) {
    //    let start = seeds.chunks_exact()[i];
    //    let range = seeds[i+1];
    //    let mut s = start;
    //    while s  < start+range {
    //        let mut loc: u64 = s;
    //        let mut len: u64 = u64::MAX;
    //        for i in 0..(*maps_r).len() {
    //            let temp = apply_map(loc, maps_r[i].clone());
    //            loc = temp.0;
    //            len = min(len,temp.1);
    //        }
    //        min_loc = min(min_loc,loc);
    //        s += len;
    //
    //    }
    //    //println!("{}",min_loc);
    //}
    //return min_loc;          // 7000s but work ! Let's try to improve it

    //let seeds = parse_maps_seeds(input);
    //
    //let mut min = u32::MAX;
    //for seed in seeds.0.chunks_exact(2) {
    //    for mut seed in seed[0]..seed[0] + seed[1] {
    //        for map in &seeds.1 {
    //            seed = apply_map(seed,map).0;
    //        }
    //
    //        min = min.min(seed as u32);
    //    }
    //}
    //
    //min.into()
    let mut seeds = Vec::new();
    let seed_line = input.lines().next().unwrap();
    for cap in regex!(r"(\d+) (\d+)").captures_iter(seed_line) {
        let start = cap[1].parse::<i64>().unwrap();
        let length = cap[2].parse::<i64>().unwrap();
        seeds.push(start..start + length);
    }
    for map in parse_maps(input) {
        seeds = map_range(&mut seeds, &map);
    }
    seeds
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap() as u64

}




pub fn main() {
    let input = std::fs::read_to_string("input/day05.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part_a(&input));
    println!("PART 2 = {}", part_b(&input));
    println!("Execution time: {:?}", now.elapsed());
}

fn hash_value(input: &str) -> u64 {
    return input.chars().fold(0, |mut acc, char| {
        acc += char as u64;
        acc *= 17;
        acc %= 256;
        acc
    });
}

fn solve_a(input: &str) -> u64 {
    return input.trim().split(',').fold(0, |acc, word| {
        acc + hash_value(word)
    });
}

fn solve_b(input: &str) -> u64 {
    let mut hashmap : Vec<Vec<(&str,u64)>> = vec![vec![];256];

    input.trim().split(',').for_each(|lens_order| {
        let ind_split = lens_order.find(['=','-']).unwrap();
        let split = lens_order.get(ind_split..ind_split + 1).unwrap().parse::<char>().unwrap();
        let (label,power) = lens_order.split_once(split).unwrap();
        let hash_label = hash_value(label) as usize;

        if split == '=' {
            if let Some(index) = hashmap[hash_label].iter().position(|x| x.0 == label) { //maybe *
                hashmap[hash_label][index] = (label,power.parse::<u64>().unwrap());
            } else {
                hashmap[hash_label].push((label,power.parse::<u64>().unwrap()));
            }


            } else {
             if let Some(index) = hashmap[hash_label].iter().position(|x| x.0 == label) { //maybe *
                hashmap[hash_label].remove(index);
            }

        }
    });

    return hashmap.into_iter().enumerate().fold(0,|acc,(index_box,boxe)| {
        boxe.into_iter().enumerate().fold(0,|acc1,(index,focal)| {
            acc1 + (index_box+1)*(index+1)*(focal.1 as usize)
        }) + acc
    }) as u64 ;
}


pub fn main() {
    let input = std::fs::read_to_string("input/day15.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_a(&input));
    println!("PART 2 = {}", solve_b(&input));
    println!("Execution time: {:?}", now.elapsed());

}
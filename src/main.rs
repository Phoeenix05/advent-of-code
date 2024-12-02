use std::collections::HashMap;

fn main() {
    let input: &'static str = include_str!("input.txt");

    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|n| n.parse::<u32>())
                .map(|n| n.unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    let mut map: HashMap<u32, u32> = HashMap::new();
    right.iter().for_each(|n| {
        // why does derefencing the map work???
        *map.entry(*n).or_insert(0) += 1;
    });

    let similarity: i32 = left
        .iter()
        .map(|n| (*n as i32 * *map.get_key_value(n).unwrap_or((&0u32, &0u32)).1 as i32).abs())
        .sum();
    println!("{}", similarity);
}

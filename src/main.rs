fn main() {
    let input: &'static str = include_str!("input.txt");

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|n| n.parse::<u32>())
                .map(|n| n.unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    left.sort();
    right.sort();

    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (*l as i32 - *r as i32).abs())
        .sum();

    println!("{}", sum);
}

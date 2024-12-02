use nom::{
    character::{
        complete::u32,
        streaming::{line_ending, space1},
    },
    combinator::map,
    multi::many1,
    sequence::tuple,
    IResult,
};

fn parse_line(input: &'static str) -> IResult<&'static str, (u32, u32)> {
    map(tuple((u32, space1, u32, line_ending)), |(l, _, r, _)| {
        (l, r)
    })(input)
}

fn main() {
    let input: &'static str = include_str!("input.txt");

    let (_, pairs) = many1(parse_line)(input).unwrap();

    // initialize two columns with the initial size of `n`
    let n = 1000;
    let mut lcol: Vec<u32> = Vec::with_capacity(n);
    let mut rcol: Vec<u32> = Vec::with_capacity(n);

    // separate pairs to columns
    for (l, r) in pairs {
        lcol.push(l);
        rcol.push(r);
    }

    // sort columns from smallest to largest
    lcol.sort();
    rcol.sort();

    // create a vector of size `n` to store the distances
    let mut distance: Vec<i32> = Vec::with_capacity(n);

    for i in 0..lcol.len() {
        println!("{} - {}", lcol[i], rcol[i]);
        distance.push(lcol[i] as i32 - rcol[i] as i32);
    }

    let sum = distance.iter().sum::<i32>();
    println!("{}", sum);
}

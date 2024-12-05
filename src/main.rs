use nom::{
    character::complete::{char, digit1},
    combinator::{map, map_res},
    complete::tag,
    multi::many0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

fn parse_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse::<i32>()),
        char(','),
        map_res(digit1, |s: &str| s.parse::<i32>()),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many0(parse_pair)(input)
}

fn main() {
    let input: &'static str = include_str!("input.txt");

    let (_, expr) = parse(input).unwrap();
}

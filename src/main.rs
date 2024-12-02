use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

fn parse_line(input: &'static str) -> IResult<&'static str, Vec<i32>> {
    terminated(
        separated_list1(tag(" "), map_res(digit1, str::parse::<i32>)),
        multispace0,
    )(input)
}

fn parse_file(input: &'static str) -> IResult<&'static str, Vec<Vec<i32>>> {
    many1(parse_line)(input)
}

fn main() {
    let input: &'static str = include_str!("input.txt");

    let (_, reports) = parse_file(input).unwrap();

    // just using `>` should filter out all duplicate numbers,
    // as `a` < `b` means `b` cannot `b` same as `a`
    let ascending = reports
        .clone()
        .into_iter()
        .filter(|r| {
            r.windows(2).all(|w| w[0] < w[1]) && r.windows(2).all(|w| (w[0] - w[1]).abs() <= 3)
        })
        .count();
    let descending = reports
        .clone()
        .into_iter()
        .filter(|r| {
            r.windows(2).all(|w| w[0] > w[1]) && r.windows(2).all(|w| (w[0] - w[1]).abs() <= 3)
        })
        .count();

    let count = ascending + descending;
    println!("{}", count);
}

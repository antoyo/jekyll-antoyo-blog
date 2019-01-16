extern crate nom;

use std::str::FromStr;

use nom::{call, digit, error_position, many1, map_res, named, recognize, sep, wrap_sep, ws};
use nom::types::CompleteStr;

named!(numbers<CompleteStr, Vec<i64>>,
    many1!(ws!(
        map_res!(recognize!(digit), |complete_str: CompleteStr| i64::from_str(&*complete_str))
    ))
);

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    let (_input, nums) = numbers(line.as_str().into()).unwrap();
    let sum: i64 = nums.iter().sum();
    println!("Sum: {}", sum);
}

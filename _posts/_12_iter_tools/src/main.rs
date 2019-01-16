extern crate itertools;

use std::io::BufRead;

use itertools::fold;

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let iter =
        line.split_whitespace()
            .map(|word| word.parse::<i64>().unwrap());
    let sum = fold(iter, 0, |a, b| a + b);
    println!("Sum: {}", sum);
}

#![feature(proc_macro_hygiene)]

extern crate lib;

use lib::scanfold;

fn main() {
    let sum = scanfold!("%ld+", |num: i64, acc: i64| num + acc);
    println!("Sum: {}", sum);
}

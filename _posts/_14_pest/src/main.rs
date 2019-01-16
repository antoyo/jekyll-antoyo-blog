extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "nums.pest"]
struct Nums;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let pairs = Nums::parse(Rule::nums, &line).unwrap();
    let mut sum = 0;
    for pair in pairs {
        for word in pair.into_inner() {
            let num = word.as_str().parse::<i64>().unwrap();
            sum += num;
        }
    }
    println!("Sum: {}", sum);
}

extern crate error_chain;

use std::io;
use std::num::ParseIntError;

use error_chain::*;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(io::Error);
        Parse(ParseIntError);
    }
}

fn main() -> Result<()> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let mut sum = 0;
    for word in line.split_whitespace() {
        let num = word.parse::<i64>()?;
        sum += num;
    }
    println!("Sum: {}", sum);
    Ok(())
}

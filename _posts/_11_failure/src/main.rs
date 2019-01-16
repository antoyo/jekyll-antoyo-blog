extern crate failure;
extern crate failure_derive;

use std::result;

use failure::bail;
use failure_derive::Fail;

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "empty string")]
    Empty,
}

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let mut sum = 0;
    let mut empty = true;
    for word in line.split_whitespace() {
        empty = false;
        let num = word.parse::<i64>()?;
        sum += num;
    }
    if empty {
        bail!(Error::Empty);
    }
    println!("Sum: {}", sum);
    Ok(())
}

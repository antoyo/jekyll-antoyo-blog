use std::fmt::{self, Display, Formatter};
use std::io;
use std::num::ParseIntError;
use std::result;

macro_rules! from {
    ($from:ty, $typ:ty, $result:expr) => {
        impl From<$from> for $typ {
            fn from(error: $from) -> Self {
                $result(error)
            }
        }
    };
}

macro_rules! display {
    ($typ:ty, [$($patterns:path),*]) => {
        impl Display for $typ {
            fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
                match *self {
                    $($patterns(ref error) => error.fmt(formatter)),*
                }
            }
        }
    };
}

macro_rules! error {
    ($typ:ty) => {
        impl ::std::error::Error for $typ {
        }
    };
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Parse(ParseIntError),
}

display!(Error, [Error::Io, Error::Parse]);
error!(Error);
from!(io::Error, Error, Error::Io);
from!(ParseIntError, Error, Error::Parse);

type Result<T> = result::Result<T, Error>;

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

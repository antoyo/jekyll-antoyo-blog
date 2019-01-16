// High score (total number of dependencies): 99

#![feature(generators, proc_macro_hygiene, try_trait)]

extern crate error_chain;
extern crate futures_await as futures;
extern crate lazy_static;
extern crate rayon;
extern crate serde;
extern crate serde_derive;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_fs;
extern crate tokio_io;
extern crate tokio_threadpool;

use std::collections::VecDeque;
use std::fmt::Display;
use std::io;
use std::num::ParseIntError;
use std::option::NoneError;
use std::sync::Mutex;

use futures::prelude::{async, await};
use lazy_static::lazy_static;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;
use serde::{Deserialize, Deserializer};
use serde::de::{self, DeserializeSeed, SeqAccess, Visitor};
use tokio::prelude::Future;
use tokio_codec::{FramedRead, LinesCodec};
use tokio_fs::{stdin, stdout};
use tokio_io::io::write_all;
use tokio_threadpool::Builder;

use error_chain::*;

lazy_static! {
    static ref NUMS: Mutex<VecDeque<i64>> = Mutex::new(VecDeque::new());
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(io::Error);
        Parse(ParseIntError);
    }

    errors {
        Async {
            description("async error")
            display("useful async error")
        }
        EmptyVec {
            description("empty vec")
            display("empty vec")
        }
    }
}

impl From<()> for Error {
    fn from((): ()) -> Self {
        ErrorKind::Async.into()
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        ErrorKind::EmptyVec.into()
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        msg.to_string().into()
    }
}

#[async]
fn future() -> Result<()> {
    let stdin = FramedRead::new(stdin(), LinesCodec::new());
    #[async]
    for line in stdin {
        let nums: Vec<i64> = from_str(&line)?;
        let sum: i64 = nums.iter().sum();
        await!(write_all(stdout(), format!("Sum: {}\n", sum).into_bytes()))?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let pool = Builder::new().pool_size(1).build();
    pool.spawn(future().map_err(|_| ()));
    pool.shutdown_on_idle().wait()?;
    Ok(())
}

pub fn from_str<T>(s: &str) -> Result<T>
where for<'a> T: Deserialize<'a>,
{
    let mut deserializer = Decoder::from_str(s.to_string())?;
    let t = T::deserialize(&mut deserializer)?;
    if NUMS.lock().unwrap().is_empty() {
        Ok(t)
    }
    else {
        Err(serde::de::Error::custom("empty input"))
    }
}

struct Decoder {
}

impl Decoder {
    fn from_str(input: String) -> Result<Self> {
        let nums: Result<VecDeque<i64>> = input.par_split_whitespace().map(|word| word.parse::<i64>().map_err(|err| err.into()))
            .collect();
        *NUMS.lock().unwrap() = nums?;
        Ok(Self {
        })
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut Decoder {
    type Error = Error;

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de> {
        visitor.visit_i64(NUMS.lock().unwrap().pop_front()?)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de> {
        visitor.visit_seq(Struct { de: self })
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!(); }
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!(); }
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
}

struct Struct<'a> {
    de: &'a mut Decoder,
}

impl<'de, 'a> SeqAccess<'de> for Struct<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as serde::de::DeserializeSeed<'de>>::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if NUMS.lock().unwrap().is_empty() {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

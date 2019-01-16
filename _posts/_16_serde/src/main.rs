extern crate serde;
extern crate serde_derive;

use std::collections::VecDeque;
use std::num::ParseIntError;

use serde::{Deserialize, Deserializer};
use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::de::value::Error;

pub fn from_str<T>(s: &str) -> Result<T, Error>
where for<'a> T: Deserialize<'a>,
{
    let mut deserializer = Decoder::from_str(s.to_string()).unwrap();
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.nums.is_empty() {
        Ok(t)
    }
    else {
        use serde::de::Error;
        Err(Error::custom("empty input"))
    }
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let nums: Vec<i64> = from_str(&line).unwrap();
    let sum: i64 = nums.iter().sum();
    println!("Sum: {}", sum);
}

struct Decoder {
    nums: VecDeque<i64>,
}

impl Decoder {
    fn from_str(input: String) -> Result<Self, ParseIntError> {
        let nums: Result<VecDeque<i64>, ParseIntError> = input.split_whitespace().map(|word| word.parse::<i64>())
            .collect();
        Ok(Self {
            nums: nums?,
        })
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut Decoder {
    type Error = Error;

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_i64(self.nums.pop_front().unwrap())
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_seq(Struct { de: self })
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!(); }
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!(); }
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> { unimplemented!() }
}

struct Struct<'a> {
    de: &'a mut Decoder,
}

impl<'de, 'a> SeqAccess<'de> for Struct<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.nums.is_empty() {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

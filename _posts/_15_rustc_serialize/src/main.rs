extern crate rustc_serialize;

use std::collections::VecDeque;
use std::num::ParseIntError;

use rustc_serialize::Decodable;

#[derive(RustcDecodable)]
struct Nums {
    nums: Vec<i64>,
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let nums: Nums = decode(line).unwrap();
    let sum: i64 = nums.nums.iter().sum();
    println!("Sum: {}", sum);
}

struct Decoder {
    nums: VecDeque<i64>,
}

impl Decoder {
    fn new(input: String) -> Result<Self, ParseIntError> {
        let nums: Result<VecDeque<i64>, ParseIntError> = input.split_whitespace().map(|word| word.parse::<i64>())
            .collect();
        Ok(Self {
            nums: nums?,
        })
    }
}

impl rustc_serialize::Decoder for Decoder {
    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        self.nums.pop_front().ok_or(())
    }

    fn read_struct<T, F>(&mut self, _struct_name: &str, _len: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        f(self)
    }

    fn read_struct_field<T, F>(&mut self, _f_name: &str, _f_idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        f(self)
    }

    fn read_seq<T, F>(&mut self, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error> {
        let len = self.nums.len();
        f(self, len)
    }

    fn read_seq_elt<T, F>(&mut self, _idx: usize, f: F) -> Result<T, Self::Error>
    where
        F: FnOnce(&mut Self) -> Result<T, Self::Error>
    {
        f(self)
    }

    type Error = ();
    fn read_nil(&mut self) -> Result<(), Self::Error> { unimplemented!() }
    fn read_usize(&mut self) -> Result<usize, Self::Error> { unimplemented!() }
    fn read_u64(&mut self) -> Result<u64, Self::Error> { unimplemented!() }
    fn read_u32(&mut self) -> Result<u32, Self::Error> { unimplemented!() }
    fn read_u16(&mut self) -> Result<u16, Self::Error> { unimplemented!() }
    fn read_u8(&mut self) -> Result<u8, Self::Error> { unimplemented!() }
    fn read_isize(&mut self) -> Result<isize, Self::Error> { unimplemented!() }
    fn read_i32(&mut self) -> Result<i32, Self::Error> { unimplemented!() }
    fn read_i16(&mut self) -> Result<i16, Self::Error> { unimplemented!() }
    fn read_i8(&mut self) -> Result<i8, Self::Error> { unimplemented!() }
    fn read_bool(&mut self) -> Result<bool, Self::Error> { unimplemented!() }
    fn read_f64(&mut self) -> Result<f64, Self::Error> { unimplemented!() }
    fn read_f32(&mut self) -> Result<f32, Self::Error> { unimplemented!() }
    fn read_char(&mut self) -> Result<char, Self::Error> { unimplemented!() }
    fn read_str(&mut self) -> Result<String, Self::Error> { unimplemented!() }
    fn read_enum<T, F>(&mut self, _name: &str, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> { unimplemented!() }
    fn read_enum_variant<T, F>(&mut self, _names: &[&str], _f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, usize) -> Result<T, Self::Error> { unimplemented!() }
    fn read_enum_variant_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> { unimplemented!() }
    fn read_enum_struct_variant<T, F>(&mut self, _names: &[&str], _f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, usize) -> Result<T, Self::Error> { unimplemented!() }
    fn read_enum_struct_variant_field<T, F>(&mut self, _f_name: &str, _f_idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> { unimplemented!() }
    fn read_tuple<T, F>(&mut self, _len: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> { unimplemented!() }
    fn read_tuple_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> { unimplemented!() }
    fn read_tuple_struct<T, F>(&mut self, _s_name: &str, _len: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> { unimplemented!() }
    fn read_tuple_struct_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> { unimplemented!() }
    fn read_option<T, F>(&mut self, _f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, bool) -> Result<T, Self::Error> { unimplemented!() }
    fn read_map<T, F>(&mut self, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error> { unimplemented!() }
    fn read_map_elt_key<T, F>(&mut self, _idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> { unimplemented!() }
    fn read_map_elt_val<T, F>(&mut self, _idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> { unimplemented!() }
    fn error(&mut self, _err: &str) -> Self::Error { unimplemented!() }
}

fn decode<T: Decodable>(string: String) -> Option<T> {
    let mut decoder = Decoder::new(string).unwrap();
    Decodable::decode(&mut decoder).ok()
}

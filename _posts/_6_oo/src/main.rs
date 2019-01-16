use std::ops::Deref;

struct Nums {
    vector: Vec<i64>,
}

impl Nums {
    fn new() -> Self {
        Nums {
            vector: Vec::new(),
        }
    }

    fn add(&mut self, num: i64) {
        self.vector.push(num);
    }

    fn get_sum(&self) -> i64 {
        (*self).iter().sum()
    }
}

impl Deref for Nums {
    type Target = Vec<i64>;

    fn deref(&self) -> &Self::Target {
        &self.vector
    }
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut nums = Nums::new();
    for word in line.split_whitespace() {
        let num = word.parse::<i64>().unwrap();
        nums.add(num);
    }
    println!("Sum: {}", nums.get_sum());
}

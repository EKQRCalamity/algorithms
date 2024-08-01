extern crate random;
use random::lcg;
use djb2::Djb2fn;
use fnv1a::FNV1Afn;

mod djb2;
mod fnv1a;

struct HashCollisionChecker {
    values: Vec<u32>,
}

impl HashCollisionChecker {
    pub fn new() -> HashCollisionChecker {
        HashCollisionChecker {
            values: Vec::new(),
        }
    }

    pub fn add_value(&mut self, i: u32) {
        self.values.push(i);
    }

    pub fn check_collision(&mut self, hash: u32) -> bool {

        for &value in &self.values {
            if hash == value {
                return true;
            }
        }



        self.add_value(hash);

        false
    }
}

fn u128_to_str(i: u128) -> String {
    let mut int = i;
    let mut output = String::new();

    while int > 0 {
        // (lcg::LCG::new().random_between(30, 70).unwrap() as u8)
        let digit: u8 = ((int % 10) as u32) as u8 + 41;
        output.push(char::from_u32(digit as u32).unwrap());

        int /= 10;
    }

    return output;
}

fn main() {
    let mut hashcollector = HashCollisionChecker::new();
    let mut rangen = lcg::LCG::new();
    let mut i = 0;
    loop {
        i += 1;
        let random = rangen.random();
        let hash = u128_to_str(random).fnv1a();
        println!("[{}]: Random: {} - Hash: {}", i, u128_to_str(random), hash);
        if hashcollector.check_collision(hash) {
            println!("Hash Collision Detected!");
            break;
        }
        if i == 200000000 {
            break
        }
    }

}

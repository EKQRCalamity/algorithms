
use std::fmt::{self, Display};

use random::lcg::LCG;
use hashes::djb2::Djb2fn;
use hashes::cityhash64::CityHash64fn;
use hashes::fnv1a::FNV1Afn;

#[derive(PartialEq)]
#[allow(dead_code)]
enum HashValue {
    None,
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Float32(f32),
    Float64(f64),
    String(String),
    Str(&'static str),
}

impl fmt::Display for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashValue::None => write!(f, ""),
            HashValue::UInt8(value) => write!(f, "{}", value),
            HashValue::UInt16(value) => write!(f, "{}", value),
            HashValue::UInt32(value) => write!(f, "{}", value),
            HashValue::UInt64(value) => write!(f, "{}", value),
            HashValue::UInt128(value) => write!(f, "{}", value),
            HashValue::Int8(value) => write!(f, "{}", value),
            HashValue::Int16(value) => write!(f, "{}", value),
            HashValue::Int32(value) => write!(f, "{}", value),
            HashValue::Int64(value) => write!(f, "{}", value),
            HashValue::Int128(value) => write!(f, "{}", value),
            HashValue::Float32(value) => write!(f, "{}", value),
            HashValue::Float64(value) => write!(f, "{}", value),
            HashValue::String(value) => write!(f, "{}", value),
            HashValue::Str(value) => write!(f, "{}", value),
        }
    }
}

struct HashChecker {
    value_type: HashValue,
    values: Vec<HashValue>,
}

#[allow(dead_code)]
impl HashChecker {
    pub fn new() -> HashChecker {
        HashChecker {
            value_type: HashValue::None,
            values: Vec::new(),
        }
    }

    pub fn add_uint8(&mut self, value: u8) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::UInt8(0);
        }
        if self.value_type == HashValue::UInt8(0) {
            self.values.push(HashValue::UInt8(value));
        }
    }

    pub fn add_uint16(&mut self, value: u16) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::UInt16(0);
        }
        if self.value_type == HashValue::UInt16(0) {
            self.values.push(HashValue::UInt16(value));
        }
    }

    pub fn add_uint32(&mut self, value: u32) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::UInt32(0);
        }
        if self.value_type == HashValue::UInt32(0) {
            self.values.push(HashValue::UInt32(value));
        }
    }

    pub fn add_uint64(&mut self, value: u64) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::UInt64(0);
        }
        if self.value_type == HashValue::UInt64(0) {
            self.values.push(HashValue::UInt64(value));
        }
    }

    pub fn add_uint128(&mut self, value: u128) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::UInt128(0);
        }
        if self.value_type == HashValue::UInt128(0) {
            self.values.push(HashValue::UInt128(value));
        }
    }

    pub fn add_int8(&mut self, value: i8) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::Int8(0);
        }
        if self.value_type == HashValue::Int8(0) {
            self.values.push(HashValue::Int8(value));
        }
    }

    pub fn add_int16(&mut self, value: i16) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::Int16(0);
        }
        if self.value_type == HashValue::Int16(0) {
            self.values.push(HashValue::Int16(value));
        }
    }

    pub fn add_int32(&mut self, value: i32) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::Int32(0);
        }
        if self.value_type == HashValue::Int32(0) {
            self.values.push(HashValue::Int32(value));
        }
    }

    pub fn add_int64(&mut self, value: i64) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::Int64(0);
        }
        if self.value_type == HashValue::Int64(0) {
            self.values.push(HashValue::Int64(value));
        }
    }

    pub fn add_int128(&mut self, value: i128) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::Int128(0);
        }
        if self.value_type == HashValue::Int128(0) {
            self.values.push(HashValue::Int128(value));
        }
    }

    pub fn add_f32(&mut self, value: f32) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::Float32(0.0);
        }
        if self.value_type == HashValue::Float32(0.0) {
            self.values.push(HashValue::Float32(value));
        }
    }

    pub fn add_f64(&mut self, value: f64) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::Float64(0.0);
        }
        if self.value_type == HashValue::Float64(0.0) {
            self.values.push(HashValue::Float64(value));
        }
    }

    pub fn add_string(&mut self, value: String) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::String(String::new());
        }
        if self.value_type == HashValue::String(String::new()) {
            self.values.push(HashValue::String(value));
        }
    }

    pub fn add_static_str(&mut self, value: &'static str) {
        if self.value_type == HashValue::None {
            self.value_type = HashValue::Str("");
        }
        if self.value_type == HashValue::Str("") {
            self.values.push(HashValue::Str(value));
        }
    }

    pub fn check_collision(&mut self, hash: HashValue) -> bool {
        return self.values.contains(&hash);
    }
}

struct ArgParser {
    pub tests: Vec<String>,
    pub max_collisions: Option<u32>,
}

impl ArgParser {
    pub fn new() -> ArgParser {
        let arguments: Vec<String> = std::env::args().map(|s| s.as_str().to_owned()).collect();

        if arguments.len() > 1 {
            let mut first = true;

            let mut parser = ArgParser { tests: Vec::new(), max_collisions: Some(10) };

            for arg in arguments {

                // Skip first argument for the executable in call
                if first {
                    first = false;
                    continue;
                }

                match arg.as_str() {
                    "djb2" => { },
                    "ch64" => { },
                    "cityhash64" => { },
                    "fnv1a" => { },
                    _ => {
                        let n = arg.parse::<u32>();
                        match n {
                            Ok(x) => parser.max_collisions = Some(x),
                            Err(_) => continue,
                        }
                        continue;
                    }
                }

                parser.tests.push(arg);
            }

            parser
        } else {
            ArgParser {
                tests: vec![String::from("lcg")],
                max_collisions: Some(10),
            }
        }
    }
}

struct TestSuite {
    parser: ArgParser,
}

impl TestSuite {
    pub fn new(parser: ArgParser) -> TestSuite {
        TestSuite {
            parser,
        }
    }

    pub fn analyze_collisions(collisions: &Vec<u32>) {
        let mut mean: u32 = 0;
        for collision in collisions {
            mean += collision;
        }
        mean = mean / collisions.len() as u32;
        println!("Mean: Collision after {} generations.", mean);
    }

    pub fn u128_to_str(i: u128) -> String {
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

    pub fn lcg_test(&self) {
        println!("Starting lcg random test!");
        let mut collisiontester = HashChecker::new();
        let mut rangen = LCG::new();
        let mut collisions: Vec<u32> = Vec::new();
        let mut i = 0;

        loop {
            i += 1;
            let random = rangen.random();
            if collisiontester.check_collision(HashValue::UInt128(random)) {
                collisions.push(i);
                println!("[{}/{}] Random Collision Detected after {} generations!", collisions.len(), self.parser.max_collisions.unwrap(), i);
                i = 0;
                collisiontester = HashChecker::new();
                rangen = LCG::new();
                if collisions.len() > (self.parser.max_collisions.unwrap() - 1) as usize {
                    TestSuite::analyze_collisions(&collisions);
                    break;
                }
            } else {
                print!("\r Gen {}: {}    ", i, random);
                collisiontester.add_uint128(random)
            }
        }
    }

    pub fn ensure_length(input: HashValue, i: usize) -> String {
        let mut s = input.to_string();
        while s.len() < i {
            s += " ";
        }
        s
    }

    pub fn fnv1a_lcg_test(&self) {
        println!("Starting fnv1a hash test with lcg random number!");
        let mut collisiontester = HashChecker::new();
        let mut rangen = LCG::new();
        let mut collisions: Vec<u32> = Vec::new();
        let mut i = 0;

        loop {
            i += 1;
            let random = rangen.random();
            let hash = TestSuite::u128_to_str(random).fnv1a();
            if collisiontester.check_collision(HashValue::UInt32(hash)) {
                collisions.push(i);
                println!("[{}/{}] Random Collision Detected after {} generations!", collisions.len(), self.parser.max_collisions.unwrap(), i);
                i = 0;
                collisiontester = HashChecker::new();
                rangen = LCG::new();
                if collisions.len() > (self.parser.max_collisions.unwrap() - 1) as usize {
                    TestSuite::analyze_collisions(&collisions);
                    break;
                }
            } else {
                print!("\rIter {}: {}", i, TestSuite::ensure_length(HashValue::UInt32(hash), 15));
                collisiontester.add_uint32(hash)
            }
        }
    }

    pub fn cityhash64_lcg_test(&self) {
        println!("Starting cityhash64 hash test with lcg random number!");
        let mut collisiontester = HashChecker::new();
        let mut rangen = LCG::new();
        let mut collisions: Vec<u32> = Vec::new();
        let mut i = 0;

        loop {
            i += 1;
            let random = rangen.random();
            let hash = TestSuite::u128_to_str(random).cityhash();
            if collisiontester.check_collision(HashValue::UInt64(hash)) {
                collisions.push(i);
                println!("[{}/{}] Random Collision Detected after {} generations!", collisions.len(), self.parser.max_collisions.unwrap(), i);
                i = 0;
                collisiontester = HashChecker::new();
                rangen = LCG::new();
                if collisions.len() > (self.parser.max_collisions.unwrap() - 1) as usize {
                    TestSuite::analyze_collisions(&collisions);
                    break;
                }
            } else {
                print!("\rIter {}: {}", i, TestSuite::ensure_length(HashValue::UInt64(hash), 25));
                collisiontester.add_uint64(hash)
            }
        }
    }

    pub fn djb2_lcg_test(&self) {
        println!("Starting djb2 hash test with lcg random number!");
        let mut collisiontester = HashChecker::new();
        let mut rangen = LCG::new();
        let mut collisions: Vec<u32> = Vec::new();
        let mut i = 0;

        loop {
            i += 1;
            let random = rangen.random();
            let hash = TestSuite::u128_to_str(random).djb2();
            if collisiontester.check_collision(HashValue::UInt128(hash)) {
                collisions.push(i);
                println!("[{}/{}] Random Collision Detected after {} generations!", collisions.len(), self.parser.max_collisions.unwrap(), i);
                i = 0;
                collisiontester = HashChecker::new();
                rangen = LCG::new();
                if collisions.len() > (self.parser.max_collisions.unwrap() - 1) as usize {
                    TestSuite::analyze_collisions(&collisions);
                    break;
                }
            } else {
                print!("\rIter {}: {}", i, TestSuite::ensure_length(HashValue::UInt128(hash), 25));
                collisiontester.add_uint128(hash)
            }
        }
    }

    pub fn run_tests(&self) {
        let start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();

        for test in &self.parser.tests {
            match test.as_str() {
                "lcg" => self.lcg_test(),
                "fnv1a" => self.fnv1a_lcg_test(),
                "ch64" => self.cityhash64_lcg_test(),
                "cityhash64" => self.cityhash64_lcg_test(),
                "djb2" => self.djb2_lcg_test(),
                _ => return
            }
        }

        let end_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
        println!("{} Tests finished after {}s!", self.parser.tests.len(), end_time - start_time);
    }
}

fn main() {
    TestSuite::new(ArgParser::new()).run_tests();
}

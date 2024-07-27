pub struct LCG {
    multiplier: u128,
    increment: u128,
    modulus: u128,
    seed: u128,
    results: Vec<u128>,
}

impl LCG {
    pub fn new() -> LCG {
        let systime = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();
        let lcg = LCG {
            multiplier: 1466251,
            increment: 3141592,
            modulus: 2_u128.pow(32),
            seed: systime.as_nanos(),
            results: Vec::new(),
        };

        lcg
    }

    pub fn set_seed(&mut self, seed: u128) {
        self.seed = seed;
    }

    pub fn set_increment(&mut self, increment: u128) {
        self.increment = increment;
    }

    pub fn set_modulus(&mut self, modulus: u128) {
        self.modulus = modulus;
    }

    pub fn set_multiplier(&mut self, multiplier: u128) {
        self.multiplier = multiplier;
    }

    pub fn random(&mut self) -> u128 {
        self.seed = (self.multiplier * self.seed + self.increment) % self.modulus;
        return self.seed;
    }

    pub fn random_between(&mut self, min: u128, max: u128) -> Result<u128, std::io::Error> {
        if min >= max {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Min can not be bigger or equal to max!"));
        }

        let mut randomval: u128 = 0;
        for _ in 0..4 {
            randomval = (randomval << 32) | (self.random() as u128);
        }

        let range = max - min + 1;
        Ok((randomval % range) + min)
    }

    pub fn add_result(&mut self, val: u128) {
        self.results.push(val);
    }

    pub fn save_to_file(&self, path: &str) -> Result<bool, std::io::Error> {
        use std::io::Write;
        let file = std::fs::File::create(path).unwrap();
        let mut bufwriter = std::io::BufWriter::new(file);

        for value in &self.results {
            let res = writeln!(bufwriter, "{}", value);
            match res {
                Ok(_) => {},
                Err(x) => { println!("Error occured while writing line! {}", x); },
            }
        }
        Ok(true)
    }

    pub fn save_to_file_seperated(&self, path: &str, separator: &str) {
        use std::io::Write;
        let file = std::fs::File::create(path).unwrap();
        let mut bufwriter = std::io::BufWriter::new(file);

        let result_string = self.results.iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(separator);

        let res = writeln!(bufwriter, "{}", result_string);
        match res {
            Ok(_) => {},
            Err(x) => { println!("Error occured while writing line! {}", x); },
        }
    }
}

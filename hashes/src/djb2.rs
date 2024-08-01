pub struct DJB2 {
    iv: u32,
}

impl DJB2 {
    pub fn new(iv: u32) -> DJB2 {
        return DJB2 {
            iv,
        };
    }

    pub fn new_default() -> DJB2 {
        return DJB2 {
            iv: 5381,
        };
    }

    pub fn hash(&self, input: &str) -> u128 {
        let mut hash: u128 = self.iv as u128;

        for char in input.chars() {
            hash = ((hash << 5) + hash) + char as u128;
        }

        return hash as u128;
    }
}

pub trait Djb2fn {
    fn djb2(&self) -> u128;
    fn djb2_iv(&self, iv: u32) -> u128;
}

impl Djb2fn for str {
    fn djb2(&self) -> u128 {
        let hasher = DJB2::new_default();
        return hasher.hash(&self);
    }

    fn djb2_iv(&self, iv: u32) -> u128 {
        let hasher = DJB2::new(iv);
        return hasher.hash(&self);
    }
}

impl Djb2fn for String {
    fn djb2(&self) -> u128 {
        return self.as_str().djb2();
    }

    fn djb2_iv(&self, iv: u32) -> u128 {
        return self.as_str().djb2_iv(iv);
    }
}

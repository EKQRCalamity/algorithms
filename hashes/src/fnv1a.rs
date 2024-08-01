pub struct FNV1A {
    offset_base: u32,
    fnv_prime: u32,
}

impl FNV1A {
    pub fn new(offset: u32, prime: u32) -> FNV1A {
        return FNV1A {
            offset_base: offset,
            fnv_prime: prime,
        }
    }

    pub fn new_default() -> FNV1A {
        return FNV1A {
            offset_base: 2166136261,
            fnv_prime: 16777619,
        }
    }

    pub fn hash(&self, input: &str) -> u32 {
        let mut hash: u32 = self.offset_base;

        for char in input.chars() {
            hash ^= char as u32;
            hash = hash.wrapping_mul(self.fnv_prime);
        }

        hash
    }
}

pub trait FNV1Afn {
    fn fnv1a(&self) -> u32;
}

impl FNV1Afn for str {
    fn fnv1a(&self) -> u32 {
        let hasher = FNV1A::new_default();
        return hasher.hash(&self);
    }
}

impl FNV1Afn for String {
    fn fnv1a(&self) -> u32 {
        return self.as_str().fnv1a();
    }
}

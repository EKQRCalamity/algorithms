pub struct CityHash64 {
    k1: u64,
    k2: u64,
    k3: u64,
}

impl CityHash64 {
    pub fn new(k1: u64, k2: u64, k3: u64) -> CityHash64 {
        return CityHash64 {
            k1,
            k2,
            k3
        }
    }

    pub fn new_default() -> CityHash64 {
        return CityHash64 {
            k1: 0x9ae16a3b2f90404f,
            k2: 0xc949d7c7509e6557,
            k3: 0x0fc2e5f69d3c5795,
        }
    }

    pub fn load64_str(&self, data: &str) -> u64 {
        let bytearray: &[u8] = data.as_bytes();
        return self.load64(bytearray);
    }

    pub fn load64(&self, data: &[u8]) -> u64 {
        let length = if data.len() > 8 { 8 } else { data.len() - 1 };
        let mut array = [0u8; 8];
        array[..length].copy_from_slice(&data[0..length]);
        u64::from_le_bytes(array)
    }

    pub fn rotate_left(&self, value: u64, shift: u32) -> u64 {
        (value << shift) | (value >> (64 - shift))
    }

    pub fn hash(&self, input: &str) -> u64 {
        let byte_input = input.as_bytes();
        let input_length = byte_input.len();
        let mut hash: u64;

        if input_length >= 16 {
            let mut a = self.load64(&byte_input[0..8]) * self.k1;
            a = self.rotate_left(a, 33) * self.k2;
            let mut b = self.load64(&byte_input[8..16]) * self.k2;
            b = self.rotate_left(b, 33) * self.k3;
            hash = a ^ b;
        } else {
            hash = self.load64(&byte_input);
        }

        hash ^= hash >> 33;
        hash = hash.wrapping_mul(self.k2);
        hash ^= hash >> 29;
        hash = hash.wrapping_mul(self.k3);
        hash ^= hash >> 32;

        return hash;
    }
}

pub trait CityHash64fn {
    fn cityhash(&self) -> u64;
}

impl CityHash64fn for str {
    fn cityhash(&self) -> u64 {
        let hasher = CityHash64::new_default();
        return hasher.hash(&self);
    }
}

impl CityHash64fn for String {
    fn cityhash(&self) -> u64 {
        let hasher = CityHash64::new_default();
        return hasher.hash(&self);
    }
}

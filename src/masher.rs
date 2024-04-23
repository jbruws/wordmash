use std::{ops, fmt};

pub struct Masher {
    value_36: String,
}

impl Masher {
    const BASE_36_ALPHABET: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    pub fn new(val: impl Into<String>) -> Masher {
        Masher { value_36: val.into() }
    }

    pub fn from_base36(raw_number: String) -> u64 {
        let mut result: u64 = 0;
        let number = raw_number.to_uppercase();
        for (i, v) in number.chars().rev().enumerate() {
            result += 36u64.pow(i as u32) * Masher::BASE_36_ALPHABET.find(v).expect("Symbol not found in base 36 alphabet") as u64;
        }
        result
    }

    pub fn to_base36(mut number: u64) -> String {
        let mut result: String = String::new();
        while number > 0 {
            result.push(Masher::BASE_36_ALPHABET.as_bytes()[(number % 36) as usize] as char);
            number = number / 36;
        }
        result.chars().rev().collect::<String>()
    }
}

impl ops::Add<Masher> for Masher {
    type Output = Masher;

    fn add(self, snd: Masher) -> Masher {
        let n1 = Masher::from_base36(self.to_string());
        let n2 = Masher::from_base36(snd.to_string());
        Masher { value_36: Masher::to_base36(n1+n2) }
    }
}

impl ops::Mul<Masher> for Masher {
    type Output = Masher;

    fn mul(self, snd: Masher) -> Masher {
        let n1 = Masher::from_base36(self.to_string());
        let n2 = Masher::from_base36(snd.to_string());
        Masher { value_36: Masher::to_base36(n1*n2) }
    }
}

impl fmt::Display for Masher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value_36)
    }
}

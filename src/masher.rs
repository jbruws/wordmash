use crate::{errors, mashable};
use rug::{Integer, Assign};
use std::{fmt, ops};

/// Wrapper for base36 strings
#[derive(Debug, Clone, PartialEq)]
pub struct Masher {
    value_36: String,
}

impl Masher {
    /// All symbols that can be used in base36 strings. `Mashable` checks against it when
    /// converting `String`s and `&str`s.
    const BASE_36_ALPHABET: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    /// Tries to construct a new `Masher` from `Mashable` object
    pub fn new(val: impl mashable::Mashable) -> Result<Masher, errors::Base36AlphabetError> {
        let vs = val.to_mashed_string();
        match vs {
            Ok(v) => Ok(Masher { value_36: v }),
            Err(e) => Err(e),
        }
    }

    /// Checks strings for illegal values (values not in `BASE_36_ALPHABET`)
    pub fn is_mashable(val: String) -> bool {
        for i in val.chars() {
            if !Masher::BASE_36_ALPHABET.contains(i) {
                return false;
            }
        }
        true
    }

    /// Converts from base36 string to base10 integer
    pub fn from_base36(raw_number: String) -> Integer {
        let mut result = Integer::new();
        let number = raw_number.to_uppercase();
        result.assign(Integer::parse_radix(number, 36).unwrap());
        result
    }

    /// Converts from base10 integer to base36 string
    pub fn to_base36(number: Integer) -> String {
        number.to_string_radix(36).to_uppercase()
    }
}

impl ops::AddAssign for Masher {
    fn add_assign(&mut self, snd: Self) {
        let n1 = Masher::from_base36(self.to_string());
        let n2 = Masher::from_base36(snd.to_string());
        *self = Self {
            value_36: Masher::to_base36(n1 + n2),
        }
    }
}

impl ops::Add<Masher> for Masher {
    type Output = Masher;

    fn add(self, snd: Masher) -> Masher {
        let mut cloned_self = self.clone();
        cloned_self += snd;
        cloned_self
    }
}

impl ops::MulAssign for Masher {
    fn mul_assign(&mut self, snd: Self) {
        let n1 = Masher::from_base36(self.to_string());
        let n2 = Masher::from_base36(snd.to_string());
        *self = Self {
            value_36: Masher::to_base36(n1 * n2),
        }
    }
}

impl ops::Mul<Masher> for Masher {
    type Output = Masher;

    fn mul(self, snd: Masher) -> Masher {
        let mut cloned_self = self.clone();
        cloned_self *= snd;
        cloned_self
    }
}

impl fmt::Display for Masher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value_36)
    }
}

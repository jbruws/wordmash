use crate::{errors, mashable};
use rug::Integer;
use std::{fmt, ops};

/// Wrapper for Masher strings as `Integer`s
#[derive(Debug, Clone, PartialEq)]
pub struct Masher {
    value_int: Integer,
}

impl Masher {
    /// All symbols that can be used in Masher strings. `Mashable` checks against it when
    /// converting `String`s and `&str`s.
    const MASHER_ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ'_-,.!? ";

    /// Length of Masher alphabet
    const MASHER_ALPHABET_LENGTH: usize = Masher::MASHER_ALPHABET.len();

    /// Tries to construct a new `Masher` from `Mashable` object
    pub fn new(val: impl mashable::Mashable) -> Result<Masher, errors::MasherAlphabetError> {
        let vs = val.to_integer_form();
        match vs {
            Ok(v) => Ok(Masher { value_int: v }),
            Err(e) => Err(e),
        }
    }

    /// Checks strings for illegal values (values not in `MASHER_ALPHABET`)
    pub fn is_mashable(raw_val: impl Into<String>) -> bool {
        let val: String = raw_val.into();
        for i in val.chars() {
            if !Masher::MASHER_ALPHABET.contains(i) {
                return false
            }
        }
        true
    }

    /// Converts from mashed string to base10 integer
    pub fn from_mashed(number: String) -> Integer {
        let mut result = Integer::new();
        for (i, v) in number.chars().rev().enumerate() {
            result += (Masher::MASHER_ALPHABET_LENGTH as u128).pow(i as u32)
                * Masher::MASHER_ALPHABET
                    .find(v)
                    .expect("Symbol not found in Masher alphabet") as u128;
        }
        result
    }

    /// Converts from base10 integer to mashed string
    pub fn to_mashed(mut number: Integer) -> String {
        let mut result: String = String::new();
        while number > 0 {
            result.push(
                Masher::MASHER_ALPHABET.as_bytes()[(number
                    .clone()
                    .modulo(&Integer::from(Masher::MASHER_ALPHABET_LENGTH)))
                .to_usize()
                .unwrap()] as char,
            );
            number /= Masher::MASHER_ALPHABET_LENGTH;
        }
        result.chars().rev().collect::<String>()
    }
}

impl ops::AddAssign for Masher {
    fn add_assign(&mut self, snd: Self) {
        *self = Self {
            value_int: snd.value_int + self.value_int.clone(),
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
        *self = Self {
            value_int: snd.value_int * self.value_int.clone(),
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
        write!(f, "{}", Masher::to_mashed(self.value_int.clone()))
    }
}

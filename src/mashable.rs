use crate::errors::MasherAlphabetError;
use crate::masher::Masher;
use rug::Integer;

/// Generates `Mashable` implementations for given types. Only produces usable code when given
/// types can be converted to `rug::Integer`.
macro_rules! impl_mashable_for_unsigned {
    ($($s:ty),+) => {
        $(impl Mashable for $s {
            fn to_integer_form(self) -> Result<Integer, MasherAlphabetError> {
                Ok(Integer::from(self))
            }
        })*
    }
}

/// Trait for types that can be converted to mashed `String`
pub trait Mashable {
    fn to_integer_form(self) -> Result<Integer, MasherAlphabetError>;
}

impl_mashable_for_unsigned!(u8, u16, u32, u64, u128);

impl Mashable for String {
    fn to_integer_form(self) -> Result<Integer, MasherAlphabetError> {
        if Masher::is_mashable(&self) {
            Ok(Masher::from_mashed(self))
        } else {
            Err(MasherAlphabetError)
        }
    }
}

impl Mashable for &str {
    fn to_integer_form(self) -> Result<Integer, MasherAlphabetError> {
        let val = self.to_string();
        if Masher::is_mashable(&val) {
            Ok(Masher::from_mashed(val))
        } else {
            Err(MasherAlphabetError)
        }
    }
}

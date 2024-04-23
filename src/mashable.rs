use crate::errors::Base36AlphabetError;
use crate::masher::Masher;

/// Generates `Mashable` implementations for given types. Only produces usable code when given
/// types can be converted to `u128`.
macro_rules! impl_mashable_for_unsigned {
    ($($s:ty),+) => {
        $(impl Mashable for $s {
            fn to_mashed_string(self) -> Result<String, Base36AlphabetError> {
                Ok(Masher::to_base36(self as u128))
            }
        })*
    }
}

/// Trait for types that can be converted to base36 `String`
pub trait Mashable {
    /// Attempts to convert given value to base36 `String`.
    fn to_mashed_string(self) -> Result<String, Base36AlphabetError>;
}

impl_mashable_for_unsigned!(u8, u16, u32, u64, u128);

impl Mashable for String {
    fn to_mashed_string(self) -> Result<String, Base36AlphabetError> {
        if Masher::is_mashable(self.clone()) {
            Ok(self)
        } else {
            Err(Base36AlphabetError)
        }
    }
}

impl Mashable for &str {
    fn to_mashed_string(self) -> Result<String, Base36AlphabetError> {
        let val = self.to_string();
        if Masher::is_mashable(val.clone()) {
            Ok(val)
        } else {
            Err(Base36AlphabetError)
        }
    }
}

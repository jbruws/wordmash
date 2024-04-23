use crate::errors::Base36AlphabetError;
use crate::masher::Masher;

macro_rules! impl_mashable_for_unsigned {
    ($($s:ty),+) => {
        $(impl Mashable for $s {
            fn to_mashed_string(self) -> Result<String, Base36AlphabetError> {
                Ok(Masher::to_base36(self as u128))
            }
        })*
    }
}

pub trait Mashable {
    fn to_mashed_string(self) -> Result<String, Base36AlphabetError>;
}

impl_mashable_for_unsigned!(u8, u16, u32, u64, u128);

impl Mashable for String {
    fn to_mashed_string(self) -> Result<String, Base36AlphabetError> {
        if Masher::is_mashable(self.clone()) {
            return Ok(self);
        } else {
            return Err(Base36AlphabetError);
        }
    }
}

impl Mashable for &str {
    fn to_mashed_string(self) -> Result<String, Base36AlphabetError> {
        let val = self.to_string();
        if Masher::is_mashable(val.clone()) {
            return Ok(val);
        } else {
            return Err(Base36AlphabetError);
        }
    }
}

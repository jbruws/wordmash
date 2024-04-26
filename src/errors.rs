use std::fmt;

/// Error for illegal values that cannot be converted to given mashed alphabet
pub struct MasherAlphabetError;

impl fmt::Display for MasherAlphabetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value can not be represented by Masher alphabet")
    }
}

impl fmt::Debug for MasherAlphabetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value can not be represented by Masher alphabet")
    }
}

impl std::error::Error for MasherAlphabetError {}

use std::fmt;

/// Error for illegal values that cannot be converted to base36
pub struct Base36AlphabetError;

impl fmt::Display for Base36AlphabetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value can not be represented by base 36 alphabet")
    }
}

impl fmt::Debug for Base36AlphabetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl std::error::Error for Base36AlphabetError {}

mod hash;
use core::convert::From;

pub struct StringId {}

impl From<&str> for StringId {
    fn from(string: &str) -> Self {
        // Hash the string
        let hashed_string = hash::Hash::from(string);

        StringId {}
    }
}

impl StringId {
    pub fn new(string: &str) -> StringId {
        Self::from(string)
    }
}

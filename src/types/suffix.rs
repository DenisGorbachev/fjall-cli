use core::convert::Infallible;
use core::str::FromStr;
use subtype::subtype_string;

subtype_string! {
    pub struct Suffix(String);
}

impl FromStr for Suffix {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

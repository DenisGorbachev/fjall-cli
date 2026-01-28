use errgonomic::{handle, handle_opt};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Separator(Vec<u8>);

impl Separator {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn parse(input: &str) -> Result<Self, SeparatorParseError> {
        use SeparatorParseError::*;
        let input = input.to_owned();
        let bytes = input.as_bytes();
        let mut out = Vec::new();
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'\\' {
                index += 1;
                let escape = handle_opt!(bytes.get(index).copied(), TrailingEscape, input);
                match escape {
                    b'n' => out.push(b'\n'),
                    b'r' => out.push(b'\r'),
                    b't' => out.push(b'\t'),
                    b'0' => out.push(b'\0'),
                    b'x' => {
                        let high = handle_opt!(bytes.get(index + 1).copied(), IncompleteHexEscape, input);
                        let low = handle_opt!(bytes.get(index + 2).copied(), IncompleteHexEscape, input);
                        let mut decoded = [0u8; 1];
                        let hex_bytes = [high, low];
                        handle!(hex::decode_to_slice(hex_bytes, &mut decoded), InvalidHexEscape, input);
                        out.push(decoded[0]);
                        index += 2;
                    }
                    _ => {
                        let escape = (escape as char).to_string();
                        return Err(InvalidEscape {
                            input,
                            escape,
                        });
                    }
                }
            } else {
                out.push(byte);
            }
            index += 1;
        }
        Ok(Self(out))
    }
}

impl AsRef<[u8]> for Separator {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl FromStr for Separator {
    type Err = SeparatorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[derive(Error, Debug)]
pub enum SeparatorParseError {
    #[error("separator has trailing escape in '{input}'")]
    TrailingEscape { input: String },

    #[error("separator has incomplete hex escape in '{input}'")]
    IncompleteHexEscape { input: String },

    #[error("separator has invalid hex escape in '{input}'")]
    InvalidHexEscape { source: hex::FromHexError, input: String },

    #[error("separator has invalid escape '\\{escape}' in '{input}'")]
    InvalidEscape { input: String, escape: String },
}

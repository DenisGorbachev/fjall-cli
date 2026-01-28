use errgonomic::{handle, handle_bool};
use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(clap::ValueEnum, Copy, Clone, Debug, Default)]
#[clap(rename_all = "kebab")]
pub enum ByteEncoding {
    Empty,
    #[default]
    String,
    Hex,
    Path,
}

impl Display for ByteEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ByteEncoding::*;
        let value = match self {
            Empty => "empty",
            String => "string",
            Hex => "hex",
            Path => "path",
        };
        write!(f, "{value}")
    }
}

impl ByteEncoding {
    pub fn decode(&self, input: &str) -> Result<Vec<u8>, ByteEncodingDecodeError> {
        use ByteEncoding::*;
        use ByteEncodingDecodeError::*;
        match self {
            Empty => {
                handle_bool!(input != "-", EmptyEncodingInvalid, input: input.to_owned());
                Ok(Vec::new())
            }
            String => Ok(input.as_bytes().to_vec()),
            Hex => Self::decode_hex(input),
            Path => Self::decode_path(input),
        }
    }

    pub fn decode_hex(input: &str) -> Result<Vec<u8>, ByteEncodingDecodeError> {
        use ByteEncodingDecodeError::*;
        let input = input.to_owned();
        let bytes = handle!(hex::decode(&input), HexDecodeFailed, input);
        Ok(bytes)
    }

    pub fn decode_path(input: &str) -> Result<Vec<u8>, ByteEncodingDecodeError> {
        use ByteEncodingDecodeError::*;
        let path = PathBuf::from(input);
        let metadata = handle!(fs::metadata(&path), MetadataFailed, path);
        handle_bool!(metadata.is_dir(), PathIsDirectory, path);
        let bytes = handle!(fs::read(&path), ReadFailed, path);
        Ok(bytes)
    }
}

#[derive(Error, Debug)]
pub enum ByteEncodingDecodeError {
    #[error("empty encoding expects '-' but got '{input}'")]
    EmptyEncodingInvalid { input: String },

    #[error("failed to decode hex input '{input}'")]
    HexDecodeFailed { source: hex::FromHexError, input: String },

    #[error("failed to read metadata for path '{path}'")]
    MetadataFailed { source: io::Error, path: PathBuf },

    #[error("path '{path}' is a directory")]
    PathIsDirectory { path: PathBuf },

    #[error("failed to read bytes from path '{path}'")]
    ReadFailed { source: io::Error, path: PathBuf },
}

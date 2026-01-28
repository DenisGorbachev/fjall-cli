use crate::{ByteEncoding, ByteEncodingDecodeError};
use errgonomic::handle;
use fjall::{Database, KeyspaceCreateOptions};
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
#[command(long_about = "Exit codes: 0 = key exists, 127 = key not found, 1 = error.")]
pub struct ContainsCommand {
    #[arg(value_name = "KEYSPACE")]
    keyspace: String,

    #[arg(value_name = "KEY")]
    key: String,

    #[arg(long, value_enum, default_value_t = ByteEncoding::String)]
    key_encoding: ByteEncoding,
}

impl ContainsCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, ContainsCommandRunError> {
        use ContainsCommandRunError::*;
        let Self {
            keyspace,
            key,
            key_encoding,
        } = self;
        let key_bytes = handle!(Self::decode_key_bytes((&key, key_encoding)), DecodeKeyBytesFailed, key, key_encoding);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        let exists = handle!(keyspace_handle.contains_key(&key_bytes), ContainsKeyFailed, keyspace);
        let exit_code = if exists { ExitCode::SUCCESS } else { ExitCode::from(127) };
        Ok(exit_code)
    }

    impl_decode_bytes_method!(decode_key_bytes, ContainsCommandDecodeKeyBytesError);
}

#[derive(Error, Debug)]
pub enum ContainsCommandRunError {
    #[error("failed to decode key '{key}' with encoding '{key_encoding}'")]
    DecodeKeyBytesFailed { source: ContainsCommandDecodeKeyBytesError, key: String, key_encoding: ByteEncoding },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to check key presence in keyspace '{keyspace}'")]
    ContainsKeyFailed { source: fjall::Error, keyspace: String },
}

#[derive(Error, Debug)]
pub enum ContainsCommandDecodeKeyBytesError {
    #[error("failed to decode key bytes")]
    DecodeFailed { source: ByteEncodingDecodeError },
}

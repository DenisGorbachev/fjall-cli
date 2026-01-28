use crate::{ByteEncoding, ByteEncodingDecodeError};
use errgonomic::{handle, handle_opt};
use fjall::{Database, KeyspaceCreateOptions};
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct GetCommand {
    #[arg(value_name = "KEYSPACE")]
    keyspace: String,

    #[arg(value_name = "KEY")]
    key: String,

    #[arg(long, value_enum, default_value_t = ByteEncoding::String)]
    key_encoding: ByteEncoding,

    #[arg(short = 'n', long = "no-newline")]
    no_newline: bool,
}

impl GetCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, GetCommandRunError> {
        use GetCommandRunError::*;
        let Self {
            keyspace,
            key,
            key_encoding,
            no_newline,
        } = self;
        let key_bytes = handle!(Self::decode_key_bytes((&key, key_encoding)), DecodeKeyBytesFailed, key, key_encoding);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        let value_opt = handle!(keyspace_handle.get(&key_bytes), GetFailed, keyspace, key);
        let value = handle_opt!(value_opt, KeyNotFound, keyspace, key);
        let mut stdout = io::stdout().lock();
        handle!(stdout.write_all(value.as_ref()), WriteAllFailed);
        if !no_newline {
            handle!(stdout.write_all(b"\n"), WriteAllFailed);
        }
        Ok(ExitCode::SUCCESS)
    }

    impl_decode_bytes_method!(decode_key_bytes, GetCommandDecodeKeyBytesError);
}

#[derive(Error, Debug)]
pub enum GetCommandRunError {
    #[error("failed to decode key '{key}' with encoding '{key_encoding}'")]
    DecodeKeyBytesFailed { source: GetCommandDecodeKeyBytesError, key: String, key_encoding: ByteEncoding },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to get key '{key}' from keyspace '{keyspace}'")]
    GetFailed { source: fjall::Error, keyspace: String, key: String },

    #[error("key '{key}' not found in keyspace '{keyspace}'")]
    KeyNotFound { keyspace: String, key: String },

    #[error("failed to write value to stdout")]
    WriteAllFailed { source: io::Error },
}

#[derive(Error, Debug)]
pub enum GetCommandDecodeKeyBytesError {
    #[error("failed to decode key bytes")]
    DecodeFailed { source: ByteEncodingDecodeError },
}

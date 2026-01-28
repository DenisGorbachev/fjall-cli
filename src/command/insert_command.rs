use crate::{ByteEncoding, ByteEncodingDecodeError};
use errgonomic::handle;
use fjall::{Database, KeyspaceCreateOptions};
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct InsertCommand {
    #[arg(value_name = "KEYSPACE")]
    keyspace: String,

    #[arg(value_name = "KEY")]
    key: String,

    #[arg(value_name = "VALUE")]
    value: String,

    #[arg(long, value_enum, default_value_t = ByteEncoding::String)]
    key_encoding: ByteEncoding,

    #[arg(long, value_enum, default_value_t = ByteEncoding::String)]
    value_encoding: ByteEncoding,
}

impl InsertCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, InsertCommandRunError> {
        use InsertCommandRunError::*;
        let Self {
            keyspace,
            key,
            value,
            key_encoding,
            value_encoding,
        } = self;
        let key_bytes = handle!(Self::decode_key_bytes((&key, key_encoding)), DecodeKeyBytesFailed, key, key_encoding);
        let value_bytes = handle!(Self::decode_value_bytes((&value, value_encoding)), DecodeValueBytesFailed, value, value_encoding);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        handle!(keyspace_handle.insert(key_bytes, value_bytes), InsertFailed, keyspace);
        Ok(ExitCode::SUCCESS)
    }

    impl_decode_bytes_method!(decode_key_bytes, InsertCommandDecodeKeyBytesError);
    impl_decode_bytes_method!(decode_value_bytes, InsertCommandDecodeValueBytesError);
}

#[derive(Error, Debug)]
pub enum InsertCommandRunError {
    #[error("failed to decode key '{key}' with encoding '{key_encoding}'")]
    DecodeKeyBytesFailed { source: InsertCommandDecodeKeyBytesError, key: String, key_encoding: ByteEncoding },

    #[error("failed to decode value '{value}' with encoding '{value_encoding}'")]
    DecodeValueBytesFailed { source: InsertCommandDecodeValueBytesError, value: String, value_encoding: ByteEncoding },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to insert value into keyspace '{keyspace}'")]
    InsertFailed { source: fjall::Error, keyspace: String },
}

#[derive(Error, Debug)]
pub enum InsertCommandDecodeKeyBytesError {
    #[error("failed to decode key bytes")]
    DecodeFailed { source: ByteEncodingDecodeError },
}

#[derive(Error, Debug)]
pub enum InsertCommandDecodeValueBytesError {
    #[error("failed to decode value bytes")]
    DecodeFailed { source: ByteEncodingDecodeError },
}

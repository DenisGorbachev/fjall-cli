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
        let key_bytes = handle!(key_encoding.decode(&key), DecodeKeyBytesFailed, key, key_encoding);
        let value_bytes = handle!(value_encoding.decode(&value), DecodeValueBytesFailed, value, value_encoding);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        handle!(keyspace_handle.insert(key_bytes, value_bytes), InsertFailed, keyspace);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum InsertCommandRunError {
    #[error("failed to decode key '{key}' with encoding '{key_encoding}'")]
    DecodeKeyBytesFailed { source: ByteEncodingDecodeError, key: String, key_encoding: ByteEncoding },

    #[error("failed to decode value '{value}' with encoding '{value_encoding}'")]
    DecodeValueBytesFailed { source: ByteEncodingDecodeError, value: String, value_encoding: ByteEncoding },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to insert value into keyspace '{keyspace}'")]
    InsertFailed { source: fjall::Error, keyspace: String },
}

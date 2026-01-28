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
        let key_bytes = handle!(key_encoding.decode(&key), DecodeKeyBytesFailed, key, key_encoding);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        let exists = handle!(keyspace_handle.contains_key(&key_bytes), ContainsKeyFailed, keyspace);
        let exit_code = if exists { ExitCode::SUCCESS } else { ExitCode::from(127) };
        Ok(exit_code)
    }
}

#[derive(Error, Debug)]
pub enum ContainsCommandRunError {
    #[error("failed to decode key '{key}' with encoding '{key_encoding}'")]
    DecodeKeyBytesFailed { source: ByteEncodingDecodeError, key: String, key_encoding: ByteEncoding },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to check key presence in keyspace '{keyspace}'")]
    ContainsKeyFailed { source: fjall::Error, keyspace: String },
}

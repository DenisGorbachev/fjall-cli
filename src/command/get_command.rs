use crate::{ByteEncoding, ByteEncodingDecodeError, PrefixKind, Suffix};
use errgonomic::{handle, handle_bool, handle_opt};
use fjall::{Database, KeyspaceCreateOptions};
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct GetCommand {
    #[arg(value_name = "KEY")]
    key: String,

    #[arg(long, value_enum, default_value_t = ByteEncoding::String)]
    key_encoding: ByteEncoding,

    #[arg(long, value_enum)]
    value_prefix: Option<PrefixKind>,

    #[arg(long)]
    value_suffix: Option<Suffix>,
}

impl GetCommand {
    pub async fn run(self, db: &Database, keyspace: impl Into<String>) -> Result<ExitCode, GetCommandRunError> {
        use GetCommandRunError::*;
        let keyspace = keyspace.into();
        let Self {
            key,
            key_encoding,
            value_prefix,
            value_suffix,
        } = self;
        let key_bytes = handle!(key_encoding.decode(&key), DecodeKeyBytesFailed, key, key_encoding);
        handle_bool!(!db.keyspace_exists(&keyspace), KeyspaceNotFound, keyspace);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        let value_opt = handle!(keyspace_handle.get(&key_bytes), GetFailed, keyspace, key);
        let value = handle_opt!(value_opt, KeyNotFound, keyspace, key);
        let mut stdout = io::stdout().lock();
        if let Some(prefix) = value_prefix {
            let bytes = prefix.write(&value);
            handle!(stdout.write_all(&bytes), WriteAllFailed);
        }
        handle!(stdout.write_all(value.as_ref()), WriteAllFailed);
        if let Some(suffix) = value_suffix {
            handle!(stdout.write_all(suffix.as_bytes()), WriteAllFailed);
        }
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum GetCommandRunError {
    #[error("failed to decode key '{key}' with encoding '{key_encoding}'")]
    DecodeKeyBytesFailed { source: ByteEncodingDecodeError, key: String, key_encoding: ByteEncoding },

    #[error("keyspace '{keyspace}' not found")]
    KeyspaceNotFound { keyspace: String },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to get key '{key}' from keyspace '{keyspace}'")]
    GetFailed { source: fjall::Error, keyspace: String, key: String },

    #[error("key '{key}' not found in keyspace '{keyspace}'")]
    KeyNotFound { keyspace: String, key: String },

    #[error("failed to write value to stdout")]
    WriteAllFailed { source: io::Error },
}

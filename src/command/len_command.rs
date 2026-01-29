use errgonomic::{handle, handle_bool};
use fjall::{Database, KeyspaceCreateOptions};
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct LenCommand {
    #[arg(value_name = "KEYSPACE")]
    keyspace: String,
}

impl LenCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, LenCommandRunError> {
        use LenCommandRunError::*;
        let Self {
            keyspace,
        } = self;
        handle_bool!(!db.keyspace_exists(&keyspace), KeyspaceNotFound, keyspace);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        let len = handle!(keyspace_handle.len(), LenFailed, keyspace);
        let mut stdout = io::stdout().lock();
        handle!(writeln!(stdout, "{len}"), WriteFailed);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum LenCommandRunError {
    #[error("keyspace '{keyspace}' not found")]
    KeyspaceNotFound { keyspace: String },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to read length for keyspace '{keyspace}'")]
    LenFailed { source: fjall::Error, keyspace: String },

    #[error("failed to write keyspace length to stdout")]
    WriteFailed { source: io::Error },
}

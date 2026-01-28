use errgonomic::{handle, handle_bool};
use fjall::{Database, KeyspaceCreateOptions};
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct ClearCommand {
    #[arg(value_name = "KEYSPACE")]
    keyspace: String,
}

impl ClearCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, ClearCommandRunError> {
        use ClearCommandRunError::*;
        let Self {
            keyspace,
        } = self;
        handle_bool!(!db.keyspace_exists(&keyspace), KeyspaceNotFound, keyspace);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        handle!(keyspace_handle.clear(), ClearFailed, keyspace);
        handle!(db.delete_keyspace(keyspace_handle), DeleteKeyspaceFailed, keyspace);
        let _keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum ClearCommandRunError {
    #[error("keyspace '{keyspace}' not found")]
    KeyspaceNotFound { keyspace: String },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to clear keyspace '{keyspace}'")]
    ClearFailed { source: fjall::Error, keyspace: String },

    #[error("failed to delete keyspace '{keyspace}'")]
    DeleteKeyspaceFailed { source: fjall::Error, keyspace: String },
}

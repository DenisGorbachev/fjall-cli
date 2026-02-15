use errgonomic::{handle, handle_bool};
use fjall::{Database, KeyspaceCreateOptions, PersistMode};
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct ClearCommand {}

impl ClearCommand {
    pub async fn run(self, db: &Database, keyspace: impl Into<String>) -> Result<ExitCode, ClearCommandRunError> {
        use ClearCommandRunError::*;
        let keyspace = keyspace.into();
        handle_bool!(!db.keyspace_exists(&keyspace), KeyspaceNotFound, keyspace);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        handle!(keyspace_handle.clear(), ClearFailed, keyspace);
        db.persist(PersistMode::SyncAll).unwrap();
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
}

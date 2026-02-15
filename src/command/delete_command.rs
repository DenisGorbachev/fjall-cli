use errgonomic::{handle, handle_bool};
use fjall::{Database, KeyspaceCreateOptions};
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct DeleteCommand {}

impl DeleteCommand {
    pub async fn run(self, db: &Database, keyspace: impl Into<String>) -> Result<ExitCode, DeleteCommandRunError> {
        use DeleteCommandRunError::*;
        let keyspace = keyspace.into();
        handle_bool!(!db.keyspace_exists(&keyspace), KeyspaceNotFound, keyspace);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        handle!(db.delete_keyspace(keyspace_handle), DeleteKeyspaceFailed, keyspace);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum DeleteCommandRunError {
    #[error("keyspace '{keyspace}' not found")]
    KeyspaceNotFound { keyspace: String },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to delete keyspace '{keyspace}'")]
    DeleteKeyspaceFailed { source: fjall::Error, keyspace: String },
}

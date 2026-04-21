use errgonomic::{handle, handle_bool};
use fjall::{Database, KeyspaceCreateOptions};
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct DiskSizeCommand {}

impl DiskSizeCommand {
    pub async fn run(self, db: &Database, keyspace: impl Into<String>) -> Result<ExitCode, DiskSizeCommandRunError> {
        use DiskSizeCommandRunError::*;
        let keyspace = keyspace.into();
        handle_bool!(!db.keyspace_exists(&keyspace), KeyspaceNotFound, keyspace);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        handle!(keyspace_handle.rotate_memtable_and_wait(), RotateMemtableFailed, keyspace);
        let disk_space = keyspace_handle.disk_space();
        let mut stdout = io::stdout().lock();
        handle!(writeln!(stdout, "{disk_space}"), WriteFailed);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum DiskSizeCommandRunError {
    #[error("keyspace '{keyspace}' not found")]
    KeyspaceNotFound { keyspace: String },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to flush keyspace '{keyspace}' before measuring disk size")]
    RotateMemtableFailed { source: fjall::Error, keyspace: String },

    #[error("failed to write keyspace disk size to stdout")]
    WriteFailed { source: io::Error },
}

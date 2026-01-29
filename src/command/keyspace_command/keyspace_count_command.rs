use errgonomic::handle;
use fjall::Database;
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct KeyspaceCountCommand {}

impl KeyspaceCountCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, KeyspaceCountCommandRunError> {
        use KeyspaceCountCommandRunError::*;
        let count = db.keyspace_count();
        let mut stdout = io::stdout().lock();
        handle!(writeln!(stdout, "{count}"), WriteFailed);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum KeyspaceCountCommandRunError {
    #[error("failed to write keyspace count to stdout")]
    WriteFailed { source: io::Error },
}

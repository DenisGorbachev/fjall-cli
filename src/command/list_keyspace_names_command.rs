use errgonomic::handle;
use fjall::Database;
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct ListKeyspaceNamesCommand {}

impl ListKeyspaceNamesCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, ListKeyspaceNamesCommandRunError> {
        use ListKeyspaceNamesCommandRunError::*;
        let mut stdout = io::stdout().lock();
        let result = db.list_keyspace_names().into_iter().try_for_each(|name| {
            stdout
                .write_all(name.as_bytes())
                .and_then(|()| stdout.write_all(b"\n"))
        });
        handle!(result, WriteFailed);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum ListKeyspaceNamesCommandRunError {
    #[error("failed to write keyspace names to stdout")]
    WriteFailed { source: io::Error },
}

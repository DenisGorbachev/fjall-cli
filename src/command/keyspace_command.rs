use errgonomic::map_err;
use fjall::Database;
use std::process::ExitCode;
use thiserror::Error;

use KeyspaceSubcommand::*;

#[derive(clap::Parser, Clone, Debug)]
pub struct KeyspaceCommand {
    #[command(subcommand)]
    subcommand: KeyspaceSubcommand,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum KeyspaceSubcommand {
    List(KeyspaceListCommand),
    Count(KeyspaceCountCommand),
}

impl KeyspaceCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, KeyspaceCommandRunError> {
        use KeyspaceCommandRunError::*;
        let Self {
            subcommand,
        } = self;
        match subcommand {
            List(command) => map_err!(command.run(db).await, RunKeyspaceListCommandFailed),
            Count(command) => map_err!(command.run(db).await, RunKeyspaceCountCommandFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum KeyspaceCommandRunError {
    #[error("failed to run keyspace list command")]
    RunKeyspaceListCommandFailed { source: KeyspaceListCommandRunError },

    #[error("failed to run keyspace count command")]
    RunKeyspaceCountCommandFailed { source: KeyspaceCountCommandRunError },
}

mod keyspace_list_command;

pub use keyspace_list_command::*;

mod keyspace_count_command;

pub use keyspace_count_command::*;

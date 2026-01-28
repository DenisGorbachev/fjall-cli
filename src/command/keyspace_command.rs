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
}

impl KeyspaceCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, KeyspaceCommandRunError> {
        use KeyspaceCommandRunError::*;
        let Self {
            subcommand,
        } = self;
        match subcommand {
            List(command) => map_err!(Self::run_keyspace_list_command(command, db).await, RunKeyspaceListCommandFailed),
        }
    }

    pub async fn run_keyspace_list_command(command: KeyspaceListCommand, db: &Database) -> Result<ExitCode, KeyspaceListCommandRunError> {
        command.run(db).await
    }
}

#[derive(Error, Debug)]
pub enum KeyspaceCommandRunError {
    #[error("failed to run keyspace list command")]
    RunKeyspaceListCommandFailed { source: KeyspaceListCommandRunError },
}

mod keyspace_list_command;
pub use keyspace_list_command::*;

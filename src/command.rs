use errgonomic::{handle, map_err};
use fjall::Database;
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

use Subcommand::*;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, propagate_version = true)]
pub struct Command {
    #[arg(long, env = "FJALL_DB")]
    db: PathBuf,

    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Subcommand {
    ListKeyspaceNames(ListKeyspaceNamesCommand),
    KeyspaceCount(KeyspaceCountCommand),
    Keyspace(KeyspaceCommand),
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;
        let Self {
            db: path,
            subcommand,
        } = self;
        let db = handle!(Database::builder(&path).open(), OpenFailed, path);
        map_err!(subcommand.run(&db).await, RunFailed)
    }
}

impl Subcommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, SubcommandRunError> {
        use SubcommandRunError::*;
        match self {
            ListKeyspaceNames(command) => map_err!(command.run(db).await, RunListKeyspaceNamesCommandFailed),
            KeyspaceCount(command) => map_err!(command.run(db).await, RunKeyspaceCountCommandFailed),
            Keyspace(command) => map_err!(command.run(db).await, RunKeyspaceCommandFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to open database at '{path}'")]
    OpenFailed { source: fjall::Error, path: PathBuf },

    #[error("failed to run command")]
    RunFailed { source: SubcommandRunError },
}

#[derive(Error, Debug)]
pub enum SubcommandRunError {
    #[error("failed to run list-keyspace-names command")]
    RunListKeyspaceNamesCommandFailed { source: ListKeyspaceNamesCommandRunError },

    #[error("failed to run keyspace-count command")]
    RunKeyspaceCountCommandFailed { source: KeyspaceCountCommandRunError },

    #[error("failed to run keyspace command")]
    RunKeyspaceCommandFailed { source: KeyspaceCommandRunError },
}

mod keyspace_command;

pub use keyspace_command::*;

mod list_keyspace_names_command;

pub use list_keyspace_names_command::*;

mod keyspace_count_command;

pub use keyspace_count_command::*;

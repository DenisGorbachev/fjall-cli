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
    Keyspace(KeyspaceCommand),
    List(ListCommand),
    Clear(ClearCommand),
    Contains(ContainsCommand),
    Get(GetCommand),
    Insert(InsertCommand),
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;
        let Self {
            db,
            subcommand,
        } = self;
        let database = handle!(Database::builder(&db).open(), OpenFailed, path: db);
        map_err!(subcommand.run(&database).await, RunFailed)
    }
}

impl Subcommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, SubcommandRunError> {
        use SubcommandRunError::*;
        match self {
            Keyspace(command) => map_err!(Self::run_keyspace_command(command, db).await, RunKeyspaceCommandFailed),
            List(command) => map_err!(Self::run_list_command(command, db).await, RunListCommandFailed),
            Clear(command) => map_err!(Self::run_clear_command(command, db).await, RunClearCommandFailed),
            Contains(command) => map_err!(Self::run_contains_command(command, db).await, RunContainsCommandFailed),
            Get(command) => map_err!(Self::run_get_command(command, db).await, RunGetCommandFailed),
            Insert(command) => map_err!(Self::run_insert_command(command, db).await, RunInsertCommandFailed),
        }
    }

    pub async fn run_keyspace_command(command: KeyspaceCommand, db: &Database) -> Result<ExitCode, KeyspaceCommandRunError> {
        command.run(db).await
    }

    pub async fn run_list_command(command: ListCommand, db: &Database) -> Result<ExitCode, ListCommandRunError> {
        command.run(db).await
    }

    pub async fn run_clear_command(command: ClearCommand, db: &Database) -> Result<ExitCode, ClearCommandRunError> {
        command.run(db).await
    }

    pub async fn run_contains_command(command: ContainsCommand, db: &Database) -> Result<ExitCode, ContainsCommandRunError> {
        command.run(db).await
    }

    pub async fn run_get_command(command: GetCommand, db: &Database) -> Result<ExitCode, GetCommandRunError> {
        command.run(db).await
    }

    pub async fn run_insert_command(command: InsertCommand, db: &Database) -> Result<ExitCode, InsertCommandRunError> {
        command.run(db).await
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
    #[error("failed to run keyspace command")]
    RunKeyspaceCommandFailed { source: KeyspaceCommandRunError },

    #[error("failed to run list command")]
    RunListCommandFailed { source: ListCommandRunError },

    #[error("failed to run clear command")]
    RunClearCommandFailed { source: ClearCommandRunError },

    #[error("failed to run contains command")]
    RunContainsCommandFailed { source: ContainsCommandRunError },

    #[error("failed to run get command")]
    RunGetCommandFailed { source: GetCommandRunError },

    #[error("failed to run insert command")]
    RunInsertCommandFailed { source: InsertCommandRunError },
}

mod keyspace_command;
pub use keyspace_command::*;

mod list_command;
pub use list_command::*;

mod clear_command;
pub use clear_command::*;

mod contains_command;
pub use contains_command::*;

mod get_command;
pub use get_command::*;

mod insert_command;
pub use insert_command::*;

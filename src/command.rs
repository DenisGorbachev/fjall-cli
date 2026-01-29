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
    Get(GetCommand),
    Insert(InsertCommand),
    Contains(ContainsCommand),
    Len(LenCommand),
    Clear(ClearCommand),
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
            Keyspace(command) => map_err!(command.run(db).await, RunKeyspaceCommandFailed),
            List(command) => map_err!(command.run(db).await, RunListCommandFailed),
            Get(command) => map_err!(command.run(db).await, RunGetCommandFailed),
            Insert(command) => map_err!(command.run(db).await, RunInsertCommandFailed),
            Contains(command) => map_err!(command.run(db).await, RunContainsCommandFailed),
            Len(command) => map_err!(command.run(db).await, RunLenCommandFailed),
            Clear(command) => map_err!(command.run(db).await, RunClearCommandFailed),
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
    #[error("failed to run keyspace command")]
    RunKeyspaceCommandFailed { source: KeyspaceCommandRunError },

    #[error("failed to run list command")]
    RunListCommandFailed { source: ListCommandRunError },

    #[error("failed to run get command")]
    RunGetCommandFailed { source: GetCommandRunError },

    #[error("failed to run insert command")]
    RunInsertCommandFailed { source: InsertCommandRunError },

    #[error("failed to run contains command")]
    RunContainsCommandFailed { source: ContainsCommandRunError },

    #[error("failed to run len command")]
    RunLenCommandFailed { source: LenCommandRunError },

    #[error("failed to run clear command")]
    RunClearCommandFailed { source: ClearCommandRunError },
}

mod keyspace_command;

pub use keyspace_command::*;

mod list_command;

pub use list_command::*;

mod get_command;

pub use get_command::*;

mod insert_command;

pub use insert_command::*;

mod contains_command;

pub use contains_command::*;

mod len_command;

pub use len_command::*;

mod clear_command;

pub use clear_command::*;

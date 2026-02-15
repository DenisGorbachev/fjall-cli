use crate::{ClearCommand, ClearCommandRunError, ContainsCommand, ContainsCommandRunError, DeleteCommand, DeleteCommandRunError, GetCommand, GetCommandRunError, InsertCommand, InsertCommandRunError, IterCommand, IterCommandRunError, LenCommand, LenCommandRunError};
use errgonomic::map_err;
use fjall::Database;
use std::process::ExitCode;
use thiserror::Error;

use KeyspaceSubcommand::*;

#[derive(clap::Parser, Clone, Debug)]
pub struct KeyspaceCommand {
    #[arg(value_name = "KEYSPACE")]
    keyspace: String,

    #[command(subcommand)]
    subcommand: KeyspaceSubcommand,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum KeyspaceSubcommand {
    Iter(IterCommand),
    Get(GetCommand),
    Insert(InsertCommand),
    Contains(ContainsCommand),
    Len(LenCommand),
    Clear(ClearCommand),
    Delete(DeleteCommand),
}

impl KeyspaceCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, KeyspaceCommandRunError> {
        use KeyspaceCommandRunError::*;
        let Self {
            keyspace,
            subcommand,
        } = self;
        match subcommand {
            Iter(command) => map_err!(command.run(db, keyspace).await, RunIterCommandFailed),
            Get(command) => map_err!(command.run(db, keyspace).await, RunGetCommandFailed),
            Insert(command) => map_err!(command.run(db, keyspace).await, RunInsertCommandFailed),
            Contains(command) => map_err!(command.run(db, keyspace).await, RunContainsCommandFailed),
            Len(command) => map_err!(command.run(db, keyspace).await, RunLenCommandFailed),
            Clear(command) => map_err!(command.run(db, keyspace).await, RunClearCommandFailed),
            Delete(command) => map_err!(command.run(db, keyspace).await, RunDeleteCommandFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum KeyspaceCommandRunError {
    #[error("failed to run iter command")]
    RunIterCommandFailed { source: IterCommandRunError },

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

    #[error("failed to run delete command")]
    RunDeleteCommandFailed { source: DeleteCommandRunError },
}

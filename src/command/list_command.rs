use crate::{OutputKind, OutputKindWriteError, Separator};
use errgonomic::{handle, handle_bool, map_err};
use fjall::{Database, Guard, Keyspace, KeyspaceCreateOptions};
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct ListCommand {
    #[arg(value_name = "KEYSPACE")]
    keyspace: String,

    #[arg(long, default_value = ": ")]
    key_value_separator: Separator,

    #[arg(long, default_value = "\\n", help = "Separator between items (written after every item, including the last one).")]
    item_separator: Separator,

    #[arg(long, value_enum, default_value_t = OutputKind::KeyValue)]
    kind: OutputKind,
}

impl ListCommand {
    pub async fn run(self, db: &Database) -> Result<ExitCode, ListCommandRunError> {
        use ListCommandRunError::*;
        let Self {
            keyspace,
            key_value_separator,
            item_separator,
            kind,
        } = self;
        handle_bool!(!db.keyspace_exists(&keyspace), KeyspaceNotFound, keyspace);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        let mut stdout = io::stdout().lock();
        handle!(Self::write_items(&mut stdout, &keyspace_handle, &kind, &key_value_separator, &item_separator), WriteItemsFailed, keyspace);
        Ok(ExitCode::SUCCESS)
    }

    pub fn write_items(writer: &mut impl Write, keyspace: &Keyspace, kind: &OutputKind, key_value_separator: &Separator, item_separator: &Separator) -> Result<(), ListCommandWriteItemsError> {
        use ListCommandWriteItemsError::*;
        let result = keyspace
            .iter()
            .try_for_each(|guard| Self::write_item(writer, kind, key_value_separator, item_separator, guard));
        map_err!(result, WriteItemFailed)
    }

    pub fn write_item(writer: &mut impl Write, kind: &OutputKind, key_value_separator: &Separator, item_separator: &Separator, guard: Guard) -> Result<(), ListCommandWriteItemError> {
        use ListCommandWriteItemError::*;
        let (key, value) = handle!(guard.into_inner(), IntoInnerFailed);
        handle!(kind.write(writer, key.as_ref(), value.as_ref(), key_value_separator), WriteFailed);
        handle!(writer.write_all(item_separator.as_bytes()), WriteAllFailed);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ListCommandRunError {
    #[error("keyspace '{keyspace}' not found")]
    KeyspaceNotFound { keyspace: String },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to write items for keyspace '{keyspace}'")]
    WriteItemsFailed { source: ListCommandWriteItemsError, keyspace: String },
}

#[derive(Error, Debug)]
pub enum ListCommandWriteItemsError {
    #[error("failed to write item")]
    WriteItemFailed { source: ListCommandWriteItemError },
}

#[derive(Error, Debug)]
pub enum ListCommandWriteItemError {
    #[error("failed to read key-value pair")]
    IntoInnerFailed { source: fjall::Error },

    #[error("failed to write output")]
    WriteFailed { source: OutputKindWriteError },

    #[error("failed to write item separator")]
    WriteAllFailed { source: io::Error },
}

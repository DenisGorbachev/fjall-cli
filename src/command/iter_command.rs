use crate::{OutputAffixes, OutputKind, OutputKindWriteError, PrefixKind, Suffix};
use errgonomic::{handle, handle_bool, map_err};
use fjall::{Database, Guard, Keyspace, KeyspaceCreateOptions};
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(clap::Parser, Clone, Debug)]
pub struct IterCommand {
    #[arg(long, value_enum, default_value_t = OutputKind::KeyValue)]
    kind: OutputKind,

    #[arg(long, value_enum)]
    item_prefix: Option<PrefixKind>,

    #[arg(long)]
    item_suffix: Option<Suffix>,

    #[arg(long, value_enum)]
    key_prefix: Option<PrefixKind>,

    #[arg(long)]
    key_suffix: Option<Suffix>,

    #[arg(long, value_enum)]
    value_prefix: Option<PrefixKind>,

    #[arg(long)]
    value_suffix: Option<Suffix>,

    #[arg(long, default_value_t = 0, help = "Number of items to skip before writing output.")]
    offset: usize,

    #[arg(long, help = "Maximum number of items to write.")]
    limit: Option<usize>,
}

impl IterCommand {
    pub async fn run(self, db: &Database, keyspace: impl Into<String>) -> Result<ExitCode, IterCommandRunError> {
        use IterCommandRunError::*;
        let keyspace = keyspace.into();
        let Self {
            kind,
            item_prefix,
            item_suffix,
            key_prefix,
            key_suffix,
            value_prefix,
            value_suffix,
            offset,
            limit,
        } = self;
        let affixes = OutputAffixes {
            item_prefix,
            item_suffix,
            key_prefix,
            key_suffix,
            value_prefix,
            value_suffix,
        };
        handle_bool!(!db.keyspace_exists(&keyspace), KeyspaceNotFound, keyspace);
        let keyspace_handle = handle!(db.keyspace(&keyspace, KeyspaceCreateOptions::default), KeyspaceFailed, keyspace);
        let mut stdout = io::stdout().lock();
        handle!(Self::write_items(&mut stdout, &keyspace_handle, &kind, &affixes, offset, limit,), WriteItemsFailed, keyspace);
        Ok(ExitCode::SUCCESS)
    }

    pub fn write_items(writer: &mut impl Write, keyspace: &Keyspace, kind: &OutputKind, affixes: &OutputAffixes, offset: usize, limit: Option<usize>) -> Result<(), IterCommandWriteItemsError> {
        use IterCommandWriteItemsError::*;
        let result = match limit {
            Some(limit) => keyspace
                .iter()
                .skip(offset)
                .take(limit)
                .try_for_each(|guard| Self::write_item(writer, kind, affixes, guard)),
            None => keyspace
                .iter()
                .skip(offset)
                .try_for_each(|guard| Self::write_item(writer, kind, affixes, guard)),
        };
        map_err!(result, WriteItemFailed)
    }

    pub fn write_item(writer: &mut impl Write, kind: &OutputKind, affixes: &OutputAffixes, guard: Guard) -> Result<(), IterCommandWriteItemError> {
        use IterCommandWriteItemError::*;
        let (key, value) = handle!(guard.into_inner(), IntoInnerFailed);
        handle!(kind.write(writer, &key, &value, affixes), WriteFailed);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum IterCommandRunError {
    #[error("keyspace '{keyspace}' not found")]
    KeyspaceNotFound { keyspace: String },

    #[error("failed to open keyspace '{keyspace}'")]
    KeyspaceFailed { source: fjall::Error, keyspace: String },

    #[error("failed to write items for keyspace '{keyspace}'")]
    WriteItemsFailed { source: IterCommandWriteItemsError, keyspace: String },
}

#[derive(Error, Debug)]
pub enum IterCommandWriteItemsError {
    #[error("failed to write item")]
    WriteItemFailed { source: IterCommandWriteItemError },
}

#[derive(Error, Debug)]
pub enum IterCommandWriteItemError {
    #[error("failed to read key-value pair")]
    IntoInnerFailed { source: fjall::Error },

    #[error("failed to write output")]
    WriteFailed { source: OutputKindWriteError },
}

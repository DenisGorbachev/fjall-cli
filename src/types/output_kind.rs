use crate::OutputAffixes;
use errgonomic::handle;
use fjall::Slice;
use std::io;
use std::io::Write;
use thiserror::Error;

#[derive(clap::ValueEnum, Copy, Clone, Debug, Default)]
#[clap(rename_all = "kebab")]
pub enum OutputKind {
    Key,
    Value,
    #[default]
    KeyValue,
}

use OutputKind::*;

impl OutputKind {
    pub fn write(&self, writer: &mut impl Write, key: &Slice, value: &Slice, affixes: &OutputAffixes) -> Result<(), OutputKindWriteError> {
        use OutputKindWriteError::*;
        let item_prefix = affixes.item_prefix;
        let key_prefix = affixes.key_prefix;
        let value_prefix = affixes.value_prefix;
        let item_suffix = affixes.item_suffix.as_ref();
        let key_suffix = affixes.key_suffix.as_ref();
        let value_suffix = affixes.value_suffix.as_ref();
        match self {
            Key => {
                if let Some(prefix) = item_prefix {
                    let bytes = prefix.write(key);
                    handle!(writer.write_all(&bytes), WriteAllFailed);
                }
                if let Some(prefix) = key_prefix {
                    let bytes = prefix.write(key);
                    handle!(writer.write_all(&bytes), WriteAllFailed);
                }
                handle!(writer.write_all(key.as_ref()), WriteAllFailed);
                if let Some(suffix) = key_suffix {
                    handle!(writer.write_all(suffix.as_bytes()), WriteAllFailed);
                }
                if let Some(suffix) = item_suffix {
                    handle!(writer.write_all(suffix.as_bytes()), WriteAllFailed);
                }
            }
            Value => {
                if let Some(prefix) = item_prefix {
                    let bytes = prefix.write(value);
                    handle!(writer.write_all(&bytes), WriteAllFailed);
                }
                if let Some(prefix) = value_prefix {
                    let bytes = prefix.write(value);
                    handle!(writer.write_all(&bytes), WriteAllFailed);
                }
                handle!(writer.write_all(value.as_ref()), WriteAllFailed);
                if let Some(suffix) = value_suffix {
                    handle!(writer.write_all(suffix.as_bytes()), WriteAllFailed);
                }
                if let Some(suffix) = item_suffix {
                    handle!(writer.write_all(suffix.as_bytes()), WriteAllFailed);
                }
            }
            KeyValue => {
                if let Some(prefix) = item_prefix {
                    let bytes = prefix.write(key);
                    handle!(writer.write_all(&bytes), WriteAllFailed);
                }
                if let Some(prefix) = key_prefix {
                    let bytes = prefix.write(key);
                    handle!(writer.write_all(&bytes), WriteAllFailed);
                }
                handle!(writer.write_all(key.as_ref()), WriteAllFailed);
                if let Some(suffix) = key_suffix {
                    handle!(writer.write_all(suffix.as_bytes()), WriteAllFailed);
                }
                if let Some(prefix) = value_prefix {
                    let bytes = prefix.write(value);
                    handle!(writer.write_all(&bytes), WriteAllFailed);
                }
                handle!(writer.write_all(value.as_ref()), WriteAllFailed);
                if let Some(suffix) = value_suffix {
                    handle!(writer.write_all(suffix.as_bytes()), WriteAllFailed);
                }
                if let Some(suffix) = item_suffix {
                    handle!(writer.write_all(suffix.as_bytes()), WriteAllFailed);
                }
            }
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum OutputKindWriteError {
    #[error("failed to write output")]
    WriteAllFailed { source: io::Error },
}

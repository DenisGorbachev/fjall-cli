//! ## Command
//!
//! The command requires `--db <PATH>` or `FJALL_DB`.
//!
//! ## Subcommands
//!
//! * `keyspace <KEYSPACE> <SUBCOMMAND>` runs keyspace-scoped operations:
//!   `iter`, `get`, `insert`, `contains`, `len`, `disk-size`, `clear`, `delete`.
//! * `list-keyspace-names` lists keyspace names (one per line).
//! * `keyspace-count` prints the number of keyspaces.
//!
//! Exit codes for `keyspace <KEYSPACE> contains <KEY>`:
//!
//! * `0` - key exists
//! * `1` - error occurred
//! * `127` - key does not exist
//!
//! ## Byte encodings
//!
//! Commands that accept key/value bytes support `--*-encoding` with:
//!
//! * `string` (default)
//! * `hex`
//! * `path`
//! * `empty` (argument must be exactly `-`)
//!
//! `keyspace <KEYSPACE> disk-size` flushes pending writes for that keyspace before
//! measuring bytes on disk, so recent inserts are reflected in the reported size.
//!
//! ## Example
//!
//! ```bash
//! DB_DIR="$(mktemp -d)"
//! export FJALL_DB="$DB_DIR"
//!
//! fjall keyspace items insert key value
//! fjall keyspace items len
//! # 1
//!
//! fjall keyspace-count
//! # 1
//!
//! fjall keyspace items contains key
//! # exit code: 0
//! fjall keyspace items contains missing
//! # exit code: 127
//!
//! fjall keyspace items get key
//! # value
//!
//! fjall keyspace items iter --key-suffix ":" --value-suffix $'\n'
//! # key:value
//!
//! fjall keyspace items disk-size
//! # decimal bytes, greater than 0 after the insert above
//!
//! fjall list-keyspace-names
//! # items
//!
//! fjall keyspace items clear
//! fjall keyspace items len
//! # 0
//!
//! fjall keyspace items delete
//! fjall keyspace-count
//! # 0
//! ```

mod command;

pub use command::*;

mod types;

pub use types::*;

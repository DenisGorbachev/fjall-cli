//! CLI for the [Fjall](https://crates.io/crates/fjall) key-value database.
//!
//! ## Command
//!
//! The command requires `--db <PATH>` or `FJALL_DB`.
//!
//! ## Subcommands
//!
//! * `keyspace <KEYSPACE> <SUBCOMMAND>` runs keyspace-scoped operations:
//!   `iter`, `get`, `insert`, `contains`, `len`, `clear`, `delete`.
//! * `list-keyspace-names` lists keyspace names (one per line).
//! * `keyspace-count` prints the number of keyspaces.
//!
//! Exit codes for `keyspace <KEYSPACE> contains <KEY>`:
//! * `0` - key exists
//! * `1` - error occurred
//! * `127` - key does not exist
//!
//! ## Byte encodings
//!
//! Commands that accept key/value bytes support `--*-encoding` with:
//! * `string` (default)
//! * `hex`
//! * `path`
//! * `empty` (argument must be exactly `-`)
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
//! fjall list-keyspace-names
//! # items
//!
//! fjall keyspace items delete
//! fjall keyspace-count
//! # 0
//! ```
//!
//! ## Known bugs
//!
//! * `clear` may not persistently clear a keyspace in fjall v3.0.1 due to
//!   [fjall-rs/fjall#241](https://github.com/fjall-rs/fjall/issues/241).

mod command;

pub use command::*;

mod types;

pub use types::*;

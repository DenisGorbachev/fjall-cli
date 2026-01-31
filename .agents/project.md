# Concepts for fjall-cli package

## fjall-cli

A package that implements a CLI for Fjall key-value database.

Requirements:

- Must produce a single binary with `fjall` name
- Must contain an integration test in `tests/full.rs`
  - Requirements:
    - Must create a temp dir for the database using `tempdir` crate
    - Must exercise every command
- Must use `xshell` version = "0.3.0-pre.2" in tests

## Command

A parent CLI command that opens a `fjall::Database` and delegates execution to a selected subcommand.

Synonyms:

- Parent command.

Requirements:

- Must accept `--db <PATH>` as a required option (may be specified as `FJALL_DB` environment variable).
- Must construct a `fjall::Database` via `Database::builder(path).open()`.
- Must pass a reference to the opened `fjall::Database` to the selected subcommand.
- Every child command that reads the keyspace must check if the keyspace exists with `db.keyspace_exists` (return an error if it doesn't exist) before opening the keyspace

Notes:

- This CLI targets `fjall::Database` (non-transactional).

## Subcommand

A child-command enum that selects one database operation exposed by this CLI.

Constructors:

- Keyspace.
- List.
- Get.
- Insert.
- Contains.
- Len.
- Clear.

## KeyspaceCommand

A command that contains Keyspace-prefixed subcommands.

## KeyspaceListCommand

A command that lists all keyspaces in a `fjall::Database`.

Requirements:

- Must call `Database::list_keyspace_names()`.
- Must write keyspace names to stdout as UTF-8 bytes.
- Must write exactly one `\n` byte after each keyspace name.

## KeyspaceCountCommand

A command that outputs the count of keyspaces in a `fjall::Database`.

Requirements:

- Must call `Database::keyspace_count()`.

## ListCommand

A command that streams key-value pairs from a single keyspace.

Examples:

- List all entries in one keyspace.
  - `fjall --db ./db list my_items`.
- List values separated by \0.
  - `fjall --db ./db list my_items --kind value --value-suffix "\0"`.

Requirements:

- Must require `keyspace` as a positional argument.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must iterate using `Keyspace::iter()`.
- Must accept `--kind <OutputKind>`
- Must accept `--item-prefix <PrefixKind>` (default: None)
- Must accept `--item-suffix <Suffix>` (default: None).
- Must accept `--key-prefix <PrefixKind>` (default: None).
- Must accept `--key-suffix <Suffix>` (default: None).
- Must accept `--value-prefix <PrefixKind>` (default: None).
- Must accept `--value-suffix <Suffix>` (default: None).
- Must accept `--offset` and apply it to iter
- Must accept `--limit` and apply it to iter
- Must call `kind.write` with `&mut stdout` for each `(key, value)` pair encountered
- Must stream output to stdout without building an unbounded in-memory list.
- Must return an error if the keyspace doesn't exist.

Notes:

- The `--key-value-separator` is replaced by `--key-suffix`

## LenCommand

A command that outputs the keyspace len.

Requirements:

- Must use `len`, not `approximate_len`

## ClearCommand

A command that clears a keyspace.

Examples:

- Clear one keyspace.
  - `fjall --db ./db clear my_items`.

Requirements:

- Must require `keyspace` as a positional argument.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must call `Keyspace::clear()`.
- Must treat a non-existent keyspace name as an error.

Notes:

- `Keyspace::clear` is buggy in fjall 3.0.1: the items are restored from the journal on the next run.
  - Waiting for <https://github.com/fjall-rs/fjall/pull/242> to be merged

## DeleteCommand

A command that deletes a keyspace.

Requirements:

- Must call `Database::delete_keyspace()`.

Notes:

- `Database::delete_keyspace` is `#[doc(hidden)]`

## ContainsCommand

A command that checks whether a key exists in a keyspace.

Synonyms:

- `contains`.

Examples:

- Check existence with UTF-8 key.
  - `fjall --db ./db contains my_items my_key`.
- Check existence with hex key.
  - `fjall --db ./db contains my_items deadbeef --key-encoding hex`.
- Check existence with key bytes from a file.
  - `fjall --db ./db contains my_items ./key.bin --key-encoding path`.

Requirements:

- Must parse `keyspace` as a required positional argument.
- Must parse `key` as a required positional argument.
- Must accept `--key-encoding <ByteEncoding>`.
- Must decode the `key` argument into a byte vector according to `--key-encoding`.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must call `Keyspace::contains_key(key_bytes)`.
- Must exit with the following codes (document this):
  - Key exists: 0.
  - Failure (any error): 1.
  - Key does not exist: 127.

Preferences:

- Should not write anything to stdout.
- Should default `--key-encoding` to `string`.

## GetCommand

A command that retrieves a value for a key from a keyspace.

Examples:

- Get a value.
  - `fjall --db ./db get my_items my_key`.
- Get a value without the trailing newline.
  - `fjall --db ./db get my_items my_key -n`.

Requirements:

- Must parse `keyspace` as a required positional argument.
- Must parse `key` as a required positional argument.
- Must accept `--key-encoding <ByteEncoding>`.
- Must accept `--no-newline` (or `-n`).
- Must decode the `key` argument into a byte vector according to `--key-encoding`.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must call `Keyspace::get(key_bytes)`.
- If the key-value pair is present:
  - Then: Must write the value bytes to stdout.
    - Must write a single `\n` byte to stdout unless `--no-newline` is true
  - Else: Must return an `KeyNotFound`.

Preferences:

- Should default `--key-encoding` to `string`.

Notes:

- Appending a newline is a convenience feature and is not a lossless operation.

## InsertCommand

A command that writes a key-value pair into a keyspace.

Examples:

- Insert a UTF-8 key and UTF-8 value.
  - `fjall --db ./db insert my_items my_key my_value`.
- Insert a hex key and value.
  - `fjall --db ./db insert my_items deadbeef cafe --key-encoding hex --value-encoding hex`.
- Insert a value from a file.
  - `fjall --db ./db insert my_items my_key ./value.bin --value-encoding path`.

Requirements:

- Must parse `keyspace` as a required positional argument.
- Must parse `key` as a required positional argument.
- Must parse `value` as a required positional argument.
- Must accept `--key-encoding <ByteEncoding>`.
- Must accept `--value-encoding <ByteEncoding>`.
- Must decode the `key` argument into a byte vector according to `--key-encoding`.
- Must decode the `value` argument into a byte vector according to `--value-encoding`.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must call `Keyspace::insert(key_bytes, value_bytes)`.
- Must exit with code `0` on success.

Preferences:

- Should default `--key-encoding` to `string`.
- Should default `--value-encoding` to `string`.

Notes:

- "InsertCommand" name was chosen to align with "insert" method name in `fjall`

## ByteEncoding

An encoding that maps a single CLI argument into an arbitrary byte vector.

Constructors:

- empty.
- string (default).
- hex.
- path.

Requirements:

- `string` must accept any valid UTF-8 string and produce the corresponding bytes.
- `hex` must accept an even-length sequence of hexadecimal characters (case-insensitive).
- `path` must accept an existing filesystem path and read bytes without modification.

- Must map the CLI argument:
  - `empty`: from `String` to an empty `Vec<u8>` (needed to represent empty values) (the CLI argument must be exactly equal to "-" in this case).
  - `string`: from `String` to `Vec<u8>` (without changes).
  - `hex`: from `String` as hex-decoded bytes (two hex digits per byte) (case-insensitive) to `Vec<u8>`.
  - `path`: from `String` as file path to `Vec<u8>` as the entire file contents as bytes.
    - Must treat a `path` pointing to a directory as an error.

## Suffix

A `String` that is inserted after output fragments.

## OutputKind

A kind of output for `ListCommand`.

Constructors:

- Key
- Value
- KeyValue (default)

Methods:

- `write`
  - Must write either a key, a value, a key-value pair (depending on the value of `self`)
  - Must write the prefixes if they are `Some`:
    - Must call `item_prefix.write` at the start of every write
    - Must call `key_prefix.write` before a key
    - Must call `value_prefix.write` before a value
  - Must write the suffixes if they are `Some`

## PrefixKind

A kind of prefix for `ListCommand`.

Constructors:

- LenU64Le
- LenU64Be

Methods:

- `write(self, slice: &Slice)`
  - Must write the prefix according to variant in `self`

Notes:

- `slice.len()` returns `usize`, so we can only losslessly cast len to `u64` (not `u32`)
- Use `#[clap(rename_all = "kebab")]`
- `Le` and `Be` refers to little-endian and big-endian

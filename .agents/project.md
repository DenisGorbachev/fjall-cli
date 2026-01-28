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

Notes:

- This CLI targets `fjall::Database` (non-transactional).

## CommandSubcommand

A child-command enum that selects one database operation exposed by this CLI.

Constructors:

- Keyspace.
- List.
- Clear.
- Contains.
- Get.
- Set.

Notes:

- The `Keyspace` constructor corresponds to a nested command group that includes `keyspace list`.

## KeyspaceCommand

A nested command group for keyspace-scoped meta-operations.

Constructors:

- List.

Requirements:

- Must contain a `KeyspaceListCommand`.

## KeyspaceListCommand

A command that lists all keyspaces in a `fjall::Database`.

Requirements:

- Must call `Database::list_keyspace_names()`.
- Must write keyspace names to stdout as UTF-8 bytes.
- Must write exactly one `\n` byte after each keyspace name.

## ListCommand

A command that streams key-value pairs from a single keyspace.

Examples:

- List all entries in one keyspace.
  - `fjall --db ./db list my_items`.
- List values separated by \0.
  - `fjall --db ./db list my_items --value-separator "\0"`.

Requirements:

- Must require `keyspace` as a positional argument.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must iterate using `Keyspace::iter()`.
- Must accept `--key-value-separator <Separator>` (default: ": " (a semicolon followed by a space)).
- Must accept `--pair-separator <Separator>` (default: "\n" (a newline)).
- Must accept `--kind <OutputKind>`
- Must call `kind.write(&mut stdout, key, value, key_value_separator)` for each `(key, value)` pair encountered
- Must write an `pair_separtor` after each `(key, value)` pair, including the last one (and document that fact)
- Must stream output to stdout without building an unbounded in-memory list.

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
- Must write the value bytes to stdout (if value is present).
- Must write a single `\n` byte to stdout unless `--no-newline` is true

Preferences:

- Should default `--key-encoding` to `string`.

Notes:

- Appending a newline is a convenience feature and is not a lossless operation.

## SetCommand

A command that writes a key-value pair into a keyspace.

Synonyms:

- `set`.

Examples:

- Set a UTF-8 key and UTF-8 value.
  - `fjall --db ./db set my_items my_key my_value`.
- Set a hex key and value.
  - `fjall --db ./db set my_items deadbeef cafe --key-encoding hex --value-encoding hex`.
- Set a value from a file.
  - `fjall --db ./db set my_items my_key ./value.bin --value-encoding path`.

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

- Must process the CLI argument:
  - `empty`: as empty `Vec<u8>` (needed to represent empty values) (the CLI argument must be exactly equal to "-" in this case).
  - `string`: as `String`.
  - `hex`: as hex-decoded bytes (two hex digits per byte) (case-insensitive).
  - `path`: as a file path and read the entire file as bytes.
    - Must treat a `path` pointing to a directory as an error.

## Separator

A byte sequence produced from a CLI argument that is inserted between output fragments.

Requirements:

- Must be representable from a single CLI argument.
- Must support `\n` to produce a single newline byte (0x0A).
- Must support `\r` to produce a single carriage-return byte (0x0D).
- Must support `\t` to produce a single tab byte (0x09).
- Must support `\xNN` (two hex digits) to produce one byte.

Preferences:

- Should support `\0` to produce a single NUL byte (0x00).

## OutputKind

A kind of output for `ListCommand`.

Constructors:

- Key
- Value
- KeyValue (default)

Methods:

- `write`
  - Must write either a key, a value, a key-value pair (depending on the value of `self`) 

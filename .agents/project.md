# Concepts for fjall-cli package

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
  - `fjall-cli --db ./db list my_items`.
- List values separated by \0.
  - `fjall-cli --db ./db list my_items --value-separator "\0"`.

Requirements:

- Must require `keyspace` as a positional argument.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must iterate using `Keyspace::iter()`.
- Must accept `--value-separator <KeyValueSeparator>`.
- Must accept `--kind <OutputKind>`
- For each `(key, value)` pair encountered:
  - Must write `key` bytes to stdout.
  - Must write `value-separator` bytes to stdout.
  - Must write `value` bytes to stdout.
  - Must write a single `` byte to stdout.
- Must stream output to stdout without building an unbounded in-memory list.

Preferences:

- Should default `--value-separator` to a single `` byte.

Notes:

- This output format is not a fully self-describing framing format when keys and values are arbitrary bytes.
- The command is intended for simple debugging and piping, not for lossless structured export.

## ClearCommand

A command that clears a keyspace.

Synonyms:

- `clear`.

Examples:

- Clear one keyspace.
  - `fjall-cli --db ./db clear my_items`.

Requirements:

- Must require `keyspace` as a positional argument.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must call `Keyspace::clear()`.
- Must treat a non-existent keyspace name as an error.

Notes:

- Clearing a keyspace is destructive.

## ContainsCommand

A command that checks whether a key exists in a keyspace.

Synonyms:

- `contains`.

Examples:

- Check existence with UTF-8 key.
  - `fjall-cli --db ./db contains my_items my_key`.
- Check existence with hex key.
  - `fjall-cli --db ./db contains my_items deadbeef --key-encoding hex`.
- Check existence with key bytes from a file.
  - `fjall-cli --db ./db contains my_items ./key.bin --key-encoding path`.

Requirements:

- Must parse `keyspace` as a required positional argument.
- Must parse `key` as a required positional argument.
- Must accept `--key-encoding <ByteEncoding>`.
- Must decode the `key` argument into a byte vector according to `--key-encoding`.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must call `Keyspace::contains_key(key_bytes)`.
- If the key exists:
  - Must exit with code `0`.
- If the key does not exist:
  - Must exit with code `1`.
- On errors:
  - Must exit with a code other than `0` and `1`.

Preferences:

- Should not write anything to stdout.
- Should default `--key-encoding` to `string`.

## GetCommand

A command that retrieves a value for a key from a keyspace.

Synonyms:

- `get`.

Examples:

- Get a value.
  - `fjall-cli --db ./db get my_items my_key`.
- Get a value without the trailing newline.
  - `fjall-cli --db ./db get my_items my_key -n`.

Requirements:

- Must parse `keyspace` as a required positional argument.
- Must parse `key` as a required positional argument.
- Must accept `--key-encoding <ByteEncoding>`.
- Must accept `--no-newline` (or `-n`).
- Must decode the `key` argument into a byte vector according to `--key-encoding`.
- Must open the keyspace via `Database::keyspace(keyspace, ...)`.
- Must call `Keyspace::get(key_bytes)`.
- If a value is present:
  - Must write the value bytes to stdout.
  - Unless `--no-newline` is present, must then write a single `` byte to stdout.
  - Must exit with code `0`.
- If a value is not present:
  - Must exit with code `1`.

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
  - `fjall-cli --db ./db set my_items my_key my_value`.
- Set a hex key and value.
  - `fjall-cli --db ./db set my_items deadbeef cafe --key-encoding hex --value-encoding hex`.
- Set a value from a file.
  - `fjall-cli --db ./db set my_items my_key ./value.bin --value-encoding path`.

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
- string.
- hex.
- path.

Examples:

- `empty`: interpret the CLI argument as empty `Vec<u8>` (needed to represent empty values) (the CLI argument must be exactly equal to "-" in this case).
- `string`: interpret the CLI argument as UTF-8 bytes.
- `hex`: interpret the CLI argument as hex-decoded bytes (two hex digits per byte) (case-insensitive).
- `path`: interpret the CLI argument as a file path and read the entire file as bytes.

Requirements:

- Should treat an empty `hex` string as an empty byte vector.
- Should treat a `path` pointing to a directory as an error.

An encoding that maps a single CLI argument into an arbitrary byte vector.

Constructors:

- string.
- hex.
- path.

Requirements:

- `string` must accept any valid UTF-8 string and produce the corresponding bytes.
- `hex` must accept an even-length sequence of hexadecimal characters (case-insensitive).
- `path` must accept an existing filesystem path and read bytes without modification.
- Must treat a `path` pointing to a directory as an error.

## KeyValueSeparator

A byte sequence produced from a CLI argument that is inserted between a key and value during `list` output.

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

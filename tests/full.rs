use errgonomic::{handle, handle_bool, handle_opt};
use std::string::FromUtf8Error;
use tempdir::TempDir;
use thiserror::Error;
use xshell::{Shell, cmd};

#[test]
fn full_cli_flow() -> Result<(), FullCliFlowError> {
    use FullCliFlowError::*;
    let bin = env!("CARGO_BIN_EXE_fjall");
    let temp_dir = handle!(TempDir::new("fjall_cli"), TempDirNewFailed);
    let db_path = temp_dir.path();
    let sh = handle!(Shell::new(), ShellNewFailed);
    let sh = sh.with_var("FJALL_DB", db_path);

    handle!(cmd!(sh, "{bin} insert items key value").run(), InsertRunFailed);

    let output = handle!(cmd!(sh, "{bin} len items").output(), LenOutputFailed);
    let stdout = handle!(String::from_utf8(output.stdout), LenUtf8Failed);
    assert_eq!(stdout, "1\n");

    let output = handle!(cmd!(sh, "{bin} keyspace count").output(), KeyspaceCountOutputFailed);
    let stdout = handle!(String::from_utf8(output.stdout), KeyspaceCountUtf8Failed);
    assert_eq!(stdout, "1\n");

    let output = handle!(
        cmd!(sh, "{bin} contains items key")
            .ignore_status()
            .output(),
        ContainsExistingOutputFailed
    );
    handle_bool!(!output.stdout.is_empty(), ContainsExistingStdoutNotEmpty);
    let status = handle_opt!(output.status.code(), ContainsExistingStatusMissing);
    assert_eq!(status, 0);

    let output = handle!(
        cmd!(sh, "{bin} contains items missing")
            .ignore_status()
            .output(),
        ContainsMissingOutputFailed
    );
    handle_bool!(!output.stdout.is_empty(), ContainsMissingStdoutNotEmpty);
    let status = handle_opt!(output.status.code(), ContainsMissingStatusMissing);
    assert_eq!(status, 127);

    let output = handle!(cmd!(sh, "{bin} get items key").output(), GetOutputFailed);
    let stdout = handle!(String::from_utf8(output.stdout), GetUtf8Failed);
    assert_eq!(stdout, "value\n");

    let output = handle!(cmd!(sh, "{bin} list items").output(), ListOutputFailed);
    let stdout = handle!(String::from_utf8(output.stdout), ListUtf8Failed);
    assert_eq!(stdout, "key: value\n");

    let output = handle!(cmd!(sh, "{bin} keyspace list").output(), KeyspaceListOutputFailed);
    let stdout = handle!(String::from_utf8(output.stdout), KeyspaceListUtf8Failed);
    assert_eq!(stdout, "items\n");

    handle!(cmd!(sh, "{bin} clear items").run(), ClearRunFailed);

    let output = handle!(cmd!(sh, "{bin} list items").output(), ListAfterClearOutputFailed);
    let stdout = handle!(String::from_utf8(output.stdout), ListAfterClearUtf8Failed);
    assert_eq!(stdout, "");

    let output = handle!(cmd!(sh, "{bin} len items").output(), LenAfterClearOutputFailed);
    let stdout = handle!(String::from_utf8(output.stdout), LenAfterClearUtf8Failed);
    assert_eq!(stdout, "0\n");

    Ok(())
}

#[derive(Error, Debug)]
pub enum FullCliFlowError {
    #[error("failed to create shell")]
    ShellNewFailed { source: xshell::Error },

    #[error("failed to create temp dir")]
    TempDirNewFailed { source: std::io::Error },

    #[error("failed to run insert command")]
    InsertRunFailed { source: xshell::Error },

    #[error("failed to run len command")]
    LenOutputFailed { source: xshell::Error },

    #[error("failed to decode len output as utf-8")]
    LenUtf8Failed { source: FromUtf8Error },

    #[error("failed to run keyspace count command")]
    KeyspaceCountOutputFailed { source: xshell::Error },

    #[error("failed to decode keyspace count output as utf-8")]
    KeyspaceCountUtf8Failed { source: FromUtf8Error },

    #[error("failed to run contains command for existing key")]
    ContainsExistingOutputFailed { source: xshell::Error },

    #[error("contains command wrote to stdout for existing key")]
    ContainsExistingStdoutNotEmpty {},

    #[error("contains command exit status missing for existing key")]
    ContainsExistingStatusMissing {},

    #[error("failed to run contains command for missing key")]
    ContainsMissingOutputFailed { source: xshell::Error },

    #[error("contains command wrote to stdout for missing key")]
    ContainsMissingStdoutNotEmpty {},

    #[error("contains command exit status missing for missing key")]
    ContainsMissingStatusMissing {},

    #[error("failed to run get command")]
    GetOutputFailed { source: xshell::Error },

    #[error("failed to decode get output as utf-8")]
    GetUtf8Failed { source: FromUtf8Error },

    #[error("failed to run list command")]
    ListOutputFailed { source: xshell::Error },

    #[error("failed to decode list output as utf-8")]
    ListUtf8Failed { source: FromUtf8Error },

    #[error("failed to run keyspace list command")]
    KeyspaceListOutputFailed { source: xshell::Error },

    #[error("failed to decode keyspace list output as utf-8")]
    KeyspaceListUtf8Failed { source: FromUtf8Error },

    #[error("failed to run clear command")]
    ClearRunFailed { source: xshell::Error },

    #[error("failed to run list command after clear")]
    ListAfterClearOutputFailed { source: xshell::Error },

    #[error("failed to decode list output after clear as utf-8")]
    ListAfterClearUtf8Failed { source: FromUtf8Error },

    #[error("failed to run len command after clear")]
    LenAfterClearOutputFailed { source: xshell::Error },

    #[error("failed to decode len output after clear as utf-8")]
    LenAfterClearUtf8Failed { source: FromUtf8Error },
}

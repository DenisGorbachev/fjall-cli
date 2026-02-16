use errgonomic::handle;
use std::string::FromUtf8Error;
use tempdir::TempDir;
use thiserror::Error;
use xshell::{Shell, cmd};

#[test]
fn clear_repro() -> Result<(), ClearReproError> {
    use ClearReproError::*;
    let bin = env!("CARGO_BIN_EXE_fjall");
    let temp_dir = handle!(TempDir::new("fjall_cli"), TempDirNewFailed);
    let db_path = temp_dir.path();
    let sh = handle!(Shell::new(), ShellNewFailed);
    let sh = sh.with_var("FJALL_DB", db_path);

    handle!(cmd!(sh, "{bin} keyspace items insert key value").run(), InsertRunFailed);
    handle!(cmd!(sh, "{bin} keyspace items clear").run(), ClearRunFailed);

    let output = handle!(cmd!(sh, "{bin} keyspace items iter").output(), IterOutputFailed);
    let stdout = handle!(String::from_utf8(output.stdout), IterUtf8Failed);
    assert_eq!(stdout, "");
    Ok(())
}

#[derive(Error, Debug)]
pub enum ClearReproError {
    #[error("failed to create shell")]
    ShellNewFailed { source: xshell::Error },

    #[error("failed to create temp dir")]
    TempDirNewFailed { source: std::io::Error },

    #[error("failed to run insert command")]
    InsertRunFailed { source: xshell::Error },

    #[error("failed to run clear command")]
    ClearRunFailed { source: xshell::Error },

    #[error("failed to run iter command after clear")]
    IterOutputFailed { source: xshell::Error },

    #[error("failed to decode iter output after clear as utf-8")]
    IterUtf8Failed { source: FromUtf8Error },
}

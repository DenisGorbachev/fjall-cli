use tempdir::TempDir;
use xshell::{Shell, cmd};

/// Same bug report from another user: https://github.com/fjall-rs/fjall/issues/241
#[ignore]
#[test]
fn clear_repro() {
    let bin = env!("CARGO_BIN_EXE_fjall");
    let temp_dir = TempDir::new("fjall_cli").unwrap();
    let db_path = temp_dir.path();
    let sh = Shell::new().unwrap();
    let sh = sh.with_var("FJALL_DB", db_path);

    cmd!(sh, "{bin} insert items key value").run().unwrap();
    cmd!(sh, "{bin} clear items").run().unwrap();

    let output = cmd!(sh, "{bin} list items").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "");
}

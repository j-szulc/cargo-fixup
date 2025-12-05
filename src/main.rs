use anyhow::{Context, Result, ensure};
use std::{path::Path, process::Command};

fn is_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

fn main() -> Result<()> {
    static COMMANDS: &[&[&str]] = &[
        &[
            "git",
            "stash",
            "push",
            "--message",
            "Snapshot before cargo fixup",
            "--keep-index",
        ],
        &["cargo", "fix", "--allow-dirty"],
        &[
            "cargo",
            "clippy",
            "--fix",
            "--allow-dirty",
            "--",
            "-D",
            "clippy::pedantic",
        ],
    ];

    ensure!(is_git_repo(Path::new(".")), "Not a git repository!");

    for command in COMMANDS {
        Command::new(command[0])
            .args(&command[1..])
            .spawn()
            .with_context(|| format!("Failed to spawn command: {command:?}"))?
            .wait()
            .with_context(|| format!("Failed to wait for command: {command:?}"))?;
    }

    Ok(())
}

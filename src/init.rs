use anyhow::{bail, Context, Result};
use std::{
    env::set_current_dir,
    fs::{create_dir, OpenOptions},
    io::{self, ErrorKind, Write},
    path::Path,
};

use crate::{embedded::EMBEDDED_FILES, info_file::ExerciseInfo};

fn create_cargo_toml(exercise_infos: &[ExerciseInfo]) -> io::Result<()> {
    let mut cargo_toml = Vec::with_capacity(1 << 13);
    cargo_toml.extend_from_slice(b"bin = [\n");
    for exercise_info in exercise_infos {
        cargo_toml.extend_from_slice(b"  { name = \"");
        cargo_toml.extend_from_slice(exercise_info.name.as_bytes());
        cargo_toml.extend_from_slice(b"\", path = \"exercises/");
        if let Some(dir) = &exercise_info.dir {
            cargo_toml.extend_from_slice(dir.as_bytes());
            cargo_toml.extend_from_slice(b"/");
        }
        cargo_toml.extend_from_slice(exercise_info.name.as_bytes());
        cargo_toml.extend_from_slice(b".rs\" },\n");
    }

    cargo_toml.extend_from_slice(
        br#"]

[package]
name = "rustlings"
edition = "2021"
publish = false
"#,
    );
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("Cargo.toml")?
        .write_all(&cargo_toml)
}

fn create_gitignore() -> io::Result<()> {
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(".gitignore")?
        .write_all(GITIGNORE)
}

fn create_vscode_dir() -> Result<()> {
    create_dir(".vscode").context("Failed to create the directory `.vscode`")?;
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(".vscode/extensions.json")?
        .write_all(VS_CODE_EXTENSIONS_JSON)?;

    Ok(())
}

pub fn init(exercise_infos: &[ExerciseInfo]) -> Result<()> {
    if Path::new("exercises").is_dir() && Path::new("Cargo.toml").is_file() {
        bail!(PROBABLY_IN_RUSTLINGS_DIR_ERR);
    }

    let rustlings_path = Path::new("rustlings");
    if let Err(e) = create_dir(rustlings_path) {
        if e.kind() == ErrorKind::AlreadyExists {
            bail!(RUSTLINGS_DIR_ALREADY_EXISTS_ERR);
        }
        return Err(e.into());
    }

    set_current_dir("rustlings")
        .context("Failed to change the current directory to `rustlings`")?;

    EMBEDDED_FILES
        .init_exercises_dir()
        .context("Failed to initialize the `rustlings/exercises` directory")?;

    create_cargo_toml(exercise_infos)
        .context("Failed to create the file `rustlings/Cargo.toml`")?;

    create_gitignore().context("Failed to create the file `rustlings/.gitignore`")?;

    create_vscode_dir().context("Failed to create the file `rustlings/.vscode/extensions.json`")?;

    Ok(())
}

const GITIGNORE: &[u8] = b"/target
/.rustlings-state.json
";

const VS_CODE_EXTENSIONS_JSON: &[u8] = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;

const PROBABLY_IN_RUSTLINGS_DIR_ERR: &str =
    "A directory with the name `exercises` and a file with the name `Cargo.toml` already exist
in the current directory. It looks like Rustlings was already initialized here.
Run `rustlings` for instructions on getting started with the exercises.

If you didn't already initialize Rustlings, please initialize it in another directory.";

const RUSTLINGS_DIR_ALREADY_EXISTS_ERR: &str =
    "A directory with the name `rustlings` already exists in the current directory.
You probably already initialized Rustlings.
Run `cd rustlings`
Then run `rustlings` again";

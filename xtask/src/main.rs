use anyhow::{Context, Error};
use once_cell::sync::Lazy;
use std::{
    fs::File,
    io::{Seek, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use structopt::StructOpt;
use zip::{write::FileOptions, ZipWriter};

fn main() -> Result<(), Error> {
    let cmd = Cmd::from_args();

    match cmd {
        Cmd::Dist { project_root } => dist(project_root)?,
    }

    Ok(())
}

#[cfg(windows)]
const CDIR_CLI_BINARY: &str = "cdir-cli.exe";
#[cfg(not(windows))]
const CDIR_CLI_BINARY: &str = "cdir-cli";

#[derive(StructOpt)]
pub enum Cmd {
    /// Create a release archive.
    Dist {
        /// The `cdir` project's root directory.
        #[structopt(short, long, parse(from_os_str), default_value = &*DEFAULT_PROJECT_ROOT)]
        project_root: PathBuf,
    },
}

fn dist(project_root: PathBuf) -> Result<(), Error> {
    compile_cdir_cli(&project_root)?;
    generate_release_bundle(&project_root)?;

    Ok(())
}

fn generate_release_bundle(project_root: &Path) -> Result<(), Error> {
    let target = project_root.join("target");
    let filename = filename(&target);

    let f = File::create(&filename)
        .with_context(|| format!("Unable to open \"{}\" for writing", filename.display()))?;

    let mut writer = ZipWriter::new(f);

    let cdir_cli = target.join("release").join(CDIR_CLI_BINARY);
    add_file(&mut writer, &cdir_cli)?;
    add_file(&mut writer, project_root.join("README.md"))?;
    add_file(&mut writer, project_root.join("LICENSE_MIT.md"))?;
    add_file(&mut writer, project_root.join("LICENSE_APACHE.md"))?;

    writer
        .finish()
        .context("Unable to finalize the zipfile")?
        .flush()
        .context("Unable to flush to disk")?;

    Ok(())
}

fn filename(target_dir: &Path) -> PathBuf {
    target_dir.join(format!("cdir.{}.zip", env!("TARGET")))
}

fn add_file<W>(w: &mut ZipWriter<W>, filename: impl AsRef<Path>) -> Result<(), Error>
where
    W: Write + Seek,
{
    let filename = filename.as_ref();
    let name = filename
        .file_name()
        .and_then(|n| n.to_str())
        .context("The file has no name")?;

    let mut f = File::open(filename)
        .with_context(|| format!("Unable to open \"{}\" for reading", filename.display()))?;

    w.start_file(name, FileOptions::default())?;
    std::io::copy(&mut f, w)?;
    w.flush()?;

    Ok(())
}

fn compile_cdir_cli(project_root: &Path) -> Result<(), Error> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));

    let status = Command::new(cargo)
        .arg("build")
        .arg("--release")
        .arg("--package=cdir-cli")
        .current_dir(project_root)
        .status()
        .context("Unable to start `cargo`")?;

    anyhow::ensure!(status.success(), "Compiling `cdir-cli` failed");

    Ok(())
}

static DEFAULT_PROJECT_ROOT: Lazy<String> =
    Lazy::new(|| git_repo_root().unwrap_or_else(|_| String::from(".")));

fn git_repo_root() -> Result<String, Error> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .context("Unable to execute `git`")?;

    let output = std::str::from_utf8(&output.stdout).context("git returned invalid UTF-8")?;

    Ok(output.trim().to_string())
}

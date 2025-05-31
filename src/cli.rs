use clap::Parser;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long = "cv", value_name = "CV")]
    cv_path: PathBuf,

    #[arg(long = "template", value_name = "TEMPLATE")]
    template_path: PathBuf,
}

#[derive(Error, Debug)]
enum CliError {
    #[error("Path does not exist")]
    PathDoesNotExist(PathBuf),

    #[error("The path {0} does not point to a file")]
    NotAFile(PathBuf),

    #[error("The path {0} does not point to a directory")]
    NotADir(PathBuf),
}

fn verify_cv_path(path: &Path) -> Result<(), CliError> {
    if !path.exists() {
        return Err(CliError::PathDoesNotExist(path.to_path_buf()));
    }
    if !path.is_file() {
        return Err(CliError::NotAFile(path.to_path_buf()));
    }

    Ok(())
}

fn verify_template_path(path: &Path) -> Result<(), CliError> {
    if !path.exists() {
        return Err(CliError::PathDoesNotExist(path.to_path_buf()));
    }
    if !path.is_dir() {
        return Err(CliError::NotADir(path.to_path_buf()));
    }

    Ok(())
}

pub fn parse_cli() -> anyhow::Result<Cli> {
    let cli = Cli::parse();
    verify_cv_path(&cli.cv_path)?;
    verify_template_path(&cli.template_path)?;
    Ok(cli)
}

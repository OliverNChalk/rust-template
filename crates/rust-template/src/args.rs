use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
#[command(version = toolbox::version!(), long_version = toolbox::long_version!())]
pub(crate) struct Args {
    /// Path to config file.
    #[clap(long, value_hint = ValueHint::FilePath)]
    pub(crate) config: PathBuf,
    /// Generate completions for provided shell.
    #[arg(long, value_name = "SHELL")]
    pub(crate) completions: Option<clap_complete::Shell>,

    /// If provided, will write hourly log files to this directory.
    #[arg(long, value_hint = ValueHint::DirPath)]
    pub(crate) logs: Option<PathBuf>,
}

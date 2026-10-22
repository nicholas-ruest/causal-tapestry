//! Command-line operator interface.
use clap::{Parser, ValueEnum};
use ct_config::Config;
use ct_errors::{Error, Result};
use ct_orchestrator::{investigate, Mode};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliMode {
    Combined,
    NoRuvector,
    NoRegorus,
    NoEprocess,
    NoRvf,
}

impl From<CliMode> for Mode {
    fn from(value: CliMode) -> Self {
        match value {
            CliMode::Combined => Self::Combined,
            CliMode::NoRuvector => Self::NoRuVector,
            CliMode::NoRegorus => Self::NoRegorus,
            CliMode::NoEprocess => Self::NoEProcess,
            CliMode::NoRvf => Self::NoRvf,
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "causal-tapestry",
    about = "Reconstruct distributed incidents into governed causal evidence"
)]
struct Args {
    #[arg(long)]
    incident_id: String,
    #[arg(long)]
    input: PathBuf,
    #[arg(long, default_value = "evidence.rvf")]
    output: PathBuf,
    #[arg(long, value_enum, default_value = "combined")]
    mode: CliMode,
}

fn run() -> Result<()> {
    let args = Args::parse();
    let input = std::fs::read_to_string(&args.input)
        .map_err(|error| Error::Io(format!("{}: {error}", args.input.display())))?;
    let report = investigate(
        &args.incident_id,
        &input,
        &args.output,
        &Config::default(),
        args.mode.into(),
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|error| Error::Io(error.to_string()))?
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("causal-tapestry: {error}");
        std::process::exit(1);
    }
}

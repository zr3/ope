use anyhow::{Context, Result};
use clap::Parser;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::OpenOptions;

// log quick snappy notes to keep track of side thoughts
#[derive(Parser)]
struct Cli {
    note: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut log_filepath = home::home_dir().unwrap();
    log_filepath.push(".ope");
    let log_config = ConfigBuilder::new().set_time_format_rfc3339().build();
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_filepath)
        .with_context(|| format!("could not open file `{}`", "test.log"))?;
    let _ = WriteLogger::init(LevelFilter::Info, log_config, log_file);
    log::info!("[NOTE] {}", args.note);
    Ok(())
}

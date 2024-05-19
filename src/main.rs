use std::path::Path;

use clap::Parser;

mod cli;
use sys_monitor::prelude::*;
use sys_monitor::{utils, LoggerBuilder};

fn main() -> Result<()> {
    let args = cli::Args::parse();

    match &args.command {
        cli::Commands::Log { log_file } => log_perf(&args, log_file)?,
        cli::Commands::Stat { log_file: _ } => todo!(),
    }

    Ok(())
}

fn log_perf(args: &cli::Args, log_file: &Path) -> Result<()> {
    utils::backup_file(log_file)?;

    LoggerBuilder::new()
        .with_duration(args.duration)
        .with_interval(args.interval)
        .with_cpu_threshold(args.cpu_threshold)
        .with_mem_threshold(args.mem_threshold)
        .with_log_file(log_file)
        .run()?;
    Ok(())
}

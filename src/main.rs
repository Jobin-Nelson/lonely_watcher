use clap::Parser;

mod cli;
use sys_monitor::prelude::*;
use sys_monitor::{utils, LoggerBuilder};

fn main() -> Result<()> {
    let args = cli::Args::parse();

    match args.command {
        cli::Commands::Log {
            log_file,
            duration,
            interval,
            cpu_threshold,
            mem_threshold,
        } => {
            log_file.exists().then(|| utils::backup_file(&log_file));

            LoggerBuilder::new()
                .with_duration(duration)
                .with_interval(interval)?
                .with_cpu_threshold(cpu_threshold)?
                .with_mem_threshold(mem_threshold)?
                .with_log_file(&log_file)?
                .run()?;
        }
        cli::Commands::Stat { .. } => todo!(),
    }

    Ok(())
}


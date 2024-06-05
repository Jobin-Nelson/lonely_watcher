use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

/// Your friendly neighbourhood System Performance Logger
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Log performance data
    Log {
        #[arg(short = 'f', long, default_value = "sys_perf.log")]
        log_file: PathBuf,
        /// Duration in hours
        #[arg(short, long)]
        duration: Option<usize>,
        /// Interval in seconds
        #[arg(short, long, default_value_t = 5)]
        interval: usize,
        /// CPU threshold in percent
        #[arg(short, long, default_value_t = 90)]
        cpu_threshold: usize,
        /// Memory threshold in percent
        #[arg(short, long, default_value_t = 90)]
        mem_threshold: usize,
    },
    /// Stat performance data
    Stat {
        #[arg(short = 'f', long, default_value = "sys_perf.log")]
        log_file: PathBuf,
    },
}

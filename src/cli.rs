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
        #[arg(short, long, default_value_t = 5, value_parser = clap::value_parser!(u64).range(1..))]
        interval: u64,
        /// CPU threshold in percent
        #[arg(short, long, default_value_t = 90, value_parser = clap::value_parser!(u8).range(0..=100))]
        cpu_threshold: u8,
        /// Memory threshold in percent
        #[arg(short, long, default_value_t = 90, value_parser = clap::value_parser!(u8).range(0..=100))]
        mem_threshold: u8,
    },
    /// Stat performance data
    Stat {
        #[arg(short = 'f', long, default_value = "sys_perf.log")]
        log_file: PathBuf,
    },
}

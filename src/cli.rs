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
        #[arg(short='f', long, default_value = "sys_perf.log")]
        log_file: PathBuf
    },
    /// Stat performance data
    Stat {
        #[arg(short='f', long, default_value = "sys_perf.log")]
        log_file: PathBuf
    },
}

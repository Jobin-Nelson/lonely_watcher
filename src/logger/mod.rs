use std::path::{Path, PathBuf};
use std::thread::sleep;

use crate::prelude::*;
use crate::utils;

use self::cpu_info::CpuInfoIterator;
use self::mem_info::MemInfoIterator;

pub mod cpu_info;
pub mod mem_info;

pub struct WithLogFile {
    log_file: PathBuf,
}
#[derive(Default)]
pub struct WithoutLogFile;

#[derive(Debug, Default)]
pub struct LoggerBuilder<State = WithoutLogFile> {
    duration: Option<usize>,
    interval: Option<usize>,
    cpu_threshold: Option<usize>,
    mem_threshold: Option<usize>,
    state: State,
}

impl LoggerBuilder {
    pub fn new() -> LoggerBuilder<WithoutLogFile> {
        Default::default()
    }
}

impl LoggerBuilder<WithLogFile> {
    pub fn run(self) -> Result<()> {
        let log_file = self.state.log_file;
        if log_file.exists() {
            utils::backup_file(&log_file)?;
        }
        let mut cpu_info_iter = CpuInfoIterator::new();
        let mut mem_info_iter = MemInfoIterator::new();
        loop {
            let cpu_info = cpu_info_iter.next().expect("Could not get cpu info");
            let mem_info = mem_info_iter.next().expect("Could not get mem info");
            println!("{cpu_info:?}; {mem_info:?}");
            sleep(std::time::Duration::from_secs(
                self.interval.unwrap_or(5) as u64
            ))
        }
    }
}

impl<State> LoggerBuilder<State> {
    pub fn with_duration(mut self, duration: Option<usize>) -> Self {
        self.duration = duration;
        self
    }
    pub fn with_interval(mut self, interval: usize) -> Self {
        let _ = self.interval.insert(interval);
        self
    }
    pub fn with_cpu_threshold(mut self, cpu_threshold: usize) -> Self {
        let _ = self.cpu_threshold.insert(cpu_threshold);
        self
    }
    pub fn with_mem_threshold(mut self, mem_threshold: usize) -> Self {
        let _ = self.mem_threshold.insert(mem_threshold);
        self
    }
    pub fn with_log_file(self, log_file: &Path) -> LoggerBuilder<WithLogFile> {
        LoggerBuilder {
            duration: self.duration,
            interval: self.interval,
            cpu_threshold: self.cpu_threshold,
            mem_threshold: self.mem_threshold,
            state: {
                WithLogFile {
                    log_file: log_file.to_path_buf(),
                }
            },
        }
    }
}

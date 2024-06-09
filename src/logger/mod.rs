use std::path::{Path, PathBuf};
use std::thread::sleep;

use crate::prelude::*;

use self::cpu_info::get_cpu_info;
use self::mem_info::get_mem_info;

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
    interval: Option<u64>,
    cpu_threshold: Option<u8>,
    mem_threshold: Option<u8>,
    state: State,
}

impl LoggerBuilder {
    pub fn new() -> LoggerBuilder<WithoutLogFile> {
        Default::default()
    }
}

impl LoggerBuilder<WithLogFile> {
    // TODO trap keyboard interrupt signal
    pub fn run(self) -> Result<()> {
        // TODO abstract loggers into a plugin system
        let mut cpu_info = get_cpu_info();
        let mut mem_info = get_mem_info();

        let interval = self.interval.unwrap_or(5);
        loop {
            let cpu_info = cpu_info.next().expect("Could not get cpu info");
            let mem_info = mem_info.next().expect("Could not get mem info");
            println!("{cpu_info:?}; {mem_info:?}");
            sleep(std::time::Duration::from_secs(interval));
        }
    }
}

impl<State> LoggerBuilder<State> {
    pub fn with_duration(mut self, duration: Option<usize>) -> Self {
        self.duration = duration;
        self
    }
    pub fn with_interval(mut self, interval: u64) -> Result<Self> {
        if interval == 0 {
            return Err(Error::LoggerValidationError(format!(
                "Expected interval > 0, got {interval}"
            )));
        }
        let _ = self.interval.insert(interval);
        Ok(self)
    }
    pub fn with_cpu_threshold(mut self, cpu_threshold: u8) -> Result<Self> {
        if cpu_threshold > 100 {
            return Err(Error::LoggerValidationError(format!(
                "Expected 0 <= cpu_threshold <= 100, got {cpu_threshold}"
            )));
        }
        let _ = self.cpu_threshold.insert(cpu_threshold);
        Ok(self)
    }
    pub fn with_mem_threshold(mut self, mem_threshold: u8) -> Result<Self> {
        if mem_threshold > 100 {
            return Err(Error::LoggerValidationError(format!(
                "Expected 0 <= mem_threshold <= 100, got {mem_threshold}"
            )));
        }
        let _ = self.mem_threshold.insert(mem_threshold);
        Ok(self)
    }
    pub fn with_log_file(self, log_file: &Path) -> Result<LoggerBuilder<WithLogFile>> {
        if log_file
            .parent()
            .is_some_and(|p| !p.as_os_str().is_empty() && !p.is_dir())
        {
            return Err(Error::LoggerValidationError(format!(
                "Expected log_file path to exist, missing path to {}",
                log_file.display()
            )));
        }
        Ok(LoggerBuilder {
            duration: self.duration,
            interval: self.interval,
            cpu_threshold: self.cpu_threshold,
            mem_threshold: self.mem_threshold,
            state: {
                WithLogFile {
                    log_file: log_file.to_path_buf(),
                }
            },
        })
    }
}

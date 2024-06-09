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
        let interval = match self.interval.unwrap_or_default() {
            0 => return Err(Error::ZeroIntervalError),
            i => i,
        };
        // TODO abstract loggers into a plugin system
        let mut cpu_info = get_cpu_info();
        let mut mem_info = get_mem_info();
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
    pub fn with_interval(mut self, interval: u64) -> Self {
        let _ = self.interval.insert(interval);
        self
    }
    pub fn with_cpu_threshold(mut self, cpu_threshold: u8) -> Self {
        let _ = self.cpu_threshold.insert(cpu_threshold);
        self
    }
    pub fn with_mem_threshold(mut self, mem_threshold: u8) -> Self {
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

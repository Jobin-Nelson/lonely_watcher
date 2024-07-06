use std::path::{Path, PathBuf};
use std::thread::sleep;

use tracing::{info, warn};

use crate::prelude::*;

use self::cpu_info::get_cpu_info;
use self::mem_info::get_mem_info;

pub mod cpu_info;
pub mod mem_info;

struct Logger {
    duration: usize,
    interval: usize,
    cpu_threshold: usize,
    mem_threshold: usize,
    log_file: PathBuf,
}

impl Logger {
    pub fn new(duration: Option<usize>, interval: Option<usize>, cpu_threshold: Option<usize>, mem_threshold: Option<usize>, log_file: PathBuf) -> Self {
        Self {
            duration: duration.unwrap_or_default(),
            interval: interval.unwrap_or(5),
            cpu_threshold: cpu_threshold.unwrap_or(90),
            mem_threshold: mem_threshold.unwrap_or(90),
            log_file,
        }
    }

    fn run(self) -> Result<()> {
        // TODO trap keyboard interrupt signal
        // TODO abstract loggers into a plugin system
        let mut cpu_info_iter = get_cpu_info();
        let mut mem_info_iter = get_mem_info();

        tracing_subscriber::fmt().json().init();

        let interval = self.interval as u64;

        let mut prev_cpu_info = cpu_info_iter.next().expect("Could not get cpu info");

        loop {
            sleep(std::time::Duration::from_secs(interval));
            let cpu_percent = cpu_info_iter
                .next()
                .expect("Could not get cpu info")
                .get_cpu_usage(&mut prev_cpu_info);
            let mem_percent = mem_info_iter
                .next()
                .expect("Could not get mem info")main
                .get_mem_usage();

            if cpu_percent >= self.cpu_threshold || mem_percent >= self.mem_threshold {
                warn!(cpu = cpu_percent, mem = mem_percent);
            } else {
                info!(cpu = cpu_percent, mem = mem_percent);
            }
        }
    }
}

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
        let logger = Logger::new(
            self.duration,
            self.interval,
            self.cpu_threshold,
            self.mem_threshold,
            self.state.log_file,
        );

        logger.run()?;

        Ok(())
    }
}

impl<State> LoggerBuilder<State> {
    pub fn with_duration(mut self, duration: Option<usize>) -> Self {
        self.duration = duration;
        self
    }
    pub fn with_interval(mut self, interval: usize) -> Result<Self> {
        if interval == 0 {
            return Err(Error::LoggerValidationError(format!(
                "Expected interval > 0, got {interval}"
            )));
        }
        let _ = self.interval.insert(interval);
        Ok(self)
    }
    pub fn with_cpu_threshold(mut self, cpu_threshold: usize) -> Result<Self> {
        if cpu_threshold > 100 {
            return Err(Error::LoggerValidationError(format!(
                "Expected 0 <= cpu_threshold <= 100, got {cpu_threshold}"
            )));
        }
        let _ = self.cpu_threshold.insert(cpu_threshold);
        Ok(self)
    }
    pub fn with_mem_threshold(mut self, mem_threshold: usize) -> Result<Self> {
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

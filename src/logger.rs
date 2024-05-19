#![allow(unused)]

use std::path::{Path, PathBuf};

use crate::prelude::*;
use crate::utils;
use crate::cpu_info::CpuInfoIterator;

pub struct LoggerWithLogFile {
    log_file: PathBuf,
}
pub struct LoggerWithoutLogFile;

#[derive(Debug)]
pub struct LoggerBuilder<State = LoggerWithoutLogFile> {
    duration: Option<usize>,
    interval: Option<usize>,
    cpu_threshold: Option<usize>,
    mem_threshold: Option<usize>,
    state: State,
}

impl Default for LoggerBuilder<LoggerWithoutLogFile> {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            interval: Default::default(),
            cpu_threshold: Default::default(),
            mem_threshold: Default::default(),
            state: LoggerWithoutLogFile,
        }
    }
}

impl LoggerBuilder {
    pub fn new() -> LoggerBuilder<LoggerWithoutLogFile> {
        Default::default()
    }
}

impl LoggerBuilder<LoggerWithLogFile> {
    pub fn run(self) -> Result<()> {
        let log_file = self.state.log_file;
        if log_file.exists() {
            utils::backup_file(&log_file)?;
        }
        let mut cpu_info_iter = CpuInfoIterator::new();
        loop {
            let cpu_info = cpu_info_iter
                .next()
                .expect("Could not get cpu info");
        }
        Ok(())
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
    pub fn with_log_file(self, log_file: &Path) -> LoggerBuilder<LoggerWithLogFile> {
        LoggerBuilder {
            duration: self.duration,
            interval: self.interval,
            cpu_threshold: self.cpu_threshold,
            mem_threshold: self.mem_threshold,
            state: {
                LoggerWithLogFile {
                    log_file: log_file.to_path_buf(),
                }
            },
        }
    }
}

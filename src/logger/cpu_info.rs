use std::{
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::Error;

const STAT_FILE_PATH: &str = "/proc/stat";

#[derive(Debug, PartialEq)]
pub struct CpuInfo {
    pub idle_time: usize,
    pub non_idle_time: usize,
}

impl FromStr for CpuInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.trim_start().starts_with("cpu ") {
            return Err(Error::ParseCpuError);
        }
        let (idle_time, non_idle_time) =
            s.split_whitespace()
                .skip(1)
                .enumerate()
                .fold((0, 0), |mut acc, w| {
                    match w.0 {
                        3 | 4 => acc.0 += w.1.parse::<usize>().unwrap(),
                        _ => acc.1 += w.1.parse::<usize>().unwrap(),
                    };
                    acc
                });

        Ok(Self {
            idle_time,
            non_idle_time,
        })
    }
}

#[derive(Default)]
pub struct CpuInfoIterator;

impl CpuInfoIterator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Iterator for CpuInfoIterator {
    type Item = CpuInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let stat_file = std::fs::File::open(STAT_FILE_PATH).expect("Could not open /proc/stat");
        let reader = BufReader::new(stat_file);

        let aggregate_cpu_line = reader.lines().find_map(|l| {
            let line = l.unwrap();
            line.starts_with("cpu ").then_some(line)
        });

        let cpu_info: CpuInfo = aggregate_cpu_line
            .map(|v| v.parse().unwrap())
            .expect("Could not parse cpu info");

        Some(cpu_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cpu_info_parse() {
        let input = "cpu  770783 692 470657 25913724 58979 93595 25985 0 0 0".to_string();
        let expected = CpuInfo {
            idle_time: 25972703,
            non_idle_time: 1361712,
        };

        assert_eq!(input.parse::<CpuInfo>().unwrap(), expected);
    }

    #[test]
    fn test_cpu_info_parse_empty_line() {
        let input = "".to_string();

        assert!(input
            .parse::<CpuInfo>()
            .is_err_and(|e| { matches!(e, Error::ParseCpuError) }));
    }
}

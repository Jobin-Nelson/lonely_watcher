use std::{
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::Error;

const MEMINFO_FILE_PATH: &str = "/proc/meminfo";

#[derive(Debug, PartialEq)]
pub(crate) struct MemInfo {
    pub total_mem: usize,
    pub free_mem: usize,
    pub available_mem: usize,
}

impl MemInfo {
    pub fn get_mem_usage(&self) -> usize {
        ((self.total_mem - self.available_mem) * 100) / self.total_mem
    }
}

impl FromStr for MemInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let get_mem_value = |line: Option<&str>, prefix: &str| {
            let line = line.ok_or_else(|| {
                Error::ParseMemError(format!("Expected {prefix}, got empty line"))
            })?;
            if !line.starts_with(prefix) {
                return Err(Error::ParseMemError(format!(
                    "Expected {prefix}, got {line}"
                )));
            };
            line.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .map_err(|e| {
                    Error::ParseMemError(format!(
                        "Expected integer value for {prefix}, got {e}"
                    ))
                })
        };

        let mut lines_iter = s.lines().take(3);

        let total_mem = get_mem_value(lines_iter.next(), "MemTotal:")?;
        let free_mem = get_mem_value(lines_iter.next(), "MemFree:")?;
        let available_mem = get_mem_value(lines_iter.next(), "MemAvailable:")?;

        Ok(Self {
            total_mem,
            free_mem,
            available_mem,
        })
    }
}

pub(crate) struct MemInfoIterator;

impl Iterator for MemInfoIterator {
    type Item = MemInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let stat_file =
            std::fs::File::open(MEMINFO_FILE_PATH).expect("Could not open /proc/meminfo");
        let reader = BufReader::new(stat_file);

        let mem_info = reader
            .lines()
            .take(3)
            .flatten()
            .map(|line| line + "\n")
            .collect::<String>()
            .parse::<MemInfo>()
            .unwrap();

        Some(mem_info)
    }
}

pub(crate) fn get_mem_info() -> MemInfoIterator {
    MemInfoIterator
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mem_parse() {
        let input = r#"MemTotal:       16244496 kB
MemFree:         5714272 kB
MemAvailable:   10846148 kB
        "#;
        let expected = MemInfo {
            total_mem: 16244496,
            free_mem: 5714272,
            available_mem: 10846148,
        };
        let actual: MemInfo = input.parse().unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_mem_parse_additional_lines() {
        let input = r#"MemTotal:       16244496 kB
MemFree:         5714272 kB
MemAvailable:   10846148 kB
Buffers:          330816 kB
Cached:          4974880 kB
SwapCached:            0 kB
Active:          5647380 kB
Inactive:        3942952 kB
Active(anon):    4390464 kB
Inactive(anon):        0 kB
Active(file):    1256916 kB
        "#;
        let expected = MemInfo {
            total_mem: 16244496,
            free_mem: 5714272,
            available_mem: 10846148,
        };
        let actual: MemInfo = input.parse().unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_mem_parse_not_enough_lines() {
        let input = r#"MemTotal:       16244496 kB
MemFree:         5714272 kB
        "#;
        let actual = input.parse::<MemInfo>();
        assert!(actual.is_err_and(|e| matches!(e, Error::ParseMemError(_))));
    }
}

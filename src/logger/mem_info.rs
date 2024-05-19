use std::{
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::Error;

const MEMINFO_FILE_PATH: &str = "/proc/meminfo";

#[derive(Debug, PartialEq)]
pub struct MemInfo {
    pub total_mem: usize,
    pub free_mem: usize,
    pub available_mem: usize,
}

impl FromStr for MemInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let get_mem_value = |s: &str| {
            s.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .map_err(|e| Error::ParseMemError(format!("Expected integer, failed due to {e}")))
        };
        let mut total_mem = 0;
        let mut free_mem = 0;
        let mut available_mem = 0;

        let mut found = [0; 3];

        for line in s.lines() {
            match line.split_whitespace().next() {
                Some("MemTotal:") => {
                    found[0] = 1;
                    total_mem = get_mem_value(line)?
                }
                Some("MemFree:") => {
                    found[1] = 1;
                    free_mem = get_mem_value(line)?
                }
                Some("MemAvailable:") => {
                    found[2] = 1;
                    available_mem = get_mem_value(line)?
                }
                _ => continue,
            }
        }
        let number_of_meminfo_values_found: u32 = found.iter().sum();

        if number_of_meminfo_values_found != 3 {
            return Err(Error::ParseMemError(format!(
                "Expected 3 mem info values, {number_of_meminfo_values_found}"
            )));
        }

        Ok(Self {
            total_mem,
            free_mem,
            available_mem,
        })
    }
}

#[derive(Default)]
pub struct MemInfoIterator;

impl MemInfoIterator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Iterator for MemInfoIterator {
    type Item = MemInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let stat_file = std::fs::File::open(MEMINFO_FILE_PATH).unwrap();
        let reader = BufReader::new(stat_file);
        let meminfo_string: String = reader.lines().map(|l| l.unwrap()).take(10).collect();
        let meminfo: MemInfo = meminfo_string.parse().unwrap();
        Some(meminfo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mem_parse() {
        let input = r#"
MemTotal:       16244496 kB
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
        let input = r#"
MemTotal:       16244496 kB
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
        let input = r#"
MemTotal:       16244496 kB
MemFree:         5714272 kB
        "#;
        let actual = input.parse::<MemInfo>();
        assert!(actual.is_err_and(|e| matches!(e, Error::ParseMemError(_))));
    }
}

use std::{cmp::Reverse, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let reports = lines
            .iter()
            .map(|line| line.parse())
            .collect::<Result<Vec<Report>, _>>()?;
        let part1 = reports.iter().filter(|r| r.is_safe()).count().to_string();
        let part2 = reports
            .iter()
            .filter(|r| r.is_safe_tolerant())
            .count()
            .to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[derive(Debug)]
struct Report {
    entries: Vec<usize>,
}

impl FromStr for Report {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .split(" ")
            .map(|num| {
                num.parse()
                    .map_err(|e| format!("failed to parse entry '{}': {}", num, e))
            })
            .collect::<Result<_, _>>()?;

        Ok(Report { entries })
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        if self.entries.is_sorted() || self.entries.is_sorted_by_key(|i| Reverse(*i)) {
            self.entries
                .iter()
                .zip(self.entries.iter().skip(1))
                .all(|(a, b)| {
                    let diff = if a > b { a - b } else { b - a };
                    diff > 0 && diff < 4
                })
        } else {
            false
        }
    }

    fn is_safe_tolerant(&self) -> bool {
        self.is_safe()
            || (0..self.entries.len()).any(|i| {
                let mut entries = self.entries.clone();
                entries.remove(i);
                (Report { entries }).is_safe()
            })
    }
}

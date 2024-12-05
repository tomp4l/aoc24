use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let lists: Lists = input.parse()?;
        let part1 = lists.total_distance().to_string();

        let part2 = Some(lists.similarity().to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug)]
struct Lists {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl FromStr for Lists {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in s.lines() {
            let mut parts = line.split("   ");
            left.push(
                parts
                    .next()
                    .unwrap()
                    .parse()
                    .map_err(|e| format!("failed to parse left in '{}': {}", line, e))?,
            );

            right.push(
                parts
                    .next()
                    .unwrap()
                    .parse()
                    .map_err(|e| format!("failed to parse right in '{}': {}", line, e))?,
            );
        }

        Ok(Lists { left, right })
    }
}

impl Lists {
    fn total_distance(&self) -> usize {
        let mut left = self.left.clone();
        let mut right = self.right.clone();

        left.sort();
        right.sort();

        left.into_iter()
            .zip(right.into_iter())
            .map(|(l, r)| if l < r { r - l } else { l - r })
            .sum()
    }

    fn similarity(&self) -> usize {
        self.left
            .iter()
            .map(|l| self.right.iter().filter(|r| l == *r).count() * l)
            .sum()
    }
}

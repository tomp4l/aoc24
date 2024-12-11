use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let mut stones: Stones = input.parse()?;
        for _ in 0..25 {
            stones.transform();
        }
        let part1 = stones.len().to_string();
        for _ in 0..50 {
            stones.transform();
        }
        let part2 = Some(stones.len().to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Stone(usize);

impl Stone {
    fn transform(&self) -> Vec<Stone> {
        let num = self.0;
        if num == 0 {
            vec![Stone(1)]
        } else if num.to_string().len() % 2 == 0 {
            let s = num.to_string();
            let half = s.len() / 2;
            let left = s[..half].parse().unwrap();
            let right = s[half..].parse().unwrap();

            vec![Stone(left), Stone(right)]
        } else {
            vec![Stone(num * 2024)]
        }
    }
}

#[derive(Debug)]
struct Stones {
    stones: HashMap<Stone, usize>,
}

impl FromStr for Stones {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones: Vec<Stone> = s
            .split_whitespace()
            .map(|s| s.parse().map(Stone))
            .collect::<Result<_, _>>()
            .map_err(|e| format!("failed to parse stones '{}': {}", s, e))?;

        let stones = stones
            .into_iter()
            .group_by(|s| *s)
            .into_iter()
            .map(|(s, group)| (s, group.count()))
            .collect::<HashMap<_, _>>();

        Ok(Stones { stones })
    }
}

impl Stones {
    fn transform(&mut self) {
        let mut new_stones = HashMap::new();
        for (stone, count) in self.stones.drain() {
            for new_stone in stone.transform() {
                *new_stones.entry(new_stone).or_insert(0) += count;
            }
        }
        self.stones = new_stones;
    }

    fn len(&self) -> usize {
        self.stones.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "125 17".to_owned();
        let day = Instance;
        let result = day.run(input).unwrap();
        assert_eq!(result.part1, "55312");
        assert_eq!(result.part2, None);
    }
}

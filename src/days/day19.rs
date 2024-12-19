use std::{collections::HashMap, slice, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let towel_patterns: TowelPatterns = input.parse()?;

        let (part1, part2) = towel_patterns.count_valid_and_patterns();
        let part1 = part1.to_string();
        let part2 = Some(part2.to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Colour {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl FromStr for Colour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Colour::White),
            "u" => Ok(Colour::Blue),
            "b" => Ok(Colour::Black),
            "r" => Ok(Colour::Red),
            "g" => Ok(Colour::Green),
            _ => Err("Invalid colour".to_owned()),
        }
    }
}

#[derive(Debug)]
struct TowelPatterns {
    patterns: Vec<Vec<Colour>>,
    towels: Vec<Vec<Colour>>,
}

impl FromStr for TowelPatterns {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let patterns = lines
            .next()
            .ok_or("No patterns")?
            .to_owned()
            .split(", ")
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse())
                    .collect::<Result<Vec<Colour>, String>>()
            })
            .collect::<Result<Vec<Vec<Colour>>, String>>()?;
        lines.next();
        let towels = lines
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse())
                    .collect::<Result<Vec<Colour>, String>>()
            })
            .collect::<Result<Vec<Vec<Colour>>, String>>()?;
        Ok(Self { patterns, towels })
    }
}

fn valid_count(towel: &[Colour], patterns: &[Vec<Colour>]) -> usize {
    let mut remainders = vec![(towel, 1)];
    let mut total_count: usize = 0;

    while remainders.len() > 0 {
        let mut new_remainders = HashMap::new();
        let longest = remainders.iter().map(|(c,_)|c.len()).max().unwrap();
        new_remainders.extend(remainders.iter().filter(|(c,_)|c.len()<longest).map(|(p, c)|((p.as_ptr(), p.len()), *c)));
        for (remainder, count) in remainders.into_iter().filter(|(c,_)|c.len()==longest) {
            for pattern in patterns {
                if remainder.len() < pattern.len() {
                    continue;
                }
                if remainder == pattern {
                    total_count += count;
                    continue;
                }
                let mut remainder = remainder;
                let mut pattern = pattern.iter();
                let mut matched = true;
                while let (Some(r), Some(p)) = (remainder.first(), pattern.next()) {
                    if r == p {
                        remainder = &remainder[1..];
                    } else {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    new_remainders
                        .entry((remainder.as_ptr(), remainder.len()))
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                }
            }
        }
        remainders = new_remainders
            .into_iter()
            .map(|((p, l), c)| 
            // safe as using same lifetime and valid slices from above
            unsafe { (slice::from_raw_parts(p, l), c) })
            .collect();
    }

    total_count
}

impl TowelPatterns {
    fn count_valid_and_patterns(&self) -> (usize, usize) {
        let counts: Vec<_> = self
            .towels
            .iter()
            .map(|towel| valid_count(towel, &self.patterns))
            .collect();

        (
            counts.iter().filter(|c| **c > 0).count(),
            counts.iter().sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "6".to_owned(),
                part2: Some("16".to_owned())
            })
        );
    }
}

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let topology: Topology = input.parse()?;
        let (part1, part2) = topology.trailheads();
        let part1 = part1.to_string();
        let part2 = Some(part2.to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(i32, i32);
#[derive(Debug)]
struct Topology {
    heights: HashMap<Coord, u32>,
}

impl FromStr for Topology {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heights = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(move |(x, h)| {
                        h.to_digit(10)
                            .ok_or(format!("bad height digit: {}", h))
                            .map(|h| (Coord(x as i32, y as i32), h))
                    })
            })
            .collect::<Result<_, _>>()?;
        Ok(Topology { heights })
    }
}

impl Topology {
    fn trailheads(&self) -> (usize, usize) {
        let (a, b): (Vec<_>, Vec<_>) = self
            .heights
            .iter()
            .filter(|(_, h)| **h == 0)
            .map(|(coord, _)| self.trailheads_from(*coord))
            .unzip();
        (a.iter().sum(), b.iter().sum())
    }

    fn trailheads_from(&self, coord: Coord) -> (usize, usize) {
        let mut candidates = vec![coord];
        let mut ends = HashSet::new();
        let mut rating = 0;

        while let Some(coord) = candidates.pop() {
            let height = self.heights[&coord];
            if height == 9 {
                ends.insert(coord);
                rating += 1;
                continue;
            }
            let mut new_candidates = vec![];
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_coord = Coord(coord.0 + dx, coord.1 + dy);
                if let Some(new_height) = self.heights.get(&new_coord) {
                    if *new_height == height + 1 {
                        new_candidates.push(new_coord);
                    }
                }
            }
            candidates.extend(new_candidates);
        }

        (ends.len(), rating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "36".to_owned(),
                part2: Some("81".to_owned())
            })
        );
    }
}

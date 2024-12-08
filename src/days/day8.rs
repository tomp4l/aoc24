use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let antennas: Antennas = input.parse()?;
        let part1 = antennas.count_antinodes().to_string();
        let part2 = Some(antennas.count_antinodes_all().to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug)]
struct Antennas {
    antennas: Vec<Antenna>,
    max_x: isize,
    max_y: isize,
}

#[derive(Debug, Eq, PartialEq)]
struct Antenna {
    x: isize,
    y: isize,
    frequency: char,
}

impl FromStr for Antennas {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas = Vec::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                max_x = x as isize;
                if c == '.' {
                    continue;
                }
                antennas.push(Antenna {
                    x: x as isize,
                    y: y as isize,
                    frequency: c,
                });
            }
            max_y = y as isize;
        }

        Ok(Antennas {
            antennas,
            max_x,
            max_y,
        })
    }
}

impl Antennas {
    fn count_antinodes(&self) -> usize {
        let mut antinodes = HashSet::new();
        for groups in &self.grouped_antennas() {
            for a in groups {
                for b in groups {
                    if a == b {
                        continue;
                    }
                    let dx = a.x - b.x;
                    let dy = a.y - b.y;

                    let x = a.x + dx;
                    let y = a.y + dy;
                    if x < 0 || y < 0 || x > self.max_x || y > self.max_y {
                        continue;
                    }
                    antinodes.insert((x, y));
                }
            }
        }
        antinodes.len()
    }

    fn count_antinodes_all(&self) -> usize {
        let mut antinodes = HashSet::new();
        for groups in &self.grouped_antennas() {
            for a in groups {
                for b in groups {
                    if a == b {
                        continue;
                    }
                    let dx = a.x - b.x;
                    let dy = a.y - b.y;

                    let mut x = a.x;
                    let mut y = a.y;
                    while x >= 0 && y >= 0 && x <= self.max_x && y <= self.max_y {
                        antinodes.insert((x, y));
                        x += dx;
                        y += dy;
                    }
                }
            }
        }
        antinodes.len()
    }

    fn grouped_antennas(&self) -> Vec<Vec<&Antenna>> {
        let mut antennas = HashMap::new();
        for antenna in &self.antennas {
            antennas
                .entry(antenna.frequency)
                .and_modify(|v: &mut Vec<_>| v.push(antenna))
                .or_insert(vec![antenna]);
        }
        antennas.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
        .to_owned();
        let expected = DayResult {
            part1: "14".to_owned(),
            part2: Some("34".to_owned()),
        };
        let instance = Instance;
        assert_eq!(instance.run(input), Ok(expected));
    }
}

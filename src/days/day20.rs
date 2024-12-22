use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::day::*;

pub struct Instance {
    threshold: usize,
}

impl Default for Instance {
    fn default() -> Self {
        Self { threshold: 100 }
    }
}

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let racetrack: Racetrack = input.parse()?;

        let part1 = racetrack.cheats_at_least(self.threshold, 2).to_string();

        let part2 = racetrack.cheats_at_least(self.threshold, 20).to_string();
        let part2 = Some(part2);
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn shift(&self, direction: Direction) -> Coord {
        match direction {
            Direction::North => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Coord {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn distance(&self, other: &Coord) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct Racetrack {
    walls: HashSet<Coord>,
    start: Coord,
    end: Coord,
}

impl FromStr for Racetrack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls = HashSet::new();
        let mut start = None;
        let mut end = None;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        walls.insert(Coord {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    'S' => {
                        start = Some(Coord {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    'E' => {
                        end = Some(Coord {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    _ => {}
                }
            }
        }
        Ok(Self {
            walls,
            start: start.ok_or("No start found")?,
            end: end.ok_or("No end found")?,
        })
    }
}

fn calculate_distances(walls: &HashSet<Coord>, start: Coord) -> HashMap<Coord, usize> {
    let mut distances: HashMap<Coord, usize> = HashMap::new();

    let mut positions = vec![(start, 0)];

    while let Some((p, d)) = positions.pop() {
        if distances.contains_key(&p) || walls.contains(&p) {
            continue;
        }

        positions.push((p.shift(Direction::East), d + 1));
        positions.push((p.shift(Direction::West), d + 1));
        positions.push((p.shift(Direction::South), d + 1));
        positions.push((p.shift(Direction::North), d + 1));

        distances.insert(p, d);
    }

    distances
}

impl Racetrack {
    fn cheats_at_least(&self, picosecond_threshold: usize, max_length: u32) -> usize {
        let start_distances = calculate_distances(&self.walls, self.start);
        let total_distance = start_distances[&self.end];
        let end_distances = calculate_distances(&self.walls, self.end);
        let mut savings = 0;
        for start in start_distances.keys() {
            for end in start_distances.keys() {
                if start != end && start.distance(end) <= max_length {
                    let new_total = start_distances[&start]
                        + end_distances[&end]
                        + start.distance(end) as usize;
                    if total_distance > new_total {
                        let saving = total_distance - new_total;
                        if saving >= picosecond_threshold {
                            savings += 1;
                        }
                    }
                }
            }
        }
        savings
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
            .to_owned();
        assert_eq!(
            Instance { threshold: 50 }.run(input),
            Ok(DayResult {
                part1: "1".to_owned(),
                part2: Some("285".to_owned())
            })
        );
    }
}

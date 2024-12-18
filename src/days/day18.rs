use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use super::day::*;

pub struct Instance {
    grid_size: usize,
    falling_bytes: usize,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            grid_size: 70,
            falling_bytes: 1024,
        }
    }
}

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let mut memory = Memory::new(self.grid_size);
        let coords: Vec<Coord> = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;

        coords
            .iter()
            .take(self.falling_bytes)
            .for_each(|coord| memory.corrupt(coord.clone()));

        let part1 = memory.shortest_path().ok_or("no path found")?.to_string();

        let mut part2 = None;

        for coord in coords.iter().skip(self.falling_bytes) {
            memory.corrupt(coord.clone());
            if memory.shortest_path().is_none() {
                part2 = Some(format!("{},{}", coord.x, coord.y));
                break;
            }
        }
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coord {
    x: isize,
    y: isize,
}

impl FromStr for Coord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or("missing x")?
            .parse::<isize>()
            .map_err(|e| e.to_string())?;
        let y = parts
            .next()
            .ok_or("missing y")?
            .parse::<isize>()
            .map_err(|e| e.to_string())?;
        Ok(Coord { x, y })
    }
}

#[derive(Debug)]
struct Memory {
    corrupted: HashSet<Coord>,
    grid_size: usize,
}

impl Memory {
    fn new(grid_size: usize) -> Self {
        Self {
            corrupted: HashSet::new(),
            grid_size,
        }
    }

    fn corrupt(&mut self, coord: Coord) {
        self.corrupted.insert(coord);
    }

    fn shortest_path(&self) -> Option<usize> {
        let target = Coord {
            x: self.grid_size as isize,
            y: self.grid_size as isize,
        };

        let mut states = VecDeque::from_iter([(Coord { x: 0, y: 0 }, 0)]);
        let mut visited = HashSet::new();

        while let Some((coord, steps)) = states.pop_back() {
            if coord == target {
                return Some(steps);
            }
            for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_coord = Coord {
                    x: coord.x + x,
                    y: coord.y + y,
                };
                if new_coord.x < 0 || new_coord.y < 0 {
                    continue;
                }
                if new_coord.x > self.grid_size as isize || new_coord.y > self.grid_size as isize {
                    continue;
                }
                if self.corrupted.contains(&new_coord) {
                    continue;
                }
                if visited.contains(&new_coord) {
                    continue;
                }
                visited.insert(new_coord.clone());
                states.push_front((new_coord, steps + 1));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
        .to_owned();
        let day = Instance {
            grid_size: 6,
            falling_bytes: 12,
        };
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "22".to_owned(),
                part2: Some("6,1".to_owned())
            })
        );
    }
}

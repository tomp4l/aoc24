use std::{collections::HashSet, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let map: Map = input.parse()?;
        let part1 = map.guard_path().to_string();
        let part2 = Some(count_loops(&map).to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug, Clone)]
struct Map {
    guard: Coord,
    guard_direction: Direction,
    obstacles: HashSet<Coord>,
    max_x: i32,
    max_y: i32,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut obstacles = HashSet::new();
        let mut guard = None;
        let mut guard_direction = None;
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = Coord {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '#' => {
                        obstacles.insert(coord);
                    }
                    '^' => {
                        guard = Some(coord);
                        guard_direction = Some(Direction::Up);
                    }
                    _ => {}
                }
                max_x = x as i32;
            }
            max_y = y as i32;
        }

        Ok(Map {
            guard: guard.ok_or("Guard not found")?,
            guard_direction: guard_direction.ok_or("Guard direction not found")?,
            obstacles,
            max_x,
            max_y,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Map {
    fn guard_path(&self) -> usize {
        self.guard_path_looped().map_or(0, |s| s.len())
    }

    fn guard_path_looped(&self) -> Option<HashSet<Coord>> {
        let mut guard = self.guard.clone();
        let mut direction = self.guard_direction;
        let min_x = 0;
        let min_y = 0;
        let mut seen_spaces = HashSet::new();
        seen_spaces.insert(guard.clone());
        let mut seen_states = HashSet::new();
        seen_states.insert((guard.clone(), direction));
        loop {
            let next_guard = guard.move_one(direction);
            if next_guard.x < min_x
                || next_guard.x > self.max_x
                || next_guard.y < min_y
                || next_guard.y > self.max_y
            {
                return Some(seen_spaces);
            }
            let state = (next_guard.clone(), direction);
            if seen_states.contains(&state) {
                return None;
            }
            if !self.obstacles.contains(&next_guard) {
                guard = next_guard;
                seen_spaces.insert(guard.clone());
                seen_states.insert(state);
            } else {
                direction = direction.turn_right();
            }
        }
    }
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Coord {
    fn move_one(&self, direction: Direction) -> Coord {
        let mut coord = self.clone();
        match direction {
            Direction::Up => {
                coord.y -= 1;
            }
            Direction::Down => {
                coord.y += 1;
            }
            Direction::Left => {
                coord.x -= 1;
            }
            Direction::Right => {
                coord.x += 1;
            }
        }
        coord
    }
}

fn count_loops(map: &Map) -> usize {
    let mut loops = 0;
    let candidates = map.guard_path_looped().unwrap();
    let mut map = map.clone();

    for coord in candidates {
        if map.obstacles.contains(&coord) {
            continue;
        }
        map.obstacles.insert(coord.clone());
        if map.guard_path_looped().is_none() {
            loops += 1;
        }
        map.obstacles.remove(&coord);
    }
    loops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_owned();
        let day = Instance;
        let result = day.run(input).unwrap();
        assert_eq!(result.part1, "41");
        assert_eq!(result.part2, Some(6.to_string()));
    }
}

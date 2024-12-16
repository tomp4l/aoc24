use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let maze: Maze = input.parse()?;
        let (part1, part2) = maze.lowest_score();
        let part1 = part1.to_string();
        let part2 = Some(part2.to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn forward(&self, orientation: Orientation) -> Coord {
        match orientation {
            Orientation::North => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Orientation::East => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Orientation::South => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Orientation::West => Coord {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
struct Maze {
    start: Coord,
    end: Coord,
    walls: HashSet<Coord>,
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut walls = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = Coord { x, y };
                match c {
                    'S' => {
                        start = Some(coord);
                    }
                    'E' => {
                        end = Some(coord);
                    }
                    '#' => {
                        walls.insert(coord);
                    }
                    _ => {}
                }
            }
        }

        Ok(Maze {
            start: start.ok_or("no start")?,
            end: end.ok_or("no end")?,
            walls,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn clockwise(&self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }

    fn anticlockwise(&self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        }
    }
}

impl Maze {
    fn lowest_score(&self) -> (usize, usize) {
        let mut seen_states: HashMap<(Coord, Orientation), usize> = HashMap::new();

        let mut open_states = VecDeque::new();
        open_states.push_back((
            self.start.clone(),
            Orientation::East,
            0,
            vec![self.start.clone()],
        ));

        let mut min_score = std::usize::MAX;
        let mut on_best_path = HashSet::new();
        while let Some((coord, orientation, score, visited)) = open_states.pop_front() {
            if coord == self.end {
                if min_score > score {
                    min_score = score;
                    on_best_path = HashSet::from_iter(visited);
                } else if min_score == score {
                    on_best_path.extend(visited);
                }
                continue;
            }

            if score > min_score {
                continue;
            }

            if let Some(&prev_score) = seen_states.get(&(coord.clone(), orientation)) {
                if prev_score < score {
                    continue;
                }
            }

            seen_states.insert((coord.clone(), orientation), score);

            let forward = coord.forward(orientation);
            if !self.walls.contains(&forward) {
                let mut new_visited = visited.clone();
                new_visited.push(forward.clone());
                open_states.push_front((forward, orientation, score + 1, new_visited));
            }

            open_states.push_back((
                coord.clone(),
                orientation.clockwise(),
                score + 1000,
                visited.clone(),
            ));
            open_states.push_back((
                coord.clone(),
                orientation.anticlockwise(),
                score + 1000,
                visited.clone(),
            ));
        }

        (min_score, on_best_path.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "7036".to_owned(),
                part2: Some("45".to_owned())
            })
        );
    }
}

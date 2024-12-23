use std::{collections::HashMap, fmt::Debug, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let numeric_keys = parse_input(&input)?;

        let part1 = complexity(&numeric_keys, 2).to_string();
        let part2 = Some(complexity(&numeric_keys, 25).to_string());
        Ok(DayResult { part1, part2 })
    }
}

fn parse_input(input: &str) -> Result<Vec<(usize, Vec<NumericKey>)>, String> {
    input
        .lines()
        .map(|line| {
            line.trim_end_matches("A")
                .parse::<usize>()
                .map_err(|e| e.to_string())
                .and_then(|numeric| {
                    line.chars()
                        .map(|c| c.to_string().parse::<NumericKey>())
                        .collect::<Result<Vec<_>, _>>()
                        .map(|keys| (numeric, keys))
                })
        })
        .collect()
}

#[derive(Debug)]
struct CachedComplexity {
    cost_cache: HashMap<(usize, Vec<DirectionalKey>), usize>,
    best_cache: HashMap<(Coord, Coord), Vec<DirectionalKey>>,
    depth: usize,
}

impl CachedComplexity {
    fn new(depth: usize) -> Self {
        CachedComplexity {
            cost_cache: HashMap::new(),
            best_cache: HashMap::new(),
            depth,
        }
    }

    fn complexity(&mut self, keys: &[NumericKey]) -> usize {
        let mut current = NumericKey::start();
        let mut total = 0;

        for key in keys {
            let mut horizontal = current.directions(&key.position(), false);
            let mut vertical = current.directions(&key.position(), true);
            horizontal.push(DirectionalKey::Activate);
            vertical.push(DirectionalKey::Activate);

            if !NumericKey::is_valid(&current, &horizontal) || horizontal == vertical {
                total += self.cost(&vertical);
            } else if !NumericKey::is_valid(&current, &vertical) {
                total += self.cost(&horizontal);
            } else {
                let h_cost = self.cost(&horizontal);
                let v_cost = self.cost(&vertical);
                total += h_cost.min(v_cost)
            }
            current = key.position();
        }
        total
    }

    fn cost(&mut self, keys: &[DirectionalKey]) -> usize {
        self.cost_depth(keys, self.depth)
    }

    fn cost_depth(&mut self, keys: &[DirectionalKey], depth: usize) -> usize {
        let cache_key = (depth, keys.to_vec());
        if let Some(existing) = self.cost_cache.get(&cache_key) {
            *existing
        } else if depth == 0 {
            keys.len()
        } else {
            let mut cost = 0;
            let mut current = DirectionalKey::Activate.position();
            for key in keys {
                let best = self.best_keys(current, key.position(), depth);
                cost += self.cost_depth(&best, depth - 1);
                current = key.position();
            }
            self.cost_cache.insert(cache_key, cost);

            cost
        }
    }

    fn best_keys(&mut self, from: Coord, to: Coord, depth: usize) -> Vec<DirectionalKey> {
        let cache_key = (from, to);
        if let Some(existing) = self.best_cache.get(&cache_key) {
            existing.clone()
        } else {
            let mut horizontal = from.directions(&to, false);
            let mut vertical = from.directions(&to, true);
            horizontal.push(DirectionalKey::Activate);
            vertical.push(DirectionalKey::Activate);

            if !DirectionalKey::is_valid(&from, &horizontal) || horizontal == vertical {
                vertical
            } else if !DirectionalKey::is_valid(&from, &vertical) {
                horizontal
            } else {
                let h_cost = self.cost_depth(&horizontal, depth - 1);
                let v_cost = self.cost_depth(&vertical, depth - 1);

                if h_cost < v_cost {
                    horizontal
                } else {
                    vertical
                }
            }
        }
    }
}

fn complexity(keys: &[(usize, Vec<NumericKey>)], depth: usize) -> usize {
    let mut sum = 0;
    let mut cache = CachedComplexity::new(depth);
    for (v, k) in keys {
        let complexity = cache.complexity(k);

        sum += complexity * v;
    }

    sum
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

trait Position {
    fn start() -> Coord;
    fn position(&self) -> Coord;
    fn is_valid(start: &Coord, directions: &[DirectionalKey]) -> bool;
}

impl Position for DirectionalKey {
    fn position(&self) -> Coord {
        match self {
            DirectionalKey::Up => Coord { x: 1, y: 0 },
            DirectionalKey::Activate => Coord { x: 2, y: 0 },

            DirectionalKey::Left => Coord { x: 0, y: 1 },
            DirectionalKey::Down => Coord { x: 1, y: 1 },
            DirectionalKey::Right => Coord { x: 2, y: 1 },
        }
    }

    fn is_valid(start: &Coord, directions: &[DirectionalKey]) -> bool {
        let mut current = *start;
        for d in directions {
            current = current.move_direction(*d);
            if current == (Coord { x: 0, y: 0 }) {
                return false;
            }
        }
        true
    }

    fn start() -> Coord {
        Self::Activate.position()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NumericKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Position for NumericKey {
    fn position(&self) -> Coord {
        match self {
            NumericKey::Seven => Coord { x: 0, y: 0 },
            NumericKey::Eight => Coord { x: 1, y: 0 },
            NumericKey::Nine => Coord { x: 2, y: 0 },

            NumericKey::Four => Coord { x: 0, y: 1 },
            NumericKey::Five => Coord { x: 1, y: 1 },
            NumericKey::Six => Coord { x: 2, y: 1 },

            NumericKey::One => Coord { x: 0, y: 2 },
            NumericKey::Two => Coord { x: 1, y: 2 },
            NumericKey::Three => Coord { x: 2, y: 2 },

            NumericKey::Zero => Coord { x: 1, y: 3 },
            NumericKey::Activate => Coord { x: 2, y: 3 },
        }
    }

    fn is_valid(start: &Coord, directions: &[DirectionalKey]) -> bool {
        let mut current = *start;
        for d in directions {
            current = current.move_direction(*d);
            if current == (Coord { x: 0, y: 3 }) {
                return false;
            }
        }
        true
    }

    fn start() -> Coord {
        Self::Activate.position()
    }
}

impl Coord {
    fn directions(&self, other: &Coord, vertical_first: bool) -> Vec<DirectionalKey> {
        let mut horizontal = vec![];
        let mut vertical = vec![];

        if self.x < other.x {
            horizontal.extend(vec![DirectionalKey::Right; other.x - self.x]);
        } else if self.x > other.x {
            horizontal.extend(vec![DirectionalKey::Left; self.x - other.x]);
        }
        if self.y < other.y {
            vertical.extend(vec![DirectionalKey::Down; other.y - self.y]);
        } else if self.y > other.y {
            vertical.extend(vec![DirectionalKey::Up; self.y - other.y]);
        }

        if vertical_first {
            vertical.extend(horizontal);
            vertical
        } else {
            horizontal.extend(vertical);
            horizontal
        }
    }

    fn move_direction(&self, direction: DirectionalKey) -> Coord {
        match direction {
            DirectionalKey::Up => Coord {
                x: self.x,
                y: self.y - 1,
            },
            DirectionalKey::Down => Coord {
                x: self.x,
                y: self.y + 1,
            },
            DirectionalKey::Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            DirectionalKey::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
            DirectionalKey::Activate => *self,
        }
    }
}

impl FromStr for NumericKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(NumericKey::One),
            "2" => Ok(NumericKey::Two),
            "3" => Ok(NumericKey::Three),
            "4" => Ok(NumericKey::Four),
            "5" => Ok(NumericKey::Five),
            "6" => Ok(NumericKey::Six),
            "7" => Ok(NumericKey::Seven),
            "8" => Ok(NumericKey::Eight),
            "9" => Ok(NumericKey::Nine),
            "0" => Ok(NumericKey::Zero),
            "A" => Ok(NumericKey::Activate),
            _ => Err("Invalid key".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "029A
980A
179A
456A
379A"
            .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "126384".to_owned(),
                part2: Some("154115708116294".to_owned())
            })
        );
    }

    #[test]
    fn test_directions() {
        let one = NumericKey::One.position();
        let nine = NumericKey::Nine.position();

        assert_eq!(
            one.directions(&nine, false),
            vec![
                DirectionalKey::Right,
                DirectionalKey::Right,
                DirectionalKey::Up,
                DirectionalKey::Up
            ]
        );
    }
}

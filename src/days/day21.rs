use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let numeric_keys = parse_input(&input)?;

        let part1 = numeric_keys
            .iter()
            .map(|(i, k)| complexity(*i, k))
            .sum::<usize>()
            .to_string();
        let part2 = None;
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

fn complexity(numeric_value: usize, keys: &[NumericKey]) -> usize {
    let start = NumericKey::Activate.position();
    let next_options = inputs(start, keys);
    let next_options: HashSet<_> = next_options
        .into_iter()
        .flat_map(|next| inputs(DirectionalKey::Activate.position(), &next))
        .collect();
    let next_options: HashSet<_> = next_options
        .into_iter()
        .flat_map(|next| inputs(DirectionalKey::Activate.position(), &next))
        .collect();

    numeric_value * next_options.into_iter().map(|k| k.len()).min().unwrap_or(0)
}

fn print_directions(directions: &[DirectionalKey]) {
    println!(
        "{}",
        directions
            .iter()
            .map(|k| match k {
                DirectionalKey::Up => "^",
                DirectionalKey::Down => "v",
                DirectionalKey::Left => "<",
                DirectionalKey::Right => ">",
                DirectionalKey::Activate => "A",
            })
            .collect::<Vec<_>>()
            .join("")
    );
}

fn inputs<P: Position>(start: Coord, keys: &[P]) -> HashSet<Vec<DirectionalKey>>
where
    P: Debug,
{
    let mut results = HashSet::new();
    results.insert(vec![]);
    let mut current = start;

    for key in keys {
        let target = key.position();

        let a = current.directions(&target, true);
        let b = current.directions(&target, false);

        results = results
            .into_iter()
            .flat_map(move |r| {
                let current = current.clone();
                vec![a.clone(), b.clone()]
                    .into_iter()
                    .filter(move |a| P::is_valid(&current, a))
                    .map(move |l| {
                        let mut v = r.clone();
                        v.extend(l);
                        v.push(DirectionalKey::Activate);
                        v
                    })
            })
            .collect();

        current = target;
    }
    results
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
}

impl NumericKey {
    fn numeric_value(&self) -> usize {
        match self {
            NumericKey::Zero => 0,
            NumericKey::One => 1,
            NumericKey::Two => 2,
            NumericKey::Three => 3,
            NumericKey::Four => 4,
            NumericKey::Five => 5,
            NumericKey::Six => 6,
            NumericKey::Seven => 7,
            NumericKey::Eight => 8,
            NumericKey::Nine => 9,
            NumericKey::Activate => 0,
        }
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
                part2: None
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

// <<^A^^A>>AvvvA
// <<vAA>^A>A<AA>AvAA^A<vAAA>^A
// <<vAA>A>^AAvA<^A>AvA^A<<vA>>^AAvA^A<vA>^AA<A>A<<vA>A>^AAAvA<^A>A

// ^<<A^^A>>AvvvA
// <Av<AA^>>A<AA>AvAA^Av<AAA^>A
// v<<A^>>Av<A<A^>>AA<Av>AA^Av<<A^>>AAvA^Av<A^>AA<A>Av<A<A^>>AAA<Av>A^A

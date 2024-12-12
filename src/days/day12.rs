use std::{
    char,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let garden: Garden = input.parse()?;
        let part1 = garden.fencing_price(false).to_string();
        let part2 = Some(garden.fencing_price(true).to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Garden {
    plants: HashMap<Coord, char>,
}

impl FromStr for Garden {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut plants = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                plants.insert(
                    Coord {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                );
            }
        }
        Ok(Garden { plants })
    }
}

fn price(next: Coord, remaining: &mut HashMap<Coord, char>, bulk_discount: bool) -> usize {
    let mut stack = vec![next.clone()];
    let plant = remaining.get(&next).unwrap().clone();
    let mut area = 0;

    let mut visited = HashSet::new();
    visited.insert(next.clone());

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum EdgeDirection {
        HorizontalUp,
        HorizontalDown,
        VerticalLeft,
        VerticalRight,
    }

    let mut edges = Vec::new();

    while let Some(next) = stack.pop() {
        remaining.remove(&next);
        area += 1;
        for &(x, y, d) in [
            (-1, 0, EdgeDirection::VerticalLeft),
            (1, 0, EdgeDirection::VerticalRight),
            (0, -1, EdgeDirection::HorizontalDown),
            (0, 1, EdgeDirection::HorizontalUp),
        ]
        .iter()
        {
            if x == 0 && y == 0 {
                continue;
            }
            let coord = Coord {
                x: next.x + x,
                y: next.y + y,
            };
            if visited.contains(&coord) {
                continue;
            }

            if let Some(c) = remaining.get(&coord) {
                if *c == plant {
                    visited.insert(coord.clone());
                    stack.push(coord.clone());
                } else {
                    edges.push((coord.clone(), d));
                }
            } else {
                edges.push((coord.clone(), d));
            }
        }
    }

    if !bulk_discount {
        return edges.len() * area;
    }

    let mut edge_count = 0;
    while let Some((next, d)) = edges.pop() {
        edge_count += 1;
        let moves = if d == EdgeDirection::HorizontalUp || d == EdgeDirection::HorizontalDown {
            [(-1, 0), (1, 0)]
        } else {
            [(0, -1), (0, 1)]
        };
        for m in moves {
            let mut coord = Coord {
                x: next.x + m.0,
                y: next.y + m.1,
            };
            while let Some((i, _)) = edges
                .iter()
                .enumerate()
                .find(|(_, (c, dir))| c == &coord && &d == dir)
            {
                edges.remove(i);
                coord = Coord {
                    x: coord.x + m.0,
                    y: coord.y + m.1,
                };
            }
        }
    }

    if edge_count % 2 != 0 {
        dbg!(edge_count, next, plant);
    }

    edge_count * area
}

impl Garden {
    fn fencing_price(&self, bulk_discount: bool) -> usize {
        let mut remaining = self.plants.clone();
        let mut prices = Vec::new();

        while !remaining.is_empty() {
            let next = remaining.keys().next().unwrap();
            let price = price(next.clone(), &mut remaining, bulk_discount);
            prices.push(price);
        }

        prices.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "1930".to_owned(),
                part2: Some("1206".to_owned())
            })
        );
    }

    #[test]
    fn test_discount_u_shape() {
        let input = "AAAAA
ABABA
ABBBA
AAAAA"
            .to_owned();
        let mut garden: Garden = input.parse().unwrap();
        assert_eq!(price(Coord { x: 1, y: 1 }, &mut garden.plants, true), 40);
    }
}

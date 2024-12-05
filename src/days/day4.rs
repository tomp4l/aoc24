use std::{collections::HashMap, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let search = input.parse::<WordSearch>()?;

        let part1 = search.count_xmas().to_string();
        let part2 = Some(search.count_mas_x().to_string());
        Ok(DayResult { part1, part2 })
    }
}

struct WordSearch {
    grid: HashMap<(usize, usize), char>,
}

impl WordSearch {
    fn count_xmas(&self) -> usize {
        let mut count = 0;
        let word = "XMAS".chars().collect::<Vec<_>>();
        for &(x, y) in self.grid.keys() {
            for (dx, dy) in (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .filter(|(dx, dy)| *dx != 0 || *dy != 0)
            {
                let found = word.iter().enumerate().all(|(i, &c)| {
                    let x = x as isize + dx * i as isize;
                    let y = y as isize + dy * i as isize;
                    if x < 0 || y < 0 {
                        return false;
                    }
                    self.grid.get(&(x as usize, y as usize)) == Some(&c)
                });

                if found {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_mas_x(&self) -> usize {
        let mut count = 0;

        let searches = ["MAS", "SAM"]
            .into_iter()
            .map(|s| s.chars().enumerate().map(|(i, c)| (c, (i, i))))
            .flat_map(|s| {
                ["SM", "MS"].into_iter().map(move |s2| {
                    s.clone()
                        .chain(s2.chars().zip([(0, 2), (2, 0)]).map(|(c, i)| (c, i)))
                })
            });

        for &(x, y) in self.grid.keys() {
            for mut search in searches.clone() {
                let found = search.all(|(c, (dx, dy))| {
                    let x = x + dx;
                    let y = y + dy;

                    self.grid.get(&(x, y)) == Some(&c)
                });

                if found {
                    count += 1;
                }
            }
        }
        count
    }
}

impl FromStr for WordSearch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                grid.insert((x, y), char);
            }
        }
        Ok(WordSearch { grid })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_owned();
        let result = Instance.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "18".to_owned(),
                part2: Some("9".to_owned())
            })
        );
    }
}

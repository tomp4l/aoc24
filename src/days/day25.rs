use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let lock_keys = input
            .split("\n\n")
            .map(|s| s.parse::<LockKey>())
            .collect::<Result<Vec<_>, _>>()?;

        let part1 = count_matches(&lock_keys).to_string();
        let part2 = Some("Merry Christmas!".to_owned());
        Ok(DayResult { part1, part2 })
    }
}

fn count_matches(lock_keys: &[LockKey]) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for lk in lock_keys {
        match lk {
            LockKey::Lock(l) => locks.push(l),
            LockKey::Key(k) => keys.push(k),
        }
    }

    let mut count = 0;
    for key in keys {
        for lock in &locks {
            if key.fits(lock) {
                count += 1;
            }
        }
    }

    count
}

struct Lock {
    pins: [u8; 5],
}

struct Key {
    height: [u8; 5],
}

enum LockKey {
    Lock(Lock),
    Key(Key),
}

impl FromStr for Key {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height = [0; 5];

        for l in s.lines() {
            for (i, c) in l.chars().enumerate() {
                if c == '#' {
                    height[i] += 1;
                }
            }
        }

        Ok(Key { height })
    }
}

impl FromStr for Lock {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pins = [0; 5];

        for l in s.lines() {
            for (i, c) in l.chars().enumerate() {
                if c == '#' {
                    pins[i] += 1;
                }
            }
        }

        Ok(Lock { pins })
    }
}

impl FromStr for LockKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("#") {
            Ok(LockKey::Lock(s.parse()?))
        } else {
            Ok(LockKey::Key(s.parse()?))
        }
    }
}

impl Key {
    fn fits(&self, lock: &Lock) -> bool {
        for (k, l) in self.height.iter().zip(lock.pins) {
            if k + l > 7 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"
        .to_owned();
        assert_eq!(
            Instance.run(input),
            Ok(DayResult {
                part1: "3".to_owned(),
                part2: Some("Merry Christmas!".to_owned())
            })
        );
    }
}

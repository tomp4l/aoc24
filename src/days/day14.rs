use std::str::FromStr;

use super::day::*;

pub struct Instance {
    width: usize,
    height: usize,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            width: 101,
            height: 103,
        }
    }
}

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let mut robots: Robots = input.parse()?;
        robots.initialise(self);
        for _ in 0..100 {
            robots.step();
        }

        let part1 = robots.safety_factor().to_string();
        let mut part2 = None;

        for _ in 0..10000 {
            robots.step();

            if robots.overlap_count() == 0 {
                robots.print();
                println!();
                part2 = Some(robots.total_steps.to_string());
                break;
            }
        }
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug)]
struct Robots {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
    total_steps: usize,
}

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_vector(s: &str) -> Result<(i32, i32), String> {
    let mut parts = s[2..].split(',');
    let x = parts
        .next()
        .ok_or("no x")?
        .parse()
        .map_err(|e| format!("failed to parse x: {}", e))?;
    let y = parts
        .next()
        .ok_or("no y")?
        .parse()
        .map_err(|e| format!("failed to parse y: {}", e))?;
    Ok((x, y))
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let position = parse_vector(parts.next().ok_or("no position")?)?;
        let velocity = parse_vector(parts.next().ok_or("no velocity")?)?;
        Ok(Robot { position, velocity })
    }
}

impl FromStr for Robots {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let robots = s
            .lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Robots {
            robots,
            width: 0,
            height: 0,
            total_steps: 0,
        })
    }
}

impl Robots {
    fn initialise(&mut self, instance: &Instance) {
        self.width = instance.width;
        self.height = instance.height;
    }

    fn step(&mut self) {
        for robot in &mut self.robots {
            robot.position.0 += robot.velocity.0;
            robot.position.1 += robot.velocity.1;

            while robot.position.0 < 0 {
                robot.position.0 += self.width as i32;
            }
            while robot.position.0 >= self.width as i32 {
                robot.position.0 -= self.width as i32;
            }
            while robot.position.1 < 0 {
                robot.position.1 += self.height as i32;
            }
            while robot.position.1 >= self.height as i32 {
                robot.position.1 -= self.height as i32;
            }
        }
        self.total_steps += 1;
    }

    fn safety_factor(&self) -> usize {
        let x_size = self.width as i32 / 2;
        let y_size = self.height as i32 / 2;

        let mut counts = vec![0; 4];

        for robot in &self.robots {
            if robot.position.0 < x_size && robot.position.1 < y_size {
                counts[0] += 1;
            } else if robot.position.0 < x_size && robot.position.1 > y_size {
                counts[1] += 1;
            } else if robot.position.0 > x_size && robot.position.1 < y_size {
                counts[2] += 1;
            } else if robot.position.0 > x_size && robot.position.1 > y_size {
                counts[3] += 1;
            }
        }

        counts.iter().product()
    }

    fn overlap_count(&self) -> usize {
        let mut grid = vec![vec![0; self.width]; self.height];
        for robot in &self.robots {
            grid[robot.position.1 as usize][robot.position.0 as usize] += 1;
        }

        grid.iter()
            .map(|row| row.iter().filter(|&&i| i > 1).count())
            .sum()
    }

    fn print(&self) {
        let mut grid = vec![vec![0; self.width]; self.height];
        for robot in &self.robots {
            grid[robot.position.1 as usize][robot.position.0 as usize] += 1;
        }

        for row in grid {
            println!(
                "{}",
                row.iter()
                    .map(|i| if *i == 0 {
                        ".".to_owned()
                    } else {
                        i.to_string()
                    })
                    .collect::<String>()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
        .to_owned();
        let day = Instance {
            width: 11,
            height: 7,
        };
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "12".to_owned(),
                part2: Some("105".to_owned()) // it doesn't print a christams tree but oh well
            })
        );
    }
}

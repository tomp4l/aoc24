use std::{collections::HashSet, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let (w, i) = input
            .split_once("\n\n")
            .ok_or(format!("bad input: {}", input))?;
        let mut warehouse: Warehouse = w.parse()?;
        let mut wide_warehouse = WideWarehouse::from_warehouse(&warehouse);
        let instuctions: Instructions = i.parse()?;
        warehouse.apply_instructions(&instuctions);
        wide_warehouse.apply_instructions(&instuctions);

        let part1 = warehouse.gps().to_string();
        let part2 = Some(wide_warehouse.gps().to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Warehouse {
    boxes: HashSet<Coord>,
    walls: HashSet<Coord>,
    robot: Coord,
    width: usize,
    height: usize,
}

impl FromStr for Warehouse {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut boxes = HashSet::new();
        let mut walls = HashSet::new();
        let mut robot = None;
        let mut width = 0;
        let mut height = 0;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'O' => {
                        boxes.insert(Coord {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    '#' => {
                        walls.insert(Coord {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    '@' => {
                        robot = Some(Coord {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    '.' => {}
                    _ => return Err(format!("unexpected character '{}'", c)),
                }
                width = x;
            }
            height = y;
        }
        Ok(Warehouse {
            boxes,
            walls,
            robot: robot.ok_or("no robot")?,
            width: width + 1,
            height: height + 1,
        })
    }
}

#[derive(Debug)]
struct Instructions {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Instructions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| match c {
                '^' => Ok(Instruction::Up),
                'v' => Ok(Instruction::Down),
                '<' => Ok(Instruction::Left),
                '>' => Ok(Instruction::Right),
                _ => Err(format!("unexpected character '{}'", c)),
            })
            .collect::<Result<_, _>>()?;
        Ok(Instructions { instructions })
    }
}

impl Coord {
    fn apply(&self, i: &Instruction) -> Coord {
        match i {
            Instruction::Up => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Instruction::Down => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Instruction::Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Instruction::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl Warehouse {
    fn apply_instructions(&mut self, instructions: &Instructions) {
        for i in &instructions.instructions {
            let robot = self.robot.apply(i);
            if self.boxes.contains(&robot) {
                let mut box_ = robot.clone();
                while self.boxes.contains(&box_) {
                    box_ = box_.apply(i);
                }
                if !self.walls.contains(&box_) {
                    self.boxes.remove(&robot);
                    self.boxes.insert(box_);
                    self.robot = robot;
                }
            } else {
                if !self.walls.contains(&robot) {
                    self.robot = robot;
                }
            }
        }
    }

    fn gps(&self) -> usize {
        self.boxes
            .iter()
            .map(|b| b.x as usize + b.y as usize * 100)
            .sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.robot.x == x as i32 && self.robot.y == y as i32 {
                    '@'
                } else if self.boxes.contains(&Coord {
                    x: x as i32,
                    y: y as i32,
                }) {
                    'O'
                } else if self.walls.contains(&Coord {
                    x: x as i32,
                    y: y as i32,
                }) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

struct WideWarehouse {
    boxes: HashSet<Coord>,
    walls: HashSet<Coord>,
    robot: Coord,
    width: usize,
    height: usize,
}

impl WideWarehouse {
    fn from_warehouse(warehouse: &Warehouse) -> Self {
        let boxes = warehouse
            .boxes
            .iter()
            .map(|b| Coord { x: b.x * 2, y: b.y })
            .collect();
        let walls = warehouse
            .walls
            .iter()
            .flat_map(|w| {
                vec![
                    Coord { x: w.x * 2, y: w.y },
                    Coord {
                        x: w.x * 2 + 1,
                        y: w.y,
                    },
                ]
            })
            .collect();
        let robot = Coord {
            x: warehouse.robot.x * 2,
            y: warehouse.robot.y,
        };
        WideWarehouse {
            boxes,
            walls,
            robot,
            width: warehouse.width * 2,
            height: warehouse.height,
        }
    }

    fn apply_instructions(&mut self, instructions: &Instructions) {
        for i in &instructions.instructions {
            let robot = self.robot.apply(i);
            if let Some(hit) = self.hits_box(&robot) {
                let original_state = self.boxes.clone();

                let mut to_resolve = vec![hit.apply(i)];
                self.boxes.remove(&hit);
                let mut has_collision = false;
                while let Some(box_) = to_resolve.pop() {
                    if let Some(hit) = self.hits_box(&box_) {
                        self.boxes.remove(&hit);
                        to_resolve.push(hit.apply(i));
                    }
                    let right_side = Coord {
                        x: box_.x + 1,
                        y: box_.y,
                    };
                    if let Some(hit) = self.hits_box(&right_side) {
                        self.boxes.remove(&hit);
                        to_resolve.push(hit.apply(i));
                    }
                    if self.walls.contains(&box_) || self.walls.contains(&right_side) {
                        self.boxes = original_state;
                        has_collision = true;
                        break;
                    }
                    self.boxes.insert(box_);
                }
                if !has_collision {
                    self.robot = robot;
                }
            } else {
                if !self.walls.contains(&robot) {
                    self.robot = robot;
                }
            }
        }
    }

    fn hits_box(&self, c: &Coord) -> Option<Coord> {
        self.boxes
            .iter()
            .find(|coord| coord.y == c.y && (coord.x == c.x || coord.x == c.x - 1))
            .cloned()
    }

    fn gps(&self) -> usize {
        self.boxes
            .iter()
            .map(|b| b.x as usize + b.y as usize * 100)
            .sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut wide_box = false;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.robot.x == x as i32 && self.robot.y == y as i32 {
                    '@'
                } else if wide_box {
                    wide_box = false;
                    ']'
                } else if self.boxes.contains(&Coord {
                    x: x as i32,
                    y: y as i32,
                }) {
                    wide_box = true;
                    '['
                } else if self.walls.contains(&Coord {
                    x: x as i32,
                    y: y as i32,
                }) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
        .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "2028".to_owned(),
                part2: Some("1751".to_owned())
            })
        );
    }

    #[test]
    fn test() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
        .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "10092".to_owned(),
                part2: Some("9021".to_owned())
            })
        );
    }

    #[test]
    fn wide_exampe() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"
        .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "908".to_owned(),
                part2: Some("618".to_owned())
            })
        );
    }
}
